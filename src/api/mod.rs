//! Jellynizer API
//! 
//! This module implements the RESTful API using Axum to provide
//! endpoints for controlling the Jellynizer application.

use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::{Stream, StreamExt};
use crate::models::MediaFile;
use crate::scanner;
use crate::organizer::{self, FileMove, Operation};
use crate::metadata::MetadataClient;
use std::sync::{Arc, RwLock};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast;
use tracing::info;

/// Status of the organization process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OrganizeStatus {
    Idle,
    Running,
    Completed,
    Failed,
}

/// Progress information for the organization process.
#[derive(Debug, Clone, Serialize)]
pub struct OrganizeProgress {
    pub status: OrganizeStatus,
    pub current: usize,
    pub total: usize,
    pub current_file: Option<String>,
}

/// Shared application state for the API.
pub struct AppState {
    /// The directories to be scanned.
    pub source_dirs: Vec<PathBuf>,
    /// The destination directories (e.g., movies, shows).
    pub destination_dirs: std::collections::HashMap<String, PathBuf>,
    /// A list of media files discovered during the last scan.
    pub discovered_files: RwLock<Vec<MediaFile>>,
    /// Flag indicating if a scan is currently in progress.
    pub is_scanning: AtomicBool,
    /// Flag indicating if the application should run automatically.
    pub run_automatically: bool,
    /// The operation to perform (move or copy).
    pub operation: Operation,
    /// The current progress of the organization process.
    pub organize_progress: RwLock<OrganizeProgress>,
    /// Broadcast channel for organization progress updates.
    pub organize_tx: broadcast::Sender<OrganizeProgress>,
}

/// Router configuration for the Jellynizer API.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/scan", post(trigger_scan))
        .route("/preview", get(get_preview))
        .route("/execute", post(execute_organize))
        .route("/organize-status", get(get_organize_status))
        .route("/organize-events", get(organize_events))
        .route("/media", get(get_media))
        .route("/media/categorize", post(categorize_media))
        .route("/status", get(get_status))
        .with_state(state)
}

/// Endpoint to preview proposed organization changes.
async fn get_preview(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<FileMove>> {
    let files = state.discovered_files.read().unwrap();
    let moves = organizer::organize_files(&files, &state.destination_dirs, state.operation, true, |_, _, _| {});
    Json(moves)
}

use serde::Deserialize;

#[derive(Deserialize)]
struct ExecuteOrganizeRequest {
    sources: Vec<String>,
}

/// Endpoint to execute the proposed organization changes.
async fn execute_organize(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ExecuteOrganizeRequest>,
) -> Json<serde_json::Value> {
    let files_to_organize_paths = payload.sources;
    
    // We need to find the MediaFile objects that match these paths from the discovered_files
    let files_to_organize: Vec<MediaFile> = {
        let discovered = state.discovered_files.read().unwrap();
        discovered.iter()
            .filter(|f| files_to_organize_paths.contains(&f.path.to_string_lossy().into_owned()))
            .cloned()
            .collect()
    };

    if files_to_organize.is_empty() {
        return Json(serde_json::json!({
            "status": "error",
            "message": "No valid files selected for organization"
        }));
    }

    let moves_count = files_to_organize.len();

    // Initialize progress state
    {
        let mut progress = state.organize_progress.write().unwrap();
        *progress = OrganizeProgress {
            status: OrganizeStatus::Running,
            current: 0,
            total: moves_count,
            current_file: None,
        };
    }

    let state_clone = Arc::clone(&state);
    let dest_dirs_clone = state.destination_dirs.clone();
    let operation_clone = state.operation;
    let tx = state.organize_tx.clone();

    tokio::spawn(async move {
        let moves = organizer::organize_files(
            &files_to_organize,
            &dest_dirs_clone,
            operation_clone,
            false,
            |current, total, file_path| {
                let mut progress = state_clone.organize_progress.write().unwrap();
                progress.current = current;
                progress.total = total;
                progress.current_file = Some(file_path.to_string_lossy().into_owned());
                
                let _ = tx.send(progress.clone());
            },
        );

        let mut progress = state_clone.organize_progress.write().unwrap();
        progress.status = OrganizeStatus::Completed;
        let _ = tx.send(progress.clone());
    });

    Json(serde_json::json!({
        "status": "success",
        "message": "Organization initiated"
    }))
}

/// Endpoint to trigger a new directory scan.
async fn trigger_scan(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    // Check if a scan is already running to prevent concurrent scans
    if state.is_scanning.load(Ordering::SeqCst) {
        return Json(serde_json::json!({ "status": "error", "message": "Scan already in progress" }));
    }

    // Set scanning flag
    state.is_scanning.store(true, Ordering::SeqCst);
    
    // We spawn the scan in a background task so the API remains responsive
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        info!("Background scan started for {:?}", state_clone.source_dirs);
        
        let mut all_files = Vec::new();

        // 1. Perform the scan for each source directory
        for source_dir in &state_clone.source_dirs {
            let mut files = scanner::scan_directory(source_dir);
            
            // 2. Classify and Enrich
            for file in files.iter_mut() {
                organizer::classify_media(file, &MetadataClient::new(Some("dummy_key".to_string()))).await;
            }
            all_files.extend(files);
        }

        // 3. Update the shared state
        {
            let mut files_lock = state_clone.discovered_files.write().unwrap();
            
            // Filter out files that are already in their correct destination
            let mut candidates: Vec<MediaFile> = all_files
                .into_iter()
                .filter(|file| {
                    if let Some((dest_path, _)) = organizer::calculate_destination(file, &state_clone.destination_dirs) {
                        file.path != dest_path
                    } else {
                        true // If we can't calculate destination, keep it as a candidate
                    }
                })
                .collect();

            // If run_automatically is true, execute organization immediately
            if state_clone.run_automatically {
                info!("Run automatically is enabled. Executing organization...");
                let moves = organizer::organize_files(
                    &candidates,
                    &state_clone.destination_dirs,
                    state_clone.operation,
                    false,
                    |_, _, _| {},
                );
                info!("Automatic organization completed. {} files processed.", moves.len());

                // Remove the successfully moved files from the candidates list
                let moved_sources: std::collections::HashSet<PathBuf> = moves.iter().map(|m| m.source.clone()).collect();
                candidates.retain(|f| !moved_sources.contains(&f.path));
            }

            *files_lock = candidates;
        }

        state_clone.is_scanning.store(false, Ordering::SeqCst);
        info!("Background scan completed.");
    });

    Json(serde_json::json!({ "status": "success", "message": "Scan initiated" }))
}

/// Endpoint to retrieve the list of discovered media files.
async fn get_media(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<MediaFile>> {
    let files = state.discovered_files.read().unwrap();
    Json(files.clone())
}

#[derive(Deserialize)]
struct CategorizeMediaRequest {
    path: String,
    media_type: crate::models::MediaType,
    title: String,
    year: Option<u32>,
    season: Option<u32>,
    episode: Option<u32>,
}

/// Endpoint to manually categorize a file that automatic classification could not identify.
async fn categorize_media(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CategorizeMediaRequest>,
) -> Json<serde_json::Value> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Json(serde_json::json!({
            "status": "error",
            "message": "A title is required"
        }));
    }

    let (season, episode) = match payload.media_type {
        crate::models::MediaType::Movie => (None, None),
        crate::models::MediaType::TvShow => match (payload.season, payload.episode) {
            (Some(season), Some(episode)) if season > 0 && episode > 0 => {
                (Some(season), Some(episode))
            }
            _ => {
                return Json(serde_json::json!({
                    "status": "error",
                    "message": "A valid season and episode are required for TV shows"
                }));
            }
        },
        crate::models::MediaType::Unknown => {
            return Json(serde_json::json!({
                "status": "error",
                "message": "Choose Movie or TvShow"
            }));
        }
    };

    let mut files = state.discovered_files.write().unwrap();
    let Some(file) = files.iter_mut().find(|file| {
        file.path.to_string_lossy() == payload.path && file.media_type == crate::models::MediaType::Unknown
    }) else {
        return Json(serde_json::json!({
            "status": "error",
            "message": "Unknown media file not found"
        }));
    };

    file.media_type = payload.media_type;
    file.title = Some(title.to_owned());
    file.year = payload.year;
    file.season = season;
    file.episode = episode;

    Json(serde_json::json!({
        "status": "success",
        "message": "Media categorized"
    }))
}

/// Endpoint to check the current status of the application.
async fn get_status(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let source_dirs_str: Vec<String> = state.source_dirs.iter().map(|p| p.to_string_lossy().into_owned()).collect();
    let dest_dirs_str: std::collections::HashMap<String, String> = state.destination_dirs.iter()
        .map(|(k, v)| (k.clone(), v.to_string_lossy().into_owned()))
        .collect();

    Json(serde_json::json!({
        "is_scanning": state.is_scanning.load(Ordering::SeqCst),
        "source_dirs": source_dirs_str,
        "destination_dirs": dest_dirs_str
    }))
}

/// Endpoint to get the current organization progress.
async fn get_organize_status(
    State(state): State<Arc<AppState>>,
) -> Json<OrganizeProgress> {
    let progress = state.organize_progress.read().unwrap();
    Json(progress.clone())
}

/// SSE endpoint to stream organization progress events.
async fn organize_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let mut rx = state.organize_tx.subscribe();

    let stream = async_stream::stream! {
        while let Ok(progress) = rx.recv().await {
            yield Ok(Event::default().json_data(progress).unwrap());
        }
    };

    Sse::new(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MediaType;
    use std::collections::HashMap;

    #[tokio::test]
    async fn categorizing_unknown_file_sets_type_and_metadata() {
        let path = PathBuf::from("/source/episode.mkv");
        let (organize_tx, _) = broadcast::channel(10);
        let mut destination_dirs = HashMap::new();
        destination_dirs.insert("shows".to_string(), PathBuf::from("/shows"));
        let state = Arc::new(AppState {
            source_dirs: Vec::new(),
            destination_dirs,
            discovered_files: RwLock::new(vec![MediaFile::new(path.clone(), MediaType::Unknown)]),
            is_scanning: AtomicBool::new(false),
            run_automatically: false,
            operation: Operation::Move,
            organize_progress: RwLock::new(OrganizeProgress {
                status: OrganizeStatus::Idle,
                current: 0,
                total: 0,
                current_file: None,
            }),
            organize_tx,
        });

        let result = categorize_media(
            State(Arc::clone(&state)),
            Json(CategorizeMediaRequest {
                path: path.to_string_lossy().into_owned(),
                media_type: MediaType::TvShow,
                title: "Example Show".to_string(),
                year: Some(2024),
                season: Some(2),
                episode: Some(3),
            }),
        )
        .await;

        assert_eq!(result.0["status"], "success");
        let files = state.discovered_files.read().unwrap();
        let file = &files[0];
        assert_eq!(file.media_type, MediaType::TvShow);
        assert_eq!(file.title.as_deref(), Some("Example Show"));
        assert_eq!(file.year, Some(2024));
        assert_eq!(file.season, Some(2));
        assert_eq!(file.episode, Some(3));
        assert!(organizer::calculate_destination(file, &state.destination_dirs).is_some());
    }
}

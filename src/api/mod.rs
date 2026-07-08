//! Jellynizer API
//! 
//! This module implements the RESTful API using Axum to provide
//! endpoints for controlling the Jellynizer application.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use crate::models::MediaFile;
use crate::scanner;
use crate::organizer::{self, FileMove, Operation};
use crate::metadata::MetadataClient;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::info;

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
}

/// Router configuration for the Jellynizer API.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/scan", post(trigger_scan))
        .route("/preview", get(get_preview))
        .route("/execute", post(execute_organize))
        .route("/media", get(get_media))
        .route("/status", get(get_status))
        .with_state(state)
}

/// Endpoint to preview proposed organization changes.
async fn get_preview(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<FileMove>> {
    let files = state.discovered_files.read().unwrap();
    let moves = organizer::organize_files(&files, &state.destination_dirs, state.operation, true);
    Json(moves)
}

/// Endpoint to execute the proposed organization changes.
async fn execute_organize(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let files = state.discovered_files.read().unwrap();
    let moves = organizer::organize_files(&files, &state.destination_dirs, state.operation, false);
    
    Json(serde_json::json!({
        "status": "success",
        "moved_count": moves.len()
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

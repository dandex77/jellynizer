//! Jellynizer - Media Organization Tool
//! 
//! This is the main entry point for the Jellynizer application.
//! It initializes the logging system and starts the application logic.

use jellynizer::api;
use jellynizer::metadata;
use jellynizer::models;
use jellynizer::organizer;
use jellynizer::scanner;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use axum::Router;
use tower_http::services::ServeDir;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging with tracing-subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    tracing::info!("Starting Jellynizer...");

    // --- Configuration from Environment Variables ---

    // 1. Source Directories: Comma-separated list, default to current directory
    let source_dirs_raw = env::var("SOURCE_DIRS").unwrap_or_else(|_| ".".to_string());
    let source_dirs: Vec<PathBuf> = source_dirs_raw
        .split(',')
        .map(|s| {
            let p = PathBuf::from(s.trim());
            std::fs::canonicalize(&p).unwrap_or(p)
        })
        .collect();

    // 2. Destination Directories
    let mut destination_dirs = HashMap::new();
    let movies_dir = env::var("DEST_MOVIES").unwrap_or_else(|_| "./movies".to_string());
    let shows_dir = env::var("DEST_SHOWS").unwrap_or_else(|_| "./shows".to_string());
    
    destination_dirs.insert("movies".to_string(), std::fs::canonicalize(PathBuf::from(movies_dir.clone())).unwrap_or_else(|_| PathBuf::from(movies_dir)));
    destination_dirs.insert("shows".to_string(), std::fs::canonicalize(PathBuf::from(shows_dir.clone())).unwrap_or_else(|_| PathBuf::from(shows_dir)));

    // 3. Run Automatically
    let run_automatically = env::var("RUN_AUTOMATICALLY")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    // 4. Operation (move/copy)
    let operation = match env::var("OPERATION").unwrap_or_else(|_| "move".to_string()).to_lowercase().as_str() {
        "copy" => jellynizer::organizer::Operation::Copy,
        _ => jellynizer::organizer::Operation::Move,
    };

    tracing::info!("Configuration loaded:");
    tracing::info!("  Source directories: {:?}", source_dirs);
    tracing::info!("  Destination directories: {:?}", destination_dirs);
    tracing::info!("  Run automatically: {}", run_automatically);
    tracing::info!("  Operation: {:?}", operation);

    // --- API MODE LOGIC ---

    tracing::info!("Starting API Server on http://0.0.0.0:3000");
    let (organize_tx, _) = tokio::sync::broadcast::channel(100);
    let shared_state = Arc::new(api::AppState {
        source_dirs,
        destination_dirs,
        discovered_files: std::sync::RwLock::new(Vec::new()),
        is_scanning: std::sync::atomic::AtomicBool::new(false),
        run_automatically,
        operation,
        organize_progress: std::sync::RwLock::new(api::OrganizeProgress {
            status: api::OrganizeStatus::Idle,
            current: 0,
            total: 0,
            current_file: None,
        }),
        organize_tx,
    });

    let api_router = api::create_router(shared_state);

    // Combine API router with static file serving for the frontend
    // The frontend build output is in ./frontend/dist relative to the working directory
    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(ServeDir::new("frontend/dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

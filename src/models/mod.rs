//! Jellynizer Models
//! 
//! This module contains the core data structures used throughout the application
//! to represent media files and their metadata.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::Serialize;

/// Represents the type of media file identified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MediaType {
    /// A movie file.
    Movie,
    /// A TV show episode file.
    TvShow,
    /// An unknown or unsupported file type.
    Unknown,
}

/// Represents a single media file found during scanning.
#[derive(Debug, Clone, Serialize)]
pub struct MediaFile {
    /// The absolute path to the file.
    pub path: PathBuf,
    /// The type of media (Movie, TvShow, etc.).
    pub media_type: MediaType,
    /// The title of the media (e.g., "Inception").
    pub title: Option<String>,
    /// The year the media was released (if available).
    pub year: Option<u32>,
    /// The season number (for TV shows).
    pub season: Option<u32>,
    /// The episode number (for TV shows).
    pub episode: Option<u32>,
    /// Metadata identifiers (e.g., {"tmdbid": "680", "imdbid": "1234"}).
    pub identifiers: HashMap<String, String>,
}

impl MediaFile {
    /// Creates a new MediaFile with basic information.
    pub fn new(path: PathBuf, media_type: MediaType) -> Self {
        Self {
            path,
            media_type,
            title: None,
            year: None,
            season: None,
            episode: None,
            identifiers: HashMap::new(),
        }
    }
}

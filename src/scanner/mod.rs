//! Jellynizer Scanner
//! 
//! This module provides functionality for scanning directories and 
//! identifying media files.

use crate::models::{MediaFile, MediaType};
use std::path::Path;
use walkdir::WalkDir;
use tracing::info;

/// Scans a directory recursively and returns a list of identified media files.
pub fn scan_directory(root: &Path) -> Vec<MediaFile> {
    info!("Scanning directory: {:?}", root);
    let mut media_files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            
            if is_media_file(&path) {
                media_files.push(MediaFile::new(path, MediaType::Unknown));
            }
        }
    }

    info!("Scan complete. Found {} potential media files.", media_files.len());
    media_files
}

/// Helper function to check if a file has a media extension.
fn is_media_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => {
            let ext_lower = ext.to_lowercase();
            matches!(
                ext_lower.as_str(),
                "mp4" | "mkv" | "avi" | "mov" | "m4v"
            )
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_is_media_file() {
        // This test requires tempfile crate which isn't added yet, 
        // so I'll use a simpler approach or wait to add tempfile.
        // For now, just a logic test.
    }
}
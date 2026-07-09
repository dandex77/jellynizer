//! Jellynizer Scanner
//! 
//! This module provides functionality for scanning directories and 
//! identifying media files.

use crate::models::{MediaFile, MediaType};
use crate::utils::parser::extract_identifiers;
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
                 let mut media_file = MediaFile::new(path.clone(), MediaType::Unknown);
                 
                 // Extract identifiers from the file name
                 if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                     media_file.identifiers = extract_identifiers(file_name);
                 }
                 
                 // Check all parent directories for identifiers (useful for shows/folders)
                 let mut current_parent = path.parent();
                 while let Some(parent) = current_parent {
                     if let Some(parent_name) = parent.file_name().and_then(|s| s.to_str()) {
                         let parent_identifiers = extract_identifiers(parent_name);
                         for (k, v) in parent_identifiers {
                             media_file.identifiers.entry(k).or_insert(v);
                         }
                     }
                     current_parent = parent.parent();
                 }

                 media_files.push(media_file);
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
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_is_media_file() {
        assert!(is_media_file(Path::new("test.mp4")));
        assert!(is_media_file(Path::new("test.mkv")));
        assert!(!is_media_file(Path::new("test.txt")));
    }

    #[test]
    fn test_scan_directory_with_identifiers() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a directory with an identifier
        let show_dir = root.join("Series Name (2018) [tvdbid-79168]");
        fs::create_dir(&show_dir).unwrap();

        // Create a file within that directory with an identifier
        let file_path = show_dir.join("Series Name B S01E01 [tmdbid-680].mkv");
        File::create(&file_path).unwrap();

        let files = scan_directory(root);

        // We expect 1 media file
        assert_eq!(files.len(), 1);
        let file = &files[0];

        // Check if identifiers from both folder and file are captured
        // Folder identifier: tvdbid -> 79168
        // File identifier: tmdbid -> 680
        assert_eq!(file.identifiers.get("tvdbid").unwrap(), "79168");
        assert_eq!(file.identifiers.get("tmdbid").unwrap(), "680");
    }

    #[test]
    fn test_scan_directory_with_tags() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a directory with a single tag identifier
        let tag_dir = root.join("Movie Name [YTS.LT]");
        fs::create_dir(&tag_dir).unwrap();

        // Create a file within that directory
        let file_path = tag_dir.join("Movie.Name.mp4");
        File::create(&file_path).unwrap();

        let files = scan_directory(root);

        assert_eq!(files.len(), 1);
        let file = &files[0];

        // Check if the single tag is captured (identifier = tag)
        assert_eq!(file.identifiers.get("YTS.LT").unwrap(), "YTS.LT");
    }
}

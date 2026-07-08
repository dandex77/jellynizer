//! Organizer Utilities
//!
//! This module contains helper functions and regex patterns used for media classification.

use regex::Regex;
use std::path::Path;

lazy_static::lazy_static! {
    /// Robust TV Show pattern. Handles:
    /// - "Show Name - S01E01 - Title"
    /// - "Show.Name.S01E01.Title"
    /// - "Show Name S01E01 Title"
    pub static ref TV_SHOW_RE: Regex = Regex::new(
        r"(?P<title>[^()]+?)[ \s._-]+S(?P<season>\d{2})E(?P<episode>\d{2})[\s._-]*(?P<ep_title>.*)"
    ).unwrap();

    /// Robust Movie pattern. Handles:
    /// - "Movie Name (2019)"
    /// - "Movie.Name.2019"
    /// - "Movie Name 2019"
    pub static ref MOVIE_RE: Regex = Regex::new(
        r"(?P<title>[^()]+?)[ \s._-]+(?:\(?(?P<year>(?:19|20)\d{2})\)?)"
    ).unwrap();
}

/// Cleans up names by replacing dots/underscores with spaces.
pub fn clean_name(name: &str) -> String {
    let cleaned = name.replace('.', " ").replace('_', " ");
    // Remove extra whitespace
    let parts: Vec<&str> = cleaned.split_whitespace().collect();
    parts.join(" ")
}

/// Attempts to extract a meaningful title from the parent directory name.
/// Useful for cases like: "The Capture (2019) Season 1 S01 (1080p...)"
pub fn extract_title_from_parent(path: &Path) -> Option<String> {
    let parent = path.parent()?;
    let parent_name = parent.file_name()?.to_str()?;
    
    // Try to find a pattern in the parent name, e.g., "Show Name (Year)"
    if let Some(caps) = MOVIE_RE.captures(parent_name) {
        return caps.name("title").map(|m| clean_name(m.as_str()));
    }
    
    // Fallback: Just return the cleaned parent name
    if !parent_name.is_empty() {
        Some(clean_name(parent_name))
    } else {
        None
    }
}

/// Helper to check if a file is a companion of the media file.
pub fn is_companion(media_path: &Path, other_path: &Path) -> bool {
    let media_stem = media_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let other_stem = other_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    if media_stem.is_empty() || other_stem.is_empty() {
        return false;
    }

    // Check if they share the same cleaned stem
    clean_name(media_stem) == clean_name(other_stem)
}

/// Helper to check if two files have the same stem after cleaning.
pub fn has_same_stem(path1: &Path, path2: &Path) -> bool {
    let stem1 = path1.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let stem2 = path2.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    clean_name(stem1) == clean_name(stem2)
}
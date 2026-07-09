//! Jellynizer Organizer
//! 
//! This module contains the logic for identifying and classifying media files
//! using pattern matching and regex.

mod utils;

use crate::models::{MediaFile, MediaType};
use crate::metadata::MetadataProvider;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use crate::organizer::utils::{TV_SHOW_RE, MOVIE_RE, clean_name, extract_title_from_parent, is_companion, has_same_stem};

/// Refines the media type and metadata of a MediaFile based on its filename and parent directory context.
pub async fn classify_media<P: MetadataProvider>(media_file: &mut MediaFile, provider: &P) {
    let filename = match media_file.path.file_stem().and_then(|s| s.to_str()) {
        Some(f) => f,
        None => return,
    };

    // 1. Attempt TV Show classification via Regex
    if let Some(caps) = TV_SHOW_RE.captures(filename) {
        media_file.media_type = MediaType::TvShow;
        media_file.title = Some(clean_name(caps.name("title").map(|m| m.as_str()).unwrap_or("")));
        media_file.season = caps.name("season").and_then(|m| m.as_str().parse().ok());
        media_file.episode = caps.name("episode").and_then(|m| m.as_str().parse().ok());
        
        // If title is empty from regex, try to inherit from parent directory
        if media_file.title.as_ref().map_or(true, |t| t.is_empty()) {
            if let Some((title, year)) = extract_title_from_parent(&media_file.path) {
                media_file.title = Some(title);
                if year.is_some() {
                    media_file.year = year;
                }
            }
        }

        // 2. Enrich with metadata provider
        let _ = provider.enrich_media(media_file).await;

        info!("Classified as TV Show: {:?}", media_file.path);
        return;
    }

    // 2. Attempt Movie classification via Regex
    if let Some(caps) = MOVIE_RE.captures(filename) {
        media_file.media_type = MediaType::Movie;
        media_file.title = Some(clean_name(caps.name("title").map(|m| m.as_str()).unwrap_or("")));
        media_file.year = caps.name("year").and_then(|m| m.as_str().parse().ok());

        // If title is empty from regex, try to inherit from parent directory
        if media_file.title.as_ref().map_or(true, |t| t.is_empty()) {
            if let Some((title, year)) = extract_title_from_parent(&media_file.path) {
                media_file.title = Some(title);
                if year.is_some() {
                    media_file.year = year;
                }
            }
        }

        // 2. Enrich with metadata provider
        let _ = provider.enrich_media(media_file).await;

        info!("Classified as Movie: {:?}", media_file.path);
        return;
    }

    // 3. Fallback: Try to extract info from parent directory name if filename is just technical garbage
    if let Some((parent_title, parent_year)) = extract_title_from_parent(&media_file.path) {
        media_file.title = Some(parent_title);
        media_file.year = parent_year;
        // If we found a title in parent, we might still be "Unknown" until we can distinguish type,
        // but we at least have a title.
        info!("Found title from parent directory: {:?} -> {:?}", media_file.path, media_file.title);
        
        // 2. Enrich with metadata provider (even if fallback was used)
        let _ = provider.enrich_media(media_file).await;
    } else {
        warn!("Could not classify media file: {:?}", media_file.path);
    }
}

/// Represents the operation to perform on files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Operation {
    /// Move the file to the destination.
    Move,
    /// Copy the file to the destination.
    Copy,
}

/// Represents a proposed file movement/copy.
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileMove {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub relative_destination: PathBuf,
    pub identifiers: std::collections::HashMap<String, String>,
}

/// Organizes media files into the Jellyfin-compliant structure.
/// If `dry_run` is true, it only calculates the moves without performing them.
pub fn organize_files<F>(
    media_files: &[MediaFile], 
    destination_map: &std::collections::HashMap<String, PathBuf>, 
    operation: Operation,
    dry_run: bool,
    on_progress: F
) -> Vec<FileMove> 
where F: Fn(usize, usize, &PathBuf) {
    let mut moves = Vec::new();

    for (index, file) in media_files.iter().enumerate() {
        on_progress(index + 1, media_files.len(), &file.path);
        if let Some((dest_path, base_dir)) = calculate_destination(file, destination_map) {
            // 1. Handle the primary media file
            if file.path != dest_path {
                let relative_destination = dest_path.strip_prefix(&*base_dir)
                    .unwrap_or(&dest_path)
                    .to_path_buf();

                moves.push(FileMove {
                    source: file.path.clone(),
                    destination: dest_path.clone(),
                    relative_destination,
                    identifiers: file.identifiers.clone(),
                });
            }

            // 2. Handle companion files (subtitles, etc.)
            if let Some(parent_dir) = file.path.parent() {
                if let Ok(entries) = fs::read_dir(parent_dir) {
                    for entry in entries.flatten() {
                        let other_path = entry.path();
                        
                        // Skip if it's the media file itself or a directory
                        if other_path == file.path || other_path.is_dir() {
                            continue;
                        }

                        // Check if it's a companion
                        if is_companion(&file.path, &other_path) {
                            // Determine the destination for the companion
                            let dest_dir = dest_path.parent().unwrap_or(&base_dir);
                            
                            // Determine the new filename for the companion
                            // If it has the same stem as the media file, we use the media file's organized name but with the companion's extension
                            let extension = other_path.extension().and_then(|s| s.to_str()).unwrap_or("");
                            let new_file_name = if has_same_stem(&file.path, &other_path) {
                                // Use the organized filename of the media file, but with this file's extension
                                // Note: dest_path is the full path of the organized media file
                                let media_stem = dest_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                                format!("{}.{}", media_stem, extension)
                            } else {
                                // Fallerback to original filename
                                other_path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string()
                            };

                            let companion_dest = dest_dir.join(new_file_name);

                            if other_path != companion_dest {
                                let relative_destination = companion_dest.strip_prefix(&*base_dir)
                                    .unwrap_or(&companion_dest)
                                    .to_path_buf();

                                 moves.push(FileMove {
                                     source: other_path,
                                     destination: companion_dest,
                                     relative_destination,
                                     identifiers: file.identifiers.clone(),
                                 });
                             }
                         }
                     }
                 }
             }
         }
     }
 
     if !dry_run {
        for m in &moves {
            match operation {
                Operation::Move => {
                    info!("Executing move: {:?} -> {:?}", m.source, m.destination);
                    if let Some(parent) = m.destination.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    if let Err(e) = std::fs::rename(&m.source, &m.destination) {
                        if e.kind() == std::io::ErrorKind::CrossesDevices {
                            info!("Cross-device move detected for {:?}, falling back to copy and delete", m.source);
                            match std::fs::copy(&m.source, &m.destination) {
                                Ok(_) => {
                                    info!("Successfully copied {:?} to {:?} during cross-device move", m.source, m.destination);
                                    if let Err(remove_err) = std::fs::remove_file(&m.source) {
                                        warn!("Failed to remove source file after cross-device copy from {:?}: {}", m.source, remove_err);
                                    } else {
                                        info!("Successfully removed source file {:?} after cross-device move", m.source);
                                    }
                                }
                                Err(copy_err) => {
                                    warn!("Failed to copy file during cross-device move from {:?} to {:?}: {}", m.source, m.destination, copy_err);
                                }
                            }
                        } else {
                            warn!("Failed to move file from {:?} to {:?}: {}", m.source, m.destination, e);
                        }
                    }
                }
                Operation::Copy => {
                    info!("Executing copy: {:?} -> {:?}", m.source, m.destination);
                    if let Some(parent) = m.destination.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    if let Err(e) = std::fs::copy(&m.source, &m.destination) {
                        warn!("Failed to copy file from {:?} to {:?}: {}", m.source, m.destination, e);
                    }
                }
            }
        }
    }

    moves
}


/// Calculates the target Jellyfin-compliant path for a media file and the base directory it belongs to.
pub fn calculate_destination(file: &MediaFile, destination_map: &std::collections::HashMap<String, PathBuf>) -> Option<(PathBuf, PathBuf)> {
    let title = file.title.as_ref()?;
    let extension = file.path.extension()?.to_str()?;
    
    // Construct the tag string from identifiers, e.g., "[tmdbid-123][imdbid-456]"
    let mut tag_suffix = String::new();
    for (provider, id) in &file.identifiers {
        if id == provider {
            tag_suffix.push_str(&format!("[{}]", provider));
        } else {
            tag_suffix.push_str(&format!("[{}-{}]", provider, id));
        }
    }

    match file.media_type {
        MediaType::Movie => {
            let year_str = file.year.map(|y| y.to_string()).unwrap_or_default();
            let folder_name = if year_str.is_empty() {
                title.clone()
            } else {
                format!("{} ({})", title, year_str)
            };
            
            // Use 'movies' key from destination_map if available, otherwise fallback to current logic
            let fallback = PathBuf::from(".");
            let base_dir = destination_map.get("movies")
                .or_else(|| destination_map.get("Movies"))
                .unwrap_or(&fallback);

            let dest_dir = base_dir.join(&folder_name);
            let file_name = format!("{}{}.{}", folder_name, tag_suffix, extension);
            Some((dest_dir.join(file_name), base_dir.clone()))
        }
        MediaType::TvShow => {
            let season = file.season.unwrap_or(1);
            let episode = file.episode.unwrap_or(1);
            let year_str = file.year.map(|y| y.to_string()).unwrap_or_default();
            
            let series_folder = if year_str.is_empty() {
                title.clone()
            } else {
                format!("{} ({})", title, year_str)
            };
            let season_folder = format!("Season {:02}", season);
            let file_name = format!(
                "{} S{:02}E{:02}{}.{}",
                title, season, episode, tag_suffix, extension
            );

            // Use 'shows' key from destination_map if available, otherwise fallback to current logic
            let fallback = PathBuf::from(".");
            let base_dir = destination_map.get("shows")
                .or_else(|| destination_map.get("Shows"))
                .unwrap_or(&fallback);

            let dest_dir = base_dir
                .join(&series_folder)
                .join(season_folder);
            Some((dest_dir.join(file_name), base_dir.clone()))
        }
        MediaType::Unknown => None,
    }
}

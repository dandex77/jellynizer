//! Jellynizer Metadata
//! 
//! This module provides functionality to fetch enriched media metadata
//! from external web services (e.g., TMDB).

use crate::models::{MediaFile, MediaType};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use tracing::{error, info, warn};

use async_trait::async_trait;

/// Represents the response from a TMDB search request.
#[derive(Debug, Deserialize)]
struct TmdbSearchResult {
    results: Vec<TmdbResult>,
}

#[derive(Debug, Deserialize)]
struct TmdbResult {
    id: u32,
    title: Option<String>,
    name: Option<String>, // TV shows use 'name' instead of 'title'
    #[serde(rename = "release_date")]
    release_date: Option<String>,
    #[serde(rename = "first_air_date")]
    first_air_date: Option<String>,
}

/// Trait for providing media metadata.
#[async_trait]
pub trait MetadataProvider: Send + Sync {
    async fn enrich_media(&self, media_file: &mut MediaFile) -> Result<(), Box<dyn Error>>;
}

/// A client for interacting with external metadata providers.
pub struct MetadataClient {
    client: Client,
    api_key: Option<String>,
}

#[async_trait]
impl MetadataProvider for MetadataClient {
    /// Fetches metadata for a given media file using its current title and type.
    async fn enrich_media(&self, media_file: &mut MediaFile) -> Result<(), Box<dyn Error>> {
        info!("Enriching metadata for: {:?}", media_file.path);

        // 1. Try with API Key if provided
        if let Some(ref key) = self.api_key {
            // A. Try direct lookup if identifiers are available
            if let Some(result) = self.try_direct_lookup(media_file, key).await? {
                self.apply_tmdb_result(media_file, result);
                return Ok(());
            }

            // B. Fallback to search-based enrichment with API Key
            let title = match &media_file.title {
                Some(t) => t.clone(),
                None => return Ok(()),
            };

            if let Ok(enriched_result) = self.fetch_from_tmdb(&title, media_file.media_type, key).await {
                if let Some(result) = enriched_result {
                    self.apply_tmdb_result(media_file, result);
                    return Ok(());
                }
            } else {
                warn!("TMDB API request failed, falling back to search-based enrichment.");
            }
        }

        // 2. Fallback to Search-based enrichment (No API key required)
        if let Err(e) = self.enrich_via_search(media_file).await {
            error!("Search-based enrichment failed: {}", e);
        }

        Ok(())
    }
}

impl MetadataClient {
    /// Creates a new instance of the MetadataClient.
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    async fn fetch_from_tmdb(
        &self,
        title: &str,
        media_type: MediaType,
        api_key: &str,
    ) -> Result<Option<TmdbResult>, Box<dyn Error>> {
        let url = match media_type {
            MediaType::Movie => {
                format!(
                    "https://api.themoviedb.org/3/search/movie?api_key={}&query={}",
                    api_key, title
                )
            }
            MediaType::TvShow => {
                format!(
                    "https://api.themoviedb.org/3/search/tv?api_key={}&query={}",
                    api_key, title
                )
            }
            _ => return Ok(None),
        };

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let search_results: TmdbSearchResult = response.json().await?;
            return Ok(search_results.results.into_iter().next());
        }
        
        Ok(None)
    }

    /// Attempts a direct lookup using TMDB/IMDB identifiers.
    async fn try_direct_lookup(
        &self,
        media_file: &MediaFile,
        api_key: &str,
    ) -> Result<Option<TmdbResult>, Box<dyn Error>> {
        let media_type = media_file.media_type;

        // Check for TMDB ID
        if let Some(tmdb_id) = media_file.identifiers.get("tmdbid") {
            let url = match media_type {
                MediaType::Movie => format!("https://api.themoviedb.org/3/movie/{}?api_key={}", tmdb_id, api_key),
                MediaType::TvShow => format!("https://api.themoviedb.org/3/tv/{}?api_key={}", tmdb_id, api_key),
                _ => return Ok(None),
            };

            if let Ok(response) = self.client.get(&url).send().await {
                if response.status().is_success() {
                    if let Ok(result) = response.json::<TmdbResult>().await {
                        return Ok(Some(result));
                    }
                }
            }
        }

        // Check for IMDB ID (Note: TMDB API doesn't directly search by IMDB ID in the same way, 
        // but this is a placeholder for where we would add it if we had the endpoint/logic)
        // For now, we focus on TMDB ID which is what Jellyfin users typically use.

        Ok(None)
    }

    fn apply_tmdb_result(&self, media_file: &mut MediaFile, result: TmdbResult) {
        // Only update title if it's not already set or if it's empty
        if media_file.title.as_ref().map_or(true, |t| t.is_empty()) {
            if let Some(new_title) = result.title.clone().or(result.name.clone()) {
                media_file.title = Some(new_title);
            }
        }

        // Only update year if it's not already set
        if media_file.year.is_none() {
            let date_str = match media_file.media_type {
                MediaType::Movie => result.release_date.clone(),
                MediaType::TvShow => result.first_air_date.clone(),
                _ => None,
            };

            if let Some(date) = date_str {
                if let Some(year_str) = date.split('-').next() {
                    if let Ok(year) = year_str.parse::<u32>() {
                        media_file.year = Some(year);
                    }
                }
            }
        }
        info!("Successfully enriched metadata via TMDB for: {:?}", media_file.title);
    }

    /// Fallback enrichment: uses a simulated search/scraping approach.
    async fn enrich_via_search(&self, media_file: &mut MediaFile) -> Result<(), Box<dyn Error>> {
        info!("Attempting search-based enrichment for: {:?}", media_file.title);
        Ok(())
    }
}
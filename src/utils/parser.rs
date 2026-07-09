//! Jellynizer Parser
//! 
//! This module provides functionality for parsing media names to extract metadata.

use std::collections::HashMap;
use regex::Regex;

/// Extracts metadata identifiers from a given string.
/// Supports formats like `[provider-id]`, `[tag]`, or `.provider-id.`
pub fn extract_identifiers(name: &str) -> HashMap<String, String> {
    let mut identifiers = HashMap::new();
    
    // Blacklist of common technical terms to ignore to prevent false positives
    let blacklist = [
        "1080p", "720p", "480p", "bluray", "web-dl", "webrip", "brrip", 
        "x264", "x265", "h264", "h265", "amzn", "dcp", "remux", "dts", "ac3",
        "web", "dl", "rip", "hd", "sd", "repack", "mkv", "mp4", "avi", "mov",
        "5.1", "7.1", "subbed", "dubbed", "uncut"
    ];

    // This regex captures content within brackets ([tag]) or dot-separated provider-id pairs (.provider-id.).
    // This avoids mistakenly capturing plain words separated by dots.
    let re = Regex::new(r"\[([^\]]+)\]|(?:\.)([^.-]+-[^.-]+)(?:\.)").unwrap();

    for cap in re.captures_iter(name) {
        // Group 1 is for brackets, Group 2 is for dot-separated provider-id
        let full_tag = if let Some(t) = cap.get(1) {
            t.as_str().trim().to_string()
        } else if let Some(t) = cap.get(2) {
            t.as_str().trim().to_string()
        } else {
            continue
        };
        
        // Skip if the tag is empty
        if full_tag.is_empty() {
            continue;
        }

        // Determine if we have a provider-id pair or just a single tag.
        // We only split by '-' to support 'provider-id' format.
        // Dot-separated tags (like YTS.LT) are treated as single tags.
        let parts: Vec<&str> = if full_tag.contains('-') {
            full_tag.splitn(2, '-').collect()
        } else {
            vec![full_tag.as_str()]
        };

        if parts.len() == 2 {
            let provider = parts[0].to_string();
            let id = parts[1].to_string();
            
            // Skip if provider is blacklisted
            if blacklist.contains(&provider.to_lowercase().as_str()) {
                continue;
            }
            
            // Check if ID contains blacklisted terms
            let id_lower = id.to_lowercase();
            if blacklist.iter().any(|term| id_lower.contains(term)) {
                // If ID is invalid, try using provider as a single tag
                if !blacklist.contains(&provider.to_lowercase().as_str()) {
                    identifiers.insert(provider.clone(), provider.clone());
                }
            } else {
                identifiers.insert(provider, id);
            }
        } else {
            // Single tag case
            let tag = parts[0].to_string();
            // Check if the tag itself contains blacklisted terms
            if !blacklist.iter().any(|term| tag.to_lowercase().contains(term)) {
                identifiers.insert(tag.clone(), tag);
            }
        }
    }

    identifiers
}
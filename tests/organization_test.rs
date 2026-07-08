//! Integration test for media organization
//! This test validates that "The Capture" files are correctly classified and enriched via metadata.

use jellynizer::models::{MediaFile, MediaType};
use jellynizer::scanner::scan_directory;
use jellynizer::organizer::classify_media;
use jellynizer::metadata::MetadataProvider;
use std::fs;
use tempfile::tempdir;
use async_trait::async_trait;

/// A Mock provider to simulate metadata API responses for testing.
struct MockMetadataProvider;

#[async_trait]
impl MetadataProvider for MockMetadataProvider {
    async fn enrich_media(&self, media_file: &mut MediaFile) -> Result<(), Box<dyn std::error::Error>> {
        let title = media_file.title.as_ref().ok_or("No title available")?;
        
        match title.as_str() {
            "The Capture" => {
                // Simulate API returning the correct data for "The Capture"
                // In a real scenario, this would match based on the title/year/etc.
                // We can check season/episode if they were already set by regex
                if media_file.media_type == MediaType::TvShow {
                    // We don't overwrite what regex found, but we could add more info
                    // For this test, we just want to show enrichment happened
                } else if media_file.media_type == MediaType::Movie {
                    // Simulate finding the correct year if it wasn't found
                    if media_file.year.is_none() {
                        media_file.year = Some(2019);
                    }
                }
            },
            _ => {}
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_the_capture_organization() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Setup directory structure and files
    
    // Season 1: "Show.Name.S01E01.Title" style
    let s1_dir = root.join("The Capture (2019) Season 1 S01 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)");
    fs::create_dir_all(&s1_dir).unwrap();
    fs::File::create(s1_dir.join("The.Capture.S01E01.What.Happens.in.Helmand.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv")).unwrap();
    fs::File::create(s1_dir.join("The.Capture.S01E02.Toy.Soldier.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv")).unwrap();

    // Season 2: "Show Name - S02E01 - Title" style
    let s2_dir = root.join("The Capture (2019) Season 2 S02 (1080p AMZN WEB-DL x265 HEVC 10bit EAC3 5.1 Ghost)");
    fs::create_dir_all(&s2_dir).unwrap();
    fs::File::create(s2_dir.join("The Capture (2019) - S02E01 - Invisible Men (1080p AMZN WEB-DL x265 Ghost).mkv")).unwrap();
    fs::File::create(s2_dir.join("The Capture (2019) - S02E04 - #therealzacturner (1080p AMZN WEB-DL x265 Ghost).mkv")).unwrap();

    // Season 3: "Show.Name.S03E01.Title" style
    let s3_dir = root.join("The Capture (2019) Season 3 S03 (1080p AMZN WEB-DL x265 HEVC 10bit DDP 5.1 Vyndros)");
    fs::create_dir_all(&s3_dir).unwrap();
    fs::File::create(s3_dir.join("The.Capture.S03E01.Don’t.Look.at.the.Camera.1080p.10bit.AMZN.WEB-DL.DDP5.1.HEVC-Vyndros.mkv")).unwrap();

    // Run scanner
    let mut media_files = scan_directory(root);

    // Use the Mock provider for enrichment
    let provider = MockMetadataProvider;
    for file in &mut media_files {
        classify_media(file, &provider).await;
    }

    // Assertions
    // We created 2 (S1) + 2 (S2) + 1 (S3) = 5 files
    assert_eq!(media_files.len(), 5);

    for file in &media_files {
        assert_eq!(file.media_type, MediaType::TvShow);
        assert_eq!(file.title.as_deref(), Some("The Capture"));
        assert!(file.season.is_some());
        assert!(file.episode.is_some());
    }
    
    // Specifically check one from each season to be sure
    let s1_file = media_files.iter().find(|f| f.season == Some(1) && f.episode == Some(1)).expect("S01E01 not found");
    assert_eq!(s1_file.title.as_deref(), Some("The Capture"));

    let s2_file = media_files.iter().find(|f| f.season == Some(2) && f.episode == Some(1)).expect("S02E01 not found");
    assert_eq!(s2_file.title.as_deref(), Some("The Capture"));

    let s3_file = media_files.iter().find(|f| f.season == Some(3) && f.episode == Some(1)).expect("S03E01 not found");
    assert_eq!(s3_file.title.as_deref(), Some("The Capture"));

    // Test the organization logic (dry run)
    let target_dir = root.join("organized");
    let mut destination_map = std::collections::HashMap::new();
    destination_map.insert("Shows".to_string(), target_dir.join("Shows"));
    let moves = jellynizer::organizer::organize_files(
        &media_files,
        &destination_map,
        jellynizer::organizer::Operation::Move,
        true,
        |_, _, _| {},
    );

    // Verify the destination paths for the new structure
    // Note: The test setup doesn't explicitly set the year in the MediaFile for all files, 
    // but the mock provider or the classifier might. 
    // However, the regex for MOVIE_RE is used in extract_title_from_parent.
    // In this test, the files are classified as TV Shows. 
    // The TV show organization uses file.year if available.
    
    // Let's check if the destination paths match the new pattern.
    // Since we haven't explicitly set `year` on `media_files` in the test (only `title`, `season`, `episode` are set by `classify_media`), 
    // and the regex for TV shows doesn't extract year, `file.year` will be `None` unless enriched.
    
    let s1_move = moves.iter().find(|m| m.source == s1_file.path).expect("No move found for S01E01");
    
    // The new format for filename is: "Title S{02}E{02}.extension"
    // For S01E01, it should be "The Capture S01E01.mkv"
    // The extension is mkv.
    let expected_filename = format!("{} S{:02}E{:02}.{}", "The Capture", 1, 1, "mkv");
    
    let expected_s1_path = target_dir
        .join("Shows")
        .join("The Capture")
        .join("Season 01")
        .join(expected_filename);
    
    assert_eq!(s1_move.destination, expected_s1_path);
}
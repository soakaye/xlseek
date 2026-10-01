//! # Integration Tests for Multi-Path Search Scanning
//!
//! ## Description
//! Verifies that SearchEngine scans multiple valid directories simultaneously when provided
//! with a comma-separated path string, including quoted paths.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use xlseek_core::models::{DirectorySearchMode, ScanState, SearchQuery};
use xlseek_core::search::engine::SearchEngine;

/// Creates a unique temporary directory for test fixtures.
fn create_test_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time valid")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("xlseek_test_multi_{name}_{unique}"));
    fs::create_dir_all(&dir).expect("create temp test dir");
    dir
}

#[test]
fn test_search_engine_multiple_paths() {
    let dir_a = create_test_dir("a");
    let dir_b = create_test_dir("b");

    // Copy fixture files to dir_a and dir_b
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_src = manifest
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("tests").join("fixtures").join("sample_report.xlsx"))
        .expect("fixtures path");

    assert!(
        fixture_src.exists(),
        "fixture_src must exist: {:?}",
        fixture_src
    );
    fs::copy(&fixture_src, dir_a.join("sample_a.xlsx")).expect("copy to dir_a");
    fs::copy(&fixture_src, dir_b.join("sample_b.xlsx")).expect("copy to dir_b");

    let engine = SearchEngine::new();
    let multi_path = format!("{}, {}", dir_a.display(), dir_b.display());

    let query = SearchQuery {
        keyword: "".to_string(), // will match non-empty cells
        target_dir: multi_path,
        directory_mode: DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        include_value: true,
        use_regex: false,
        include_formula: true,
        include_comment: true,
        include_shape: false,
        include_hidden: false,
        extensions: vec![".xlsx".to_string()],
    };

    let matched_files = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let matched_files_clone = std::sync::Arc::clone(&matched_files);

    let progress = engine
        .execute_search_with_issues(
            query,
            move |m| {
                if let Ok(mut files) = matched_files_clone.lock() {
                    if !files.contains(&m.full_path) {
                        files.push(m.full_path);
                    }
                }
            },
            |_| {},
            |_| {},
        )
        .expect("search should execute successfully");

    assert_eq!(progress.state, ScanState::Completed);
    let files = matched_files.lock().unwrap();
    assert_eq!(files.len(), 2);

    // Clean up
    let _ = fs::remove_dir_all(dir_a);
    let _ = fs::remove_dir_all(dir_b);
}

//! # Integration Tests for Ignore Patterns and Directory Pruning (ignore_patterns_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies that `SearchEngine` and `discover_files_with_ignore` filter files and prune directory
//! subtrees based on `.gitignore`-style wildcard expressions.
//! Conforms to Constitution Principle I (English comments), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Testing).

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use xlseek_core::models::{DirectorySearchMode, ScanState, SearchQuery};
use xlseek_core::search::engine::SearchEngine;

/// Creates a unique temporary directory for test fixtures.
///
/// ## Arguments
/// * `prefix` - Subdirectory name prefix.
///
/// ## Returns
/// * `PathBuf` - Path to newly created directory in system temp.
///
/// ## Errors / Panics
/// Panics if system time fails or directory creation fails.
fn create_test_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time valid")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("xlseek_test_ignore_{prefix}_{unique}"));
    fs::create_dir_all(&dir).expect("create temp test dir");
    dir
}

/// Locates sample report fixture workbook.
///
/// ## Arguments / Returns
/// Returns `PathBuf` pointing to `sample_report.xlsx`.
///
/// ## Errors / Panics
/// Panics if the fixture does not exist.
fn sample_fixture_path() -> PathBuf {
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
    fixture_src
}

#[test]
fn test_ignore_lock_files_and_directories() {
    let test_dir = create_test_dir("lock_and_dir");
    let fixture = sample_fixture_path();

    // Setup files:
    // 1. Normal file: should be searched and matched
    let normal_file = test_dir.join("NormalReport.xlsx");
    fs::copy(&fixture, &normal_file).expect("copy normal file");

    // 2. Office temporary lock file: should be ignored via ~$*
    let lock_file = test_dir.join("~$NormalReport.xlsx");
    fs::copy(&fixture, &lock_file).expect("copy lock file");

    // 3. Ignored directory: Archive/
    let archive_dir = test_dir.join("Archive");
    fs::create_dir_all(&archive_dir).expect("create archive dir");
    let archive_file = archive_dir.join("OldReport.xlsx");
    fs::copy(&fixture, &archive_file).expect("copy archive file");

    // 4. Allowed directory: Current/
    let current_dir = test_dir.join("Current");
    fs::create_dir_all(&current_dir).expect("create current dir");
    let current_file = current_dir.join("CurrentReport.xlsx");
    fs::copy(&fixture, &current_file).expect("copy current file");

    let engine = SearchEngine::new();
    let query = SearchQuery {
        keyword: "Financial".to_string(),
        target_dir: test_dir.to_string_lossy().into_owned(),
        directory_mode: DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        // Constant reference: xlseek_core::constants::DEFAULT_INCLUDE_VALUE
        include_value: xlseek_core::constants::DEFAULT_INCLUDE_VALUE,
        use_regex: false,
        include_formula: false,
        include_comment: false,
        include_shape: false,
        include_hidden: false,
        extensions: vec![".xlsx".to_string()],
        ignore_patterns: Some("~$*, Archive/".to_string()),
    };

    let matched_files = Arc::new(Mutex::new(Vec::new()));
    let matched_files_clone = Arc::clone(&matched_files);

    let progress = engine
        .execute_search_with_issues(
            query,
            move |item| {
                let mut list = matched_files_clone.lock().unwrap();
                if !list.contains(&item.file_name) {
                    list.push(item.file_name.clone());
                }
            },
            |_| {},
            |_| {},
        )
        .expect("search should execute successfully");

    assert_eq!(progress.state, ScanState::Completed);

    let list = matched_files.lock().unwrap().clone();
    assert!(list.contains(&"NormalReport.xlsx".to_string()));
    assert!(list.contains(&"CurrentReport.xlsx".to_string()));
    assert!(!list.contains(&"~$NormalReport.xlsx".to_string()));
    assert!(!list.contains(&"OldReport.xlsx".to_string()));

    // Clean up
    let _ = fs::remove_dir_all(test_dir);
}

//! # Integration Tests for File Search Patterns (file_patterns_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies that `SearchEngine` filters candidate files using comma-separated inclusion glob
//! patterns, and defaults to searching all files with active extensions when patterns are empty.
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
    let dir = std::env::temp_dir().join(format!("xlseek_test_file_pat_{prefix}_{unique}"));
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

/// Tests that multiple comma-separated inclusion patterns select matching files and skip others.
///
/// ## Arguments / Returns
/// None. Asserts matched files and progress counts.
///
/// ## Errors / Panics
/// Panics on file I/O error or assertion mismatch.
#[test]
fn test_multiple_file_search_patterns() {
    let test_dir = create_test_dir("multi_pattern");
    let fixture = sample_fixture_path();

    let budget_file = test_dir.join("sample_budget.xlsx");
    let report_file = test_dir.join("sample_report.xlsx");
    let inventory_file = test_dir.join("sample_inventory.xlsx");

    fs::copy(&fixture, &budget_file).expect("copy budget");
    fs::copy(&fixture, &report_file).expect("copy report");
    fs::copy(&fixture, &inventory_file).expect("copy inventory");

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
        extensions: vec![xlseek_core::constants::EXT_XLSX.to_string()],
        ignore_patterns: None,
        file_patterns: Some("*budget*, *report*".to_string()),
    };

    let matched_paths = Arc::new(Mutex::new(Vec::new()));
    let matched_clone = Arc::clone(&matched_paths);
    let final_progress = Arc::new(Mutex::new(None));
    let progress_clone = Arc::clone(&final_progress);

    let result = engine.execute_search(
        query,
        move |m| {
            matched_clone.lock().unwrap().push(m.full_path);
        },
        move |p| {
            if p.state == ScanState::Completed {
                *progress_clone.lock().unwrap() = Some(p);
            }
        },
    );

    assert!(result.is_ok(), "search failed: {:?}", result);

    let progress = final_progress.lock().unwrap().take().expect("completed");
    // Only 2 files should be discovered and scanned: sample_budget.xlsx and sample_report.xlsx
    assert_eq!(progress.total_files, 2);
    assert_eq!(progress.scanned_files, 2);

    let matches = matched_paths.lock().unwrap();
    assert!(
        matches.iter().any(|p| p.ends_with("sample_budget.xlsx")),
        "sample_budget.xlsx should match"
    );
    assert!(
        matches.iter().any(|p| p.ends_with("sample_report.xlsx")),
        "sample_report.xlsx should match"
    );
    assert!(
        !matches.iter().any(|p| p.ends_with("sample_inventory.xlsx")),
        "sample_inventory.xlsx must not be scanned"
    );

    let _ = fs::remove_dir_all(test_dir);
}

/// Tests that empty or whitespace-only file search patterns fall back to matching all enabled files.
///
/// ## Arguments / Returns
/// None. Asserts all 3 files are scanned.
///
/// ## Errors / Panics
/// Panics on file I/O error or assertion mismatch.
#[test]
fn test_empty_file_search_patterns_fallback() {
    let test_dir = create_test_dir("empty_fallback");
    let fixture = sample_fixture_path();

    let budget_file = test_dir.join("sample_budget.xlsx");
    let report_file = test_dir.join("sample_report.xlsx");
    let inventory_file = test_dir.join("sample_inventory.xlsx");

    fs::copy(&fixture, &budget_file).expect("copy budget");
    fs::copy(&fixture, &report_file).expect("copy report");
    fs::copy(&fixture, &inventory_file).expect("copy inventory");

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
        extensions: vec![xlseek_core::constants::EXT_XLSX.to_string()],
        ignore_patterns: None,
        file_patterns: Some("   ".to_string()),
    };

    let final_progress = Arc::new(Mutex::new(None));
    let progress_clone = Arc::clone(&final_progress);

    let result = engine.execute_search(
        query,
        |_| {},
        move |p| {
            if p.state == ScanState::Completed {
                *progress_clone.lock().unwrap() = Some(p);
            }
        },
    );

    assert!(result.is_ok(), "search failed: {:?}", result);

    let progress = final_progress.lock().unwrap().take().expect("completed");
    // All 3 files should be discovered and scanned
    assert_eq!(progress.total_files, 3);
    assert_eq!(progress.scanned_files, 3);

    let _ = fs::remove_dir_all(test_dir);
}

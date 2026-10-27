//! # Integration Tests for Sequential Pattern Filtering (sequential_pattern_filter_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies the strict sequential evaluation of candidate files:
//! Step 1: Input file search pattern (inclusion filter)
//! Step 2: Ignore pattern (exclusion filter)
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
    let dir = std::env::temp_dir().join(format!("xlseek_test_seq_{prefix}_{unique}"));
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

/// Tests that a file matching include patterns is excluded if it matches ignore patterns.
///
/// ## Arguments / Returns
/// None. Asserts only the non-ignored budget file is scanned.
///
/// ## Errors / Panics
/// Panics on file I/O error or assertion mismatch.
#[test]
fn test_sequential_include_then_ignore_filtering() {
    let test_dir = create_test_dir("seq_filter");
    let fixture = sample_fixture_path();

    // 1. Included file: sample_budget.xlsx (matches *budget*, not ignored)
    let normal_budget = test_dir.join("sample_budget.xlsx");
    fs::copy(&fixture, &normal_budget).expect("copy normal budget");

    // 2. Archived budget file: archive/sample_budget.xlsx (matches *budget*, but excluded by archive/*)
    let archive_dir = test_dir.join("archive");
    fs::create_dir_all(&archive_dir).expect("create archive dir");
    let archive_budget = archive_dir.join("sample_budget.xlsx");
    fs::copy(&fixture, &archive_budget).expect("copy archive budget");

    // 3. Ignored temporary file: ~$sample_budget.xlsx (matches *budget*, but excluded by ~$*)
    let lock_budget = test_dir.join("~$sample_budget.xlsx");
    fs::copy(&fixture, &lock_budget).expect("copy lock budget");

    // 4. Non-matching file: sample_other.xlsx (does not match *budget*)
    let other_file = test_dir.join("sample_other.xlsx");
    fs::copy(&fixture, &other_file).expect("copy other file");

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
        ignore_patterns: Some("archive/*, ~$*".to_string()),
        file_patterns: Some("*budget*".to_string()),
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
    // Only 1 file should survive both filters: sample_budget.xlsx
    assert_eq!(progress.total_files, 1);
    assert_eq!(progress.scanned_files, 1);

    let matches = matched_paths.lock().unwrap();
    assert!(
        !matches.is_empty(),
        "expected matches in sample_budget.xlsx"
    );
    for path in matches.iter() {
        assert!(
            path.ends_with("sample_budget.xlsx")
                && !path.contains("archive")
                && !path.contains("~$"),
            "only top-level sample_budget.xlsx should match, got: {:?}",
            path
        );
    }

    let _ = fs::remove_dir_all(test_dir);
}

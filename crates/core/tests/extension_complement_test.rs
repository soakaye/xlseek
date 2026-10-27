//! # Integration Tests for Extension Complement and Override (extension_complement_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies extension complement for pattern tokens lacking extensions and explicit extension
//! precedence overriding unchecked extension toggles for supported document formats.
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
    let dir = std::env::temp_dir().join(format!("xlseek_test_ext_{prefix}_{unique}"));
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

/// Tests that extensionless patterns are complemented with active extension options.
///
/// ## Arguments / Returns
/// None. Asserts both .xlsx and .xlsm files matching the prefix are scanned.
///
/// ## Errors / Panics
/// Panics on file I/O error or assertion mismatch.
#[test]
fn test_extensionless_pattern_complemented_with_active_extensions() {
    let test_dir = create_test_dir("ext_complement");
    let fixture = sample_fixture_path();

    let budget_xlsx = test_dir.join("budget_2024.xlsx");
    let budget_xlsm = test_dir.join("budget_macro.xlsm");
    let report_xlsx = test_dir.join("report.xlsx");

    fs::copy(&fixture, &budget_xlsx).expect("copy xlsx");
    fs::copy(&fixture, &budget_xlsm).expect("copy xlsm");
    fs::copy(&fixture, &report_xlsx).expect("copy report");

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
        extensions: vec![
            xlseek_core::constants::EXT_XLSX.to_string(),
            xlseek_core::constants::EXT_XLSM.to_string(),
        ],
        ignore_patterns: None,
        file_patterns: Some("budget*".to_string()),
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
    // Both budget_2024.xlsx and budget_macro.xlsm should be scanned
    assert_eq!(progress.total_files, 2);
    assert_eq!(progress.scanned_files, 2);

    let matches = matched_paths.lock().unwrap();
    assert!(
        matches.iter().any(|p| p.ends_with("budget_2024.xlsx")),
        "budget_2024.xlsx should match"
    );
    assert!(
        matches.iter().any(|p| p.ends_with("budget_macro.xlsm")),
        "budget_macro.xlsm should match"
    );
    assert!(
        !matches.iter().any(|p| p.ends_with("report.xlsx")),
        "report.xlsx must not be scanned"
    );

    let _ = fs::remove_dir_all(test_dir);
}

/// Tests that explicit pattern extensions override unchecked extension options for supported formats.
///
/// ## Arguments / Returns
/// None. Asserts legacy file is matched even when active options only specify .xlsx.
///
/// ## Errors / Panics
/// Panics on file I/O error or assertion mismatch.
#[test]
fn test_explicit_extension_overrides_unchecked_options() {
    let test_dir = create_test_dir("ext_override");
    let fixture = sample_fixture_path();

    // Copy sample fixture as .xlsm
    let macro_file = test_dir.join("quarterly.xlsm");
    let regular_file = test_dir.join("quarterly.xlsx");

    fs::copy(&fixture, &macro_file).expect("copy xlsm");
    fs::copy(&fixture, &regular_file).expect("copy xlsx");

    let engine = SearchEngine::new();
    // Options only contain .xlsx, but pattern explicitly specifies *.xlsm
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
        file_patterns: Some("*.xlsm".to_string()),
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
    // Explicit pattern *.xlsm should match quarterly.xlsm despite .xlsm being omitted from extensions
    assert_eq!(progress.total_files, 1);
    assert_eq!(progress.scanned_files, 1);

    let matches = matched_paths.lock().unwrap();
    assert!(
        matches.iter().any(|p| p.ends_with("quarterly.xlsm")),
        "quarterly.xlsm should match due to explicit extension override"
    );

    let _ = fs::remove_dir_all(test_dir);
}

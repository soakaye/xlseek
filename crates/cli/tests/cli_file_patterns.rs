//! # CLI File Search Patterns Integration Tests (cli_file_patterns.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies that `xlseek-cli` respects `-g` and `--file-patterns` flags, filtering
//! candidate files to only those matching the specified inclusion glob patterns.
//! Conforms to Constitution Principle I (English comments), Principle II (No Hardcoded Constants),
//! and Principle III (Comprehensive Header Comments).

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const TEST_DIR_PREFIX: &str = "xlseek-cli-file-pat";
const TEST_DIR_SEP: &str = "-";
static TEST_SEQ: AtomicUsize = AtomicUsize::new(0);

/// Generates a unique temporary directory for isolation.
///
/// ## Arguments / Returns
/// Returns a unique `PathBuf` within system temp directory.
///
/// ## Errors / Panics
/// Does not panic.
fn unique_test_directory() -> PathBuf {
    let seq = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "{TEST_DIR_PREFIX}{TEST_DIR_SEP}{}{TEST_DIR_SEP}{seq}",
        std::process::id()
    ))
}

/// Resolves path to sample report fixture.
///
/// ## Arguments / Returns
/// Returns `PathBuf` to `sample_report.xlsx`.
///
/// ## Errors / Panics
/// Panics if fixture path cannot be found.
fn sample_fixture_path() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture = manifest
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("tests").join("fixtures").join("sample_report.xlsx"))
        .expect("fixture path");
    assert!(fixture.exists(), "fixture must exist: {:?}", fixture);
    fixture
}

#[test]
fn test_cli_short_file_patterns_option() {
    let test_dir = unique_test_directory();
    fs::create_dir_all(&test_dir).expect("create test dir");
    let fixture = sample_fixture_path();

    let budget = test_dir.join("sample_budget.xlsx");
    let report = test_dir.join("sample_report.xlsx");
    let inventory = test_dir.join("sample_inventory.xlsx");

    fs::copy(&fixture, &budget).expect("copy budget");
    fs::copy(&fixture, &report).expect("copy report");
    fs::copy(&fixture, &inventory).expect("copy inventory");

    let cli_bin = env!("CARGO_BIN_EXE_xlseek-cli");
    let output = Command::new(cli_bin)
        .arg("-p")
        .arg(&test_dir)
        .arg("-q")
        .arg("Financial")
        .arg("-g")
        .arg("*budget*, *report*")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // sample_budget and sample_report must be found
    assert!(stdout.contains("sample_budget.xlsx"));
    assert!(stdout.contains("sample_report.xlsx"));
    // sample_inventory must be excluded
    assert!(!stdout.contains("sample_inventory.xlsx"));

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_cli_long_file_patterns_option() {
    let test_dir = unique_test_directory();
    fs::create_dir_all(&test_dir).expect("create test dir");
    let fixture = sample_fixture_path();

    let budget = test_dir.join("sample_budget.xlsx");
    let report = test_dir.join("sample_report.xlsx");

    fs::copy(&fixture, &budget).expect("copy budget");
    fs::copy(&fixture, &report).expect("copy report");

    let cli_bin = env!("CARGO_BIN_EXE_xlseek-cli");
    let output = Command::new(cli_bin)
        .arg("-p")
        .arg(&test_dir)
        .arg("-q")
        .arg("Financial")
        .arg("--file-patterns")
        .arg("*budget*")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("sample_budget.xlsx"));
    assert!(!stdout.contains("sample_report.xlsx"));

    let _ = fs::remove_dir_all(test_dir);
}

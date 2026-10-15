//! # CLI Ignore Patterns Integration Tests (cli_ignore_patterns.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies that `xlseek-cli` respects `-i` and `--ignore-patterns` flags, filtering
//! out temporary lock files and pruning excluded directories from search results.
//! Conforms to Constitution Principle I (English comments), Principle II (No Hardcoded Constants),
//! and Principle III (Comprehensive Header Comments).

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const TEST_DIR_PREFIX: &str = "xlseek-cli-ignore";
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
fn test_cli_short_ignore_patterns_option() {
    let test_dir = unique_test_directory();
    fs::create_dir_all(&test_dir).expect("create test dir");
    let fixture = sample_fixture_path();

    // Normal file
    let normal = test_dir.join("TargetReport.xlsx");
    fs::copy(&fixture, &normal).expect("copy normal");

    // Lock file
    let lock = test_dir.join("~$TargetReport.xlsx");
    fs::copy(&fixture, &lock).expect("copy lock");

    // Excluded directory
    let backup_dir = test_dir.join("Backup");
    fs::create_dir_all(&backup_dir).expect("create backup dir");
    fs::copy(&fixture, backup_dir.join("BackupReport.xlsx")).expect("copy backup");

    let cli_bin = env!("CARGO_BIN_EXE_xlseek-cli");
    let output = Command::new(cli_bin)
        .arg("-p")
        .arg(&test_dir)
        .arg("-q")
        .arg("Financial")
        .arg("-i")
        .arg("~$*, Backup/")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // TargetReport must be found
    assert!(stdout.contains("TargetReport.xlsx"));
    // ~$TargetReport and Backup must be excluded
    assert!(!stdout.contains("~$TargetReport.xlsx"));
    assert!(!stdout.contains("BackupReport.xlsx"));

    let _ = fs::remove_dir_all(test_dir);
}

#[test]
fn test_cli_long_ignore_patterns_option() {
    let test_dir = unique_test_directory();
    fs::create_dir_all(&test_dir).expect("create test dir");
    let fixture = sample_fixture_path();

    let normal = test_dir.join("Budget.xlsx");
    fs::copy(&fixture, &normal).expect("copy normal");
    let lock = test_dir.join("~$Budget.xlsx");
    fs::copy(&fixture, &lock).expect("copy lock");

    let cli_bin = env!("CARGO_BIN_EXE_xlseek-cli");
    let output = Command::new(cli_bin)
        .arg("--path")
        .arg(&test_dir)
        .arg("--query")
        .arg("Financial")
        .arg("--ignore-patterns=~$*")
        .output()
        .expect("run cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Budget.xlsx"));
    assert!(!stdout.contains("~$Budget.xlsx"));

    let _ = fs::remove_dir_all(test_dir);
}

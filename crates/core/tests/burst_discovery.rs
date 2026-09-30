//! Purpose: Verifies nested file discovery, mode parity, and the configured worker bound.
//! Inputs: Temporary directory trees and discovery settings; output: assertions over delivered paths.
//! Errors: Filesystem setup failures fail the test; discovery failures are asserted through `Result`.

use exlgrep_core::models::DirectorySearchMode;
use exlgrep_core::search::discovery::discover_files;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Creates an isolated nested fixture and returns its root and cleanup path.
///
/// ## Arguments / Returns
/// No arguments; returns `(root, cleanup_path)` for the test directory tree.
///
/// ## Errors
/// Panics if temporary directories or files cannot be created.
fn nested_fixture() -> (PathBuf, PathBuf) {
    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "xlseek-burst-{}-{}-{unique_id}",
        std::process::id(),
        exlgrep_core::constants::CLI_TEST_BURST_FIXTURE_ID
    ));
    fs::create_dir_all(root.join("one/deep")).unwrap();
    fs::create_dir_all(root.join("two")).unwrap();
    fs::write(root.join("root.xlsx"), []).unwrap();
    fs::write(root.join("one/first.xlsx"), []).unwrap();
    fs::write(root.join("one/deep/deep.xlsx"), []).unwrap();
    fs::write(root.join("two/second.xlsx"), []).unwrap();
    (root.clone(), root)
}

/// Confirms Burst discovery overlaps file delivery without exceeding its worker limit.
///
/// ## Arguments / Returns
/// No arguments; returns `()` after asserting observed callback concurrency.
///
/// ## Errors
/// Panics if fixture creation, discovery, or concurrency assertions fail.
#[test]
fn burst_discovery_respects_the_configured_worker_bound() {
    let (root, cleanup) = nested_fixture();
    for index in 0..exlgrep_core::constants::CLI_TEST_BURST_DIRECTORY_COUNT {
        let directory = root.join(format!("parallel-{index}"));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("book.xlsx"), []).unwrap();
    }
    let active = Arc::new(AtomicUsize::new(0));
    let maximum = Arc::new(AtomicUsize::new(0));
    let callback_active = Arc::clone(&active);
    let callback_maximum = Arc::clone(&maximum);
    discover_files(
        std::slice::from_ref(&root),
        &[".xlsx".to_string()],
        DirectorySearchMode::Burst,
        Some(exlgrep_core::constants::BURST_WORKERS_MIN),
        &AtomicBool::new(false),
        move |_| {
            let current = callback_active.fetch_add(1, Ordering::SeqCst) + 1;
            callback_maximum.fetch_max(current, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(
                exlgrep_core::constants::CLI_TEST_BURST_CALLBACK_DELAY_MS,
            ));
            callback_active.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        },
        |_| {},
    )
    .unwrap();
    assert!(
        maximum.load(Ordering::SeqCst) >= exlgrep_core::constants::CLI_TEST_BURST_MIN_CONCURRENCY
    );
    assert!(maximum.load(Ordering::SeqCst) <= exlgrep_core::constants::BURST_WORKERS_MIN);
    fs::remove_dir_all(cleanup).unwrap();
}

/// Confirms discovery stops after its cancellation flag is set by an in-flight file callback.
///
/// ## Arguments / Returns
/// No arguments; returns `()` after checking callback delivery remains bounded.
///
/// ## Errors
/// Panics if discovery errors or continues through the full fixture after cancellation.
#[test]
fn burst_discovery_stops_scheduling_after_cancellation() {
    let (root, cleanup) = nested_fixture();
    for index in 0..exlgrep_core::constants::CLI_TEST_BURST_DIRECTORY_COUNT {
        let directory = root.join(format!("cancel-{index}"));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("book.xlsx"), []).unwrap();
    }
    let cancelled = Arc::new(AtomicBool::new(false));
    let callback_cancelled = Arc::clone(&cancelled);
    let delivered = Arc::new(AtomicUsize::new(0));
    let callback_delivered = Arc::clone(&delivered);
    discover_files(
        std::slice::from_ref(&root),
        &[".xlsx".to_string()],
        DirectorySearchMode::Burst,
        Some(exlgrep_core::constants::BURST_WORKERS_MIN),
        cancelled.as_ref(),
        move |_| {
            callback_delivered.fetch_add(1, Ordering::SeqCst);
            callback_cancelled.store(true, Ordering::Release);
            Ok(())
        },
        |_| {},
    )
    .unwrap();
    assert!(delivered.load(Ordering::SeqCst) <= exlgrep_core::constants::BURST_WORKERS_MIN);
    assert!(
        delivered.load(Ordering::SeqCst) < exlgrep_core::constants::CLI_TEST_BURST_DIRECTORY_COUNT
    );
    fs::remove_dir_all(cleanup).unwrap();
}

/// Checks that both discovery modes return every nested workbook exactly once.
///
/// ## Arguments / Returns
/// No arguments; returns `()` after comparing sorted file paths.
///
/// ## Errors
/// Panics if fixture creation or discovery fails or the file sets differ.
#[test]
fn sequential_and_burst_discover_the_same_nested_files_once() {
    let (root, cleanup) = nested_fixture();
    let extensions = vec![".xlsx".to_string()];
    let sequential = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    discover_files(
        std::slice::from_ref(&root),
        &extensions,
        DirectorySearchMode::Sequential,
        None,
        &AtomicBool::new(false),
        |path| {
            sequential.lock().unwrap().push(path.to_path_buf());
            Ok(())
        },
        |_| {},
    )
    .unwrap();
    let burst = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    discover_files(
        std::slice::from_ref(&root),
        &extensions,
        DirectorySearchMode::Burst,
        Some(exlgrep_core::constants::BURST_WORKERS_MIN),
        &AtomicBool::new(false),
        |path| {
            burst.lock().unwrap().push(path.to_path_buf());
            Ok(())
        },
        |_| {},
    )
    .unwrap();
    let mut sequential = Arc::try_unwrap(sequential).unwrap().into_inner().unwrap();
    let mut burst = Arc::try_unwrap(burst).unwrap().into_inner().unwrap();
    sequential.sort();
    burst.sort();
    assert_eq!(sequential, burst);
    assert_eq!(
        sequential.len(),
        exlgrep_core::constants::CLI_TEST_NESTED_FILE_COUNT
    );
    fs::remove_dir_all(cleanup).unwrap();
}

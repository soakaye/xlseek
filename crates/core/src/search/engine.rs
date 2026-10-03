//! # Search Engine Core Module (search/engine.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Recursively scans Excel files under a specified directory and executes high-speed traversal
//! using Rayon multi-threaded parallel processing. Provides throttled progress notifications,
//! cancellation control, and panic safety.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{ScanProgress, ScanState, SearchMatch, SearchQuery};
use crate::search::parser::parse_and_search_file;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// ## Description
/// Search engine struct managing parallel search scans and cancellation control.
pub struct SearchEngine {
    is_cancelled: Arc<AtomicBool>,
}

impl Default for SearchEngine {
    /// ## Description
    /// Creates a default instance of the search engine.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// - `Self`: Initialized search engine
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    /// ## Description
    /// Creates a new search engine instance.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// - `Self`: Search engine with unflagged cancellation state
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    pub fn new() -> Self {
        Self {
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// ## Description
    /// Retrieves an atomic reference to the cancellation flag.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// - `Arc<AtomicBool>`: Reference to the cancellation flag
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    pub fn get_cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.is_cancelled)
    }

    /// ## Description
    /// Requests cancellation of the running scan process.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }

    /// ## Description
    /// Resets the cancellation flag, preparing the engine for a new scan execution.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    pub fn reset_cancel(&self) {
        self.is_cancelled.store(false, Ordering::Relaxed);
    }

    /// ## Description
    /// Recursively scans target directory and collects file paths matching specified extensions.
    /// Temporary files (e.g. Excel lock files starting with `~$`) are excluded.
    /// When cancellation flag is provided, checks cancellation status on each iteration and aborts immediately if signalled.
    ///
    /// ## Arguments
    /// - `target_dir`: `&str` - Target directory path
    /// - `extensions`: `&[String]` - List of file extensions to include (with leading dot)
    /// - `cancel_flag`: `Option<&AtomicBool>` - Optional atomic cancellation flag
    ///
    /// ## Returns
    /// - `Vec<PathBuf>`: List of collected file paths
    ///
    /// ## Errors / Exceptions
    /// Ignores individual directory read errors and continues scan. Does not panic.
    pub fn collect_files(
        target_dir: &str,
        extensions: &[String],
        cancel_flag: Option<&AtomicBool>,
    ) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let exts_lower: Vec<String> = extensions.iter().map(|e| e.to_lowercase()).collect();

        for entry in WalkDir::new(target_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Some(flag) = cancel_flag {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
            }

            if entry.file_type().is_file() {
                let path = entry.path();
                // Exclude temporary files (e.g. ~$ Excel lock files)
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Constant reference: crate::constants::EXCEL_TEMP_FILE_PREFIX
                    if file_name.starts_with(crate::constants::EXCEL_TEMP_FILE_PREFIX) {
                        continue;
                    }
                }

                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_with_dot = format!(".{}", ext.to_lowercase());
                    if exts_lower.contains(&ext_with_dot) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        files
    }

    /// ## Description
    /// Executes parallel producer-consumer pipeline search using bounded synchronous channels based on query criteria.
    /// Emits matches via callbacks and throttles progress notifications (at least 50ms interval).
    ///
    /// ## Arguments
    /// - `query`: `SearchQuery` - Search criteria including keyword, directory, and extensions
    /// - `on_match`: `FMatch` - Callback receiving each matched search item
    /// - `on_progress`: `FProgress` - Callback receiving scan progress updates
    ///
    /// ## Returns
    /// - `Result<ScanProgress, String>`: Final scan progress state or error string
    ///
    /// ## Errors / Exceptions
    /// - Returns `Err` if target directory does not exist or regex is invalid.
    /// - Catches panics from individual corrupted files using `catch_unwind` and skips them safely.
    pub fn execute_search<FMatch, FProgress>(
        &self,
        query: SearchQuery,
        on_match: FMatch,
        on_progress: FProgress,
    ) -> Result<ScanProgress, String>
    where
        FMatch: FnMut(SearchMatch) + Send + Sync + 'static,
        FProgress: FnMut(ScanProgress) + Send + Sync + 'static,
    {
        self.execute_search_with_issues(query, on_match, on_progress, |_| {})
    }

    /// Executes directory discovery and workbook parsing, forwarding nonfatal folder failures.
    ///
    /// ## Arguments
    /// - `query`: `SearchQuery` - Search criteria, directory mode, and optional Burst count.
    /// - `on_match`: `FMatch` - Callback receiving each matched search item.
    /// - `on_progress`: `FProgress` - Callback receiving scan progress updates.
    /// - `on_issue`: `FIssue` - Callback receiving descendant directory discovery issues.
    ///
    /// ## Returns
    /// - `Result<ScanProgress, String>`: Final scan progress state or fatal discovery error.
    ///
    /// ## Errors / Exceptions
    /// Returns `Err` for an invalid root, count, regex, worker failure, or file delivery failure.
    /// Catches panics from workbook parsing and discovery callbacks.
    pub fn execute_search_with_issues<FMatch, FProgress, FIssue>(
        &self,
        query: SearchQuery,
        on_match: FMatch,
        on_progress: FProgress,
        on_issue: FIssue,
    ) -> Result<ScanProgress, String>
    where
        FMatch: FnMut(SearchMatch) + Send + Sync + 'static,
        FProgress: FnMut(ScanProgress) + Send + Sync + 'static,
        FIssue: FnMut(crate::search::discovery::DiscoveryIssue) + Send + Sync + 'static,
    {
        self.reset_cancel();
        let cancel_flag = Arc::clone(&self.is_cancelled);

        // Compile regex if requested
        let regex_obj = if query.use_regex {
            let re = RegexBuilder::new(&query.keyword)
                .case_insensitive(!query.match_case)
                .build()
                .map_err(|e| {
                    // Constant reference: crate::constants::ERR_INVALID_REGEX
                    format!("{}: {}", crate::constants::ERR_INVALID_REGEX, e)
                })?;
            Some(re)
        } else {
            None
        };

        let start_time = Instant::now();
        // Constant reference: crate::constants::ERR_PATH_UNCLOSED_QUOTE, crate::constants::ERR_NO_VALID_SEARCH_PATHS
        let parsed_paths =
            crate::search::path::parse_search_paths(&query.target_dir).map_err(|e| match e {
                crate::search::path::PathParseError::UnclosedQuote => {
                    crate::constants::ERR_PATH_UNCLOSED_QUOTE.to_string()
                }
                crate::search::path::PathParseError::EmptyInput => {
                    crate::constants::ERR_NO_VALID_SEARCH_PATHS.to_string()
                }
            })?;

        let partition_result =
            crate::search::path::resolve_and_partition_paths(&parsed_paths, None);
        if partition_result.valid_roots.is_empty() {
            // Constant reference: crate::constants::ERR_FILE_NOT_FOUND
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                query.target_dir
            ));
        }

        let scanned_count = Arc::new(AtomicUsize::new(0));
        let match_count = Arc::new(AtomicUsize::new(0));
        let discovered_count = Arc::new(AtomicUsize::new(0));
        let scan_completed = Arc::new(AtomicBool::new(false));

        let on_match = Arc::new(std::sync::Mutex::new(on_match));
        let on_progress = Arc::new(std::sync::Mutex::new(on_progress));

        // Initial progress notification (total_files: 0 while discovering)
        if let Ok(mut prog) = on_progress.lock() {
            // Fixed message is derived on UI side from phase rather than current_file.
            prog(ScanProgress {
                state: ScanState::Scanning,
                phase: crate::models::ScanPhase::Discovering,
                error_code: None,
                scanned_files: 0,
                total_files: 0,
                matches_found: 0,
                current_file: String::new(),
                elapsed_ms: 0,
            });
        }

        // Suppress standard stderr dumps on third-party library panics handled by catch_unwind
        static INIT_HOOK: std::sync::Once = std::sync::Once::new();
        INIT_HOOK.call_once(|| {
            std::panic::set_hook(Box::new(|_info| {
                // Suppress stderr dump since catch_unwind handles the panic safely
            }));
        });

        let last_notify_ms = Arc::new(std::sync::atomic::AtomicU64::new(0));

        // Constant reference: crate::constants::CHANNEL_BUFFER_SIZE
        let (tx, rx) =
            std::sync::mpsc::sync_channel::<PathBuf>(crate::constants::CHANNEL_BUFFER_SIZE);
        let rx = Arc::new(std::sync::Mutex::new(rx));

        // Forward non-fatal path errors as discovery issues
        let discovery_issues = Arc::new(std::sync::Mutex::new(on_issue));
        for (invalid_path, error) in partition_result.invalid_paths {
            if let Ok(mut issues) = discovery_issues.lock() {
                issues(crate::search::discovery::DiscoveryIssue {
                    path: PathBuf::from(invalid_path),
                    stage: crate::constants::SEARCH_STAGE_DISCOVERY,
                    code: match error {
                        crate::search::path::PathError::PermissionDenied => {
                            crate::constants::DISCOVERY_ERROR_PERMISSION_DENIED
                        }
                        _ => crate::constants::DISCOVERY_ERROR_READ_FAILED,
                    },
                    cause: String::new(),
                });
            }
        }

        // Producer: Directory scan in separate background thread
        let valid_roots = partition_result.valid_roots;
        let extensions = query.extensions.clone();
        let directory_mode = query.directory_mode;
        let burst_workers = query.burst_workers;
        let cancel_flag_scanner = Arc::clone(&cancel_flag);
        let discovered_count_clone = Arc::clone(&discovered_count);
        let scan_completed_clone = Arc::clone(&scan_completed);
        let discovery_issues_scanner = Arc::clone(&discovery_issues);
        let scanner_error = Arc::new(std::sync::Mutex::new(None::<String>));
        let scanner_error_thread = Arc::clone(&scanner_error);

        let scanner_handle = std::thread::spawn(move || {
            let delivery = crate::search::discovery::discover_files(
                &valid_roots,
                &extensions,
                directory_mode,
                burst_workers,
                &cancel_flag_scanner,
                |path| {
                    let mut pending_path = path.to_path_buf();
                    loop {
                        if cancel_flag_scanner.load(Ordering::Acquire) {
                            return Ok(());
                        }
                        match tx.try_send(pending_path) {
                            Ok(()) => {
                                discovered_count_clone.fetch_add(1, Ordering::Relaxed);
                                return Ok(());
                            }
                            Err(std::sync::mpsc::TrySendError::Full(path)) => {
                                pending_path = path;
                                std::thread::sleep(std::time::Duration::from_millis(
                                    crate::constants::DISCOVERY_RETRY_DELAY_MS,
                                ));
                            }
                            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                                return Err(
                                    crate::constants::ERR_DISCOVERY_FILE_CHANNEL.to_string()
                                );
                            }
                        }
                    }
                },
                |issue| {
                    if let Ok(mut callback) = discovery_issues_scanner.lock() {
                        callback(issue);
                    }
                },
            );
            if let Err(error) = delivery {
                if !cancel_flag_scanner.load(Ordering::Acquire) {
                    if let Ok(mut stored_error) = scanner_error_thread.lock() {
                        *stored_error = Some(error);
                    }
                }
            }
            if scanner_error_thread
                .lock()
                .is_ok_and(|error| error.is_none())
                && !cancel_flag_scanner.load(Ordering::Relaxed)
            {
                scan_completed_clone.store(true, Ordering::Relaxed);
            }
            // tx drops here, closing the channel for receivers
        });

        // Consumer: Rayon multi-threaded parallel processing immediately scans incoming paths
        let num_threads = rayon::current_num_threads();
        (0..num_threads).into_par_iter().for_each(|_| {
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                let file_path = {
                    let Ok(rx_guard) = rx.lock() else {
                        break;
                    };
                    match rx_guard.recv() {
                        Ok(p) => p,
                        Err(_) => break, // All paths received and queue drained
                    }
                };

                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                let file_name = file_path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Parse Excel file (corrupted files or panics are safely caught without aborting)
                let cancel_flag_ref = Arc::clone(&cancel_flag);
                let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    parse_and_search_file(
                        &file_path,
                        &query,
                        regex_obj.as_ref(),
                        Some(&cancel_flag_ref),
                    )
                }));

                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                match parse_result {
                    Ok(Ok(matches)) => {
                        let m_count = matches.len();
                        if m_count > 0 {
                            match_count.fetch_add(m_count, Ordering::Relaxed);
                            if let Ok(mut match_cb) = on_match.lock() {
                                for m in matches {
                                    match_cb(m);
                                }
                            }
                        }
                    }
                    Ok(Err(_err_msg)) => {
                        // Skip unreadable or corrupted files and continue
                    }
                    Err(_) => {
                        // Safely recover from library internal panics
                    }
                }

                let scanned = scanned_count.fetch_add(1, Ordering::Relaxed) + 1;
                let current_matches = match_count.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed().as_millis() as u64;

                // Do not emit Scanning progress events after cancellation
                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                // Before scan completion total_files = 0; after completion it is exact count
                let is_scan_done = scan_completed.load(Ordering::Relaxed);
                let current_total = if is_scan_done {
                    discovered_count.load(Ordering::Relaxed)
                } else {
                    0
                };

                // Notify on first file, on completion, or after progress interval
                let last = last_notify_ms.load(Ordering::Relaxed);
                // Constant reference: crate::constants::PROGRESS_NOTIFY_INTERVAL_MS
                let should_notify = scanned == 1
                    || (is_scan_done && scanned == current_total)
                    || (elapsed.saturating_sub(last)
                        >= crate::constants::PROGRESS_NOTIFY_INTERVAL_MS);

                if should_notify {
                    last_notify_ms.store(elapsed, Ordering::Relaxed);
                    if let Ok(mut prog_cb) = on_progress.lock() {
                        prog_cb(ScanProgress {
                            state: ScanState::Scanning,
                            phase: if current_total == 0 {
                                crate::models::ScanPhase::Discovering
                            } else {
                                crate::models::ScanPhase::Scanning
                            },
                            error_code: None,
                            scanned_files: scanned,
                            total_files: current_total,
                            matches_found: current_matches,
                            current_file: file_name,
                            elapsed_ms: elapsed,
                        });
                    }
                }
            }
        });

        // Wait for scanner thread to complete
        if scanner_handle.join().is_err() {
            return Err(crate::constants::ERR_DISCOVERY_WORKER_PANIC.to_string());
        }
        if let Some(error) = scanner_error
            .lock()
            .map_err(|error| error.to_string())?
            .take()
        {
            return Err(error);
        }

        let is_cancelled = cancel_flag.load(Ordering::Relaxed);
        let final_state = if is_cancelled {
            ScanState::Cancelled
        } else {
            ScanState::Completed
        };
        let final_total = discovered_count.load(Ordering::Relaxed);

        // UI generates completion/cancellation messages using phase and state.
        let final_progress = ScanProgress {
            state: final_state,
            phase: crate::models::ScanPhase::Finished,
            error_code: None,
            scanned_files: scanned_count.load(Ordering::Relaxed),
            total_files: final_total,
            matches_found: match_count.load(Ordering::Relaxed),
            current_file: String::new(),
            elapsed_ms: start_time.elapsed().as_millis() as u64,
        };

        // Final notification
        if let Ok(mut prog_cb) = on_progress.lock() {
            prog_cb(final_progress.clone());
        }

        Ok(final_progress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::{export_to_csv, export_to_xlsx};
    use crate::search::preview::extract_cell_preview;
    use std::collections::BTreeMap;
    /// ## Description
    /// Resolves the absolute path to workspace test fixtures directory (tests/fixtures).
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// - `PathBuf`: Existing fixtures directory path
    ///
    /// ## Errors / Exceptions
    /// Panics if fixtures directory cannot be found.
    fn fixtures_dir() -> PathBuf {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let candidate_grandparent = manifest
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("tests").join("fixtures"));
        let candidate_parent = manifest.parent().map(|p| p.join("tests").join("fixtures"));

        if let Some(path) = candidate_grandparent.filter(|p| p.exists()) {
            path
        } else if let Some(path) = candidate_parent.filter(|p| p.exists()) {
            path
        } else {
            panic!("Fixtures dir not found from manifest: {:?}", manifest);
        }
    }

    /// ## Description
    /// Executes search against Excel files in test fixtures directory, verifying
    /// that matching cells, preview extraction, and export work properly.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_search_engine_on_fixtures() {
        let fixtures_dir = fixtures_dir();

        let engine = SearchEngine::new();
        let query = SearchQuery {
            keyword: "Financial".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        let matches = Arc::new(std::sync::Mutex::new(Vec::new()));
        let matches_clone = Arc::clone(&matches);

        let result = engine.execute_search(
            query,
            move |m| {
                matches_clone.lock().unwrap().push(m);
            },
            |_| {},
        );

        assert!(result.is_ok());
        let found = matches.lock().unwrap();
        assert!(!found.is_empty(), "Should find matches for 'Financial'");
        let first = &found[0];
        assert_eq!(first.sheet_name, "Data");

        // Test preview extraction
        let preview = extract_cell_preview(
            &first.full_path,
            &first.sheet_name,
            first.row_index,
            first.col_index,
        );
        assert!(preview.is_ok(), "Preview extraction should succeed");
        let p_data = preview.unwrap();
        assert!(!p_data.rows.is_empty(), "Preview rows should not be empty");
        assert!(
            !p_data.columns.is_empty(),
            "Preview columns should not be empty"
        );

        // Test exports
        let temp_csv = std::env::temp_dir().join("test_export.csv");
        let temp_xlsx = std::env::temp_dir().join("test_export.xlsx");
        let test_keys = crate::constants::EXPORT_HEADER_KEYS.iter().copied().chain([
            crate::constants::EXPORT_SHEET_NAME_KEY,
            crate::constants::EXPORT_MATCH_VALUE_KEY,
            crate::constants::EXPORT_MATCH_FORMULA_KEY,
            crate::constants::EXPORT_MATCH_COMMENT_KEY,
            crate::constants::EXPORT_MATCH_HIDDEN_SHEET_KEY,
            crate::constants::TRANSLATION_UNAVAILABLE_KEY,
        ]);
        let test_catalog = test_keys
            .map(|key| (key.to_string(), key.to_string()))
            .collect::<BTreeMap<_, _>>();
        let test_catalogs = BTreeMap::from([
            (
                crate::constants::LANGUAGE_EN.to_string(),
                test_catalog.clone(),
            ),
            (crate::constants::LANGUAGE_JA.to_string(), test_catalog),
        ]);

        assert!(export_to_csv(temp_csv.to_str().unwrap(), &found, "ja", &test_catalogs).is_ok());
        assert!(export_to_xlsx(temp_xlsx.to_str().unwrap(), &found, "ja", &test_catalogs).is_ok());

        assert!(temp_csv.exists());
        assert!(temp_xlsx.exists());

        let _ = std::fs::remove_file(temp_csv);
        let _ = std::fs::remove_file(temp_xlsx);
    }

    /// ## Description
    /// Verifies that snippet generation on multibyte strings operates safely at UTF-8 boundaries without panicking.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_snippet_utf8_boundary_safety() {
        use crate::search::parser::make_snippet;

        // Verify snippet generation at arbitrary positions without panicking
        let japanese_text =
            "財務報告書2026年第3四半期における監査報告書の承認およびシステム移行計画の進捗状況確認";

        // Match on sample keyword
        let mat_start = japanese_text.find("監査報告書").unwrap();
        let mat_end = mat_start + "監査報告書".len();

        let snippet = make_snippet(japanese_text, mat_start, mat_end);
        assert!(snippet.contains("<mark"));
        assert!(snippet.contains("監査報告書"));
        assert!(snippet.contains("</mark>"));

        // Test prefix match
        let mat_start_head = 0;
        let mat_end_head = "財務".len();
        let snippet_head = make_snippet(japanese_text, mat_start_head, mat_end_head);
        assert!(snippet_head.contains("財務"));

        // Test suffix match
        let mat_start_tail = japanese_text.rfind("確認").unwrap();
        let mat_end_tail = japanese_text.len();
        let snippet_tail = make_snippet(japanese_text, mat_start_tail, mat_end_tail);
        assert!(snippet_tail.contains("確認"));
    }

    /// ## Description
    /// Verifies that when cancel() is invoked during search, scanning is aborted safely and final status is Cancelled.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_search_engine_cancellation() {
        let fixtures_dir = fixtures_dir();

        let engine = Arc::new(SearchEngine::new());
        let engine_clone = Arc::clone(&engine);

        let query = SearchQuery {
            keyword: "Financial".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        // Set cancellation flag during search execution
        let result = engine.execute_search(
            query,
            move |_m| {
                // Cancel once first match is received
                engine_clone.cancel();
            },
            |_| {},
        );

        assert!(result.is_ok());
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Cancelled);
    }

    /// ## Description
    /// Verifies that collect_files aborts immediately and returns empty or minimal files when cancel flag is set.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_collect_files_cancellation() {
        let fixtures_dir = fixtures_dir();

        let extensions = vec![".xlsx".to_string()];

        // 1. Without cancel flag (None): Files should be collected
        let all_files =
            SearchEngine::collect_files(fixtures_dir.to_str().unwrap(), &extensions, None);
        assert!(!all_files.is_empty(), "Fixtures files should be collected");

        // 2. Pre-cancelled at start: Must abort immediately and return 0 files
        let cancelled_flag = AtomicBool::new(true);
        let cancelled_files = SearchEngine::collect_files(
            fixtures_dir.to_str().unwrap(),
            &extensions,
            Some(&cancelled_flag),
        );
        assert_eq!(
            cancelled_files.len(),
            0,
            "When cancelled at start, collected files must be empty"
        );
    }

    /// ## Description
    /// Verifies that pipeline parallel search using bounded channels completes scanning and returns consistent results.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_search_engine_parallel_pipeline() {
        let fixtures_dir = fixtures_dir();

        let engine = SearchEngine::new();
        let query = SearchQuery {
            keyword: "Total".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        let matches = Arc::new(std::sync::Mutex::new(Vec::new()));
        let matches_clone = Arc::clone(&matches);
        let progress_events = Arc::new(std::sync::Mutex::new(Vec::new()));
        let progress_clone = Arc::clone(&progress_events);

        let result = engine.execute_search(
            query,
            move |m| {
                matches_clone.lock().unwrap().push(m);
            },
            move |p| {
                progress_clone.lock().unwrap().push(p);
            },
        );

        assert!(result.is_ok(), "Parallel pipeline search must succeed");
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Completed);
        assert!(final_prog.total_files > 0);
        assert_eq!(final_prog.scanned_files, final_prog.total_files);
        assert_eq!(final_prog.matches_found, matches.lock().unwrap().len());

        let events = progress_events.lock().unwrap();
        assert!(!events.is_empty(), "Progress events should be emitted");
        // Verify initial progress event has total_files: 0 (discovering state)
        assert_eq!(events[0].total_files, 0);
    }

    /// ## Description
    /// Verifies that when cancellation is requested during active directory scanning, execution halts safely within 1 second.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_search_engine_cancellation_during_scan() {
        let fixtures_dir = fixtures_dir();

        let engine = Arc::new(SearchEngine::new());
        let engine_clone = Arc::clone(&engine);

        let query = SearchQuery {
            keyword: "a".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        // Cancel immediately upon receiving initial progress notification
        let result = engine.execute_search(
            query,
            |_| {},
            move |_p| {
                engine_clone.cancel();
            },
        );

        assert!(result.is_ok());
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Cancelled);
    }
}

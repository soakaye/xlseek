//! # Search Command Handler (commands/search_cmd.rs)
//!
//! ## Description
//! Receives search start requests (start_search) and cancellation requests (cancel_search) from the frontend,
//! running the search engine in background threads and emitting matches and progress events.
//! Conforms to Constitution Principle I (English comments/logs), Principle II (Constant references),
//! and Principle III (Header comments).

use crate::models::CommandError;
use crate::models::{
    resolve_burst_workers, ErrorCode, ScanProgress, ScanState, SearchMatch, SearchQuery,
};
use crate::search::engine::SearchEngine;
use crate::search::path::{
    complete_directory_path as complete_path, resolve_search_path, validate_search_directory,
    PathError,
};
use regex::RegexBuilder;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

/// ## Description
/// Shared state struct across the Tauri application holding the search engine instance.
pub struct AppState {
    pub engine: Arc<SearchEngine>,
}

/// ## Description
/// Receives search requests, validates path and regex, and spawns the search engine on background threads.
/// Emits matching cell info as `search-match` events and progress as `scan-progress` events.
///
/// ## Arguments
/// - `app`: `AppHandle` - Tauri handle for emitting events
/// - `query`: `SearchQuery` - Search query parameters
/// - `state`: `State<'_, AppState>` - Shared application state
///
/// ## Returns
/// - `Result<(), CommandError>`: `Ok(())` on successful acceptance
///
/// ## Errors / Exceptions
/// Does not panic. Search errors are emitted via `scan-progress` events (State: Error).
#[tauri::command]
pub async fn start_search(
    app: AppHandle,
    query: SearchQuery,
    state: State<'_, AppState>,
) -> Result<(), CommandError> {
    let home_dir = app.path().home_dir().ok();
    let query = tauri::async_runtime::spawn_blocking(move || {
        validate_required_search_fields(&query)?;
        validate_directory_settings(&query)?;
        validate_search_regex(&query).map_err(|code| CommandError { code })?;
        let target_path = resolve_search_path(&query.target_dir, home_dir.as_deref())
            .map_err(path_error_to_command_error)?;
        validate_search_directory(&target_path).map_err(path_error_to_command_error)?;
        let mut validated_query = query;
        validated_query.target_dir = target_path.to_string_lossy().into_owned();
        Ok::<SearchQuery, CommandError>(validated_query)
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::InternalError,
    })??;
    let engine = Arc::clone(&state.engine);
    println!("{}", crate::constants::LOG_SEARCH_STARTED);

    tauri::async_runtime::spawn_blocking(move || {
        let app_handle_match = app.clone();
        let app_handle_prog = app.clone();
        let app_handle_err = app.clone();
        let app_handle_final = app.clone();
        let app_handle_issue = app.clone();

        let match_buffer = Arc::new(std::sync::Mutex::new(Vec::new()));
        let last_emit_instant = Arc::new(std::sync::Mutex::new(std::time::Instant::now()));

        let match_buffer_clone = Arc::clone(&match_buffer);
        let last_emit_clone = Arc::clone(&last_emit_instant);

        let flush_matches = |app_handle: &AppHandle, buf: &mut Vec<SearchMatch>| {
            if buf.is_empty() {
                return;
            }
            let batch: Vec<SearchMatch> = std::mem::take(buf);
            // Constant reference: crate::constants::EVENT_SEARCH_MATCH
            if let Err(e) = app_handle.emit(crate::constants::EVENT_SEARCH_MATCH, &batch) {
                eprintln!("{}: {:?}", crate::constants::LOG_EVENT_EMIT_FAILED, e);
            }
        };

        match engine.execute_search_with_issues(
            query,
            move |search_match: SearchMatch| {
                let mut buf = match_buffer_clone.lock().unwrap();
                buf.push(search_match);
                let mut last = last_emit_clone.lock().unwrap();
                // Constant reference: crate::constants::SEARCH_MATCH_BATCH_SIZE, crate::constants::SEARCH_MATCH_BATCH_INTERVAL_MS
                if buf.len() >= crate::constants::SEARCH_MATCH_BATCH_SIZE
                    || last.elapsed().as_millis()
                        >= crate::constants::SEARCH_MATCH_BATCH_INTERVAL_MS
                {
                    flush_matches(&app_handle_match, &mut buf);
                    *last = std::time::Instant::now();
                }
            },
            move |progress| {
                // Constant reference: crate::constants::EVENT_SCAN_PROGRESS
                if let Err(e) =
                    app_handle_prog.emit(crate::constants::EVENT_SCAN_PROGRESS, &progress)
                {
                    eprintln!("{}: {:?}", crate::constants::LOG_EVENT_EMIT_FAILED, e);
                }
            },
            move |issue| {
                // Constant reference: crate::constants::EVENT_SEARCH_ISSUE.
                if let Err(error) =
                    app_handle_issue.emit(crate::constants::EVENT_SEARCH_ISSUE, &issue)
                {
                    eprintln!("{}: {:?}", crate::constants::LOG_EVENT_EMIT_FAILED, error);
                }
            },
        ) {
            Ok(final_prog) => {
                if let Ok(mut buf) = match_buffer.lock() {
                    flush_matches(&app_handle_final, &mut buf);
                }
                println!("{}", crate::constants::LOG_SEARCH_COMPLETED);
                let _ = final_prog;
            }
            Err(err_msg) => {
                if let Ok(mut buf) = match_buffer.lock() {
                    flush_matches(&app_handle_final, &mut buf);
                }
                eprintln!("{}", crate::constants::LOG_SEARCH_FAILED);
                let error_code = if err_msg.contains(crate::constants::ERR_INVALID_REGEX) {
                    crate::models::ErrorCode::InvalidRegex
                } else {
                    crate::models::ErrorCode::SearchFailed
                };
                // Constant reference: crate::constants::EVENT_SCAN_PROGRESS
                let _ = app_handle_err.emit(
                    crate::constants::EVENT_SCAN_PROGRESS,
                    ScanProgress {
                        state: ScanState::Error,
                        phase: crate::models::ScanPhase::Finished,
                        error_code: Some(error_code),
                        scanned_files: 0,
                        total_files: 0,
                        matches_found: 0,
                        current_file: String::new(),
                        elapsed_ms: 0,
                    },
                );
            }
        }
    });

    Ok(())
}

/// ## Description
/// Validates required search parameters (keyword, target path, extensions) prior to accepting search.
///
/// ## Arguments
/// - `query`: `&SearchQuery` - Query to inspect
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) if valid, Err(CommandError) if missing or whitespace-only
///
/// ## Errors / Exceptions
/// Does not panic.
fn validate_required_search_fields(query: &SearchQuery) -> Result<(), CommandError> {
    if query.keyword.trim().is_empty()
        || query.target_dir.trim().is_empty()
        || query.extensions.is_empty()
    {
        return Err(CommandError {
            code: ErrorCode::SearchFailed,
        });
    }
    Ok(())
}

/// Validates a custom directory visitor count before accepting the search request.
///
/// ## Arguments / Returns
/// Accepts `&SearchQuery`; returns `Ok(())` for Automatic or a supported custom count.
///
/// ## Errors
/// Returns `SearchFailed` when the requested count falls outside the supported range.
fn validate_directory_settings(query: &SearchQuery) -> Result<(), CommandError> {
    if query
        .burst_workers
        .is_some_and(|count| resolve_burst_workers(Some(count)).is_err())
    {
        return Err(CommandError {
            code: ErrorCode::SearchFailed,
        });
    }
    Ok(())
}

/// ## Description
/// Retrieves directory path completion candidates on a blocking thread.
///
/// ## Arguments
/// - `app`: `AppHandle` - Tauri handle for home directory access
/// - `path_input`: `String` - Directory input string
///
/// ## Returns
/// - `Vec<String>`: Candidate path strings
///
/// ## Errors / Exceptions
/// Returns empty vector on failure; does not panic.
#[tauri::command]
pub async fn complete_directory_path(app: AppHandle, path_input: String) -> Vec<String> {
    let home_dir = app.path().home_dir().ok();
    tauri::async_runtime::spawn_blocking(move || complete_path(&path_input, home_dir.as_deref()))
        .await
        .unwrap_or_default()
}

/// ## Description
/// Validates regular expression syntax by precompiling prior to launching search.
///
/// ## Arguments
/// - `query`: `&SearchQuery` - Query containing keyword and regex flag
///
/// ## Returns
/// - `Result<(), ErrorCode>`: Ok(()) if valid, Err(ErrorCode::InvalidRegex) if malformed
///
/// ## Errors / Exceptions
/// Does not panic.
fn validate_search_regex(query: &SearchQuery) -> Result<(), ErrorCode> {
    if !query.use_regex {
        return Ok(());
    }
    RegexBuilder::new(&query.keyword)
        .case_insensitive(!query.match_case)
        .build()
        .map(|_| ())
        .map_err(|_| ErrorCode::InvalidRegex)
}

/// ## Description
/// Converts internal PathError into structured CommandError for Tauri IPC.
///
/// ## Arguments
/// - `error`: `PathError` - Internal path error
///
/// ## Returns
/// - `CommandError`: Mapped command error
///
/// ## Errors / Exceptions
/// Does not panic.
fn path_error_to_command_error(error: PathError) -> CommandError {
    let code = match error {
        PathError::NotFound => ErrorCode::PathNotFound,
        PathError::PermissionDenied => ErrorCode::PermissionDenied,
        PathError::SearchFailed => ErrorCode::SearchFailed,
    };
    CommandError { code }
}

/// ## Description
/// Sets the cancellation flag on running search operations to halt scanning.
///
/// ## Arguments
/// - `state`: `State<'_, AppState>` - Shared application state
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) on success
///
/// ## Errors / Exceptions
/// Does not panic.
#[tauri::command]
pub fn cancel_search(state: State<'_, AppState>) -> Result<(), CommandError> {
    println!("{}", crate::constants::LOG_CANCEL_REQUESTED);
    state.engine.cancel();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        validate_directory_settings, validate_required_search_fields, validate_search_regex,
    };
    use crate::models::{CommandError, ErrorCode, SearchQuery};

    /// ## Description
    /// Verifies detecting invalid regex patterns before starting search execution.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn rejects_invalid_regex_before_search_is_started() {
        let query = SearchQuery {
            keyword: "(".to_string(),
            target_dir: String::new(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: true,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: Vec::new(),
        };

        assert!(matches!(
            validate_search_regex(&query),
            Err(ErrorCode::InvalidRegex)
        ));
    }

    /// ## Description
    /// Verifies that empty or whitespace-only required search parameters are rejected.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn rejects_missing_required_search_fields() {
        let query = SearchQuery {
            keyword: " ".to_string(),
            target_dir: "/tmp".to_string(),
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
        assert!(matches!(
            validate_required_search_fields(&query),
            Err(CommandError {
                code: ErrorCode::SearchFailed
            })
        ));
    }

    /// Rejects an invalid custom directory worker count before launching a search.
    ///
    /// ## Arguments / Returns
    /// No arguments; returns `()` after validating the request error.
    ///
    /// ## Errors
    /// Panics if the invalid worker count is not rejected.
    #[test]
    fn rejects_invalid_burst_worker_count_before_search_is_started() {
        let query = SearchQuery {
            keyword: "term".to_string(),
            target_dir: "directory".to_string(),
            directory_mode: crate::models::DirectorySearchMode::Burst,
            burst_workers: Some(xlseek_core::constants::BURST_WORKERS_MIN - 1),
            match_case: false,
            // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
            include_value: crate::constants::DEFAULT_INCLUDE_VALUE,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![crate::constants::EXT_XLSX.to_string()],
        };
        assert!(matches!(
            validate_directory_settings(&query),
            Err(CommandError {
                code: ErrorCode::SearchFailed
            })
        ));
    }
}

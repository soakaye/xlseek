//! Purpose: Discovers eligible workbook paths serially or with bounded concurrent directory visitors.
//! Inputs: Root paths, file extensions, traversal settings, a cancellation flag, and file/issue callbacks; output: successful discovery or a fatal root/delivery error.
//! Errors: Root failures and callback failures return `Err`; descendant read failures are delivered as issues; cancellation is cooperative between filesystem operations.

use crate::constants;
use crate::models::{resolve_burst_workers, DirectorySearchMode};
use serde::Serialize;
use std::collections::VecDeque;
use std::fs::{self, ReadDir};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

/// Describes one nonfatal descendant directory read failure.
///
/// ## Arguments / Returns
/// Carries a local path, locale-neutral issue code, and diagnostic cause.
///
/// ## Errors
/// This data value does not perform fallible operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscoveryIssue {
    pub path: PathBuf,
    pub stage: &'static str,
    pub code: &'static str,
    #[serde(skip_serializing)]
    pub cause: String,
}

/// Tracks scheduler state shared by the bounded visitor threads.
///
/// ## Arguments / Returns
/// Holds pending roots/children, outstanding task count, and the first fatal error.
///
/// ## Errors
/// Mutex poisoning is recovered by taking the inner scheduler state.
#[derive(Debug)]
struct SchedulerState {
    pending: VecDeque<(PathBuf, bool)>,
    outstanding: usize,
    fatal: Option<String>,
}

/// Provides shared scheduling state and notifications to directory visitors.
///
/// ## Arguments / Returns
/// Owns the synchronized queue state and condition variable.
///
/// ## Errors
/// Lock poisoning is recovered without discarding pending task accounting.
#[derive(Debug)]
struct Scheduler {
    state: Mutex<SchedulerState>,
    wake: Condvar,
}

/// Holds one streaming directory iterator on a visitor's iterative continuation stack.
///
/// ## Arguments / Returns
/// Associates a directory path and root marker with its open entry iterator.
///
/// ## Errors
/// Iterator advancement may report an I/O issue handled by the visitor.
#[derive(Debug)]
struct DirectoryFrame {
    path: PathBuf,
    is_root: bool,
    entries: ReadDir,
}

/// Discovers eligible files under the given roots with bounded folder concurrency.
///
/// ## Arguments / Returns
/// `roots` are files' parent directories to traverse; `extensions` are dot-prefixed file types;
/// `mode` selects one visitor or a bounded Burst pool; `requested_workers` is optional custom
/// count; `cancelled` is checked between operations; `on_file` receives each eligible path and
/// may return a fatal delivery error; `on_issue` receives nonfatal descendant failures.
/// Returns `Ok(())` when discovery completes or is cancelled, and `Err` for invalid settings,
/// invalid roots, or file-delivery failures.
///
/// ## Errors
/// Returns `Err` for invalid roots/settings or callback failure. Descendant read failures are
/// reported through `on_issue`. Callback panics are caught and converted to `Err`.
pub fn discover_files<FFile, FIssue>(
    roots: &[PathBuf],
    extensions: &[String],
    mode: DirectorySearchMode,
    requested_workers: Option<usize>,
    cancelled: &AtomicBool,
    on_file: FFile,
    on_issue: FIssue,
) -> Result<(), String>
where
    FFile: Fn(&Path) -> Result<(), String> + Send + Sync,
    FIssue: Fn(DiscoveryIssue) + Send + Sync,
{
    let custom_count = requested_workers
        .map(|count| resolve_burst_workers(Some(count)))
        .transpose()?;
    let automatic_worker_count = resolve_burst_workers(None)?;
    let worker_count = match mode {
        DirectorySearchMode::Sequential => constants::DISCOVERY_SEQUENTIAL_WORKERS,
        DirectorySearchMode::Burst => custom_count.unwrap_or(automatic_worker_count),
    };
    let extensions = Arc::new(
        extensions
            .iter()
            .map(|extension| extension.to_ascii_lowercase())
            .collect::<Vec<_>>(),
    );
    let mut pending = VecDeque::new();
    for root in roots {
        if !root.is_dir() {
            return Err(format!(
                "{}: {}",
                constants::ERR_FILE_NOT_FOUND,
                root.display()
            ));
        }
        pending.push_back((root.clone(), true));
    }
    if pending.is_empty() {
        return Ok(());
    }
    let scheduler = Arc::new(Scheduler {
        state: Mutex::new(SchedulerState {
            outstanding: pending.len(),
            pending,
            fatal: None,
        }),
        wake: Condvar::new(),
    });
    let on_file = Arc::new(on_file);
    let on_issue = Arc::new(on_issue);

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            let scheduler = Arc::clone(&scheduler);
            let extensions = Arc::clone(&extensions);
            let on_file = Arc::clone(&on_file);
            let on_issue = Arc::clone(&on_issue);
            scope.spawn(move || {
                visit_directories(
                    &scheduler,
                    &extensions,
                    cancelled,
                    on_file.as_ref(),
                    on_issue.as_ref(),
                );
            });
        }
    });

    let mut state = lock_state(&scheduler.state);
    if let Some(error) = state.fatal.take() {
        return Err(error);
    }
    Ok(())
}

/// Advances a visitor's local continuation stack and shared pending queue.
///
/// ## Arguments / Returns
/// Accepts shared scheduler, normalized extensions, cancellation flag, and callbacks; returns `()`.
///
/// ## Errors
/// Fatal root/delivery errors stop all visitors; descendant filesystem errors go to `on_issue`.
fn visit_directories<FFile, FIssue>(
    scheduler: &Scheduler,
    extensions: &[String],
    cancelled: &AtomicBool,
    on_file: &FFile,
    on_issue: &FIssue,
) where
    FFile: Fn(&Path) -> Result<(), String>,
    FIssue: Fn(DiscoveryIssue),
{
    let mut frames = Vec::<DirectoryFrame>::new();
    loop {
        if cancelled.load(Ordering::Acquire) {
            scheduler.wake.notify_all();
            return;
        }
        {
            let mut state = lock_state(&scheduler.state);
            if state.fatal.is_some() {
                scheduler.wake.notify_all();
                return;
            }
            if frames.is_empty() {
                if let Some((path, is_root)) = state.pending.pop_front() {
                    drop(state);
                    push_directory_frame(path, is_root, &mut frames, scheduler, on_issue);
                    continue;
                }
            }
            if frames.is_empty() {
                if state.outstanding == 0 {
                    return;
                }
                let (next_state, _) = scheduler
                    .wake
                    .wait_timeout(
                        state,
                        std::time::Duration::from_millis(constants::DISCOVERY_RETRY_DELAY_MS),
                    )
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                drop(next_state);
                continue;
            }
        }

        let next_entry = frames.last_mut().and_then(|frame| frame.entries.next());
        let Some(next_entry) = next_entry else {
            frames.pop();
            let mut state = lock_state(&scheduler.state);
            state.outstanding = state
                .outstanding
                .saturating_sub(constants::DISCOVERY_ONE_TASK);
            scheduler.wake.notify_all();
            continue;
        };
        let entry = match next_entry {
            Ok(entry) => entry,
            Err(error) => {
                let (path, is_root) = frames
                    .last()
                    .map(|frame| (frame.path.clone(), frame.is_root))
                    .unwrap_or_default();
                if is_root {
                    set_fatal(
                        scheduler,
                        format!(
                            "{}: {}: {error}",
                            constants::ERR_FILE_NOT_FOUND,
                            path.display()
                        ),
                    );
                    return;
                }
                report_issue(path, error, on_issue);
                continue;
            }
        };
        let entry_path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                report_issue(entry_path, error, on_issue);
                continue;
            }
        };
        if file_type.is_dir() {
            {
                let mut state = lock_state(&scheduler.state);
                state.outstanding += constants::DISCOVERY_ONE_TASK;
                if state.pending.len() < constants::DISCOVERY_QUEUE_CAPACITY {
                    state.pending.push_back((entry_path.clone(), false));
                    scheduler.wake.notify_one();
                    continue;
                }
            }
            push_directory_frame(entry_path, false, &mut frames, scheduler, on_issue);
        } else if file_type.is_file() && is_eligible_file(&entry_path, extensions) {
            let delivery =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_file(&entry_path)));
            match delivery {
                Ok(Ok(())) => {}
                Ok(Err(error)) => set_fatal(scheduler, error),
                Err(_) => set_fatal(
                    scheduler,
                    constants::ERR_DISCOVERY_CALLBACK_PANIC.to_string(),
                ),
            }
        }
    }
}

/// Opens a directory and adds its iterator to the visitor's continuation stack.
///
/// ## Arguments / Returns
/// `path`, root marker, frame stack, scheduler, and issue callback; returns `()`.
///
/// ## Errors
/// Root open failures become fatal; descendant open failures are emitted as issues and retire their task.
fn push_directory_frame<FIssue>(
    path: PathBuf,
    is_root: bool,
    frames: &mut Vec<DirectoryFrame>,
    scheduler: &Scheduler,
    on_issue: &FIssue,
) where
    FIssue: Fn(DiscoveryIssue),
{
    match fs::read_dir(&path) {
        Ok(entries) => frames.push(DirectoryFrame {
            path,
            is_root,
            entries,
        }),
        Err(error) if is_root => set_fatal(
            scheduler,
            format!(
                "{}: {}: {error}",
                constants::ERR_FILE_NOT_FOUND,
                path.display()
            ),
        ),
        Err(error) => {
            report_issue(path, error, on_issue);
            let mut state = lock_state(&scheduler.state);
            state.outstanding = state
                .outstanding
                .saturating_sub(constants::DISCOVERY_ONE_TASK);
            scheduler.wake.notify_all();
        }
    }
}

/// Emits a stable issue value for a failed descendant directory operation.
///
/// ## Arguments / Returns
/// Accepts the failed path, I/O error, and issue callback; returns `()`.
///
/// ## Errors
/// A panic in the issue callback is contained to prevent a visitor thread from unwinding.
fn report_issue<FIssue>(path: PathBuf, error: io::Error, on_issue: &FIssue)
where
    FIssue: Fn(DiscoveryIssue),
{
    let code = if error.kind() == io::ErrorKind::PermissionDenied {
        constants::DISCOVERY_ERROR_PERMISSION_DENIED
    } else {
        constants::DISCOVERY_ERROR_READ_FAILED
    };
    let issue = DiscoveryIssue {
        path,
        stage: constants::SEARCH_STAGE_DISCOVERY,
        code,
        cause: error.to_string(),
    };
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_issue(issue)));
}

/// Filters temporary and unsupported files without following symlinks.
///
/// ## Arguments / Returns
/// `path` is the file path and `extensions` are normalized dot-prefixed values; returns `true` if eligible.
///
/// ## Errors
/// Missing or non-Unicode extensions are treated as ineligible.
fn is_eligible_file(path: &Path, extensions: &[String]) -> bool {
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with(constants::EXCEL_TEMP_FILE_PREFIX))
    {
        return false;
    }
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            let extension = format!(
                "{}{}",
                constants::CLI_EXTENSION_PREFIX,
                extension.to_ascii_lowercase()
            );
            extensions.contains(&extension)
        })
        .unwrap_or(false)
}

/// Sets the first fatal scheduler error and wakes all waiting visitors.
///
/// ## Arguments / Returns
/// `scheduler` is shared state and `error` is a diagnostic string; returns `()`.
///
/// ## Errors
/// Lock poisoning is recovered while preserving the fatal state.
fn set_fatal(scheduler: &Scheduler, error: String) {
    let mut state = lock_state(&scheduler.state);
    if state.fatal.is_none() {
        state.fatal = Some(error);
    }
    scheduler.wake.notify_all();
}

/// Acquires scheduler state while recovering from a poisoned mutex.
///
/// ## Arguments / Returns
/// `state` is the scheduler mutex; returns its guard.
///
/// ## Errors
/// Does not return an error; poisoned locks yield their inner state.
fn lock_state(state: &Mutex<SchedulerState>) -> MutexGuard<'_, SchedulerState> {
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

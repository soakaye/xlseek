//! # CLI Execution Layer
//!
//! ## Description
//! Connects arguments, common Excel search, translated diagnostics, temporary storage,
//! and result publication.
//! Supports multi-path asynchronous pipelined scanning, multi-threaded parallel search,
//! immediate stderr error notifications, and CSV streaming to stdout.
//!
//! ## Arguments / Returns
//! Receives operating system arguments and returns process exit code `i32`.
//!
//! ## Errors
//! Returns 2 for invalid requests, total search failures, or storage errors;
//! returns 1 for partial failures where results are still published.

pub mod args;
pub mod constants;
pub mod output;

use args::{CliOptions, CliOutputTarget, ParseOutcome};
use exlgrep_core::models::{SearchIssue, SearchMatch, SearchReport};
use same_file::Handle;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

/// Parses process arguments and executes Excel search/save without initializing a GUI.
/// Suppresses summary text when streaming directly to stdout.
///
/// ## Arguments / Returns
/// Accepts OS arguments iterator and returns exit status code `i32`.
///
/// ## Errors
/// Outputs translation, parsing, search, and save errors to stderr and returns a non-zero exit code.
pub fn run<I, S>(args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let catalogs = match exlgrep_core::i18n::load_embedded_catalogs() {
        Ok(catalogs) => catalogs,
        Err(error) => {
            eprintln!("{}: {error}", constants::CLI_ERROR_LABEL_EN);
            return constants::CLI_EXIT_FAILURE;
        }
    };
    match args::parse_args(args, &catalogs) {
        Ok(ParseOutcome::Help(help)) => {
            println!("{help}");
            constants::CLI_EXIT_SUCCESS
        }
        Ok(ParseOutcome::Run(options)) => {
            let language = options.language.clone();
            let is_stdout = matches!(options.output_target, CliOutputTarget::Stdout);
            match execute(options, &catalogs) {
                Ok((report, output_path, exit_code, language)) => {
                    // Do not print summary in stdout mode to avoid breaking pipe streams
                    if !is_stdout {
                        if let Some(ref path) = output_path {
                            let summary = translate(
                                &catalogs,
                                &language,
                                constants::CLI_TRANSLATION_SUMMARY_KEY,
                            )
                            .replace(
                                constants::CLI_SUMMARY_SCANNED_PLACEHOLDER,
                                &report.scanned_files.to_string(),
                            )
                            .replace(
                                constants::CLI_SUMMARY_MATCHES_PLACEHOLDER,
                                &report.matches_found.to_string(),
                            )
                            .replace(
                                constants::CLI_SUMMARY_PATH_PLACEHOLDER,
                                &path.display().to_string(),
                            );
                            println!("{summary}");
                        }
                    }
                    if !report.issues.is_empty() {
                        let label =
                            translate(&catalogs, &language, constants::CLI_TRANSLATION_ISSUES_KEY)
                                .replace(
                                    constants::CLI_SUMMARY_COUNT_PLACEHOLDER,
                                    &report.issues.len().to_string(),
                                );
                        eprintln!("{label}");
                    }
                    exit_code
                }
                Err(error) => {
                    eprintln!(
                        "{}: {}",
                        translate(&catalogs, &language, constants::CLI_TRANSLATION_ERROR_KEY),
                        translate_error(&error, &catalogs, &language)
                    );
                    constants::CLI_EXIT_FAILURE
                }
            }
        }
        Err(error) => {
            eprintln!(
                "{}: {}",
                translate(
                    &catalogs,
                    constants::CLI_DEFAULT_LANGUAGE,
                    constants::CLI_TRANSLATION_ERROR_KEY
                ),
                translate_error(&error, &catalogs, constants::CLI_DEFAULT_LANGUAGE)
            );
            constants::CLI_EXIT_FAILURE
        }
    }
}

/// Explores and parses multiple input paths in parallel using an async pipeline, aggregating SearchReport and outputting results.
/// Reports discovery and parsing errors in real time to stderr.
///
/// ## Arguments / Returns
/// Accepts validated `CliOptions` and translation catalogs, returning report, published path (or None if stdout), exit code, and language.
///
/// ## Errors
/// Returns `Err` on I/O collisions, root walk failure, or result publication failure.
fn execute(
    options: CliOptions,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
) -> Result<(SearchReport, Option<PathBuf>, i32, String), String> {
    let started = Instant::now();
    let output_guard = match &options.output_target {
        CliOutputTarget::File(ref path) => {
            Some(output::OutputGuard::new(path.clone(), options.overwrite)?)
        }
        CliOutputTarget::Stdout => None,
    };

    let regex = if options.query.use_regex {
        Some(
            regex::RegexBuilder::new(&options.query.keyword)
                .case_insensitive(!options.query.match_case)
                .build()
                .map_err(|_| constants::ERR_CLI_REGEX.to_string())?,
        )
    } else {
        None
    };

    // Constant reference: constants::CHANNEL_BUFFER_SIZE
    let (sender, receiver) = sync_channel::<PathBuf>(constants::CHANNEL_BUFFER_SIZE);
    let issues = Arc::new(Mutex::new(Vec::<SearchIssue>::new()));
    let matches = Arc::new(Mutex::new(Vec::<SearchMatch>::new()));
    let scanned_files = Arc::new(AtomicUsize::new(0));
    let readable_files = Arc::new(AtomicUsize::new(0));
    let discovered_files = Arc::new(AtomicUsize::new(0));
    let root_error = Arc::new(Mutex::new(None::<String>));

    // Run discovery thread (producer) and Rayon parallel parsing (consumers) within a thread scope
    std::thread::scope(|scope| {
        // Discovery thread (Producer)
        let issues_for_producer = Arc::clone(&issues);
        let discovered_for_producer = Arc::clone(&discovered_files);
        let root_error_for_producer = Arc::clone(&root_error);
        let input_paths = options.input_paths.clone();
        let extensions = options.query.extensions.clone();
        let guard_ref = output_guard.as_ref();

        scope.spawn(move || {
            let mut visited_handles = HashSet::new();

            for input in &input_paths {
                if input.is_file() {
                    if let Some(guard) = guard_ref {
                        if let Err(collision_err) = guard.check_input(input) {
                            eprintln!(
                                "{}{}{}",
                                input.display(),
                                constants::CLI_ERROR_SEPARATOR,
                                collision_err
                            );
                            let mut err_lock = root_error_for_producer.lock().unwrap();
                            *err_lock = Some(collision_err);
                            return;
                        }
                    }
                    if let Ok(handle) = Handle::from_path(input) {
                        if !visited_handles.insert(handle) {
                            continue;
                        }
                    }
                    discovered_for_producer.fetch_add(1, Ordering::Relaxed);
                    if sender.send(input.clone()).is_err() {
                        return;
                    }
                    continue;
                }

                // Directory discovery
                let entries = WalkDir::new(input).follow_links(false).into_iter();
                for entry in entries {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(error) => {
                            let cause = error.to_string();
                            let path = error
                                .path()
                                .map(Path::to_path_buf)
                                .unwrap_or_else(|| input.clone());

                            // Emit immediately to stderr
                            eprintln!(
                                "{}{}{}",
                                path.display(),
                                constants::CLI_ERROR_SEPARATOR,
                                cause
                            );

                            let mut iss = issues_for_producer.lock().unwrap();
                            iss.push(SearchIssue {
                                path,
                                stage: constants::SEARCH_STAGE_DISCOVERY.to_string(),
                                sheet_name: None,
                                cause,
                            });

                            if error.depth() == constants::CLI_ROOT_WALK_DEPTH {
                                let mut err_lock = root_error_for_producer.lock().unwrap();
                                *err_lock = Some(format!(
                                    "{}: {}",
                                    constants::ERR_CLI_INPUT_PATH,
                                    input.display()
                                ));
                                return;
                            }
                            continue;
                        }
                    };

                    if !entry.file_type().is_file() {
                        continue;
                    }
                    let path = entry.path();
                    if path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.starts_with(constants::EXCEL_TEMP_FILE_PREFIX))
                    {
                        continue;
                    }
                    let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
                        continue;
                    };
                    let extension = format!(
                        "{}{}",
                        constants::CLI_EXTENSION_PREFIX,
                        extension.to_ascii_lowercase()
                    );
                    if !extensions.contains(&extension) {
                        continue;
                    }

                    if let Some(guard) = guard_ref {
                        if let Err(collision_err) = guard.check_input(path) {
                            eprintln!(
                                "{}{}{}",
                                path.display(),
                                constants::CLI_ERROR_SEPARATOR,
                                collision_err
                            );
                            let mut err_lock = root_error_for_producer.lock().unwrap();
                            *err_lock = Some(collision_err);
                            return;
                        }
                    }

                    if let Ok(handle) = Handle::from_path(path) {
                        if !visited_handles.insert(handle) {
                            continue;
                        }
                    }

                    discovered_for_producer.fetch_add(1, Ordering::Relaxed);
                    if sender.send(path.to_path_buf()).is_err() {
                        return;
                    }
                }
            }
        });

        // Parallel parsing pool (Consumers: Rayon)
        let receiver = Arc::new(Mutex::new(receiver));
        rayon::scope(|rayon_scope| {
            let num_threads = rayon::current_num_threads();
            for _ in 0..num_threads {
                let rx = Arc::clone(&receiver);
                let matches_consumer = Arc::clone(&matches);
                let issues_consumer = Arc::clone(&issues);
                let scanned_consumer = Arc::clone(&scanned_files);
                let readable_consumer = Arc::clone(&readable_files);
                let query = &options.query;
                let regex_ref = regex.as_ref();

                rayon_scope.spawn(move |_| {
                    loop {
                        let path = {
                            let lock = rx.lock().unwrap();
                            match lock.recv() {
                                Ok(p) => p,
                                Err(_) => break, // sender was dropped, work finished
                            }
                        };
                        scanned_consumer.fetch_add(1, Ordering::Relaxed);
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            exlgrep_core::search::parser::parse_and_search_file(
                                &path, query, regex_ref, None,
                            )
                        })) {
                            Ok(Ok(mut found)) => {
                                readable_consumer.fetch_add(1, Ordering::Relaxed);
                                let mut m = matches_consumer.lock().unwrap();
                                m.append(&mut found);
                            }
                            Ok(Err(cause)) => {
                                // Emit immediately to stderr
                                eprintln!(
                                    "{}{}{}",
                                    path.display(),
                                    constants::CLI_ERROR_SEPARATOR,
                                    cause
                                );
                                let mut iss = issues_consumer.lock().unwrap();
                                iss.push(SearchIssue {
                                    path,
                                    stage: constants::SEARCH_STAGE_WORKBOOK.to_string(),
                                    sheet_name: None,
                                    cause,
                                });
                            }
                            Err(_) => {
                                let cause = constants::ERR_CLI_PANIC.to_string();
                                eprintln!(
                                    "{}{}{}",
                                    path.display(),
                                    constants::CLI_ERROR_SEPARATOR,
                                    cause
                                );
                                let mut iss = issues_consumer.lock().unwrap();
                                iss.push(SearchIssue {
                                    path,
                                    stage: constants::SEARCH_STAGE_WORKBOOK.to_string(),
                                    sheet_name: None,
                                    cause,
                                });
                            }
                        }
                    }
                });
            }
        });
    });

    if let Some(err) = root_error.lock().unwrap().take() {
        return Err(err);
    }

    let mut matches_list = matches.lock().unwrap();
    // Re-index matched item IDs starting sequentially from 1
    // Constant reference: constants::DEFAULT_MATCH_ID_START
    for (index, item) in matches_list.iter_mut().enumerate() {
        item.id = constants::DEFAULT_MATCH_ID_START + index as u64;
    }

    let issues_list = std::mem::take(&mut *issues.lock().unwrap());
    let discovered_count = discovered_files.load(Ordering::Relaxed);
    let scanned_count = scanned_files.load(Ordering::Relaxed);
    let readable_count = readable_files.load(Ordering::Relaxed);

    let report = SearchReport {
        discovered_files: discovered_count,
        scanned_files: scanned_count,
        readable_files: readable_count,
        failed_files: issues_list
            .iter()
            .filter(|issue| issue.stage == constants::SEARCH_STAGE_WORKBOOK)
            .count(),
        matches_found: matches_list.len(),
        elapsed_ms: started.elapsed().as_millis() as u64,
        issues: issues_list,
    };

    let published_path = match options.output_target {
        CliOutputTarget::File(_) => {
            let guard = output_guard.expect("output_guard must be present for file target");
            let output_path =
                guard.publish(&matches_list, options.format, &options.language, catalogs)?;
            Some(output_path)
        }
        CliOutputTarget::Stdout => {
            let mut stdout = std::io::stdout().lock();
            exlgrep_core::export::write_csv_to_writer(
                &mut stdout,
                &matches_list,
                &options.language,
                catalogs,
                false,
            )?;
            None
        }
    };

    let exit_code = if report.discovered_files > 0 && report.readable_files == 0 {
        constants::CLI_EXIT_FAILURE
    } else if !report.issues.is_empty() {
        constants::CLI_EXIT_PARTIAL
    } else {
        constants::CLI_EXIT_SUCCESS
    };

    Ok((report, published_path, exit_code, options.language))
}

/// Retrieves a translated text from catalogs with English fallback.
///
/// ## Arguments / Returns
/// Accepts catalogs, language code, and translation key, returning the resolved string.
///
/// ## Errors
/// Falls back to English if the key is missing from the catalog.
fn translate(
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    language: &str,
    key: &str,
) -> String {
    exlgrep_core::i18n::resolve_catalog_text(catalogs, language, key)
        .unwrap_or_else(|| constants::CLI_FALLBACK_ENGLISH.to_string())
}

/// Translates a CLI error message into the designated display language.
///
/// ## Arguments / Returns
/// Accepts error string, catalogs, and language code, returning the translated message.
///
/// ## Errors
/// Returns the original error string if no matching translation template is found.
fn translate_error(
    error: &str,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    language: &str,
) -> String {
    for (prefix, key) in constants::CLI_ERROR_TRANSLATION_KEYS {
        if let Some(detail) = error.strip_prefix(prefix) {
            let label = translate(catalogs, language, key);
            let detail = detail.trim_start_matches(constants::CLI_ERROR_SEPARATOR);
            if label.contains(constants::CLI_TRANSLATION_PATH_PLACEHOLDER) {
                return label.replace(constants::CLI_TRANSLATION_PATH_PLACEHOLDER, detail);
            }
            if label.contains(constants::CLI_TRANSLATION_OPTION_PLACEHOLDER) {
                return label.replace(constants::CLI_TRANSLATION_OPTION_PLACEHOLDER, detail);
            }
            return if detail.is_empty() {
                label
            } else {
                format!("{label}: {detail}")
            };
        }
    }
    error.to_string()
}

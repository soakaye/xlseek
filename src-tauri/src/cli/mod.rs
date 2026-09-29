//! ## 処理内容
//! 引数、共通Excel検索、翻訳済み診断、一時保存と結果公開をつなぐCLI実行層。
//! ## 引数・戻り値
//! OS引数を受け、プロセス終了コード`i32`を返す。
//! ## エラー
//! 不正要求・全体検索失敗・保存失敗は2、部分失敗は結果を保存して1。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): CLI実行処理を追加。

pub mod args;
pub mod output;

use crate::constants;
use crate::models::{SearchIssue, SearchMatch, SearchReport};
use args::{CliOptions, ParseOutcome};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// ## 処理内容
/// プロセス引数を解析し、GUIを起動せずExcel検索・保存を実行する。
/// ## 引数・戻り値
/// OS引数イテレーターを受け、終了状態を表す`i32`を返す。
/// ## エラー
/// 翻訳・解析・検索・保存失敗は標準エラーへ出し、失敗終了コードを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI実行フローを実装。
pub fn run<I, S>(args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let catalogs = match crate::i18n::load_embedded_catalogs() {
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
            match execute(options, &catalogs) {
                Ok((report, output_path, exit_code, language)) => {
                    let summary =
                        translate(&catalogs, &language, constants::CLI_TRANSLATION_SUMMARY_KEY)
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
                                &output_path.display().to_string(),
                            );
                    println!("{summary}");
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

/// ## 処理内容
/// 検索ファイルを検出・抽出し、SearchReportと出力ファイルを作成する。
/// ## 引数・戻り値
/// 検証済みCliOptionsと翻訳辞書を受け、レポート・公開パス・終了コードを返す。
/// ## エラー
/// 入出力競合、全体的な検索不能、exportまたは公開失敗をErrで返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI検索と出力接続を追加。
fn execute(
    options: CliOptions,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
) -> Result<(SearchReport, PathBuf, i32, String), String> {
    let started = Instant::now();
    let output_guard = output::OutputGuard::new(options.output_path.clone(), options.overwrite)?;
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
    let mut report = SearchReport::default();
    let mut matches = Vec::<SearchMatch>::new();
    let input = &options.input_path;
    let entries = WalkDir::new(input).follow_links(false).into_iter();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                report.issues.push(SearchIssue {
                    path: error
                        .path()
                        .map(Path::to_path_buf)
                        .unwrap_or_else(|| input.clone()),
                    stage: constants::SEARCH_STAGE_DISCOVERY.to_string(),
                    sheet_name: None,
                    cause: error.to_string(),
                });
                if error.depth() == constants::CLI_ROOT_WALK_DEPTH {
                    return Err(format!(
                        "{}: {}",
                        constants::ERR_CLI_INPUT_PATH,
                        input.display()
                    ));
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
        let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
            continue;
        };
        let extension = format!(
            "{}{}",
            constants::CLI_EXTENSION_PREFIX,
            extension.to_ascii_lowercase()
        );
        if !options.query.extensions.contains(&extension) {
            continue;
        }
        output_guard.check_input(path)?;
        report.discovered_files += 1;
        report.scanned_files += 1;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::search::parser::parse_and_search_file(path, &options.query, regex.as_ref(), None)
        })) {
            Ok(Ok(mut found)) => {
                report.readable_files += 1;
                matches.append(&mut found);
            }
            Ok(Err(cause)) => report.issues.push(SearchIssue {
                path: path.to_path_buf(),
                stage: constants::SEARCH_STAGE_WORKBOOK.to_string(),
                sheet_name: None,
                cause,
            }),
            Err(_) => report.issues.push(SearchIssue {
                path: path.to_path_buf(),
                stage: constants::SEARCH_STAGE_WORKBOOK.to_string(),
                sheet_name: None,
                cause: constants::ERR_CLI_PANIC.to_string(),
            }),
        }
    }
    report.failed_files = report
        .issues
        .iter()
        .filter(|issue| issue.stage == constants::SEARCH_STAGE_WORKBOOK)
        .count();
    report.matches_found = matches.len();
    report.elapsed_ms = started.elapsed().as_millis() as u64;
    for issue in &report.issues {
        eprintln!(
            "{}{}{}",
            issue.path.display(),
            constants::CLI_ERROR_SEPARATOR,
            issue.cause
        );
    }
    let output_path =
        output_guard.publish(&matches, options.format, &options.language, catalogs)?;
    let exit_code = if report.discovered_files > 0 && report.readable_files == 0 {
        constants::CLI_EXIT_FAILURE
    } else if !report.issues.is_empty() {
        constants::CLI_EXIT_PARTIAL
    } else {
        constants::CLI_EXIT_SUCCESS
    };
    Ok((report, output_path, exit_code, options.language))
}

/// ## 処理内容
/// 指定言語のカタログから文字列を取得し、必須フォールバックを返す。
/// ## 引数・戻り値
/// カタログ、言語、キーを受け、翻訳文字列を返す。
/// ## エラー
/// キー欠落時は必須英語フォールバックを使う。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI翻訳参照を追加。
fn translate(
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    language: &str,
    key: &str,
) -> String {
    crate::i18n::resolve_catalog_text(catalogs, language, key)
        .unwrap_or_else(|| constants::CLI_FALLBACK_ENGLISH.to_string())
}

/// ## 処理内容
/// CLI処理エラーを指定言語で表示可能な文面へ変換する。
/// ## 引数・戻り値
/// エラー文面、翻訳辞書、言語を受け、翻訳済み文字列を返す。
/// ## エラー
/// 対応する翻訳がない場合は元のエラー文面を返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLIエラー文面の翻訳を追加。
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

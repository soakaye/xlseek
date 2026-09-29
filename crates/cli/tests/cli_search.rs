//! ## 処理内容
//! CLI実バイナリがGUIを開かずにExcel検索結果を保存する契約を検証する。
//! ## 引数・戻り値
//! テストはCargo提供のCLI実行ファイルを起動し、プロセス状態と出力内容を検証する。
//! ## エラー
//! プロセス起動・ファイル操作・アサーションの失敗でテストが失敗する。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): US1 CLI実プロセステストを追加。

use calamine::Reader;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const TEST_DIRECTORY_PREFIX: &str = "exlgrep-cli-search";
const QUERY_TEXT: &str = "Financial Report Q3";
const FORMAT_CSV: &str = "csv";
const OUTPUT_CSV_NAME: &str = "result.csv";
const OUTPUT_XLSX_NAME: &str = "result.xlsx";
const TEST_EMPTY_DIRECTORY_NAME: &str = "empty";
const TEST_BROKEN_DIRECTORY_NAME: &str = "broken";
const TEST_PARTIAL_DIRECTORY_NAME: &str = "partial";
const TEST_BROKEN_WORKBOOK_NAME: &str = "broken.xlsx";
const TEST_DIRECTORY_SEPARATOR: &str = "-";
static TEST_DIRECTORY_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// ## 処理内容
/// 同一テストプロセス内で競合しない一時フォルダーを生成する。
/// ## 引数・戻り値
/// 固定prefixを受け、プロセスIDと連番を含むPathBufを返す。
/// ## エラー
/// パスの組み立てのみを行い、I/Oエラーやpanicは発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 並行CLI統合テスト用に追加。
fn unique_test_directory() -> std::path::PathBuf {
    let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "{TEST_DIRECTORY_PREFIX}{TEST_DIRECTORY_SEPARATOR}{}{TEST_DIRECTORY_SEPARATOR}{sequence}",
        std::process::id()
    ))
}

/// ## 処理内容
/// リポジトリルートの絶対パスを解決する。
/// ## 引数・戻り値
/// 引数なし。`PathBuf` を返す。
/// ## エラー
/// ルートが見つからない場合は panic する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Antigravity): クレート分離に伴うルートパス解決。
fn repo_root() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("../../tests/fixtures").exists() {
        manifest.join("../..")
    } else {
        manifest
            .parent()
            .expect("repository root must exist")
            .to_path_buf()
    }
}

/// ## 処理内容
/// 有効な単一ブックを検索し、GUIなしでCSVへ一致結果を保存する。
/// ## 引数・戻り値
/// 引数なし。失敗条件ではassertがテストを失敗させる。
/// ## エラー
/// プロセス起動・出力読取失敗、終了コードまたは内容の不一致でテスト失敗。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CSV CLI検索のREDテストを追加。
#[test]
fn exports_search_results_from_a_single_workbook_without_gui() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");
    let output = output_dir.join(OUTPUT_CSV_NAME);

    let result = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg("--path")
        .arg(fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg(FORMAT_CSV)
        .arg("--output")
        .arg(&output)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    let contents = fs::read_to_string(&output).expect("CSV result must be written");
    assert!(contents.contains(QUERY_TEXT));
    let xlsx = output_dir.join(OUTPUT_XLSX_NAME);
    let excel_result = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg("--path")
        .arg(root.join("tests/fixtures/sample_report.xlsx"))
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg(exlgrep_cli::constants::CLI_FORMAT_XLSX)
        .arg("--output")
        .arg(&xlsx)
        .output()
        .expect("CLI Excel process must start");
    assert_eq!(
        excel_result.status.code(),
        Some(exlgrep_cli::constants::CLI_EXIT_SUCCESS)
    );
    let mut workbook = calamine::open_workbook_auto(&xlsx).expect("Excel output must open");
    let sheet_name = workbook.sheet_names().first().cloned().unwrap();
    let range = workbook.worksheet_range(&sheet_name).unwrap();
    assert!(range
        .rows()
        .flatten()
        .any(|cell| cell.to_string().contains(QUERY_TEXT)));
    let _ = fs::remove_dir_all(output_dir);
}

/// ## 処理内容
/// 空フォルダー、読取不能ブック、部分成功、不正regexを終了コードと保存結果で区別する。
/// ## 引数・戻り値
/// 引数なし。各CLIプロセスの終了状態と出力ファイルを検証する。
/// ## エラー
/// ファイル操作、CLI起動、終了状態または出力内容の不一致でテストが失敗する。
/// ## 変更履歴
/// - v1.1.0 (2026-09-29, Codex): CLI失敗状態のプロセス検証を追加。
#[test]
fn distinguishes_empty_partial_total_and_invalid_regex_runs() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let work = unique_test_directory();
    let empty = work.join(TEST_EMPTY_DIRECTORY_NAME);
    let broken = work.join(TEST_BROKEN_DIRECTORY_NAME);
    let partial = work.join(TEST_PARTIAL_DIRECTORY_NAME);
    fs::create_dir_all(&empty).unwrap();
    fs::create_dir_all(&broken).unwrap();
    fs::create_dir_all(&partial).unwrap();
    let broken_file = broken.join(TEST_BROKEN_WORKBOOK_NAME);
    fs::write(
        &broken_file,
        exlgrep_cli::constants::CLI_TEST_INVALID_WORKBOOK_BYTES,
    )
    .unwrap();
    fs::copy(
        &fixture,
        partial.join(exlgrep_cli::constants::CLI_TEST_VALID_WORKBOOK_NAME),
    )
    .unwrap();
    fs::copy(&broken_file, partial.join(TEST_BROKEN_WORKBOOK_NAME)).unwrap();

    let empty_output = work.join(exlgrep_cli::constants::CLI_TEST_EMPTY_OUTPUT_NAME);
    assert_eq!(
        run_cli(&empty, QUERY_TEXT, &empty_output, FORMAT_CSV)
            .status
            .code(),
        Some(0)
    );
    assert!(fs::read(&empty_output)
        .unwrap()
        .starts_with(&exlgrep_cli::constants::CSV_UTF8_BOM));

    let broken_output = work.join(exlgrep_cli::constants::CLI_TEST_BROKEN_OUTPUT_NAME);
    assert_eq!(
        run_cli(&broken, QUERY_TEXT, &broken_output, FORMAT_CSV)
            .status
            .code(),
        Some(2)
    );
    assert!(fs::read_to_string(&broken_output)
        .unwrap()
        .contains(exlgrep_cli::constants::CLI_TEST_EXPECTED_HEADER_ID_JA));

    let partial_output = work.join(exlgrep_cli::constants::CLI_TEST_PARTIAL_OUTPUT_NAME);
    assert_eq!(
        run_cli(&partial, QUERY_TEXT, &partial_output, FORMAT_CSV)
            .status
            .code(),
        Some(1)
    );
    assert!(fs::read_to_string(&partial_output)
        .unwrap()
        .contains(QUERY_TEXT));

    let invalid_regex_output = work.join(exlgrep_cli::constants::CLI_TEST_REGEX_OUTPUT_NAME);
    let invalid_regex = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(exlgrep_cli::constants::CLI_TEST_INVALID_REGEX)
        .arg("--regex")
        .arg(exlgrep_cli::constants::CLI_BOOLEAN_TRUE)
        .arg("--format")
        .arg(FORMAT_CSV)
        .arg("--output")
        .arg(&invalid_regex_output)
        .output()
        .unwrap();
    assert_eq!(invalid_regex.status.code(), Some(2));
    assert!(!invalid_regex_output.exists());
    let _ = fs::remove_dir_all(work);
}

/// ## 処理内容
/// 位置引数（クエリ＋複数パス）を指定し、標準出力（stdout）へ純粋なCSVが出力され、サマリー文言が抑制されることを検証する。
/// ## 引数・戻り値
/// 引数なし。プロセスの stdout/stderr 出力内容と終了コードを検証する。
/// ## エラー
/// プロセス起動失敗、サマリー混入、またはCSVフォーマット不一致時にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, AI Agent): 位置引数とstdoutストリーミングのテストを追加。
#[test]
fn exports_search_results_to_stdout_with_positional_arguments() {
    let root = repo_root();
    let fixture1 = root.join("tests/fixtures/sample_report.xlsx");
    let fixture2 = root.join("tests/fixtures");

    let result = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg(QUERY_TEXT)
        .arg(&fixture1)
        .arg(&fixture2)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));

    // stdoutはBOMなしプレーンUTF-8 CSVであること
    assert!(!result
        .stdout
        .starts_with(&exlgrep_cli::constants::CSV_UTF8_BOM));

    let stdout_str = String::from_utf8(result.stdout).expect("stdout must be valid UTF-8");
    assert!(stdout_str.contains(QUERY_TEXT));

    // サマリー文言が出力されていないこと（UNIXパイプライン保護）
    assert!(!stdout_str.contains("検索完了:"));
    assert!(!stdout_str.contains("Files scanned:"));
}

/// ## 処理内容
/// 短縮オプション（-q, -p, -o）および出力拡張子（.xlsx）からのフォーマット自動推論を検証する。
/// ## 引数・戻り値
/// 引数なし。プロセスの終了コードと生成されたExcelファイル内容を検証する。
/// ## エラー
/// プロセス起動失敗、推論失敗、またはExcel検証失敗時にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, AI Agent): 短縮オプションとフォーマット推論テストを追加。
#[test]
fn supports_short_options_and_format_inference() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");
    let xlsx_output = output_dir.join("inferred.xlsx");

    let result = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg("-q")
        .arg(QUERY_TEXT)
        .arg("-p")
        .arg(&fixture)
        .arg("-o")
        .arg(&xlsx_output)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    assert!(xlsx_output.exists());

    // ファイル保存時はサマリーがstdoutに出力されること
    let stdout_str = String::from_utf8_lossy(&result.stdout);
    assert!(stdout_str.contains("検索完了:"));

    let mut workbook = calamine::open_workbook_auto(&xlsx_output).expect("Excel output must open");
    let sheet_name = workbook.sheet_names().first().cloned().unwrap();
    let range = workbook.worksheet_range(&sheet_name).unwrap();
    assert!(range
        .rows()
        .flatten()
        .any(|cell| cell.to_string().contains(QUERY_TEXT)));

    let _ = fs::remove_dir_all(output_dir);
}

/// ## 処理内容
/// 破損ファイルを含むディレクトリを探索・並行解析する際、即時にstderrへエラーが通知され、
/// かつ正常ファイルの結果が収集されて部分成功（終了コード1）となることを検証する。
/// ## 引数・戻り値
/// 引数なし。stderrへの即時エラー出力と終了コードを検証する。
/// ## エラー
/// プロセス起動失敗、stderr未出力、または終了コード不一致時にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, AI Agent): 非同期パイプライン即時エラー通知テストを追加。
#[test]
fn notifies_errors_in_realtime_during_async_pipeline() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let work = unique_test_directory();
    let mixed_dir = work.join("mixed");
    fs::create_dir_all(&mixed_dir).unwrap();

    let broken_file = mixed_dir.join(TEST_BROKEN_WORKBOOK_NAME);
    fs::write(
        &broken_file,
        exlgrep_cli::constants::CLI_TEST_INVALID_WORKBOOK_BYTES,
    )
    .unwrap();
    fs::copy(
        &fixture,
        mixed_dir.join(exlgrep_cli::constants::CLI_TEST_VALID_WORKBOOK_NAME),
    )
    .unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg(QUERY_TEXT)
        .arg(&mixed_dir)
        .output()
        .expect("CLI process must start");

    // 部分成功で exit_code 1
    assert_eq!(result.status.code(), Some(1));

    // stderrに壊れたファイルに関するエラーが出力されていること
    let stderr_str = String::from_utf8_lossy(&result.stderr);
    assert!(stderr_str.contains(TEST_BROKEN_WORKBOOK_NAME));

    // stdoutには正常ファイルの一致結果が出力されていること
    let stdout_str = String::from_utf8_lossy(&result.stdout);
    assert!(stdout_str.contains(QUERY_TEXT));

    let _ = fs::remove_dir_all(work);
}

/// ## 処理内容
/// CLIへ必須の入力、検索語、形式、出力引数を渡して検索を実行する。
/// ## 引数・戻り値
/// 入力・検索語・出力・形式を受け、OSの`Output`を返す。
/// ## エラー
/// CLI起動失敗でテストを失敗させる。
/// ## 変更履歴
/// - v1.1.0 (2026-09-29, Codex): 失敗状態テスト実行関数を追加。
fn run_cli(
    input: &std::path::Path,
    query: &str,
    output: &std::path::Path,
    format: &str,
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"))
        .arg("--path")
        .arg(input)
        .arg("--query")
        .arg(query)
        .arg("--format")
        .arg(format)
        .arg("--output")
        .arg(output)
        .output()
        .expect("CLI process must start")
}

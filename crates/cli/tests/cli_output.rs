//! ## 処理内容
//! CLI出力公開の上書き方針と検索入力保護を実プロセスで検証する。
//! ## 引数・戻り値
//! CargoのCLIバイナリを実行し、終了状態とファイル内容を照合する。
//! ## エラー
//! I/O・実行・アサーション失敗でテストが失敗する。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): 出力保護の統合テストを追加。

use std::fs;
use std::path::PathBuf;
use std::process::Command;

const TEST_DIRECTORY_PREFIX: &str = "exlgrep-cli-output";
const TEST_EXISTING_CONTENT: &str = "preserve-existing-output";
const TEST_QUERY: &str = "Financial Report Q3";
const TEST_OUTPUT_NAME: &str = "results.csv";

/// ## 処理内容
/// リポジトリルートの絶対パスを解決する。
/// ## 引数・戻り値
/// 引数なし。`PathBuf` を返す。
/// ## エラー
/// ルートが見つからない場合は panic する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Antigravity): クレート分離に伴うルートパス解決。
fn repo_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
/// 既定の出力拒否、明示上書き、出力先と入力の同一性保護を確認する。
/// ## 引数・戻り値
/// 引数なし。テスト用一時パスとプロセス終了状態を検証する。
/// ## エラー
/// ファイル操作失敗や既存ファイル内容の変化でテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 出力衝突・入力保護テストを追加。
/// - v1.1.0 (2026-09-29, Antigravity): repo_rootおよびexlgrep_cli定数参照へ更新。
#[test]
fn rejects_existing_output_by_default_and_never_overwrites_input() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let directory =
        std::env::temp_dir().join(format!("{TEST_DIRECTORY_PREFIX}-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("temporary directory must be created");
    let output = directory.join(TEST_OUTPUT_NAME);
    fs::write(&output, TEST_EXISTING_CONTENT).expect("existing output must be created");

    let denied = run_cli(&fixture, TEST_QUERY, &output, false);
    assert_eq!(
        denied.status.code(),
        Some(exlgrep_cli::constants::CLI_EXIT_FAILURE)
    );
    assert_eq!(fs::read_to_string(&output).unwrap(), TEST_EXISTING_CONTENT);

    let replaced = run_cli(&fixture, TEST_QUERY, &output, true);
    assert_eq!(
        replaced.status.code(),
        Some(exlgrep_cli::constants::CLI_EXIT_SUCCESS)
    );
    assert!(fs::read_to_string(&output).unwrap().contains(TEST_QUERY));

    let original_input = fs::read(&fixture).expect("input workbook must be readable");
    let input_output = run_cli(&fixture, TEST_QUERY, &fixture, true);
    assert_eq!(
        input_output.status.code(),
        Some(exlgrep_cli::constants::CLI_EXIT_FAILURE)
    );
    assert_eq!(fs::read(&fixture).unwrap(), original_input);
    let _ = fs::remove_dir_all(directory);
}

/// ## 処理内容
/// 一回分の検索引数を実CLIへ渡す。
/// ## 引数・戻り値
/// 入力、検索語、出力先、上書き指定を受け、`Output`を返す。
/// ## エラー
/// プロセス起動失敗時はテストを失敗させる。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI出力テスト実行関数を追加。
fn run_cli(
    input: &std::path::Path,
    query: &str,
    output: &std::path::Path,
    overwrite: bool,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_exlgrep-cli"));
    command
        .arg("--path")
        .arg(input)
        .arg("--query")
        .arg(query)
        .arg("--format")
        .arg(
            if output.extension().and_then(|value| value.to_str())
                == Some(exlgrep_cli::constants::CLI_FORMAT_XLSX)
            {
                exlgrep_cli::constants::CLI_FORMAT_XLSX
            } else {
                exlgrep_cli::constants::CLI_FORMAT_CSV
            },
        )
        .arg("--output")
        .arg(output);
    if overwrite {
        command.arg(exlgrep_cli::constants::CLI_TEST_OVERWRITE_OPTION);
    }
    command.output().expect("CLI process must start")
}

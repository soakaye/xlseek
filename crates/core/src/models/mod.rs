//! # データモデル定義モジュール (models/mod.rs)
//!
//! ## 処理内容
//! 検索条件、検索結果、スキャン進捗、プレビュー情報、エクスポート要求、
//! および連携アプリケーション情報に関するデータ構造体を定義する。
//! 憲章原則II（定数参照）および原則III（4要素ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照への置換と4要素ヘッダコメントの網羅。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ## 処理内容
/// 検索実行時の検索条件を保持するクエリ構造体。
/// ## 引数・戻り値
/// キーワード、対象フォルダ、検索オプション、対象拡張子を保持する。
/// `include_shape` は Shape テキスト検索の有効状態を表し、既定値は true。
/// `include_value` はセル値検索を表し、既定値は true。
/// ## エラー / 例外発生条件
/// Serde のデシリアライズで必須キーワードまたは対象フォルダが欠けると失敗する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
/// - v1.3.0 (2026-09-28, Codex): Shape 検索条件を追加。
/// - v1.4.0 (2026-09-29, Codex): JSON互換の値検索条件を追加。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
    pub target_dir: String,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default = "default_include_value")]
    pub include_value: bool,
    #[serde(default)]
    pub use_regex: bool,
    #[serde(default = "default_true")]
    pub include_formula: bool,
    #[serde(default = "default_true")]
    pub include_comment: bool,
    #[serde(default = "default_true")]
    pub include_shape: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
}

/// ## 処理内容
/// 旧JSON要求で省略された値検索フラグの既定値を返す。
/// ## 引数・戻り値
/// 引数なし。定数 `DEFAULT_INCLUDE_VALUE` のbool値を返す。
/// ## エラー
/// panicは発生しない。
/// ## 変更履歴
/// - v1.4.0 (2026-09-29, Codex): 値検索のJSON互換既定値を追加。
fn default_include_value() -> bool {
    // 定数参照: crate::constants::DEFAULT_INCLUDE_VALUE を使用。
    crate::constants::DEFAULT_INCLUDE_VALUE
}

/// ## 処理内容
/// serdeデフォルト値用の真値（true）を返却する補助関数。
///
/// ## 引数
/// なし
///
/// ## 戻り値
/// - `bool`: 常に `true`
///
/// ## エラー / 例外発生条件
/// panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
fn default_true() -> bool {
    true
}

/// ## 処理内容
/// 検索対象のデフォルトExcel拡張子一覧（.xlsx, .xlsm, .xlsb, .xls）を返却する。
///
/// ## 引数
/// なし
///
/// ## 戻り値
/// - `Vec<String>`: 定数定義から取得した拡張子リスト
///
/// ## エラー / 例外発生条件
/// panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
fn default_extensions() -> Vec<String> {
    // 定数参照: crate::constants::DEFAULT_EXTENSIONS を使用
    crate::constants::DEFAULT_EXTENSIONS
        .iter()
        .map(|&ext| ext.to_string())
        .collect()
}

/// ## 処理内容
/// 検索結果がヒットしたセルの要素種別を表す列挙型。
/// ## 引数・戻り値
/// バリアントは `SearchMatch.match_type` に保存され、Serde で IPC JSON へ直列化される。
/// ## エラー / 例外発生条件
/// 既知バリアント以外をデシリアライズすると失敗する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
/// - v1.3.0 (2026-09-28, Codex): Shape バリアントを追加。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchType {
    CellValue,
    Formula,
    Comment,
    HiddenSheet,
    Shape,
}

/// ## 処理内容
/// 検索に一致したセルまたは Shape の結果を格納する構造体。
/// ## 引数・戻り値
/// Shape は `shape_name` を持ち、`sheet_hidden` がシート可視状態を表す。
/// 未知アンカーではセル番地が空、内部行列が 0、数式は `None` となる。
/// ## エラー / 例外発生条件
/// 値保持と Serde 変換のみで、変換不能な型やデータの場合にエラーとなる。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
/// - v1.3.0 (2026-09-28, Codex): Shape 名とシート可視状態を追加。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub id: u64,
    pub file_name: String,
    pub full_path: String,
    pub sheet_name: String,
    pub cell_address: String,
    pub row_index: u32,
    pub col_index: u32,
    pub col_name: String,
    pub match_type: MatchType,
    pub shape_name: Option<String>,
    pub sheet_hidden: bool,
    pub snippet: String,
    pub full_content: String,
    pub formula: Option<String>,
    pub sheets_in_workbook: Vec<String>,
}

/// ## 処理内容
/// ファイル検索から得た一致結果とファイル内の部分問題を保持する。
/// ## 引数・戻り値
/// matchesは`Vec<SearchMatch>`、issuesは`Vec<SearchIssue>`。
/// ## エラー
/// 値保持のみでエラーは発生しない。ブック全体を開けない失敗は呼び出し側がErrとする。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 詳細検索ファイル結果を追加。
#[derive(Debug, Default)]
pub struct FileSearchResult {
    pub matches: Vec<SearchMatch>,
    pub issues: Vec<SearchIssue>,
}

/// ## 処理内容
/// 検索中に発生した入力ファイル・シート・抽出段階の問題を記録する。
/// ## 引数・戻り値
/// PathBuf、stage文字列、任意のシート名、原因文字列を保持する。
/// ## エラー
/// 値保持のみでエラーは発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 検索問題モデルを追加。
#[derive(Debug)]
pub struct SearchIssue {
    pub path: std::path::PathBuf,
    pub stage: String,
    pub sheet_name: Option<String>,
    pub cause: String,
}

/// ## 処理内容
/// 一回の検索における対象数、成功数、一致数、時間、問題を集計する。
/// ## 引数・戻り値
/// 件数はusize、経過時間はu64ミリ秒、問題は`Vec<SearchIssue>`。
/// ## エラー
/// 値保持のみでエラーは発生しない。failed_filesはブック単位で集計する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 検索実行レポートを追加。
#[derive(Debug, Default)]
pub struct SearchReport {
    pub discovered_files: usize,
    pub scanned_files: usize,
    pub readable_files: usize,
    pub failed_files: usize,
    pub matches_found: usize,
    pub elapsed_ms: u64,
    pub issues: Vec<SearchIssue>,
}

/// ## 処理内容
/// 検索走査エンジンの現在の進捗状態を表す列挙型。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanState {
    Scanning,
    Completed,
    Cancelled,
    Error,
}

/// ## 処理内容
/// ロケール非依存の進捗段階を表す。
/// ## 引数・戻り値
/// 引数なし。シリアライズ可能な段階値を表す。
/// ## エラー
/// panic は発生しない。
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, AI Agent): UI 文言と進捗段階を分離。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanPhase {
    Preparing,
    Discovering,
    Scanning,
    Finished,
}

/// ## 処理内容
/// 利用者向けに翻訳可能な安定した失敗コード。
/// ## 引数・戻り値
/// 引数なし。API 契約用のコードを表す。
/// ## エラー
/// panic は発生しない。
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, AI Agent): エラーコード契約を追加。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidRegex,
    PathNotFound,
    WorkbookOpenFailed,
    PreviewFailed,
    ExportFailed,
    AppLaunchFailed,
    FolderOpenFailed,
    PermissionDenied,
    SearchFailed,
    InternalError,
}

/// ## 処理内容
/// Tauri コマンドが利用者へ返す構造化エラーを保持する。
/// ## 引数・戻り値
/// `ErrorCode` を受け取り、`code` プロパティを持つシリアライズ可能な値を生成する。
/// ## エラー
/// 外部ライブラリの文言やユーザーデータは含めない。
/// ## 変更履歴
/// - v1.2.0 (2026-09-26, AI Agent): Tauri エラー応答を JSON コード形式に統一。
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CommandError {
    pub code: ErrorCode,
}

impl From<String> for CommandError {
    fn from(_: String) -> Self {
        Self {
            code: ErrorCode::InternalError,
        }
    }
}

/// ## 処理内容
/// 検索進捗通知時にTauriイベントペイロードとして送信される情報構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub state: ScanState,
    pub phase: ScanPhase,
    pub error_code: Option<ErrorCode>,
    pub scanned_files: usize,
    pub total_files: usize,
    pub matches_found: usize,
    pub current_file: String,
    pub elapsed_ms: u64,
}

/// ## 処理内容
/// セル周辺プレビュー機能で返却されるグリッドデータ構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPreviewData {
    pub target_row: u32,
    pub target_col: u32,
    pub columns: Vec<PreviewColumn>,
    pub rows: Vec<PreviewRow>,
    pub sheets_in_workbook: Vec<String>,
}

/// ## 処理内容
/// プレビューグリッドの列ヘッダー情報構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewColumn {
    pub key: String,
    pub label: String,
}

/// ## 処理内容
/// プレビューグリッドの1行分のデータ情報構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRow {
    pub row_number: u32,
    pub cells: HashMap<String, CellValueInfo>,
}

/// ## 処理内容
/// プレビュー内の単一セルの値およびターゲット判定構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellValueInfo {
    pub value: String,
    pub is_target: bool,
    pub formula: Option<String>,
}

/// ## 処理内容
/// エクスポート形式（CSVまたはExcel）を指定する列挙型。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

/// ## 処理内容
/// フロントエンドからのエクスポート要求パラメータ構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    #[serde(default = "default_export_language")]
    pub language: String,
    pub output_path: String,
    pub items: Vec<SearchMatch>,
}

/// ## 処理内容
/// 旧クライアントからのエクスポート要求に英語設定を補う。
/// ## 引数・戻り値
/// 引数なし。`String` の `en` を返す。
/// ## エラー
/// panic は発生しない。
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, AI Agent): ExportRequest 互換デフォルトを追加。
fn default_export_language() -> String {
    crate::constants::DEFAULT_EXPORT_LANGUAGE.to_string()
}

/// ## 処理内容
/// OS上で利用可能な連携アプリケーション（Excel等）の情報構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedApp {
    pub id: String,
    pub name: String,
    pub executable_path: String,
    pub is_default: bool,
    pub icon_hint: Option<String>,
}

#[cfg(test)]
mod localization_tests {
    use super::{
        CommandError, ErrorCode, ExportRequest, ScanPhase, ScanProgress, ScanState, SearchQuery,
    };

    /// ## 処理内容
    /// API 進捗段階とエラーコードがロケール非依存の snake_case で出力されることを確認する。
    /// ## 引数・戻り値
    /// 引数なし。アサーションのみを実行する。
    /// ## エラー
    /// シリアライズ失敗または期待値不一致でテストが失敗する。
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): ローカライズ契約テストを追加。
    #[test]
    fn serializes_language_neutral_codes() {
        assert_eq!(
            serde_json::to_string(&ScanPhase::Discovering).unwrap(),
            "\"discovering\""
        );
        assert_eq!(
            serde_json::to_string(&ErrorCode::InternalError).unwrap(),
            "\"internal_error\""
        );
        assert_eq!(
            serde_json::to_string(&CommandError {
                code: ErrorCode::InternalError
            })
            .unwrap(),
            "{\"code\":\"internal_error\"}"
        );
        let progress = ScanProgress {
            state: ScanState::Scanning,
            phase: ScanPhase::Discovering,
            error_code: None,
            scanned_files: 0,
            total_files: 0,
            matches_found: 0,
            current_file: String::new(),
            elapsed_ms: 0,
        };
        let serialized = serde_json::to_value(progress).unwrap();
        assert_eq!(serialized["phase"], "discovering");
        assert_eq!(serialized["current_file"], "");
        assert!(serialized["error_code"].is_null());
    }

    /// ## 処理内容
    /// エクスポート言語コードの明示値と旧要求用デフォルトを検証する。
    /// ## 引数・戻り値
    /// 引数なし。JSON 契約のアサーションを実行する。
    /// ## エラー
    /// 不正なデータ形状または期待値でテストが失敗する。
    /// ## 変更履歴
    /// - v1.2.0 (2026-09-26, AI Agent): 出力言語のシリアライズ契約を追加。
    #[test]
    fn export_request_reads_language_and_defaults_legacy_requests_to_english() {
        let request: ExportRequest =
            serde_json::from_str(r#"{"format":"csv","output_path":"","items":[],"language":"ja"}"#)
                .unwrap();
        assert_eq!(request.language, "ja");
        let legacy: ExportRequest =
            serde_json::from_str(r#"{"format":"csv","output_path":"","items":[]}"#).unwrap();
        assert_eq!(legacy.language, crate::constants::DEFAULT_EXPORT_LANGUAGE);
    }

    /// ## 処理内容
    /// 旧JSON要求で値検索が有効になり、明示falseは維持されることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。SearchQueryをJSONへ変換して値検索フィールドを確認する。
    /// ## エラー
    /// JSON変換失敗または期待値不一致でテストが失敗する。
    /// ## 変更履歴
    /// - v1.4.0 (2026-09-29, Codex): CLI検索互換の回帰テストを追加。
    #[test]
    fn legacy_search_query_defaults_value_search_to_true() {
        let legacy: SearchQuery = serde_json::from_value(serde_json::json!({
            "keyword": "needle",
            "target_dir": "input",
        }))
        .unwrap();
        let serialized = serde_json::to_value(legacy).unwrap();
        assert_eq!(
            serialized[crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD],
            true
        );

        let explicit: SearchQuery = serde_json::from_value(serde_json::json!({
            "keyword": "needle",
            "target_dir": "input",
            (crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD): false,
        }))
        .unwrap();
        let serialized = serde_json::to_value(explicit).unwrap();
        assert_eq!(
            serialized[crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD],
            false
        );
    }
}

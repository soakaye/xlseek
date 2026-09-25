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
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
    pub target_dir: String,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default)]
    pub use_regex: bool,
    #[serde(default = "default_true")]
    pub include_formula: bool,
    #[serde(default = "default_true")]
    pub include_comment: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
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
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchType {
    CellValue,
    Formula,
    Comment,
    HiddenSheet,
}

/// ## 処理内容
/// 検索に一致した単一セル/行の検索結果アイテム構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
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
    pub snippet: String,
    pub full_content: String,
    pub formula: Option<String>,
    pub sheets_in_workbook: Vec<String>,
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
/// 検索進捗通知時にTauriイベントペイロードとして送信される情報構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub state: ScanState,
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
    pub output_path: String,
    pub items: Vec<SearchMatch>,
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

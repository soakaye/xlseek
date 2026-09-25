use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 検索条件クエリ
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

fn default_true() -> bool {
    true
}

fn default_extensions() -> Vec<String> {
    vec![
        ".xlsx".to_string(),
        ".xlsm".to_string(),
        ".xlsb".to_string(),
        ".xls".to_string(),
    ]
}

/// 一致種別 Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchType {
    CellValue,
    Formula,
    Comment,
    HiddenSheet,
}

/// 検索一致アイテム (行単位)
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

/// スキャン進捗状態 Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanState {
    Scanning,
    Completed,
    Cancelled,
    Error,
}

/// スキャン進捗通知イベントペイロード
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub state: ScanState,
    pub scanned_files: usize,
    pub total_files: usize,
    pub matches_found: usize,
    pub current_file: String,
    pub elapsed_ms: u64,
}

/// 周辺セルプレビュー情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPreviewData {
    pub target_row: u32,
    pub target_col: u32,
    pub columns: Vec<PreviewColumn>,
    pub rows: Vec<PreviewRow>,
    pub sheets_in_workbook: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewColumn {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRow {
    pub row_number: u32,
    pub cells: HashMap<String, CellValueInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellValueInfo {
    pub value: String,
    pub is_target: bool,
    pub formula: Option<String>,
}

/// エクスポートフォーマット Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

/// エクスポートリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub output_path: String,
    pub items: Vec<SearchMatch>,
}

/// サポートアプリケーション情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedApp {
    pub id: String,
    pub name: String,
    pub executable_path: String,
    pub is_default: bool,
    pub icon_hint: Option<String>,
}


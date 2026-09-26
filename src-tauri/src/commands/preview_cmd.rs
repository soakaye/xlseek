//! # セルプレビューコマンドハンドラ (commands/preview_cmd.rs)
//!
//! ## 処理内容
//! フロントエンドからのセルプレビュー要求（get_cell_preview）を受信し、
//! 指定セルの周辺セルデータおよび数式を抽出して返却する。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、4要素ヘッダコメント付与。

use crate::models::{CellPreviewData, CommandError, ErrorCode};
use crate::search::preview::extract_cell_preview;

/// ## 処理内容
/// 指定されたExcelファイルの特定シート・特定セル番地を中心とした周辺セルプレビューデータを取得する。
///
/// ## 引数
/// - `file_path`: `String` - 対象Excelファイルのパス
/// - `sheet_name`: `String` - プレビュー対象のシート名
/// - `row_index`: `u32` - 1始まりの対象セル行番号
/// - `col_index`: `u32` - 1始まりの対象セル列番号
///
/// ## 戻り値
/// - `Result<CellPreviewData, String>`: 成功時はプレビューグリッドデータ、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイルオープン失敗、シート不在、または非同期タスク失敗時に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn get_cell_preview(
    file_path: String,
    sheet_name: String,
    row_index: u32,
    col_index: u32,
) -> Result<CellPreviewData, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        extract_cell_preview(&file_path, &sheet_name, row_index, col_index)
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })?
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })
}

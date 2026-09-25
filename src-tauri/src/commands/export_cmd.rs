//! # エクスポートコマンドハンドラ (commands/export_cmd.rs)
//!
//! ## 処理内容
//! フロントエンドからのエクスポート要求（export_results）を受信し、
//! 指定フォーマット（CSVまたはExcel）に応じて検索一致アイテムをファイルへ出力する。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、4要素ヘッダコメント付与。

use crate::export::{export_to_csv, export_to_xlsx};
use crate::models::{ExportFormat, ExportRequest};

/// ## 処理内容
/// 検索結果アイテム一覧を指定されたフォーマット（CSVまたはExcel）で指定パスへ書き出す。
///
/// ## 引数
/// - `request`: `ExportRequest` - エクスポート形式、出力先パス、およびアイテム一覧
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイル書き込み失敗時、または非同期タスク実行エラー時に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn export_results(request: ExportRequest) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || match request.format {
        ExportFormat::Csv => export_to_csv(&request.output_path, &request.items),
        ExportFormat::Xlsx => export_to_xlsx(&request.output_path, &request.items),
    })
    .await
    .map_err(|e| {
        // 定数参照: crate::constants::ERR_TASK_EXECUTION を使用
        format!("{}: {}", crate::constants::ERR_TASK_EXECUTION, e)
    })?
}

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
use crate::models::{CommandError, ErrorCode, ExportFormat, ExportRequest};
use tauri_plugin_i18n::PluginI18nExt;

/// ## 処理内容
/// 検索結果アイテム一覧を指定されたフォーマット（CSVまたはExcel）で指定パスへ書き出す。
///
/// ## 引数
/// - `app_handle`: `tauri::AppHandle` - プラグイン翻訳カタログを読み取るアプリハンドル
/// - `request`: `ExportRequest` - エクスポート形式、出力先パス、出力言語、およびアイテム一覧
///
/// ## 戻り値
/// - `Result<(), CommandError>`: 成功時 `Ok(())`、対応外言語または出力失敗時はコマンドエラー
///
/// ## エラー / 例外発生条件
/// 対応外言語、ファイル書き込み失敗、または非同期タスク実行エラー時に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, Codex): 固定言語とプラグインカタログを出力処理へ渡す。
#[tauri::command]
pub async fn export_results(
    app_handle: tauri::AppHandle,
    request: ExportRequest,
) -> Result<(), CommandError> {
    if request.language != crate::constants::LANGUAGE_JA
        && request.language != crate::constants::LANGUAGE_EN
    {
        return Err(CommandError {
            code: ErrorCode::InternalError,
        });
    }
    let catalogs = app_handle.i18n().get_translations_data();
    tauri::async_runtime::spawn_blocking(move || match request.format {
        ExportFormat::Csv => export_to_csv(
            &request.output_path,
            &request.items,
            &request.language,
            &catalogs,
        ),
        ExportFormat::Xlsx => export_to_xlsx(
            &request.output_path,
            &request.items,
            &request.language,
            &catalogs,
        ),
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::ExportFailed,
    })?
    .map_err(|_| CommandError {
        code: ErrorCode::ExportFailed,
    })
}

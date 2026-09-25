//! # Excel Grep コアライブラリ (lib.rs)
//!
//! ## 処理内容
//! Tauriデスクトップアプリケーションの初期化、プラグイン（dialog, shell）の登録、
//! 各種IPCコマンドハンドラの登録、およびアプリケーション実行ループを管理する。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。constantsモジュールの公開と憲章準拠ヘッダコメントの追加。

pub mod commands;
pub mod constants;
pub mod export;
pub mod models;
pub mod search;

use commands::AppState;
use search::engine::SearchEngine;
use std::sync::Arc;

/// ## 処理内容
/// Tauriアプリケーションを構築し、ステート管理、プラグイン、メニュー、
/// コマンドハンドラを登録してメインイベントループを実行する。
///
/// ## 引数
/// なし
///
/// ## 戻り値
/// なし
///
/// ## エラー / 例外発生条件
/// アプリケーション起動時のコンテキスト生成やランタイムエラー時にパニックする。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(SearchEngine::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                let menu = tauri::menu::Menu::default(app.handle())?;
                app.set_menu(menu)?;
            }
            Ok(())
        })
        .manage(AppState { engine })
        .invoke_handler(tauri::generate_handler![
            commands::start_search,
            commands::cancel_search,
            commands::get_cell_preview,
            commands::open_in_excel,
            commands::open_in_folder,
            commands::export_results,
            commands::resolve_dropped_path,
            commands::get_supported_apps,
            commands::launch_associated_app,
            commands::show_open_with_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//! # Excel Grep コアライブラリ (lib.rs)
//!
//! ## 処理内容
//! Tauriデスクトップアプリケーションの初期化、プラグイン（dialog, shell, i18n）の登録、
//! 各種IPCコマンドハンドラの登録、およびアプリケーション実行ループを管理する。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。constantsモジュールの公開と憲章準拠ヘッダコメントの追加。
//! - v1.1.0 (2026-09-26, AI Agent): macOSシステムメニューのカスタム構築 (create_app_menu) および About ダイアログ用メニューイベントハンドラ (on_menu_event) の追加。
//! - v1.3.0 (2026-09-29, Codex): GUIと独立したCLIモジュールを公開。
//! - v1.4.0 (2026-09-29, Antigravity): CLIモジュールをcrates/cliへ分離し、共有モジュールをexlgrep_coreから再エクスポート。

pub mod commands;
pub mod constants;

pub use exlgrep_core::{export, i18n, models, search};

use commands::AppState;
use search::engine::SearchEngine;
use std::sync::Arc;
#[cfg(target_os = "macos")]
use tauri_plugin_i18n::PluginI18nExt;

/// ## 処理内容
/// macOS向けのアプリケーションメニューを構築する。
/// 標準の About 項目をカスタムメニュー項目に置き換え、クリック時にアプリ内
/// About ダイアログを開くためのイベント発行を可能にする。
///
/// ## 引数
/// - `app_handle`: &tauri::AppHandle<R>
///
/// ## 戻り値
/// - `tauri::Result<tauri::menu::Menu<R>>`: 構築されたメニュー構造体
///
/// ## エラー / 例外発生条件
/// メニュー要素の生成やOS側メニュー登録時にエラーが発生した場合は Err を返す。
///
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, AI Agent): 初版作成。カスタムAboutメニュー項目の統合。
#[cfg(target_os = "macos")]
fn create_app_menu<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
    language: &str,
) -> tauri::Result<tauri::menu::Menu<R>> {
    use constants::{
        MENU_ITEM_ABOUT_ID, MENU_KEY_ABOUT, MENU_KEY_EDIT, MENU_KEY_FILE, MENU_KEY_HELP,
        MENU_KEY_VIEW, MENU_KEY_WINDOW,
    };
    use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

    let pkg_name = app_handle.package_info().name.clone();
    let catalogs = app_handle.i18n().get_translations_data();
    let menu_text = |key| -> tauri::Result<String> {
        crate::i18n::resolve_catalog_text(&catalogs, language, key).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                crate::constants::ERR_TRANSLATION_MISSING,
            )
            .into()
        })
    };

    // 1. アプリケーションサブメニュー (App Submenu)
    // 定数参照: constants::MENU_ITEM_ABOUT_ID と MENU_KEY_ABOUT
    let about_item = MenuItem::with_id(
        app_handle,
        MENU_ITEM_ABOUT_ID,
        menu_text(MENU_KEY_ABOUT)?,
        true,
        None::<&str>,
    )?;

    let app_submenu = Submenu::with_items(
        app_handle,
        &pkg_name,
        true,
        &[
            &about_item,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::services(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::hide(app_handle, None)?,
            &PredefinedMenuItem::hide_others(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::quit(app_handle, None)?,
        ],
    )?;

    // 2. ファイルサブメニュー (File Submenu)
    // 定数参照: constants::MENU_KEY_FILE
    let file_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_FILE)?,
        true,
        &[&PredefinedMenuItem::close_window(app_handle, None)?],
    )?;

    // 3. 編集サブメニュー (Edit Submenu)
    // 定数参照: constants::MENU_KEY_EDIT
    let edit_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_EDIT)?,
        true,
        &[
            &PredefinedMenuItem::undo(app_handle, None)?,
            &PredefinedMenuItem::redo(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::cut(app_handle, None)?,
            &PredefinedMenuItem::copy(app_handle, None)?,
            &PredefinedMenuItem::paste(app_handle, None)?,
            &PredefinedMenuItem::select_all(app_handle, None)?,
        ],
    )?;

    // 4. 表示サブメニュー (View Submenu)
    // 定数参照: constants::MENU_KEY_VIEW
    let view_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_VIEW)?,
        true,
        &[&PredefinedMenuItem::fullscreen(app_handle, None)?],
    )?;

    // 5. ウィンドウサブメニュー (Window Submenu)
    // 定数参照: constants::MENU_KEY_WINDOW
    let window_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_WINDOW)?,
        true,
        &[
            &PredefinedMenuItem::minimize(app_handle, None)?,
            &PredefinedMenuItem::maximize(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::close_window(app_handle, None)?,
        ],
    )?;

    // 6. ヘルプサブメニュー (Help Submenu)
    // 定数参照: constants::MENU_KEY_HELP
    let help_submenu = Submenu::with_items(app_handle, menu_text(MENU_KEY_HELP)?, true, &[])?;

    Menu::with_items(
        app_handle,
        &[
            &app_submenu,
            &file_submenu,
            &edit_submenu,
            &view_submenu,
            &window_submenu,
            &help_submenu,
        ],
    )
}

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
/// - v1.1.0 (2026-09-26, AI Agent): カスタムAboutメニューの登録とメニューイベントハンドラ (on_menu_event) の追加。
/// - v1.1.1 (2026-09-27, AI Agent): non-macOS向けに PluginI18nExt の import をスコープ化、setup クロージャの unused variable を解消。
/// - v1.2.0 (2026-09-27, Codex): ディレクトリ補完コマンドを invoke handler に登録。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(SearchEngine::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_i18n::init(None))
        .setup(|_app| {
            #[cfg(target_os = "macos")]
            {
                let menu = create_app_menu(_app.handle(), "en")?;
                _app.set_menu(menu)?;
            }
            Ok(())
        })
        .on_menu_event(|app_handle, event| {
            use constants::{EVENT_OPEN_ABOUT_DIALOG, MENU_ITEM_ABOUT_ID};
            use tauri::Emitter;

            // 定数参照: constants::MENU_ITEM_ABOUT_ID, constants::EVENT_OPEN_ABOUT_DIALOG
            if event.id() == MENU_ITEM_ABOUT_ID {
                let _ = app_handle.emit(EVENT_OPEN_ABOUT_DIALOG, ());
            }
        })
        .manage(AppState { engine })
        .invoke_handler(tauri::generate_handler![
            set_menu_locale,
            commands::start_search,
            commands::complete_directory_path,
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

/// ## 処理内容
/// macOS アプリメニューを現在の表示言語に合わせて再構築する。
/// ## 引数・戻り値
/// Tauri アプリハンドルと `ja` または `en` を受け取り、成功時 `Ok(())` を返す。
/// ## エラー
/// 不正な言語コードおよび macOS メニュー更新失敗時は英語エラーコードを返す。他 OS では成功する。
/// ## 変更履歴
/// - v1.2.0 (2026-09-26, AI Agent): 言語別メニュー更新を追加。
#[tauri::command]
fn set_menu_locale(
    app_handle: tauri::AppHandle,
    language: String,
) -> Result<(), models::CommandError> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(models::CommandError {
            code: models::ErrorCode::InternalError,
        });
    }
    #[cfg(target_os = "macos")]
    {
        let menu = create_app_menu(&app_handle, &language).map_err(|_| models::CommandError {
            code: models::ErrorCode::InternalError,
        })?;
        app_handle
            .set_menu(menu)
            .map_err(|_| models::CommandError {
                code: models::ErrorCode::InternalError,
            })?;
    }
    #[cfg(not(target_os = "macos"))]
    let _ = app_handle;
    Ok(())
}

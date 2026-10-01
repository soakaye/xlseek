//! # Excel Seek Core Library (lib.rs)
//!
//! ## Description
//! Initializes the Tauri desktop application, registers plugins (dialog, os, shell, i18n),
//! registers IPC command handlers, and manages the application event loop.
pub mod commands;
pub mod constants;

pub use xlseek_core::{export, i18n, models, search};

use commands::AppState;
use search::engine::SearchEngine;
use std::sync::Arc;
#[cfg(target_os = "macos")]
use tauri_plugin_i18n::PluginI18nExt;

/// ## Description
/// Constructs the application menu for macOS.
/// Replaces the default About item with a custom menu item that emits an event to open the in-app About dialog.
///
/// ## Arguments
/// - `app_handle`: &tauri::AppHandle<R>
/// - `language`: &str - Current UI language code (`ja` or `en`)
///
/// ## Returns
/// - `tauri::Result<tauri::menu::Menu<R>>`: Constructed menu struct
///
/// ## Errors / Exceptions
/// Returns Err if menu item construction or OS registration fails.
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

    // 1. App Submenu
    // Constant reference: constants::MENU_ITEM_ABOUT_ID and MENU_KEY_ABOUT
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

    // 2. File Submenu
    // Constant reference: constants::MENU_KEY_FILE
    let file_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_FILE)?,
        true,
        &[&PredefinedMenuItem::close_window(app_handle, None)?],
    )?;

    // 3. Edit Submenu
    // Constant reference: constants::MENU_KEY_EDIT
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

    // 4. View Submenu
    // Constant reference: constants::MENU_KEY_VIEW
    let view_submenu = Submenu::with_items(
        app_handle,
        menu_text(MENU_KEY_VIEW)?,
        true,
        &[&PredefinedMenuItem::fullscreen(app_handle, None)?],
    )?;

    // 5. Window Submenu
    // Constant reference: constants::MENU_KEY_WINDOW
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

    // 6. Help Submenu
    // Constant reference: constants::MENU_KEY_HELP
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

/// ## Description
/// Builds the Tauri application, registers state management, plugins, menus,
/// and command handlers, and starts the main event loop.
///
/// ## Arguments
/// None
///
/// ## Returns
/// None
///
/// ## Errors / Exceptions
/// Panics on context generation or runtime error during application launch.
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

            // Constant reference: constants::MENU_ITEM_ABOUT_ID, constants::EVENT_OPEN_ABOUT_DIALOG
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

/// ## Description
/// Rebuilds the macOS application menu according to the current display language.
/// ## Arguments / Returns
/// Accepts Tauri AppHandle and language string (`ja` or `en`), returning `Ok(())` on success.
/// ## Errors / Exceptions
/// Returns error on invalid language code or menu update failure; succeeds on other OSes.
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

pub mod commands;
pub mod export;
pub mod models;
pub mod search;

use commands::AppState;
use search::engine::SearchEngine;
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(SearchEngine::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { engine })
        .invoke_handler(tauri::generate_handler![
            commands::start_search,
            commands::cancel_search,
            commands::get_cell_preview,
            commands::open_in_excel,
            commands::open_in_folder,
            commands::export_results,
            commands::resolve_dropped_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

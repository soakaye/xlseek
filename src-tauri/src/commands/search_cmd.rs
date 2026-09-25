use crate::models::{SearchMatch, SearchQuery};
use crate::search::engine::SearchEngine;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

pub struct AppState {
    pub engine: Arc<SearchEngine>,
}

#[tauri::command]
pub async fn start_search(
    app: AppHandle,
    query: SearchQuery,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = Arc::clone(&state.engine);

    // バックグラウンドスレッドで検索処理を実行し、結果と進捗をイベント送信
    tauri::async_runtime::spawn_blocking(move || {
        let app_handle_match = app.clone();
        let app_handle_prog = app.clone();

        let _ = engine.execute_search(
            query,
            move |search_match: SearchMatch| {
                let _ = app_handle_match.emit("search-match", search_match);
            },
            move |progress| {
                let _ = app_handle_prog.emit("scan-progress", progress);
            },
        );
    });

    Ok(())
}

#[tauri::command]
pub fn cancel_search(state: State<'_, AppState>) -> Result<(), String> {
    state.engine.cancel();
    Ok(())
}

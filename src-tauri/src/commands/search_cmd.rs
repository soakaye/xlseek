use crate::models::{ScanProgress, ScanState, SearchMatch, SearchQuery};
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
    println!(
        "[start_search] 検索リクエスト受信: keyword='{}', target_dir='{}', extensions={:?}",
        query.keyword, query.target_dir, query.extensions
    );

    tauri::async_runtime::spawn_blocking(move || {
        let app_handle_match = app.clone();
        let app_handle_prog = app.clone();
        let app_handle_err = app.clone();

        match engine.execute_search(
            query,
            move |search_match: SearchMatch| {
                if let Err(e) = app_handle_match.emit("search-match", search_match) {
                    eprintln!("[start_search] search-match イベント送信失敗: {}", e);
                }
            },
            move |progress| {
                if let Err(e) = app_handle_prog.emit("scan-progress", &progress) {
                    eprintln!("[start_search] scan-progress イベント送信失敗: {}", e);
                }
            },
        ) {
            Ok(final_prog) => {
                println!(
                    "[start_search] 検索完了: 走査ファイル数={}件, 一致件数={}件, 経過時間={}ms",
                    final_prog.scanned_files, final_prog.matches_found, final_prog.elapsed_ms
                );
            }
            Err(err_msg) => {
                eprintln!("[start_search] 検索エンジンエラー: {}", err_msg);
                let _ = app_handle_err.emit(
                    "scan-progress",
                    ScanProgress {
                        state: ScanState::Error,
                        scanned_files: 0,
                        total_files: 0,
                        matches_found: 0,
                        current_file: format!("エラー: {}", err_msg),
                        elapsed_ms: 0,
                    },
                );
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn cancel_search(state: State<'_, AppState>) -> Result<(), String> {
    println!("[cancel_search] 検索中断要求を受信");
    state.engine.cancel();
    Ok(())
}

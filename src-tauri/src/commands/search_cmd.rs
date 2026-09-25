//! # 検索コマンドハンドラ (commands/search_cmd.rs)
//!
//! ## 処理内容
//! フロントエンドからの検索開始リクエスト（start_search）および中断リクエスト（cancel_search）を
//! 受信し、バックグラウンドスレッドで検索エンジンを起動して結果および進捗をイベント送信する。
//! 憲章原則I（日本語ログ・通知）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、4要素ヘッダコメント付与。

use crate::models::{ScanProgress, ScanState, SearchMatch, SearchQuery};
use crate::search::engine::SearchEngine;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

/// ## 処理内容
/// Tauriアプリケーション全体で共有されるステート構造体。検索エンジンインスタンスを保持する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
pub struct AppState {
    pub engine: Arc<SearchEngine>,
}

/// ## 処理内容
/// 検索リクエストを受信し、非同期スレッド上でExcel検索エンジンを起動する。
/// ヒットしたセル情報は `search-match` イベント、進捗状況は `scan-progress` イベントとしてフロントエンドへ通知する。
///
/// ## 引数
/// - `app`: `AppHandle` - イベント送信用Tauriハンドル
/// - `query`: `SearchQuery` - 検索クエリ（キーワード、ディレクトリ、オプション）
/// - `state`: `State<'_, AppState>` - 共有アプリケーション状態
///
/// ## 戻り値
/// - `Result<(), String>`: コマンド受付成功時は `Ok(())`
///
/// ## エラー / 例外発生条件
/// panicは発生しない。検索エンジンのエラーは `scan-progress` イベント（State: Error）で通知される。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
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
                // 定数参照: crate::constants::EVENT_SEARCH_MATCH を使用
                if let Err(e) =
                    app_handle_match.emit(crate::constants::EVENT_SEARCH_MATCH, search_match)
                {
                    eprintln!("[start_search] search-match イベント送信失敗: {}", e);
                }
            },
            move |progress| {
                // 定数参照: crate::constants::EVENT_SCAN_PROGRESS を使用
                if let Err(e) =
                    app_handle_prog.emit(crate::constants::EVENT_SCAN_PROGRESS, &progress)
                {
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
                // 定数参照: crate::constants::EVENT_SCAN_PROGRESS を使用
                let _ = app_handle_err.emit(
                    crate::constants::EVENT_SCAN_PROGRESS,
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

/// ## 処理内容
/// 実行中の検索処理の中断フラグを設定し、スキャン処理を停止させる。
///
/// ## 引数
/// - `state`: `State<'_, AppState>` - 共有アプリケーション状態
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`
///
/// ## エラー / 例外発生条件
/// panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
#[tauri::command]
pub fn cancel_search(state: State<'_, AppState>) -> Result<(), String> {
    println!("[cancel_search] 検索中断要求を受信");
    state.engine.cancel();
    Ok(())
}

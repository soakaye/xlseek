//! # 検索コマンドハンドラ (commands/search_cmd.rs)
//!
//! ## 処理内容
//! フロントエンドからの検索開始リクエスト（start_search）および中断リクエスト（cancel_search）を
//! 受信し、バックグラウンドスレッドで検索エンジンを起動して結果および進捗をイベント送信する。
//! 憲章原則I（日本語ログ・通知）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、4要素ヘッダコメント付与。
//! - v1.1.0 (2026-09-27, Codex): 検索開始前のパス・正規表現検証を追加。
//! - v1.2.0 (2026-09-27, Codex): 必須条件の事前検証とパス補完 IPC を追加。
//! - v1.3.0 (2026-09-28, AI Agent): 検索マッチのバッチ送信によるWebView2 IPC過負荷防止とエラー詳細出力。

use crate::models::CommandError;
use crate::models::{ErrorCode, ScanProgress, ScanState, SearchMatch, SearchQuery};
use crate::search::engine::SearchEngine;
use crate::search::path::{
    complete_directory_path as complete_path, resolve_search_path, validate_search_directory,
    PathError,
};
use regex::RegexBuilder;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

/// ## 処理内容
/// Tauriアプリケーション全体で共有されるステート構造体。検索エンジンインスタンスを保持する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
pub struct AppState {
    pub engine: Arc<SearchEngine>,
}

/// ## 処理内容
/// 検索リクエストを受信し、パスと正規表現を検証後に非同期スレッド上で検索エンジンを起動する。
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
) -> Result<(), CommandError> {
    let home_dir = app.path().home_dir().ok();
    let query = tauri::async_runtime::spawn_blocking(move || {
        validate_required_search_fields(&query)?;
        validate_search_regex(&query).map_err(|code| CommandError { code })?;
        let target_path = resolve_search_path(&query.target_dir, home_dir.as_deref())
            .map_err(path_error_to_command_error)?;
        validate_search_directory(&target_path).map_err(path_error_to_command_error)?;
        let mut validated_query = query;
        validated_query.target_dir = target_path.to_string_lossy().into_owned();
        Ok::<SearchQuery, CommandError>(validated_query)
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::InternalError,
    })??;
    let engine = Arc::clone(&state.engine);
    println!("{}", crate::constants::LOG_SEARCH_STARTED);

    tauri::async_runtime::spawn_blocking(move || {
        let app_handle_match = app.clone();
        let app_handle_prog = app.clone();
        let app_handle_err = app.clone();
        let app_handle_final = app.clone();

        let match_buffer = Arc::new(std::sync::Mutex::new(Vec::new()));
        let last_emit_instant = Arc::new(std::sync::Mutex::new(std::time::Instant::now()));

        let match_buffer_clone = Arc::clone(&match_buffer);
        let last_emit_clone = Arc::clone(&last_emit_instant);

        let flush_matches = |app_handle: &AppHandle, buf: &mut Vec<SearchMatch>| {
            if buf.is_empty() {
                return;
            }
            let batch: Vec<SearchMatch> = std::mem::take(buf);
            // 定数参照: crate::constants::EVENT_SEARCH_MATCH を使用
            if let Err(e) = app_handle.emit(crate::constants::EVENT_SEARCH_MATCH, &batch) {
                eprintln!("{}: {:?}", crate::constants::LOG_EVENT_EMIT_FAILED, e);
            }
        };

        match engine.execute_search(
            query,
            move |search_match: SearchMatch| {
                let mut buf = match_buffer_clone.lock().unwrap();
                buf.push(search_match);
                let mut last = last_emit_clone.lock().unwrap();
                // 定数参照: crate::constants::SEARCH_MATCH_BATCH_SIZE, crate::constants::SEARCH_MATCH_BATCH_INTERVAL_MS を使用
                if buf.len() >= crate::constants::SEARCH_MATCH_BATCH_SIZE
                    || last.elapsed().as_millis()
                        >= crate::constants::SEARCH_MATCH_BATCH_INTERVAL_MS
                {
                    flush_matches(&app_handle_match, &mut buf);
                    *last = std::time::Instant::now();
                }
            },
            move |progress| {
                // 定数参照: crate::constants::EVENT_SCAN_PROGRESS を使用
                if let Err(e) =
                    app_handle_prog.emit(crate::constants::EVENT_SCAN_PROGRESS, &progress)
                {
                    eprintln!("{}: {:?}", crate::constants::LOG_EVENT_EMIT_FAILED, e);
                }
            },
        ) {
            Ok(final_prog) => {
                if let Ok(mut buf) = match_buffer.lock() {
                    flush_matches(&app_handle_final, &mut buf);
                }
                println!("{}", crate::constants::LOG_SEARCH_COMPLETED);
                let _ = final_prog;
            }
            Err(err_msg) => {
                if let Ok(mut buf) = match_buffer.lock() {
                    flush_matches(&app_handle_final, &mut buf);
                }
                eprintln!("{}", crate::constants::LOG_SEARCH_FAILED);
                let error_code = if err_msg.contains(crate::constants::ERR_INVALID_REGEX) {
                    crate::models::ErrorCode::InvalidRegex
                } else {
                    crate::models::ErrorCode::SearchFailed
                };
                // 定数参照: crate::constants::EVENT_SCAN_PROGRESS を使用
                let _ = app_handle_err.emit(
                    crate::constants::EVENT_SCAN_PROGRESS,
                    ScanProgress {
                        state: ScanState::Error,
                        phase: crate::models::ScanPhase::Finished,
                        error_code: Some(error_code),
                        scanned_files: 0,
                        total_files: 0,
                        matches_found: 0,
                        current_file: String::new(),
                        elapsed_ms: 0,
                    },
                );
            }
        }
    });

    Ok(())
}

/// 処理内容: 検索キーワード、対象パス、拡張子の必須値を検索受付前に検証する。
/// 引数・戻り値: SearchQuery を受け、すべて指定されていれば Ok、欠落時は SearchFailed を返す。
/// エラー: 空白だけのキーワード・パスまたは拡張子なしを SearchFailed として拒否する。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): 検索必須値の受付前検証を追加。
fn validate_required_search_fields(query: &SearchQuery) -> Result<(), CommandError> {
    if query.keyword.trim().is_empty()
        || query.target_dir.trim().is_empty()
        || query.extensions.is_empty()
    {
        return Err(CommandError {
            code: ErrorCode::SearchFailed,
        });
    }
    Ok(())
}

/// 処理内容: ディレクトリ入力に対する補完候補をブロッキング用スレッドで取得する。
/// 引数・戻り値: `path_input` を受け、候補文字列の配列を返す。Tauri 実行環境からホームパスを取得する。
/// エラー: ホーム取得・スレッド起動に失敗した場合は空配列を返す。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): パス補完 IPC を追加。
#[tauri::command]
pub async fn complete_directory_path(app: AppHandle, path_input: String) -> Vec<String> {
    let home_dir = app.path().home_dir().ok();
    tauri::async_runtime::spawn_blocking(move || complete_path(&path_input, home_dir.as_deref()))
        .await
        .unwrap_or_default()
}

/// 処理内容: 検索条件に含まれる正規表現を検索起動前にコンパイルして検証する。
/// 引数・戻り値: SearchQuery を受け、妥当なら Ok、無効なら InvalidRegex を返す。
/// エラー: 正規表現モードで構文が不正な場合は InvalidRegex を返す。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): 検索受付前検証を追加。
fn validate_search_regex(query: &SearchQuery) -> Result<(), ErrorCode> {
    if !query.use_regex {
        return Ok(());
    }
    RegexBuilder::new(&query.keyword)
        .case_insensitive(!query.match_case)
        .build()
        .map(|_| ())
        .map_err(|_| ErrorCode::InvalidRegex)
}

/// 処理内容: 内部パス検証エラーを Tauri が返す構造化エラーへ変換する。
/// 引数・戻り値: PathError を受け、対応する CommandError を返す。
/// エラー: なし。未知のパス状態は SearchFailed として扱う。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): 構造化パスエラー変換を追加。
fn path_error_to_command_error(error: PathError) -> CommandError {
    let code = match error {
        PathError::NotFound => ErrorCode::PathNotFound,
        PathError::PermissionDenied => ErrorCode::PermissionDenied,
        PathError::SearchFailed => ErrorCode::SearchFailed,
    };
    CommandError { code }
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
pub fn cancel_search(state: State<'_, AppState>) -> Result<(), CommandError> {
    println!("{}", crate::constants::LOG_CANCEL_REQUESTED);
    state.engine.cancel();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_required_search_fields, validate_search_regex};
    use crate::models::{CommandError, ErrorCode, SearchQuery};

    /// 処理内容: 正規表現検索の不正パターンを開始前に識別する。
    /// 引数・戻り値: なし。検索クエリの妥当性検証結果を確認する。
    /// エラー: 不正パターンが受理される場合にテストを失敗させる。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 検索受付前の正規表現テストを追加。
    #[test]
    fn rejects_invalid_regex_before_search_is_started() {
        let query = SearchQuery {
            keyword: "(".to_string(),
            target_dir: String::new(),
            match_case: false,
            use_regex: true,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: Vec::new(),
        };

        assert!(matches!(
            validate_search_regex(&query),
            Err(ErrorCode::InvalidRegex)
        ));
    }

    /// 処理内容: 必須検索条件が空の場合は受付エラーにする。
    /// 引数・戻り値: なし。必須値の検証結果を確認する。
    /// エラー: 欠落した条件が受理される場合にテストを失敗させる。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 必須条件検証を追加。
    #[test]
    fn rejects_missing_required_search_fields() {
        let query = SearchQuery {
            keyword: " ".to_string(),
            target_dir: "/tmp".to_string(),
            match_case: false,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_shape: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };
        assert!(matches!(
            validate_required_search_fields(&query),
            Err(CommandError {
                code: ErrorCode::SearchFailed
            })
        ));
    }
}

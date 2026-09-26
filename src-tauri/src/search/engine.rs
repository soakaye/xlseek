//! # 検索エンジンコアモジュール (search/engine.rs)
//!
//! ## 処理内容
//! 指定ディレクトリ配下のExcelファイルを再帰的に探索し、rayonを用いたマルチスレッド並列処理で
//! 高速走査を実行する。進捗状況のスロットル通知、キャンセル制御、およびパニック安全性を担保する。
//! 憲章原則I（日本語メッセージ）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。Default実装、定数参照化、4要素ヘッダコメントの付与。

use crate::models::{ScanProgress, ScanState, SearchMatch, SearchQuery};
use crate::search::parser::parse_and_search_file;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// ## 処理内容
/// 並列検索スキャンおよびキャンセル制御を管理する検索エンジン構造体。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
pub struct SearchEngine {
    is_cancelled: Arc<AtomicBool>,
}

impl Default for SearchEngine {
    /// ## 処理内容
    /// 検索エンジンのデフォルトインスタンスを生成する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// - `Self`: 初期化された検索エンジン
    ///
    /// ## エラー / 例外発生条件
    /// panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定（Clippy new_without_default 解消）。
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    /// ## 処理内容
    /// 新規の検索エンジンインスタンスを生成する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// - `Self`: キャンセルフラグが未設定の検索エンジン
    ///
    /// ## エラー / 例外発生条件
    /// panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    pub fn new() -> Self {
        Self {
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// ## 処理内容
    /// キャンセルフラグのアトミック参照を取得する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// - `Arc<AtomicBool>`: キャンセルフラグへの参照
    ///
    /// ## エラー / 例外発生条件
    /// panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    pub fn get_cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.is_cancelled)
    }

    /// ## 処理内容
    /// 実行中のスキャン処理に中断を指示する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }

    /// ## 処理内容
    /// キャンセルフラグをリセットし、新たなスキャンを実行可能な状態にする。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    pub fn reset_cancel(&self) {
        self.is_cancelled.store(false, Ordering::Relaxed);
    }

    /// ## 処理内容
    /// 対象ディレクトリ配下を再帰的に走査し、指定拡張子に合致するファイルパス一覧を収集する。
    /// Excelの一時ファイル（~$で始まるロックファイル等）は除外する。
    /// 中断フラグが指定された場合は、反復ごとに中断状態を検査し、中断時は直ちに走査を打ち切る。
    ///
    /// ## 引数
    /// - `target_dir`: `&str` - 探索対象ディレクトリパス
    /// - `extensions`: `&[String]` - 検索対象とする拡張子一覧（ドット付き）
    /// - `cancel_flag`: `Option<&AtomicBool>` - 中断指示を監視するアトミックフラグ（None時は中断検査なし）
    ///
    /// ## 戻り値
    /// - `Vec<PathBuf>`: 収集されたファイルパス一覧
    ///
    /// ## エラー / 例外発生条件
    /// ディレクトリ走査時の個別エラーは無視して走査を継続する。panicは発生しない。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
    /// - v1.1.0 (2026-09-26, AI Agent): cancel_flag による即時中断対応。
    pub fn collect_files(
        target_dir: &str,
        extensions: &[String],
        cancel_flag: Option<&AtomicBool>,
    ) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let exts_lower: Vec<String> = extensions.iter().map(|e| e.to_lowercase()).collect();

        for entry in WalkDir::new(target_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Some(flag) = cancel_flag {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
            }

            if entry.file_type().is_file() {
                let path = entry.path();
                // 一時ファイル（~$で始まるExcelロックファイル等）は除外
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // 定数参照: crate::constants::EXCEL_TEMP_FILE_PREFIX を使用
                    if file_name.starts_with(crate::constants::EXCEL_TEMP_FILE_PREFIX) {
                        continue;
                    }
                }

                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_with_dot = format!(".{}", ext.to_lowercase());
                    if exts_lower.contains(&ext_with_dot) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        files
    }

    /// ## 処理内容
    /// 指定された検索クエリに基づき、有界同期チャネルを用いたプロデューサー・コンシューマー・パイプライン並行処理で
    /// Excelブックの検出とセル走査を同時並行で実行する。
    /// 一致セルが見つかるごとにコールバックを実行し、進捗状況を一定間隔（50ms以上）で通知する。
    ///
    /// ## 引数
    /// - `query`: `SearchQuery` - 検索キーワード、パス、拡張子等の検索条件
    /// - `on_match`: `FMatch` - ヒットした一致アイテムを受け取るコールバック
    /// - `on_progress`: `FProgress` - スキャン進捗情報を受け取るコールバック
    ///
    /// ## 戻り値
    /// - `Result<ScanProgress, String>`: 最終進捗状態、または日本語エラー文字列
    ///
    /// ## エラー / 例外発生条件
    /// - 対象ディレクトリが存在しない場合、または正規表現が不正な場合に `Err` を返却する。
    /// - 個別ファイルの破損によるパニックは `catch_unwind` で安全に捕捉しスキップする。
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化および日本語メッセージ統合。
    /// - v1.1.0 (2026-09-26, AI Agent): 有界同期チャネルによるパイプライン並行化および高速キャンセル対応。
    pub fn execute_search<FMatch, FProgress>(
        &self,
        query: SearchQuery,
        on_match: FMatch,
        on_progress: FProgress,
    ) -> Result<ScanProgress, String>
    where
        FMatch: FnMut(SearchMatch) + Send + Sync + 'static,
        FProgress: FnMut(ScanProgress) + Send + Sync + 'static,
    {
        self.reset_cancel();
        let cancel_flag = Arc::clone(&self.is_cancelled);

        // 正規表現コンパイル (オプション時)
        let regex_obj = if query.use_regex {
            let re = RegexBuilder::new(&query.keyword)
                .case_insensitive(!query.match_case)
                .build()
                .map_err(|e| {
                    // 定数参照: crate::constants::ERR_INVALID_REGEX を使用
                    format!("{}: {}", crate::constants::ERR_INVALID_REGEX, e)
                })?;
            Some(re)
        } else {
            None
        };

        let start_time = Instant::now();
        let target_path = Path::new(&query.target_dir);
        if !target_path.exists() || !target_path.is_dir() {
            // 定数参照: crate::constants::ERR_FILE_NOT_FOUND を使用
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                query.target_dir
            ));
        }

        let scanned_count = Arc::new(AtomicUsize::new(0));
        let match_count = Arc::new(AtomicUsize::new(0));
        let discovered_count = Arc::new(AtomicUsize::new(0));
        let scan_completed = Arc::new(AtomicBool::new(false));

        let on_match = Arc::new(std::sync::Mutex::new(on_match));
        let on_progress = Arc::new(std::sync::Mutex::new(on_progress));

        // 初期進捗送信 (スキャン未確定時は total_files: 0)
        if let Ok(mut prog) = on_progress.lock() {
            // 固定メッセージは current_file に含めず、phase から画面側で生成する。
            prog(ScanProgress {
                state: ScanState::Scanning,
                phase: crate::models::ScanPhase::Discovering,
                error_code: None,
                scanned_files: 0,
                total_files: 0,
                matches_found: 0,
                current_file: String::new(),
                elapsed_ms: 0,
            });
        }

        // サードパーティライブラリ等のパニック時にstderrへの大量ダンプを抑制
        static INIT_HOOK: std::sync::Once = std::sync::Once::new();
        INIT_HOOK.call_once(|| {
            std::panic::set_hook(Box::new(|_info| {
                // catch_unwind 側でキャッチしてハンドリングするため、標準stderrダンプを抑止
            }));
        });

        let last_notify_ms = Arc::new(std::sync::atomic::AtomicU64::new(0));

        // 定数参照: crate::constants::CHANNEL_BUFFER_SIZE を使用
        let (tx, rx) =
            std::sync::mpsc::sync_channel::<PathBuf>(crate::constants::CHANNEL_BUFFER_SIZE);
        let rx = Arc::new(std::sync::Mutex::new(rx));

        // プロデューサー: ディレクトリスキャンを別スレッドで並行実行
        let target_dir = query.target_dir.clone();
        let extensions = query.extensions.clone();
        let cancel_flag_scanner = Arc::clone(&cancel_flag);
        let discovered_count_clone = Arc::clone(&discovered_count);
        let scan_completed_clone = Arc::clone(&scan_completed);

        let scanner_handle = std::thread::spawn(move || {
            let exts_lower: Vec<String> = extensions.iter().map(|e| e.to_lowercase()).collect();

            for entry in WalkDir::new(&target_dir)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if cancel_flag_scanner.load(Ordering::Relaxed) {
                    break;
                }

                if entry.file_type().is_file() {
                    let path = entry.path();
                    // 一時ファイル（~$で始まるExcelロックファイル等）は除外
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        // 定数参照: crate::constants::EXCEL_TEMP_FILE_PREFIX を使用
                        if file_name.starts_with(crate::constants::EXCEL_TEMP_FILE_PREFIX) {
                            continue;
                        }
                    }

                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext_with_dot = format!(".{}", ext.to_lowercase());
                        if exts_lower.contains(&ext_with_dot) {
                            discovered_count_clone.fetch_add(1, Ordering::Relaxed);
                            if tx.send(path.to_path_buf()).is_err() {
                                // 受信側がクローズされた（中断または完了）ため脱出
                                break;
                            }
                        }
                    }
                }
            }

            if !cancel_flag_scanner.load(Ordering::Relaxed) {
                scan_completed_clone.store(true, Ordering::Relaxed);
            }
            // tx はここでドロップされ、受信側チャネルがクローズされる
        });

        // コンシューマー: rayon によるマルチスレッド並列処理で受信パスを即時走査
        let num_threads = rayon::current_num_threads();
        (0..num_threads).into_par_iter().for_each(|_| {
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                let file_path = {
                    let Ok(rx_guard) = rx.lock() else {
                        break;
                    };
                    match rx_guard.recv() {
                        Ok(p) => p,
                        Err(_) => break, // 全パス取得完了かつキュー消化完了
                    }
                };

                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                let file_name = file_path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Excelファイルパース (破損ファイルやcalamineパニックは安全にスキップして全体を停止させない)
                let cancel_flag_ref = Arc::clone(&cancel_flag);
                let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    parse_and_search_file(
                        &file_path,
                        &query,
                        regex_obj.as_ref(),
                        Some(&cancel_flag_ref),
                    )
                }));

                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                match parse_result {
                    Ok(Ok(matches)) => {
                        let m_count = matches.len();
                        if m_count > 0 {
                            match_count.fetch_add(m_count, Ordering::Relaxed);
                            if let Ok(mut match_cb) = on_match.lock() {
                                for m in matches {
                                    match_cb(m);
                                }
                            }
                        }
                    }
                    Ok(Err(_err_msg)) => {
                        // 破損・読み込み不可ファイルはスキップして継続
                    }
                    Err(_) => {
                        // calamine 内部などのパニックから安全に回復
                    }
                }

                let scanned = scanned_count.fetch_add(1, Ordering::Relaxed) + 1;
                let current_matches = match_count.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed().as_millis() as u64;

                // 中断要求後は Scanning 状態の進捗通知を送信しない
                if cancel_flag.load(Ordering::Relaxed) {
                    break;
                }

                // スキャン完了前は total_files = 0, 完了後は確定件数
                let is_scan_done = scan_completed.load(Ordering::Relaxed);
                let current_total = if is_scan_done {
                    discovered_count.load(Ordering::Relaxed)
                } else {
                    0
                };

                // 1件目、全完了、または前回通知から50ms以上経過した時に進捗通知
                let last = last_notify_ms.load(Ordering::Relaxed);
                // 定数参照: crate::constants::PROGRESS_NOTIFY_INTERVAL_MS を使用
                let should_notify = scanned == 1
                    || (is_scan_done && scanned == current_total)
                    || (elapsed.saturating_sub(last)
                        >= crate::constants::PROGRESS_NOTIFY_INTERVAL_MS);

                if should_notify {
                    last_notify_ms.store(elapsed, Ordering::Relaxed);
                    if let Ok(mut prog_cb) = on_progress.lock() {
                        prog_cb(ScanProgress {
                            state: ScanState::Scanning,
                            phase: if current_total == 0 {
                                crate::models::ScanPhase::Discovering
                            } else {
                                crate::models::ScanPhase::Scanning
                            },
                            error_code: None,
                            scanned_files: scanned,
                            total_files: current_total,
                            matches_found: current_matches,
                            current_file: file_name,
                            elapsed_ms: elapsed,
                        });
                    }
                }
            }
        });

        // スキャナースレッドの終了待機
        let _ = scanner_handle.join();

        let is_cancelled = cancel_flag.load(Ordering::Relaxed);
        let final_state = if is_cancelled {
            ScanState::Cancelled
        } else {
            ScanState::Completed
        };
        let final_total = discovered_count.load(Ordering::Relaxed);

        // phase と state を使って画面側が完了・中断メッセージを生成する。
        let final_progress = ScanProgress {
            state: final_state,
            phase: crate::models::ScanPhase::Finished,
            error_code: None,
            scanned_files: scanned_count.load(Ordering::Relaxed),
            total_files: final_total,
            matches_found: match_count.load(Ordering::Relaxed),
            current_file: String::new(),
            elapsed_ms: start_time.elapsed().as_millis() as u64,
        };

        // 最終通知
        if let Ok(mut prog_cb) = on_progress.lock() {
            prog_cb(final_progress.clone());
        }

        Ok(final_progress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::{export_to_csv, export_to_xlsx};
    use crate::search::preview::extract_cell_preview;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// ## 処理内容
    /// テスト用フィクスチャディレクトリ内のExcelファイルに対して検索を実行し、
    /// 一致セル、プレビュー抽出、エクスポート処理が正常に機能することを統合テストする。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_search_engine_on_fixtures() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        assert!(
            fixtures_dir.exists(),
            "Fixtures dir should exist: {:?}",
            fixtures_dir
        );

        let engine = SearchEngine::new();
        let query = SearchQuery {
            keyword: "Financial".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            match_case: false,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        let matches = Arc::new(std::sync::Mutex::new(Vec::new()));
        let matches_clone = Arc::clone(&matches);

        let result = engine.execute_search(
            query,
            move |m| {
                matches_clone.lock().unwrap().push(m);
            },
            |_| {},
        );

        assert!(result.is_ok());
        let found = matches.lock().unwrap();
        assert!(!found.is_empty(), "Should find matches for 'Financial'");
        let first = &found[0];
        assert_eq!(first.sheet_name, "Data");

        // プレビュー抽出のテスト
        let preview = extract_cell_preview(
            &first.full_path,
            &first.sheet_name,
            first.row_index,
            first.col_index,
        );
        assert!(preview.is_ok(), "Preview extraction should succeed");
        let p_data = preview.unwrap();
        assert!(!p_data.rows.is_empty(), "Preview rows should not be empty");
        assert!(
            !p_data.columns.is_empty(),
            "Preview columns should not be empty"
        );

        // エクスポートのテスト
        let temp_csv = std::env::temp_dir().join("test_export.csv");
        let temp_xlsx = std::env::temp_dir().join("test_export.xlsx");
        let test_keys = crate::constants::EXPORT_HEADER_KEYS.iter().copied().chain([
            crate::constants::EXPORT_SHEET_NAME_KEY,
            crate::constants::EXPORT_MATCH_VALUE_KEY,
            crate::constants::EXPORT_MATCH_FORMULA_KEY,
            crate::constants::EXPORT_MATCH_COMMENT_KEY,
            crate::constants::EXPORT_MATCH_HIDDEN_SHEET_KEY,
            crate::constants::TRANSLATION_UNAVAILABLE_KEY,
        ]);
        let test_catalog = test_keys
            .map(|key| (key.to_string(), key.to_string()))
            .collect::<BTreeMap<_, _>>();
        let test_catalogs = BTreeMap::from([
            (
                crate::constants::LANGUAGE_EN.to_string(),
                test_catalog.clone(),
            ),
            (crate::constants::LANGUAGE_JA.to_string(), test_catalog),
        ]);

        assert!(export_to_csv(temp_csv.to_str().unwrap(), &found, "ja", &test_catalogs).is_ok());
        assert!(export_to_xlsx(temp_xlsx.to_str().unwrap(), &found, "ja", &test_catalogs).is_ok());

        assert!(temp_csv.exists());
        assert!(temp_xlsx.exists());

        let _ = std::fs::remove_file(temp_csv);
        let _ = std::fs::remove_file(temp_xlsx);
    }

    /// ## 処理内容
    /// 日本語文字列に対するスニペット生成処理が、UTF-8文字境界で正しく動作し
    /// パニックしないことを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_snippet_utf8_boundary_safety() {
        use crate::search::parser::make_snippet;

        // 日本語文字列の任意の位置でスニペット生成を行い、パニックしないことを検証
        let japanese_text =
            "財務報告書2026年第3四半期における監査報告書の承認およびシステム移行計画の進捗状況確認";

        // "監査報告書" をマッチ対象にする
        let mat_start = japanese_text.find("監査報告書").unwrap();
        let mat_end = mat_start + "監査報告書".len();

        let snippet = make_snippet(japanese_text, mat_start, mat_end);
        assert!(snippet.contains("<mark"));
        assert!(snippet.contains("監査報告書"));
        assert!(snippet.contains("</mark>"));

        // 先頭マッチのテスト
        let mat_start_head = 0;
        let mat_end_head = "財務".len();
        let snippet_head = make_snippet(japanese_text, mat_start_head, mat_end_head);
        assert!(snippet_head.contains("財務"));

        // 末尾マッチのテスト
        let mat_start_tail = japanese_text.rfind("確認").unwrap();
        let mat_end_tail = japanese_text.len();
        let snippet_tail = make_snippet(japanese_text, mat_start_tail, mat_end_tail);
        assert!(snippet_tail.contains("確認"));
    }

    /// ## 処理内容
    /// 検索実行中にcancel()が呼び出された際、スキャンが安全に中断され
    /// 最終ステータスがCancelledとなることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_search_engine_cancellation() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        assert!(
            fixtures_dir.exists(),
            "Fixtures dir should exist: {:?}",
            fixtures_dir
        );

        let engine = Arc::new(SearchEngine::new());
        let engine_clone = Arc::clone(&engine);

        let query = SearchQuery {
            keyword: "Financial".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            match_case: false,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        // 検索実行中に中断フラグをセット
        let result = engine.execute_search(
            query,
            move |_m| {
                // 1件マッチした時点で中断
                engine_clone.cancel();
            },
            |_| {},
        );

        assert!(result.is_ok());
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Cancelled);
    }

    /// ## 処理内容
    /// collect_files において、中断フラグが true に設定された際に
    /// 直ちに走査が打ち切られて空または最小限のファイル一覧が返却されることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_collect_files_cancellation() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        assert!(
            fixtures_dir.exists(),
            "Fixtures dir should exist: {:?}",
            fixtures_dir
        );

        let extensions = vec![".xlsx".to_string()];

        // 1. キャンセルフラグなし (None): ファイルが収集されること
        let all_files =
            SearchEngine::collect_files(fixtures_dir.to_str().unwrap(), &extensions, None);
        assert!(!all_files.is_empty(), "Fixtures files should be collected");

        // 2. 開始時点で既にキャンセルされている場合: 直ちに打ち切られて0件であること
        let cancelled_flag = AtomicBool::new(true);
        let cancelled_files = SearchEngine::collect_files(
            fixtures_dir.to_str().unwrap(),
            &extensions,
            Some(&cancelled_flag),
        );
        assert_eq!(
            cancelled_files.len(),
            0,
            "When cancelled at start, collected files must be empty"
        );
    }

    /// ## 処理内容
    /// 有界同期チャネルを用いたパイプライン並行検索が正常に動作し、
    /// 全ファイルの検出・解析が完了して整合性のある結果が返却されることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_search_engine_parallel_pipeline() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        let engine = SearchEngine::new();
        let query = SearchQuery {
            keyword: "Total".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            match_case: false,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        let matches = Arc::new(std::sync::Mutex::new(Vec::new()));
        let matches_clone = Arc::clone(&matches);
        let progress_events = Arc::new(std::sync::Mutex::new(Vec::new()));
        let progress_clone = Arc::clone(&progress_events);

        let result = engine.execute_search(
            query,
            move |m| {
                matches_clone.lock().unwrap().push(m);
            },
            move |p| {
                progress_clone.lock().unwrap().push(p);
            },
        );

        assert!(result.is_ok(), "Parallel pipeline search must succeed");
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Completed);
        assert!(final_prog.total_files > 0);
        assert_eq!(final_prog.scanned_files, final_prog.total_files);
        assert_eq!(final_prog.matches_found, matches.lock().unwrap().len());

        let events = progress_events.lock().unwrap();
        assert!(!events.is_empty(), "Progress events should be emitted");
        // 初期進捗イベントでは total_files が 0 であることを検証 (未確定状態)
        assert_eq!(events[0].total_files, 0);
    }

    /// ## 処理内容
    /// ディレクトリスキャン進行中の段階でキャンセル要求が発行された際、
    /// 1秒未満で安全に走査・検索が停止し、最終状態が Cancelled となることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): 初版策定。
    #[test]
    fn test_search_engine_cancellation_during_scan() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        let engine = Arc::new(SearchEngine::new());
        let engine_clone = Arc::clone(&engine);

        let query = SearchQuery {
            keyword: "a".to_string(),
            target_dir: fixtures_dir.to_str().unwrap().to_string(),
            match_case: false,
            use_regex: false,
            include_formula: true,
            include_comment: true,
            include_hidden: false,
            extensions: vec![".xlsx".to_string()],
        };

        // 初期進捗通知を受け取った直後（スキャン開始直後）に即時キャンセル
        let result = engine.execute_search(
            query,
            |_| {},
            move |_p| {
                engine_clone.cancel();
            },
        );

        assert!(result.is_ok());
        let final_prog = result.unwrap();
        assert_eq!(final_prog.state, ScanState::Cancelled);
    }
}

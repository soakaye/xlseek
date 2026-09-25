use crate::models::{ScanProgress, ScanState, SearchMatch, SearchQuery};
use crate::search::parser::parse_and_search_file;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

pub struct SearchEngine {
    is_cancelled: Arc<AtomicBool>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.is_cancelled)
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }

    pub fn reset_cancel(&self) {
        self.is_cancelled.store(false, Ordering::Relaxed);
    }

    /// 対象ディレクトリ内の対象拡張子ファイルパスを高速走査
    pub fn collect_files(target_dir: &str, extensions: &[String]) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let exts_lower: Vec<String> = extensions.iter().map(|e| e.to_lowercase()).collect();

        for entry in WalkDir::new(target_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                // 一時ファイル（~$で始まるExcelロックファイル等）は除外
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("~$") {
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

    /// 並列スキャン実行
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
                .map_err(|e| format!("無効な正規表現パターンです: {}", e))?;
            Some(re)
        } else {
            None
        };

        let start_time = Instant::now();
        let target_path = Path::new(&query.target_dir);
        if !target_path.exists() || !target_path.is_dir() {
            return Err(format!("指定されたディレクトリが存在しません: {}", query.target_dir));
        }

        // ファイルリスト収集
        let file_list = Self::collect_files(&query.target_dir, &query.extensions);
        let total_files = file_list.len();

        let scanned_count = Arc::new(AtomicUsize::new(0));
        let match_count = Arc::new(AtomicUsize::new(0));

        let on_match = Arc::new(std::sync::Mutex::new(on_match));
        let on_progress = Arc::new(std::sync::Mutex::new(on_progress));

        // 初期進捗送信
        {
            let mut prog = on_progress.lock().unwrap();
            prog(ScanProgress {
                state: ScanState::Scanning,
                scanned_files: 0,
                total_files,
                matches_found: 0,
                current_file: "スキャン開始中...".to_string(),
                elapsed_ms: 0,
            });
        }

        // rayon によるマルチスレッド並列走査
        file_list.par_iter().for_each(|file_path| {
            if cancel_flag.load(Ordering::Relaxed) {
                return;
            }

            let file_name = file_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            // Excelファイルパース (破損ファイルやcalamineパニックは安全にスキップして全体を停止させない)
            let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                parse_and_search_file(file_path, &query, regex_obj.as_ref())
            }));

            match parse_result {
                Ok(Ok(matches)) => {
                    let m_count = matches.len();
                    if m_count > 0 {
                        match_count.fetch_add(m_count, Ordering::Relaxed);
                        let mut match_cb = on_match.lock().unwrap();
                        for m in matches {
                            match_cb(m);
                        }
                    }
                }
                Ok(Err(err_msg)) => {
                    // スキップして処理継続 (T036)
                    eprintln!("Skipping corrupted/unreadable file {}: {}", file_name, err_msg);
                }
                Err(_) => {
                    // calamine 内部などのパニックから安全に回復 (T036)
                    eprintln!("Recovered from panic while parsing file {}. Skipping safely.", file_name);
                }
            }

            let scanned = scanned_count.fetch_add(1, Ordering::Relaxed) + 1;
            let current_matches = match_count.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed().as_millis() as u64;

            // 1件目、5ファイルごと、または全完了時に進捗通知
            if scanned == 1 || scanned % 5 == 0 || scanned == total_files {
                let mut prog_cb = on_progress.lock().unwrap();
                prog_cb(ScanProgress {
                    state: ScanState::Scanning,
                    scanned_files: scanned,
                    total_files,
                    matches_found: current_matches,
                    current_file: file_name,
                    elapsed_ms: elapsed,
                });
            }
        });

        let is_cancelled = cancel_flag.load(Ordering::Relaxed);
        let final_state = if is_cancelled {
            ScanState::Cancelled
        } else {
            ScanState::Completed
        };

        let final_progress = ScanProgress {
            state: final_state,
            scanned_files: scanned_count.load(Ordering::Relaxed),
            total_files,
            matches_found: match_count.load(Ordering::Relaxed),
            current_file: if is_cancelled {
                "スキャンが中断されました".to_string()
            } else {
                "スキャン完了".to_string()
            },
            elapsed_ms: start_time.elapsed().as_millis() as u64,
        };

        // 最終通知
        {
            let mut prog_cb = on_progress.lock().unwrap();
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
    use std::path::PathBuf;

    #[test]
    fn test_search_engine_on_fixtures() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("tests")
            .join("fixtures");

        assert!(fixtures_dir.exists(), "Fixtures dir should exist: {:?}", fixtures_dir);

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
        assert!(!p_data.columns.is_empty(), "Preview columns should not be empty");

        // エクスポートのテスト
        let temp_csv = std::env::temp_dir().join("test_export.csv");
        let temp_xlsx = std::env::temp_dir().join("test_export.xlsx");

        assert!(export_to_csv(temp_csv.to_str().unwrap(), &found).is_ok());
        assert!(export_to_xlsx(temp_xlsx.to_str().unwrap(), &found).is_ok());

        assert!(temp_csv.exists());
        assert!(temp_xlsx.exists());

        let _ = std::fs::remove_file(temp_csv);
        let _ = std::fs::remove_file(temp_xlsx);
    }

    #[test]
    fn test_snippet_utf8_boundary_safety() {
        use crate::search::parser::make_snippet;

        // 日本語文字列の任意の位置でスニペット生成を行い、パニックしないことを検証
        let japanese_text = "財務報告書2026年第3四半期における監査報告書の承認およびシステム移行計画の進捗状況確認";
        
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
}


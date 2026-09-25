//! # Excelファイル解析・検索モジュール (search/parser.rs)
//!
//! ## 処理内容
//! calamineを用いて単一のExcelブックを開き、セル値および数式を走査して検索条件に一致するセルを抽出する。
//! UTF-8文字境界を考慮した安全なスニペット生成およびセル番地変換機能を提供する。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、Clippy指摘修正（is_some_and）、日本語エラー化、4要素ヘッダコメント付与。

use crate::models::{MatchType, SearchMatch, SearchQuery};
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use regex::Regex;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// 定数参照: crate::constants::DEFAULT_MATCH_ID_START を使用
static MATCH_ID_COUNTER: AtomicU64 = AtomicU64::new(crate::constants::DEFAULT_MATCH_ID_START);

/// ## 処理内容
/// 0始まりの列インデックスをExcel列名（例: 0 -> A, 1 -> B, 26 -> AA）に変換する。
///
/// ## 引数
/// - `col_idx`: `u32` - 0始まりの列インデックス
///
/// ## 戻り値
/// - `String`: アルファベット表記の列名文字列
///
/// ## エラー / 例外発生条件
/// panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
pub fn col_to_name(mut col_idx: u32) -> String {
    let mut name = String::new();
    loop {
        let rem = (col_idx % 26) as u8;
        name.push((b'A' + rem) as char);
        if col_idx < 26 {
            break;
        }
        col_idx = col_idx / 26 - 1;
    }
    name.chars().rev().collect()
}

/// ## 処理内容
/// 行・列インデックス（0始まり）からExcel形式のセル番地文字列（例: A12）と列名を生成する。
///
/// ## 引数
/// - `row_idx`: `u32` - 0始まりの行インデックス
/// - `col_idx`: `u32` - 0始まりの列インデックス
///
/// ## 戻り値
/// - `(String, String)`: (セル番地文字列, 列名アルファベット)
///
/// ## エラー / 例外発生条件
/// panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
pub fn format_cell_address(row_idx: u32, col_idx: u32) -> (String, String) {
    let col_str = col_to_name(col_idx);
    let address = format!("{}{}", col_str, row_idx + 1);
    (address, col_str)
}

/// ## 処理内容
/// ヒットした文字列の前後にコンテキストを付与し、ハイライト表示用HTMLマークアップを含む
/// UTF-8文字境界安全なスニペット文字列を生成する。
///
/// ## 引数
/// - `text`: `&str` - 対象セルの全体テキスト
/// - `mat_start`: `usize` - 一致箇所の開始バイト位置
/// - `mat_end`: `usize` - 一致箇所の終了バイト位置
///
/// ## 戻り値
/// - `String`: `<mark>` タグ付きハイライトスニペット文字列
///
/// ## エラー / 例外発生条件
/// 不正なバイト境界指定時にもパニックせず安全にスライス範囲を算出する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
pub fn make_snippet(text: &str, mat_start: usize, mat_end: usize) -> String {
    let before_str = &text[..mat_start];
    let matched = &text[mat_start..mat_end];
    let after_str = &text[mat_end..];

    // 定数参照: crate::constants::SNIPPET_CONTEXT_CHARS を使用
    let before_chars_count = crate::constants::SNIPPET_CONTEXT_CHARS;
    let before_byte_len = before_str
        .char_indices()
        .rev()
        .take(before_chars_count)
        .last()
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    let before = &before_str[before_byte_len..];
    // 定数参照: crate::constants::SNIPPET_ELLIPSIS を使用
    let prefix = if before_byte_len > 0 {
        crate::constants::SNIPPET_ELLIPSIS
    } else {
        ""
    };

    // 定数参照: crate::constants::SNIPPET_CONTEXT_CHARS を使用
    let after_chars_count = crate::constants::SNIPPET_CONTEXT_CHARS;
    // 定数参照: crate::constants::SNIPPET_ELLIPSIS を使用
    let (after, suffix) = match after_str.char_indices().nth(after_chars_count) {
        Some((idx, _)) => (&after_str[..idx], crate::constants::SNIPPET_ELLIPSIS),
        None => (after_str, ""),
    };

    format!(
        "{}{}<mark class='bg-yellow-500/30 text-yellow-300 px-0.5 rounded font-semibold'>{}</mark>{}{}",
        prefix, before, matched, after, suffix
    )
}

/// ## 処理内容
/// 指定された単一のExcelファイルを開き、シートごとのセル値および数式を走査して検索クエリに一致するセルを抽出する。
/// キャンセル通知フラグを定期的に確認し、要求があれば速やかに処理を中断する。
///
/// ## 引数
/// - `path`: `P` - Excelファイルのパス
/// - `query`: `&SearchQuery` - 検索キーワード、正規表現設定、数式含むフラグ等の検索条件
/// - `regex_opt`: `Option<&Regex>` - コンパイル済みの正規表現オブジェクト（使用時のみ）
/// - `cancel_flag`: `Option<&AtomicBool>` - 外部からのスキャン中断通知フラグ
///
/// ## 戻り値
/// - `Result<Vec<SearchMatch>, String>`: 一致アイテムのベクター、または日本語エラー文字列
///
/// ## エラー / 例外発生条件
/// - ワークブックのオープンに失敗した場合に `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、Clippy指摘修正（is_some_and）。
pub fn parse_and_search_file<P: AsRef<Path>>(
    path: P,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    if cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed)) {
        return Ok(Vec::new());
    }

    let path_ref = path.as_ref();
    let full_path = path_ref.to_string_lossy().to_string();
    let file_name = path_ref
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut workbook: Sheets<_> = open_workbook_auto(path_ref).map_err(|e| {
        // 定数参照: crate::constants::ERR_WORKBOOK_OPEN を使用
        format!(
            "{}: {} ({})",
            crate::constants::ERR_WORKBOOK_OPEN,
            full_path,
            e
        )
    })?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut matches = Vec::new();

    for sheet_name in &sheet_names {
        if cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed)) {
            return Ok(matches);
        }

        // 非表示シート判定 (calamine の sheet メタデータ)
        let is_hidden_sheet = sheet_name.to_lowercase().contains("hidden");

        if is_hidden_sheet && !query.include_hidden {
            continue;
        }

        // ワークシートのセル値読み込み
        if let Ok(range) = workbook.worksheet_range(sheet_name) {
            for (row_idx, row) in range.rows().enumerate() {
                // 定数参照: crate::constants::CANCEL_CHECK_ROW_INTERVAL を使用
                if (row_idx & crate::constants::CANCEL_CHECK_ROW_INTERVAL) == 0
                    && cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed))
                {
                    return Ok(matches);
                }

                for (col_idx, cell) in row.iter().enumerate() {
                    let cell_str = match cell {
                        Data::Empty => continue,
                        Data::String(s) => s.clone(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(d) => d.to_string(),
                        Data::DateTimeIso(d) => d.clone(),
                        Data::DurationIso(d) => d.clone(),
                        Data::Error(e) => format!("{:?}", e),
                    };

                    if cell_str.is_empty() {
                        continue;
                    }

                    // 一致判定 (正規表現 or テキスト部分一致)
                    let match_pos = if let Some(re) = regex_opt {
                        re.find(&cell_str).map(|m| (m.start(), m.end()))
                    } else if query.match_case {
                        cell_str
                            .find(&query.keyword)
                            .map(|idx| (idx, idx + query.keyword.len()))
                    } else {
                        let lower_text = cell_str.to_lowercase();
                        let lower_key = query.keyword.to_lowercase();
                        lower_text
                            .find(&lower_key)
                            .map(|idx| (idx, idx + lower_key.len()))
                    };

                    if let Some((start, end)) = match_pos {
                        let (address, col_name) =
                            format_cell_address(row_idx as u32, col_idx as u32);
                        let snippet = make_snippet(&cell_str, start, end);
                        let match_type = if is_hidden_sheet {
                            MatchType::HiddenSheet
                        } else {
                            MatchType::CellValue
                        };

                        let id = MATCH_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                        matches.push(SearchMatch {
                            id,
                            file_name: file_name.clone(),
                            full_path: full_path.clone(),
                            sheet_name: sheet_name.clone(),
                            cell_address: address,
                            row_index: (row_idx + 1) as u32,
                            col_index: (col_idx + 1) as u32,
                            col_name,
                            match_type,
                            snippet,
                            full_content: cell_str.clone(),
                            formula: None,
                            sheets_in_workbook: sheet_names.clone(),
                        });
                    }
                }
            }
        }

        // 数式 (Formula) の検索
        if query.include_formula {
            if let Ok(formula_range) = workbook.worksheet_formula(sheet_name) {
                for (row_idx, row) in formula_range.rows().enumerate() {
                    // 定数参照: crate::constants::CANCEL_CHECK_ROW_INTERVAL を使用
                    if (row_idx & crate::constants::CANCEL_CHECK_ROW_INTERVAL) == 0
                        && cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed))
                    {
                        return Ok(matches);
                    }
                    for (col_idx, formula_str) in row.iter().enumerate() {
                        if formula_str.is_empty() {
                            continue;
                        }

                        let match_pos = if let Some(re) = regex_opt {
                            re.find(formula_str).map(|m| (m.start(), m.end()))
                        } else if query.match_case {
                            formula_str
                                .find(&query.keyword)
                                .map(|idx| (idx, idx + query.keyword.len()))
                        } else {
                            let lower_text = formula_str.to_lowercase();
                            let lower_key = query.keyword.to_lowercase();
                            lower_text
                                .find(&lower_key)
                                .map(|idx| (idx, idx + lower_key.len()))
                        };

                        if let Some((start, end)) = match_pos {
                            let (address, col_name) =
                                format_cell_address(row_idx as u32, col_idx as u32);
                            let snippet = make_snippet(formula_str, start, end);
                            let id = MATCH_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

                            matches.push(SearchMatch {
                                id,
                                file_name: file_name.clone(),
                                full_path: full_path.clone(),
                                sheet_name: sheet_name.clone(),
                                cell_address: address,
                                row_index: (row_idx + 1) as u32,
                                col_index: (col_idx + 1) as u32,
                                col_name,
                                match_type: MatchType::Formula,
                                snippet,
                                full_content: formula_str.clone(),
                                formula: Some(formula_str.clone()),
                                sheets_in_workbook: sheet_names.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(matches)
}

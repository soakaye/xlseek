//! # Excelファイル解析・検索モジュール (search/parser.rs)
//!
//! ## 処理内容
//! calamineを用いて単一のExcelブックを開き、セル値・数式・任意の Shape テキストを検索する。
//! UTF-8境界補正・HTMLエスケープ済みスニペットと、実シート可視状態・アンカー情報も扱う。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、Clippy指摘修正（is_some_and）、日本語エラー化、4要素ヘッダコメント付与。
//! - v1.3.0 (2026-09-28, Codex): Shape 抽出・検索、シート可視状態、HTML 安全化を追加。

use crate::models::{MatchType, SearchMatch, SearchQuery};
use calamine::{open_workbook_auto, Data, Reader, SheetVisible, Sheets};
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
/// 検索条件に応じた部分一致または正規表現一致のバイト範囲を得る。
/// ## 引数・戻り値
/// `text` は対象文字列、`query` は検索条件、`regex_opt` は任意の正規表現、戻り値は一致範囲。
/// ## エラー / 例外発生条件
/// 正規表現は事前コンパイル済みのためエラーを返さず、一致しない場合は `None`。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 値・数式・Shape 共通の一致判定を追加。
fn find_match_position(
    text: &str,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
) -> Option<(usize, usize)> {
    if let Some(regex) = regex_opt {
        regex.find(text).map(|found| (found.start(), found.end()))
    } else if query.match_case {
        text.find(&query.keyword)
            .map(|index| (index, index + query.keyword.len()))
    } else {
        let lower_text = text.to_lowercase();
        let lower_keyword = query.keyword.to_lowercase();
        lower_text
            .find(&lower_keyword)
            .map(|index| (index, index + lower_keyword.len()))
    }
}

/// ## 処理内容
/// ヒットした文字列の前後にコンテキストを付与し、未信頼文字列をHTMLエスケープした
/// ハイライト表示用スニペットを生成する。
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
/// 範囲外または UTF-8 文字境界外の位置は安全な境界へ補正する。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
/// - v1.3.0 (2026-09-28, Codex): スニペットに未信頼文字列の HTML エスケープを追加。
pub fn make_snippet(text: &str, mat_start: usize, mat_end: usize) -> String {
    let mut start = mat_start.min(text.len());
    while !text.is_char_boundary(start) {
        start -= 1;
    }
    let mut end = mat_end.min(text.len()).max(start);
    while !text.is_char_boundary(end) {
        end += 1;
    }
    let before_str = &text[..start];
    let matched = &text[start..end];
    let after_str = &text[end..];

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

    // 定数参照: crate::constants::HTML_ESCAPE_* を使用し、検索対象由来の HTML を無効化する。
    let escape_html = |value: &str| {
        value
            .replace('&', crate::constants::HTML_ESCAPE_AMPERSAND)
            .replace('<', crate::constants::HTML_ESCAPE_LESS_THAN)
            .replace('>', crate::constants::HTML_ESCAPE_GREATER_THAN)
            .replace('"', crate::constants::HTML_ESCAPE_DOUBLE_QUOTE)
            .replace('\'', crate::constants::HTML_ESCAPE_APOSTROPHE)
    };

    // 定数参照: crate::constants::SNIPPET_MARK_OPEN/CLOSE を使用。
    format!(
        "{}{}{}{}{}{}{}",
        prefix,
        escape_html(before),
        crate::constants::SNIPPET_MARK_OPEN,
        escape_html(matched),
        crate::constants::SNIPPET_MARK_CLOSE,
        escape_html(after),
        suffix
    )
}

/// ## 処理内容
/// 指定された単一のExcelファイルを開き、シートごとのセル値および数式を走査して検索クエリに一致するセルを抽出する。
/// 有効時は Shape テキストも検索し、Shape 結果をセルプレビュー向けの形式で返す。
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
/// - v1.3.0 (2026-09-28, Codex): Shape 検索と実シート可視状態を追加。
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
    let sheet_visibility = workbook
        .sheets_metadata()
        .iter()
        .map(|sheet| (sheet.name.clone(), sheet.visible != SheetVisible::Visible))
        .collect::<std::collections::HashMap<_, _>>();
    let mut matches = Vec::new();
    let shape_texts = if query.include_shape {
        crate::search::shape::extract_shapes(path_ref, &sheet_names, cancel_flag)
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    for sheet_name in &sheet_names {
        if cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed)) {
            return Ok(matches);
        }

        // 定数参照: SheetVisible メタデータで Hidden / VeryHidden を判定する。
        let is_hidden_sheet = sheet_visibility.get(sheet_name).copied().unwrap_or(false);

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
                    let match_pos = find_match_position(&cell_str, query, regex_opt);

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
                            shape_name: None,
                            sheet_hidden: is_hidden_sheet,
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

                        let match_pos = find_match_position(formula_str, query, regex_opt);

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
                                shape_name: None,
                                sheet_hidden: is_hidden_sheet,
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

        for shape in shape_texts
            .iter()
            .filter(|shape| shape.sheet_name == *sheet_name)
        {
            if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
                return Ok(matches);
            }
            if shape.text.is_empty() {
                continue;
            }
            if let Some((start, end)) = find_match_position(&shape.text, query, regex_opt) {
                let (cell_address, row_index, col_index, col_name) = match shape.anchor {
                    Some((row, col)) if row > 0 && col > 0 => {
                        let (address, name) = format_cell_address(row - 1, col - 1);
                        (address, row, col, name)
                    }
                    _ => (String::new(), 0, 0, String::new()),
                };
                matches.push(SearchMatch {
                    id: MATCH_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                    file_name: file_name.clone(),
                    full_path: full_path.clone(),
                    sheet_name: shape.sheet_name.clone(),
                    cell_address,
                    row_index,
                    col_index,
                    col_name,
                    match_type: MatchType::Shape,
                    shape_name: Some(shape.shape_name.clone()),
                    sheet_hidden: is_hidden_sheet,
                    snippet: make_snippet(&shape.text, start, end),
                    full_content: shape.text.clone(),
                    formula: None,
                    sheets_in_workbook: sheet_names.clone(),
                });
            }
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod snippet_tests {
    use super::make_snippet;

    /// ## 処理内容
    /// スニペットへ渡す未信頼 HTML がエスケープされ、無効な UTF-8 範囲も安全に処理されることを確認する。
    /// ## 引数・戻り値
    /// 引数なし。アサーションが失敗した場合にテストが失敗する。
    /// ## エラー / 例外発生条件
    /// 関数は panic せず、期待した文字列でない場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): 未信頼文字列のエスケープ確認を追加。
    #[test]
    fn escapes_html_and_repairs_invalid_utf8_ranges() {
        let snippet = make_snippet("<img src=x>&'\"needle", 14, 20);
        assert!(snippet.contains("&lt;img src=x&gt;&amp;&#39;&quot;"));
        assert!(snippet.contains("<mark"));
        let safe = make_snippet("猫needle", 1, 99);
        assert!(safe.contains("<mark"));
        assert!(safe.contains("needle"));
    }
}

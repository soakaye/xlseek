use crate::models::{MatchType, SearchMatch, SearchQuery};
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use regex::Regex;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static MATCH_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// 列インデックス（0始まり）から Excel 列名（A, B, AA...）に変換
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

/// セル番地文字列の生成 (例: A12)
pub fn format_cell_address(row_idx: u32, col_idx: u32) -> (String, String) {
    let col_str = col_to_name(col_idx);
    let address = format!("{}{}", col_str, row_idx + 1);
    (address, col_str)
}

/// ハイライト付き抜粋スニペットの生成 (UTF-8 char boundary安全)
pub fn make_snippet(text: &str, mat_start: usize, mat_end: usize) -> String {
    let before_str = &text[..mat_start];
    let matched = &text[mat_start..mat_end];
    let after_str = &text[mat_end..];

    // 前方は末尾から最大 20 文字を安全にスライス
    let before_chars_count = 20;
    let before_byte_len = before_str
        .char_indices()
        .rev()
        .take(before_chars_count)
        .last()
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    let before = &before_str[before_byte_len..];
    let prefix = if before_byte_len > 0 { "..." } else { "" };

    // 後方は先頭から最大 20 文字を安全にスライス
    let after_chars_count = 20;
    let (after, suffix) = match after_str.char_indices().nth(after_chars_count) {
        Some((idx, _)) => (&after_str[..idx], "..."),
        None => (after_str, ""),
    };

    format!(
        "{}{}<mark class='bg-yellow-500/30 text-yellow-300 px-0.5 rounded font-semibold'>{}</mark>{}{}",
        prefix, before, matched, after, suffix
    )
}

/// 単一 Excel ファイル内の検索
pub fn parse_and_search_file<P: AsRef<Path>>(
    path: P,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    if cancel_flag.map_or(false, |f| f.load(Ordering::Relaxed)) {
        return Ok(Vec::new());
    }

    let path_ref = path.as_ref();
    let full_path = path_ref.to_string_lossy().to_string();
    let file_name = path_ref
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut workbook: Sheets<_> = open_workbook_auto(path_ref)
        .map_err(|e| format!("Failed to open workbook {}: {}", full_path, e))?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut matches = Vec::new();

    for sheet_name in &sheet_names {
        if cancel_flag.map_or(false, |f| f.load(Ordering::Relaxed)) {
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
                if (row_idx & 0x7F) == 0 && cancel_flag.map_or(false, |f| f.load(Ordering::Relaxed)) {
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
                        cell_str.find(&query.keyword).map(|idx| (idx, idx + query.keyword.len()))
                    } else {
                        let lower_text = cell_str.to_lowercase();
                        let lower_key = query.keyword.to_lowercase();
                        lower_text.find(&lower_key).map(|idx| (idx, idx + lower_key.len()))
                    };

                    if let Some((start, end)) = match_pos {
                        let (address, col_name) = format_cell_address(row_idx as u32, col_idx as u32);
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
                    if (row_idx & 0x7F) == 0 && cancel_flag.map_or(false, |f| f.load(Ordering::Relaxed)) {
                        return Ok(matches);
                    }
                    for (col_idx, formula_str) in row.iter().enumerate() {
                        if formula_str.is_empty() {
                            continue;
                        }

                        let match_pos = if let Some(re) = regex_opt {
                            re.find(formula_str).map(|m| (m.start(), m.end()))
                        } else if query.match_case {
                            formula_str.find(&query.keyword).map(|idx| (idx, idx + query.keyword.len()))
                        } else {
                            let lower_text = formula_str.to_lowercase();
                            let lower_key = query.keyword.to_lowercase();
                            lower_text.find(&lower_key).map(|idx| (idx, idx + lower_key.len()))
                        };

                        if let Some((start, end)) = match_pos {
                            let (address, col_name) = format_cell_address(row_idx as u32, col_idx as u32);
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

//! # Excel File Parsing & Search Module (search/parser.rs)
//!
//! ## Description
//! Opens a single Excel workbook using `calamine` and searches cell values, formulas,
//! and optional shape text. Handles UTF-8 boundary correction, HTML-escaped snippets,
//! actual sheet visibility state, and cell anchors.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{MatchType, SearchMatch, SearchQuery};
use calamine::{open_workbook_auto, Data, Reader, SheetVisible, Sheets};
use regex::Regex;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// Constant reference: crate::constants::DEFAULT_MATCH_ID_START
static MATCH_ID_COUNTER: AtomicU64 = AtomicU64::new(crate::constants::DEFAULT_MATCH_ID_START);

/// ## Description
/// Converts a 0-based column index to an Excel column name (e.g., 0 -> A, 1 -> B, 26 -> AA).
///
/// ## Arguments
/// - `col_idx`: `u32` - 0-based column index
///
/// ## Returns
/// - `String`: Alphabetic column name string
///
/// ## Errors / Exceptions
/// Does not panic.
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

/// ## Description
/// Generates an Excel-format cell address string (e.g., A12) and column name from 0-based row and column indices.
///
/// ## Arguments
/// - `row_idx`: `u32` - 0-based row index
/// - `col_idx`: `u32` - 0-based column index
///
/// ## Returns
/// - `(String, String)`: Tuple of (cell address string, column name)
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn format_cell_address(row_idx: u32, col_idx: u32) -> (String, String) {
    let col_str = col_to_name(col_idx);
    let address = format!("{}{}", col_str, row_idx + 1);
    (address, col_str)
}

/// ## Description
/// Finds the byte range of a partial text match or regular expression match based on query conditions.
///
/// ## Arguments
/// - `text`: `&str` - Target string to search within
/// - `query`: `&SearchQuery` - Search query parameters
/// - `regex_opt`: `Option<&Regex>` - Optional precompiled regular expression
///
/// ## Returns
/// - `Option<(usize, usize)>`: Matching start and end byte offsets, or None if not matched
///
/// ## Errors / Exceptions
/// Does not panic; invalid regexes are caught prior to invocation.
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

/// ## Description
/// Generates a highlighted display snippet with surrounding context and HTML escaping for untrusted content.
///
/// ## Arguments
/// - `text`: `&str` - Full content string of the matching cell
/// - `mat_start`: `usize` - Match start byte offset
/// - `mat_end`: `usize` - Match end byte offset
///
/// ## Returns
/// - `String`: Snippet string containing `<mark>` highlight tags
///
/// ## Errors / Exceptions
/// Safely adjusts out-of-range or non-UTF-8 boundary offsets.
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

    // Constant reference: crate::constants::SNIPPET_CONTEXT_CHARS
    let before_chars_count = crate::constants::SNIPPET_CONTEXT_CHARS;
    let before_byte_len = before_str
        .char_indices()
        .rev()
        .take(before_chars_count)
        .last()
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    let before = &before_str[before_byte_len..];
    // Constant reference: crate::constants::SNIPPET_ELLIPSIS
    let prefix = if before_byte_len > 0 {
        crate::constants::SNIPPET_ELLIPSIS
    } else {
        ""
    };

    // Constant reference: crate::constants::SNIPPET_CONTEXT_CHARS
    let after_chars_count = crate::constants::SNIPPET_CONTEXT_CHARS;
    // Constant reference: crate::constants::SNIPPET_ELLIPSIS
    let (after, suffix) = match after_str.char_indices().nth(after_chars_count) {
        Some((idx, _)) => (&after_str[..idx], crate::constants::SNIPPET_ELLIPSIS),
        None => (after_str, ""),
    };

    // Constant reference: crate::constants::HTML_ESCAPE_*
    let escape_html = |value: &str| {
        value
            .replace('&', crate::constants::HTML_ESCAPE_AMPERSAND)
            .replace('<', crate::constants::HTML_ESCAPE_LESS_THAN)
            .replace('>', crate::constants::HTML_ESCAPE_GREATER_THAN)
            .replace('"', crate::constants::HTML_ESCAPE_DOUBLE_QUOTE)
            .replace('\'', crate::constants::HTML_ESCAPE_APOSTROPHE)
    };

    // Constant reference: crate::constants::SNIPPET_MARK_OPEN/CLOSE
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

/// ## Description
/// Opens a specified Excel workbook, scanning per-sheet cell values, formulas, comments, and shapes
/// matching the search query. Periodically checks the cancellation flag to abort execution promptly when requested.
///
/// ## Arguments
/// - `path`: `P` - Path to the Excel file
/// - `query`: `&SearchQuery` - Search criteria including keyword, regex, and include flags
/// - `regex_opt`: `Option<&Regex>` - Compiled regular expression object if regex mode is enabled
/// - `cancel_flag`: `Option<&AtomicBool>` - External cancellation signal flag
///
/// ## Returns
/// - `Result<Vec<SearchMatch>, String>`: Vector of matching search items, or error string
///
/// ## Errors / Exceptions
/// - Returns `Err` if opening the workbook or parsing fails.
/// - Does not panic.
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
        // Constant reference: crate::constants::ERR_WORKBOOK_OPEN
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
            .map_err(|error| format!("{}: {}", crate::constants::ERR_SHAPE_READ, error))?
    } else {
        Vec::new()
    };
    let comments = if query.include_comment {
        crate::search::comments::extract(path_ref)?
    } else {
        Vec::new()
    };

    for sheet_name in &sheet_names {
        if cancel_flag.is_some_and(|f| f.load(Ordering::Relaxed)) {
            return Ok(matches);
        }

        // Constant reference: SheetVisible metadata
        let is_hidden_sheet = sheet_visibility.get(sheet_name).copied().unwrap_or(false);

        if is_hidden_sheet && !query.include_hidden {
            continue;
        }

        // Read worksheet cell values
        if query.include_value {
            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                let (range_start_row, range_start_col) = range.start().unwrap_or_default();
                for (row_idx, row) in range.rows().enumerate() {
                    // Constant reference: crate::constants::CANCEL_CHECK_ROW_INTERVAL
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

                        // Match evaluation (regex or substring match)
                        let match_pos = find_match_position(&cell_str, query, regex_opt);

                        if let Some((start, end)) = match_pos {
                            let absolute_row = row_idx as u32 + range_start_row;
                            let absolute_col = col_idx as u32 + range_start_col;
                            let (address, col_name) =
                                format_cell_address(absolute_row, absolute_col);
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
                                row_index: absolute_row + 1,
                                col_index: absolute_col + 1,
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
        }

        for comment in comments
            .iter()
            .filter(|comment| comment.sheet_name == *sheet_name)
        {
            if let Some((start, end)) = find_match_position(&comment.text, query, regex_opt) {
                let (cell_address, col_name) =
                    format_cell_address(comment.row - 1, comment.col - 1);
                matches.push(SearchMatch {
                    id: MATCH_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                    file_name: file_name.clone(),
                    full_path: full_path.clone(),
                    sheet_name: sheet_name.clone(),
                    cell_address,
                    row_index: comment.row,
                    col_index: comment.col,
                    col_name,
                    match_type: MatchType::Comment,
                    shape_name: None,
                    sheet_hidden: is_hidden_sheet,
                    snippet: make_snippet(&comment.text, start, end),
                    full_content: comment.text.clone(),
                    formula: None,
                    sheets_in_workbook: sheet_names.clone(),
                });
            }
        }

        // Formula search
        if query.include_formula {
            if let Ok(formula_range) = workbook.worksheet_formula(sheet_name) {
                let (range_start_row, range_start_col) = formula_range.start().unwrap_or_default();
                for (row_idx, row) in formula_range.rows().enumerate() {
                    // Constant reference: crate::constants::CANCEL_CHECK_ROW_INTERVAL
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
                            let absolute_row = row_idx as u32 + range_start_row;
                            let absolute_col = col_idx as u32 + range_start_col;
                            let (address, col_name) =
                                format_cell_address(absolute_row, absolute_col);
                            let snippet = make_snippet(formula_str, start, end);
                            let id = MATCH_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

                            matches.push(SearchMatch {
                                id,
                                file_name: file_name.clone(),
                                full_path: full_path.clone(),
                                sheet_name: sheet_name.clone(),
                                cell_address: address,
                                row_index: absolute_row + 1,
                                col_index: absolute_col + 1,
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

    /// ## Description
    /// Verifies that untrusted HTML passed to make_snippet is escaped and invalid UTF-8 ranges are handled safely.
    ///
    /// ## Arguments / Returns
    /// No arguments. Fails if assertions fail.
    ///
    /// ## Errors / Exceptions
    /// Fails if output string does not match expected escaped markup.
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

//! # AI / LLM Context Extraction Engine (search/llm_context.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements context-aware header extraction and prompt formatting for LLMs.
//! Automatically associates Excel search hits with table column headers and row keys,
//! formats tabular context with bolded hit values, applies column windowing with ellipsis (`...`)
//! markers for wide sheets, and extracts Word document paragraphs with heading structure.
//! Conforms to Constitution Principle I (English comments), Principle II (Constant references),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust error handling and testing).

use crate::models::{HeaderContext, LlmCopyOptions, LlmCopyType, SearchMatch, WordExtractionMode};
use crate::search::parser::col_to_name;
use crate::search::word::extract_word_document;
use calamine::{open_workbook_auto, Data, Range, Reader, Sheets};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// ## Description
/// Extracts and formats structured Markdown context for a collection of search matches.
///
/// ## Arguments
/// - `matches`: `&[SearchMatch]` - List of search match results to extract context from
/// - `options`: `&LlmCopyOptions` - Extraction parameters (column window size, copy format, Word mode)
///
/// ## Returns
/// - `Result<String, String>`: Formatted Markdown string on success, or an English error message on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if matches is empty or file access fails catastrophically. Does not panic.
pub fn extract_llm_context(
    matches: &[SearchMatch],
    options: &LlmCopyOptions,
) -> Result<String, String> {
    if matches.is_empty() {
        return Err("No search matches provided for LLM context generation".to_string());
    }

    // Group matches by file path and worksheet name
    let mut grouped: BTreeMap<(&str, &str), Vec<&SearchMatch>> = BTreeMap::new();
    for m in matches {
        grouped
            .entry((m.full_path.as_str(), m.sheet_name.as_str()))
            .or_default()
            .push(m);
    }

    let mut output_sections: Vec<String> = Vec::new();

    for ((file_path_str, sheet_name), sheet_matches) in grouped {
        let path = Path::new(file_path_str);
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        // Check if Word document (.docx, .docm, .doc)
        // Constant references: crate::constants::EXT_DOCX, EXT_DOCM, EXT_DOC
        let ext_with_dot = format!(".{}", ext);
        if ext_with_dot == crate::constants::EXT_DOCX
            || ext_with_dot == crate::constants::EXT_DOCM
            || ext_with_dot == crate::constants::EXT_DOC
        {
            let section = extract_word_context(path, &sheet_matches, options)?;
            output_sections.push(section);
        } else {
            let section = extract_excel_context(path, sheet_name, &sheet_matches, options)?;
            output_sections.push(section);
        }
    }

    Ok(output_sections.join("\n\n---\n\n"))
}

/// ## Description
/// Extracts spreadsheet context for matched cells belonging to a specific workbook worksheet.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the Excel workbook file
/// - `sheet_name`: `&str` - Name of the worksheet
/// - `matches`: `&[&SearchMatch]` - Slice of search matches in this sheet
/// - `options`: `&LlmCopyOptions` - Extraction configuration options
///
/// ## Returns
/// - `Result<String, String>`: Markdown formatted snippet for this sheet
///
/// ## Errors / Exceptions
/// Returns `Err` if the workbook cannot be opened or the worksheet does not exist.
fn extract_excel_context(
    path: &Path,
    sheet_name: &str,
    matches: &[&SearchMatch],
    options: &LlmCopyOptions,
) -> Result<String, String> {
    let mut workbook: Sheets<_> = open_workbook_auto(path).map_err(|e| {
        // Constant reference: crate::constants::ERR_WORKBOOK_OPEN
        format!(
            "{}: {} ({})",
            crate::constants::ERR_WORKBOOK_OPEN,
            path.display(),
            e
        )
    })?;

    let range = workbook.worksheet_range(sheet_name).map_err(|e| {
        // Constant reference: crate::constants::ERR_SHEET_NOT_FOUND
        format!(
            "{}: '{}' ({})",
            crate::constants::ERR_SHEET_NOT_FOUND,
            sheet_name,
            e
        )
    })?;

    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    // Parse cell coordinates (0-based) for each match
    let mut match_coords: Vec<(u32, u32, &SearchMatch)> = Vec::new();
    for m in matches {
        let (r, c) = if m.row_index > 0 && m.col_index > 0 {
            (m.row_index - 1, m.col_index - 1)
        } else if let Some((r, c)) = parse_cell_address_coords(&m.cell_address) {
            (r, c)
        } else {
            (0, 0)
        };
        match_coords.push((r, c, *m));
    }

    if options.copy_type == LlmCopyType::HeaderCellOnly {
        return Ok(format_header_only_excel_context(
            file_name,
            sheet_name,
            &range,
            &match_coords,
        ));
    }

    format_table_excel_context(file_name, sheet_name, &range, &match_coords, options)
}

/// ## Description
/// Parses an Excel-style alphanumeric cell reference (e.g. "G42" or "AA10") into 0-based row and column coordinates.
///
/// ## Arguments
/// - `addr`: `&str` - Cell reference string
///
/// ## Returns
/// - `Option<(u32, u32)>`: (0-based row, 0-based column) if successful
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn parse_cell_address_coords(addr: &str) -> Option<(u32, u32)> {
    let alpha_part: String = addr
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect();
    let num_part: String = addr
        .chars()
        .skip_while(|c| c.is_ascii_alphabetic())
        .collect();
    if alpha_part.is_empty() || num_part.is_empty() {
        return None;
    }
    let mut col: u32 = 0;
    for ch in alpha_part.to_ascii_uppercase().chars() {
        col = col
            .saturating_mul(26)
            .saturating_add((ch as u32) - ('A' as u32) + 1);
    }
    let row: u32 = num_part.parse().ok()?;
    if col == 0 || row == 0 {
        return None;
    }
    Some((row - 1, col - 1))
}

/// ## Description
/// Identifies the column header text and key column label for an individual cell coordinate using heuristics.
///
/// ## Arguments
/// - `range`: `&Range<Data>` - Calamine worksheet range
/// - `target_row`: `u32` - 0-based row index of target cell
/// - `target_col`: `u32` - 0-based column index of target cell
///
/// ## Returns
/// - `HeaderContext`: Spatial and label information resolved for the cell
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn discover_header_context(
    range: &Range<Data>,
    target_row: u32,
    target_col: u32,
) -> HeaderContext {
    let header_row_idx = find_header_row_heuristic(range, target_row, target_col);
    let key_col_idx = find_key_col_heuristic(range, target_row, target_col);

    let column_header = get_cell_text(range, header_row_idx, target_col);
    let resolved_col_header = if column_header.trim().is_empty() {
        // Constant reference: crate::constants::LLM_COLUMN_PREFIX
        format!(
            "{}{}",
            crate::constants::LLM_COLUMN_PREFIX,
            col_to_name(target_col)
        )
    } else {
        column_header
    };

    let row_key = get_cell_text(range, target_row, key_col_idx);
    let resolved_row_key = if row_key.trim().is_empty() {
        format!("Row {}", target_row + 1)
    } else {
        row_key
    };

    HeaderContext {
        column_header: resolved_col_header,
        header_row_index: header_row_idx,
        row_key: resolved_row_key,
        key_col_index: key_col_idx,
    }
}

/// ## Description
/// Heuristically finds the most probable table header row by scanning upward from the target row.
///
/// ## Arguments
/// - `range`: `&Range<Data>` - Calamine worksheet range
/// - `target_row`: `u32` - 0-based row index of the target cell
/// - `target_col`: `u32` - 0-based column index of the target cell
///
/// ## Returns
/// - `u32`: 0-based row index of detected header row
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn find_header_row_heuristic(range: &Range<Data>, target_row: u32, target_col: u32) -> u32 {
    let (start_row, start_col) = range.start().unwrap_or((0, 0));
    if target_row <= start_row {
        return target_row;
    }

    let end_col = range.end().map(|(_, c)| c).unwrap_or(target_col);

    // Scan upward from target_row - 1 down to start_row
    let mut best_row = start_row;
    let mut highest_score = 0;

    for r in (start_row..target_row).rev() {
        let cell_text = get_cell_text(range, r, target_col);
        if !cell_text.trim().is_empty() {
            // Count populated cells along row r to verify if it forms a header row
            let mut populated_count = 0;
            for c in start_col..=end_col {
                if !get_cell_text(range, r, c).trim().is_empty() {
                    populated_count += 1;
                }
            }

            if populated_count > highest_score {
                highest_score = populated_count;
                best_row = r;
            }

            // If the row directly above has strong label density, choose it immediately
            if populated_count >= 2 {
                return r;
            }
        }
    }

    best_row
}

/// ## Description
/// Heuristically finds the leftmost key / identifier column for a given target row.
///
/// ## Arguments
/// - `range`: `&Range<Data>` - Calamine worksheet range
/// - `target_row`: `u32` - 0-based row index of the target cell
/// - `target_col`: `u32` - 0-based column index of the target cell
///
/// ## Returns
/// - `u32`: 0-based column index of detected key column
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn find_key_col_heuristic(range: &Range<Data>, target_row: u32, target_col: u32) -> u32 {
    let (start_row, start_col) = range.start().unwrap_or((0, 0));
    if target_col <= start_col {
        return target_col;
    }

    // Inspect columns from start_col up to target_col - 1 in target_row
    for c in start_col..target_col {
        let val = get_cell_text(range, target_row, c);
        if !val.trim().is_empty() {
            return c;
        }
    }

    // Fallback: Check header row or default to start_col
    for c in start_col..target_col {
        let val = get_cell_text(range, start_row, c);
        if !val.trim().is_empty() {
            return c;
        }
    }

    start_col
}

/// ## Description
/// Safely retrieves stringified cell text from a Calamine Range.
///
/// ## Arguments
/// - `range`: `&Range<Data>` - Range to read from
/// - `row`: `u32` - 0-based row index
/// - `col`: `u32` - 0-based col index
///
/// ## Returns
/// - `String`: Cell text representation
///
/// ## Errors / Exceptions
/// Does not panic.
fn get_cell_text(range: &Range<Data>, row: u32, col: u32) -> String {
    let (start_r, start_c) = range.start().unwrap_or((0, 0));
    if row < start_r || col < start_c {
        return String::new();
    }
    let rel_r = (row - start_r) as usize;
    let rel_c = (col - start_c) as usize;

    range
        .get((rel_r, rel_c))
        .map(data_to_string)
        .unwrap_or_default()
}

/// ## Description
/// Formats a Calamine Data enum variant into a human-readable clean string.
///
/// ## Arguments
/// - `data`: `&Data` - Data value
///
/// ## Returns
/// - `String`: String representation
///
/// ## Errors / Exceptions
/// Does not panic.
fn data_to_string(data: &Data) -> String {
    match data {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => {
            if (f.fract() == 0.0) && f.abs() < 1e15 {
                format!("{:.0}", f)
            } else {
                f.to_string()
            }
        }
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(d) => d.clone(),
        Data::DurationIso(d) => d.clone(),
        Data::Error(e) => format!("{:?}", e),
    }
}

/// ## Description
/// Formats the Target Cell metadata line including the worksheet tab name enclosed in brackets,
/// or omitting the brackets if the worksheet name is empty.
///
/// ## Arguments
/// - `sheet_name`: `&str` - Worksheet tab name
/// - `cell_address`: `&str` - Cell coordinate address (e.g., "A1")
/// - `column_header`: `&str` - Detected column header text
/// - `row_key`: `&str` - Detected row key text
///
/// ## Returns
/// - `String`: Formatted "- Target Cell: ..." metadata line
///
/// ## Errors / Exceptions
/// Does not panic.
fn format_target_cell_line(
    sheet_name: &str,
    cell_address: &str,
    column_header: &str,
    row_key: &str,
) -> String {
    let trimmed_sheet = sheet_name.trim();
    if trimmed_sheet.is_empty() {
        // Constant reference: crate::constants::LLM_TARGET_CELL_PREFIX
        format!(
            "{}{} (Column: {}, Row: {})",
            crate::constants::LLM_TARGET_CELL_PREFIX,
            cell_address,
            column_header,
            row_key
        )
    } else {
        // Constant reference: crate::constants::LLM_TARGET_CELL_PREFIX
        format!(
            "{}[{}] {} (Column: {}, Row: {})",
            crate::constants::LLM_TARGET_CELL_PREFIX,
            trimmed_sheet,
            cell_address,
            column_header,
            row_key
        )
    }
}

/// ## Description
/// Formats matches in compact Header & Cell Only mode.
///
/// ## Arguments
/// - `file_name`: `&str` - Display file name
/// - `sheet_name`: `&str` - Sheet name
/// - `range`: `&Range<Data>` - Worksheet range
/// - `match_coords`: `&[(u32, u32, &SearchMatch)]` - Matched coordinates
///
/// ## Returns
/// - `String`: Formatted Markdown bullet list
///
/// ## Errors / Exceptions
/// Does not panic.
fn format_header_only_excel_context(
    file_name: &str,
    sheet_name: &str,
    range: &Range<Data>,
    match_coords: &[(u32, u32, &SearchMatch)],
) -> String {
    let mut lines = Vec::new();
    lines.push(format!("### {} - {}", file_name, sheet_name));

    for &(r, c, m) in match_coords {
        let headers = discover_header_context(range, r, c);
        let cell_val = get_cell_text(range, r, c);
        let display_val = if cell_val.is_empty() {
            &m.full_content
        } else {
            &cell_val
        };

        lines.push(format_target_cell_line(
            sheet_name,
            &m.cell_address,
            &headers.column_header,
            &headers.row_key,
        ));
        lines.push(format!("  - Matched Value: **{}**", display_val));
    }

    lines.join("\n")
}

/// ## Description
/// Formats matches into a structured Markdown table including metadata headers, bold hit values,
/// and column windowing with ellipsis (`...`) markers for wide sheets.
///
/// ## Arguments
/// - `file_name`: `&str` - File name
/// - `sheet_name`: `&str` - Worksheet name
/// - `range`: `&Range<Data>` - Worksheet range
/// - `match_coords`: `&[(u32, u32, &SearchMatch)]` - Matched coordinates
/// - `options`: `&LlmCopyOptions` - Copy options
///
/// ## Returns
/// - `Result<String, String>`: Formatted Markdown table section
///
/// ## Errors / Exceptions
/// Does not panic.
fn format_table_excel_context(
    file_name: &str,
    sheet_name: &str,
    range: &Range<Data>,
    match_coords: &[(u32, u32, &SearchMatch)],
    options: &LlmCopyOptions,
) -> Result<String, String> {
    let mut lines = Vec::new();
    lines.push(format!("### {} - {}", file_name, sheet_name));

    // Enumerate metadata entries for each hit cell
    for &(r, c, m) in match_coords {
        let headers = discover_header_context(range, r, c);
        let cell_val = get_cell_text(range, r, c);
        let display_val = if cell_val.is_empty() {
            &m.full_content
        } else {
            &cell_val
        };
        lines.push(format_target_cell_line(
            sheet_name,
            &m.cell_address,
            &headers.column_header,
            &headers.row_key,
        ));
        lines.push(format!("  - Matched Value: **{}**", display_val));
    }
    lines.push(String::new());

    // Determine common header row and key column
    let (start_r, start_c) = range.start().unwrap_or((0, 0));
    let (_, end_c) = range.end().unwrap_or((0, 0));

    // Find primary header row from the first match
    let header_row_idx = if let Some(&(r, c, _)) = match_coords.first() {
        find_header_row_heuristic(range, r, c)
    } else {
        start_r
    };

    // Find primary key column from the first match
    let key_col_idx = if let Some(&(r, c, _)) = match_coords.first() {
        find_key_col_heuristic(range, r, c)
    } else {
        start_c
    };

    // Calculate column windowing across all matches in this sheet
    // Constant reference: crate::constants::MIN_LLM_COLUMN_WINDOW_SIZE, MAX_LLM_COLUMN_WINDOW_SIZE
    let window_size = options.column_window_size.clamp(
        crate::constants::MIN_LLM_COLUMN_WINDOW_SIZE,
        crate::constants::MAX_LLM_COLUMN_WINDOW_SIZE,
    );

    // Build ordered list of column indices and ellipsis placements
    let mut included_cols: BTreeSet<u32> = BTreeSet::new();
    included_cols.insert(key_col_idx);

    for &(_, c, _) in match_coords {
        let min_w = c.saturating_sub(window_size).max(start_c);
        let max_w = (c + window_size).min(end_c);
        for col_i in min_w..=max_w {
            included_cols.insert(col_i);
        }
    }

    // Build display columns with ellipsis representation
    // An entry is (Option<u32>, label) where None denotes ellipsis
    let mut display_cols: Vec<(Option<u32>, String)> = Vec::new();
    let cols_vec: Vec<u32> = included_cols.into_iter().collect();

    for (i, &col_idx) in cols_vec.iter().enumerate() {
        if i > 0 {
            let prev_col = cols_vec[i - 1];
            if col_idx > prev_col + 1 {
                // Constant reference: crate::constants::LLM_ELLIPSIS_COL
                display_cols.push((None, crate::constants::LLM_ELLIPSIS_COL.to_string()));
            }
        }
        let header_txt = get_cell_text(range, header_row_idx, col_idx);
        let label = if header_txt.trim().is_empty() {
            // Constant reference: crate::constants::LLM_COLUMN_PREFIX
            format!(
                "{}{}",
                crate::constants::LLM_COLUMN_PREFIX,
                col_to_name(col_idx)
            )
        } else {
            header_txt
        };
        display_cols.push((Some(col_idx), label));
    }

    // If there are more columns beyond the last included column, add trailing ellipsis
    if let Some(&last_col) = cols_vec.last() {
        if last_col < end_c {
            // Constant reference: crate::constants::LLM_ELLIPSIS_COL
            display_cols.push((None, crate::constants::LLM_ELLIPSIS_COL.to_string()));
        }
    }

    // Build Markdown Table Header
    let mut header_cells = Vec::new();
    let mut separator_cells = Vec::new();
    for (_, label) in &display_cols {
        header_cells.push(escape_markdown_cell(label));
        separator_cells.push("---");
    }
    lines.push(format!("| {} |", header_cells.join(" | ")));
    lines.push(format!("| {} |", separator_cells.join(" | ")));

    // Group hits by row index to assemble rows
    let mut rows_set: BTreeSet<u32> = BTreeSet::new();
    let mut hit_cells_set: BTreeSet<(u32, u32)> = BTreeSet::new();
    for &(r, c, _) in match_coords {
        rows_set.insert(r);
        hit_cells_set.insert((r, c));
    }

    for row_idx in rows_set {
        let mut row_cells = Vec::new();
        for (col_opt, _) in &display_cols {
            match col_opt {
                Some(col_idx) => {
                    let cell_text = get_cell_text(range, row_idx, *col_idx);
                    let clean = escape_markdown_cell(&cell_text);
                    if hit_cells_set.contains(&(row_idx, *col_idx)) {
                        row_cells.push(format!("**{}**", clean));
                    } else {
                        row_cells.push(clean);
                    }
                }
                None => {
                    // Constant reference: crate::constants::LLM_ELLIPSIS_COL
                    row_cells.push(crate::constants::LLM_ELLIPSIS_COL.to_string());
                }
            }
        }
        lines.push(format!("| {} |", row_cells.join(" | ")));
    }

    Ok(lines.join("\n"))
}

/// ## Description
/// Extracts and formats Word document matches (.docx, .docm, .doc) according to configured mode.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the Word document file
/// - `matches`: `&[&SearchMatch]` - Word matches in this file
/// - `options`: `&LlmCopyOptions` - Extraction configuration options
///
/// ## Returns
/// - `Result<String, String>`: Formatted Markdown snippet
///
/// ## Errors / Exceptions
/// Returns `Err` if the Word document cannot be parsed.
fn extract_word_context(
    path: &Path,
    matches: &[&SearchMatch],
    options: &LlmCopyOptions,
) -> Result<String, String> {
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let mut lines = Vec::new();
    lines.push(format!("### {}", file_name));

    for m in matches {
        let target_p = if m.row_index > 0 { m.row_index } else { 1 };

        let preview = extract_word_document(path, target_p)?;

        // Find the matched paragraph text
        let target_para = preview
            .paragraphs
            .iter()
            .find(|p| p.index == target_p)
            .or_else(|| preview.paragraphs.first());

        let raw_text = target_para
            .map(|p| p.text.as_str())
            .unwrap_or(&m.full_content);

        // Highlight matched text in bold if keyword or snippet is identifiable
        let highlighted_text = highlight_word_match(raw_text, &m.snippet);

        if options.word_mode == WordExtractionMode::HeadingsAndParagraph {
            let heading = find_preceding_heading(&preview.paragraphs, target_p);
            lines.push(format!(
                "- Location: Paragraph {}\n- Section: {}\n\n### {}\n> {}",
                target_p, heading, heading, highlighted_text
            ));
        } else {
            lines.push(format!(
                "- Location: Paragraph {}\n\n> {}",
                target_p, highlighted_text
            ));
        }
    }

    Ok(lines.join("\n\n"))
}

/// ## Description
/// Locates the nearest preceding heading paragraph above the target paragraph in a Word document.
///
/// ## Arguments
/// - `paragraphs`: `&[WordParagraph]` - Document paragraphs
/// - `target_p`: `u32` - Target paragraph number
///
/// ## Returns
/// - `String`: Identified heading title or default fallback string
///
/// ## Errors / Exceptions
/// Does not panic.
fn find_preceding_heading(paragraphs: &[crate::models::WordParagraph], target_p: u32) -> String {
    for p in paragraphs.iter().rev() {
        if p.index < target_p {
            let trimmed = p.text.trim();
            if !trimmed.is_empty() && is_probable_heading(trimmed) {
                return trimmed.to_string();
            }
        }
    }
    "Document Section".to_string()
}

/// ## Description
/// Determines whether a paragraph string looks like a section heading.
///
/// ## Arguments
/// - `text`: `&str` - Paragraph text
///
/// ## Returns
/// - `bool`: True if text matches heading characteristics
///
/// ## Errors / Exceptions
/// Does not panic.
fn is_probable_heading(text: &str) -> bool {
    if text.starts_with('#') {
        return true;
    }
    if text.len() <= 80 && !text.ends_with('。') && !text.ends_with('.') {
        // Starts with chapter/section numbering like "1.", "1.1", "第1章", etc.
        let first_word = text.split_whitespace().next().unwrap_or_default();
        if first_word.ends_with('.')
            || text.starts_with("第")
            || text.starts_with("Chapter")
            || text.starts_with("Section")
        {
            return true;
        }
    }
    false
}

/// ## Description
/// Bolds the search match text within a Word paragraph.
///
/// ## Arguments
/// - `raw_text`: `&str` - Original paragraph text
/// - `snippet`: `&str` - Search result snippet containing mark tags
///
/// ## Returns
/// - `String`: Formatted text with Markdown bolding
///
/// ## Errors / Exceptions
/// Does not panic.
fn highlight_word_match(raw_text: &str, snippet: &str) -> String {
    // If snippet contains <mark> tags, extract matched keyword
    if let Some(start_tag) = snippet.find("<mark") {
        if let Some(tag_end) = snippet[start_tag..].find('>') {
            let inner_start = start_tag + tag_end + 1;
            if let Some(close_tag) = snippet[inner_start..].find("</mark>") {
                let matched_keyword = &snippet[inner_start..inner_start + close_tag];
                if !matched_keyword.is_empty() && raw_text.contains(matched_keyword) {
                    return raw_text.replace(matched_keyword, &format!("**{}**", matched_keyword));
                }
            }
        }
    }
    raw_text.to_string()
}

/// ## Description
/// Escapes pipe characters and cleans newlines for Markdown table compatibility.
///
/// ## Arguments
/// - `text`: `&str` - Cell text
///
/// ## Returns
/// - `String`: Escaped safe cell text
///
/// ## Errors / Exceptions
/// Does not panic.
fn escape_markdown_cell(text: &str) -> String {
    text.replace('|', "\\|")
        .replace('\n', " ")
        .replace('\r', "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{LlmCopyType, MatchType, SearchMatch, WordExtractionMode};
    use calamine::Data;

    #[test]
    fn test_parse_cell_address_coords() {
        assert_eq!(parse_cell_address_coords("A1"), Some((0, 0)));
        assert_eq!(parse_cell_address_coords("G42"), Some((41, 6)));
        assert_eq!(parse_cell_address_coords("AA10"), Some((9, 26)));
        assert_eq!(parse_cell_address_coords(""), None);
        assert_eq!(parse_cell_address_coords("42"), None);
        assert_eq!(parse_cell_address_coords("ABC"), None);
    }

    #[test]
    fn test_escape_markdown_cell() {
        assert_eq!(escape_markdown_cell("Normal"), "Normal");
        assert_eq!(escape_markdown_cell("Col | Val"), "Col \\| Val");
        assert_eq!(escape_markdown_cell("Multi\nLine"), "Multi Line");
    }

    #[test]
    fn test_header_heuristic_on_synthetic_range() {
        let mut range = Range::new((0, 0), (2, 3));
        // Row 0: Banner
        range.set_value(
            (0, 0),
            Data::String("Company Financial Report 2026".to_string()),
        );
        // Row 1: Column Headers
        range.set_value((1, 0), Data::String("ID".to_string()));
        range.set_value((1, 1), Data::String("Name".to_string()));
        range.set_value((1, 2), Data::String("Status".to_string()));
        range.set_value((1, 3), Data::String("Amount".to_string()));
        // Row 2: Data row
        range.set_value((2, 0), Data::String("101".to_string()));
        range.set_value((2, 1), Data::String("Alpha Project".to_string()));
        range.set_value((2, 2), Data::String("Open".to_string()));
        range.set_value((2, 3), Data::Int(150000));

        // Testing target cell at G3 (row 2, col 3)
        let header = discover_header_context(&range, 2, 3);
        assert_eq!(header.header_row_index, 1);
        assert_eq!(header.column_header, "Amount");
        assert_eq!(header.row_key, "101");
        assert_eq!(header.key_col_index, 0);
    }

    #[test]
    fn test_header_only_and_table_format_synthetic() {
        let mut range = Range::new((0, 0), (1, 1));
        range.set_value((0, 0), Data::String("Task".to_string()));
        range.set_value((0, 1), Data::String("Cost".to_string()));
        range.set_value((1, 0), Data::String("Design".to_string()));
        range.set_value((1, 1), Data::Int(5000));

        let m = SearchMatch {
            id: 1,
            file_name: "test.xlsx".to_string(),
            full_path: "test.xlsx".to_string(),
            sheet_name: "Sheet1".to_string(),
            cell_address: "B2".to_string(),
            row_index: 2,
            col_index: 2,
            col_name: "B".to_string(),
            match_type: MatchType::CellValue,
            shape_name: None,
            sheet_hidden: false,
            snippet: "5000".to_string(),
            full_content: "5000".to_string(),
            formula: None,
            sheets_in_workbook: vec!["Sheet1".to_string()],
        };

        let coords = [(1, 1, &m)];

        // Header only format
        let header_only = format_header_only_excel_context("test.xlsx", "Sheet1", &range, &coords);
        assert!(header_only.contains("Column: Cost"));
        assert!(header_only.contains("Row: Design"));
        assert!(header_only.contains("**5000**"));
        assert!(header_only.contains("- Target Cell: [Sheet1] B2 (Column: Cost, Row: Design)"));

        // Table format
        let options = LlmCopyOptions {
            column_window_size: 5,
            copy_type: LlmCopyType::Table,
            word_mode: WordExtractionMode::HeadingsAndParagraph,
        };
        let table =
            format_table_excel_context("test.xlsx", "Sheet1", &range, &coords, &options).unwrap();
        assert!(table.contains("| Task | Cost |"));
        assert!(table.contains("| Design | **5000** |"));
        assert!(table.contains("- Target Cell: [Sheet1] B2 (Column: Cost, Row: Design)"));
    }

    #[test]
    fn test_wide_sheet_column_windowing_synthetic() {
        let mut range = Range::new((0, 0), (1, 20));
        // Set headers across 20 columns
        for c in 0..=20 {
            range.set_value((0, c), Data::String(format!("Col{}", c)));
        }
        // Set row 1 data
        for c in 0..=20 {
            range.set_value((1, c), Data::Int(c as i64 * 10));
        }

        let m = SearchMatch {
            id: 1,
            file_name: "wide.xlsx".to_string(),
            full_path: "wide.xlsx".to_string(),
            sheet_name: "WideSheet".to_string(),
            cell_address: "P2".to_string(),
            row_index: 2,
            col_index: 16,
            col_name: "P".to_string(),
            match_type: MatchType::CellValue,
            shape_name: None,
            sheet_hidden: false,
            snippet: "150".to_string(),
            full_content: "150".to_string(),
            formula: None,
            sheets_in_workbook: vec!["WideSheet".to_string()],
        };

        let coords = [(1, 15, &m)];
        let options = LlmCopyOptions {
            column_window_size: 3,
            copy_type: LlmCopyType::Table,
            word_mode: WordExtractionMode::HeadingsAndParagraph,
        };

        let table = format_table_excel_context("wide.xlsx", "WideSheet", &range, &coords, &options)
            .unwrap();
        // Check that key column (Col0) is present
        assert!(table.contains("Col0"));
        // Check that ellipsis is present
        assert!(table.contains("..."));
        // Check that target cell is bolded
        assert!(table.contains("**150**"));
    }

    /// ## Description
    /// Validates `format_target_cell_line` across standard, Japanese, and special character sheet names,
    /// ensuring bracket notation `[SheetName]` is correctly applied without corruption.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// None.
    ///
    /// ## Errors / Exceptions
    /// Panics on assertion failure.
    #[test]
    fn test_target_cell_line_formatting() {
        // Standard alphanumeric sheet name
        let line = format_target_cell_line("Sheet1", "B2", "Status", "Item 1");
        assert_eq!(
            line,
            "- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)"
        );

        // Japanese synthetic sheet name
        let line_ja = format_target_cell_line("シート1", "C10", "Category", "Row 5");
        assert_eq!(
            line_ja,
            "- Target Cell: [シート1] C10 (Column: Category, Row: Row 5)"
        );

        // Special characters, spaces, and parentheses in sheet name
        let line_special = format_target_cell_line("FY2026 Q3 (Final)", "A1", "Header", "Key");
        assert_eq!(
            line_special,
            "- Target Cell: [FY2026 Q3 (Final)] A1 (Column: Header, Row: Key)"
        );
    }

    /// ## Description
    /// Validates `format_target_cell_line` fallback behavior when the sheet name is empty
    /// or consists solely of whitespace, ensuring empty brackets `[]` are omitted.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// None.
    ///
    /// ## Errors / Exceptions
    /// Panics on assertion failure.
    #[test]
    fn test_target_cell_empty_sheet_fallback() {
        // Empty sheet name fallback
        let line_empty = format_target_cell_line("", "B2", "Status", "Item 1");
        assert_eq!(
            line_empty,
            "- Target Cell: B2 (Column: Status, Row: Item 1)"
        );

        // Whitespace-only sheet name fallback
        let line_ws = format_target_cell_line("   ", "B2", "Status", "Item 1");
        assert_eq!(line_ws, "- Target Cell: B2 (Column: Status, Row: Item 1)");
    }
}

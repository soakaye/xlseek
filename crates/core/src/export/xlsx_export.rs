//! # Excel Export Module (export/xlsx_export.rs)
//!
//! ## Description
//! Outputs a list of search result items to a formatted Excel (.xlsx) workbook.
//! Provides header styling (background/font color, borders), column width adjustments,
//! auto-filter application, and error handling with English error strings.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! and Principle III (Header comments).

use crate::models::{MatchType, SearchMatch};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};
use std::collections::BTreeMap;

/// ## Description
/// Outputs a slice of search result items to a formatted Excel (.xlsx) file.
/// Performs automatic column width sizing, header styling, and auto-filter configuration.
///
/// ## Arguments
/// - `path`: `&str` - Target Excel file path
/// - `items`: `&[SearchMatch]` - Slice of search match items to export
/// - `language`: `&str` - Output language (`ja` or `en`)
/// - `catalogs`: `&BTreeMap<String, BTreeMap<String, String>>` - Translation catalogs
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an English error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if an unsupported language is provided, required translations are missing,
///   or worksheet creation/writing/saving fails.
/// - Does not panic.
pub fn export_to_xlsx(
    path: &str,
    items: &[SearchMatch],
    language: &str,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Constant reference: crate::constants::EXPORT_SHEET_NAME_KEY
    let sheet_name = crate::i18n::resolve_catalog_text(
        catalogs,
        language,
        crate::constants::EXPORT_SHEET_NAME_KEY,
    )
    .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())?;
    worksheet.set_name(&sheet_name).map_err(|e| {
        // Constant reference: crate::constants::ERR_XLSX_WORKSHEET
        format!("{}: {}", crate::constants::ERR_XLSX_WORKSHEET, e)
    })?;

    // Style definitions
    // Constant reference: crate::constants::XLSX_HEADER_BG_COLOR, XLSX_HEADER_FG_COLOR
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(crate::constants::XLSX_HEADER_BG_COLOR))
        .set_font_color(Color::RGB(crate::constants::XLSX_HEADER_FG_COLOR))
        .set_border(FormatBorder::Thin);

    let cell_format = Format::new().set_border(FormatBorder::Thin);

    let id_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    // Constant reference: crate::constants::EXPORT_HEADER_KEYS and XLSX_COLUMN_WIDTHS
    let headers = crate::constants::EXPORT_HEADER_KEYS
        .iter()
        .map(|key| {
            crate::i18n::resolve_catalog_text(catalogs, language, key)
                .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (col_idx, (header, width)) in headers
        .iter()
        .zip(crate::constants::XLSX_COLUMN_WIDTHS.iter())
        .enumerate()
    {
        worksheet
            .write_string_with_format(0, col_idx as u16, header, &header_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .set_column_width(col_idx as u16, *width)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
    }

    // Write data rows
    for (row_idx, item) in items.iter().enumerate() {
        let r = (row_idx + 1) as u32;
        // Constant reference: crate::constants::EXPORT_MATCH_*_KEY
        let match_key = match item.match_type {
            MatchType::CellValue => crate::constants::EXPORT_MATCH_VALUE_KEY,
            MatchType::Formula => crate::constants::EXPORT_MATCH_FORMULA_KEY,
            MatchType::Comment => crate::constants::EXPORT_MATCH_COMMENT_KEY,
            MatchType::HiddenSheet => crate::constants::EXPORT_MATCH_HIDDEN_SHEET_KEY,
            MatchType::Shape => crate::constants::EXPORT_MATCH_SHAPE_KEY,
        };
        let match_type_str = crate::i18n::resolve_catalog_text(catalogs, language, match_key)
            .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())?;

        worksheet
            .write_number_with_format(r, 0, item.id as f64, &id_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 1, &item.file_name, &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        let clean_path = crate::export::normalize_export_path(&item.full_path);
        worksheet
            .write_string_with_format(r, 2, clean_path.as_ref(), &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 3, &item.sheet_name, &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 4, &item.cell_address, &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 5, item.shape_name.as_deref().unwrap_or(""), &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 6, &match_type_str, &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 7, &item.full_content, &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 8, item.formula.as_deref().unwrap_or(""), &cell_format)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_XLSX_WRITE
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
    }

    // Enable auto-filter if items exist
    if !items.is_empty() {
        let last_row = items.len() as u32;
        worksheet.autofilter(0, 0, last_row, 8).map_err(|e| {
            // Constant reference: crate::constants::ERR_XLSX_WRITE
            format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
        })?;
    }

    workbook.save(path).map_err(|e| {
        // Constant reference: crate::constants::ERR_XLSX_SAVE
        format!("{}: {}", crate::constants::ERR_XLSX_SAVE, e)
    })?;
    Ok(())
}

#[cfg(test)]
mod localization_tests {
    use calamine::Reader;
    use std::collections::BTreeMap;

    const TEST_OUTPUT_PREFIX: &str = "xlseek-i18n-test";

    /// ## Description
    /// Verifies that Excel column headers use the requested catalog language.
    ///
    /// ## Arguments / Returns
    /// No arguments. Reads output files and compares the second column header.
    ///
    /// ## Errors / Exceptions
    /// Fails if file operations, workbook parsing, or header matching fails.
    #[test]
    fn uses_requested_catalog_for_headers() {
        let mut english = BTreeMap::from([(
            crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
            "Some text could not be translated.".to_string(),
        )]);
        for key in crate::constants::EXPORT_HEADER_KEYS {
            english.insert(key.to_string(), key.to_string());
        }
        english.insert(
            "export.header.fileName".to_string(),
            "File name".to_string(),
        );
        english.insert(
            crate::constants::EXPORT_SHEET_NAME_KEY.to_string(),
            "Search Results".to_string(),
        );
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        japanese.insert(
            "export.header.fileName".to_string(),
            "ファイル名".to_string(),
        );
        japanese.insert(
            crate::constants::EXPORT_SHEET_NAME_KEY.to_string(),
            "検索結果".to_string(),
        );
        let catalogs = BTreeMap::from([
            (crate::constants::LANGUAGE_EN.to_string(), english),
            (crate::constants::LANGUAGE_JA.to_string(), japanese),
        ]);

        for (language, expected) in [
            (crate::constants::LANGUAGE_EN, "File name"),
            (crate::constants::LANGUAGE_JA, "ファイル名"),
        ] {
            // Constant reference: TEST_OUTPUT_PREFIX
            let path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}-{language}.xlsx"));
            super::export_to_xlsx(path.to_str().unwrap(), &[], language, &catalogs).unwrap();
            let mut workbook = calamine::open_workbook_auto(&path).unwrap();
            let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();
            assert_eq!(
                sheet.get_value((0, 1)).map(ToString::to_string).as_deref(),
                Some(expected)
            );
            std::fs::remove_file(path).unwrap();
        }
    }

    /// ## Description
    /// Verifies that Excel export rejects unsupported languages before creating workbooks.
    ///
    /// ## Arguments / Returns
    /// No arguments. Executes assertions.
    ///
    /// ## Errors / Exceptions
    /// Fails on unexpected export result.
    #[test]
    fn rejects_unsupported_language() {
        assert_eq!(
            super::export_to_xlsx("", &[], "fr", &BTreeMap::new()),
            Err(crate::constants::ERR_INVALID_LANGUAGE.to_string())
        );
    }

    /// ## Description
    /// Verifies that `export_to_xlsx` strips Windows verbatim prefixes from full_path in worksheets.
    ///
    /// ## Arguments / Returns
    /// No arguments. Outputs a test SearchMatch to a temporary .xlsx file and inspects column 2.
    ///
    /// ## Errors / Exceptions
    /// Panics on assertion failure.
    #[test]
    fn export_to_xlsx_strips_windows_verbatim_paths() {
        let mut english = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            english.insert(key.to_string(), key.to_string());
        }
        english.insert(
            crate::constants::EXPORT_SHEET_NAME_KEY.to_string(),
            "Results".to_string(),
        );
        english.insert(
            crate::constants::EXPORT_MATCH_VALUE_KEY.to_string(),
            "Match".to_string(),
        );
        let catalogs = BTreeMap::from([(crate::constants::LANGUAGE_EN.to_string(), english)]);

        let item = crate::models::SearchMatch {
            id: 1,
            file_name: "test.xlsx".to_string(),
            full_path: r"\\?\C:\Projects\test.xlsx".to_string(),
            sheet_name: "Sheet1".to_string(),
            cell_address: "A1".to_string(),
            row_index: 0,
            col_index: 0,
            col_name: "A".to_string(),
            shape_name: None,
            match_type: crate::models::MatchType::CellValue,
            sheet_hidden: false,
            snippet: "foo".to_string(),
            full_content: "foo".to_string(),
            formula: None,
            sheets_in_workbook: vec!["Sheet1".to_string()],
        };

        let path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}-verbatim-strip.xlsx"));
        super::export_to_xlsx(
            path.to_str().unwrap(),
            &[item],
            crate::constants::LANGUAGE_EN,
            &catalogs,
        )
        .unwrap();

        let mut workbook = calamine::open_workbook_auto(&path).unwrap();
        let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();
        let actual_path = sheet.get_value((1, 2)).map(ToString::to_string).unwrap();
        assert_eq!(actual_path, r"C:\Projects\test.xlsx");
        assert!(!actual_path.contains(r"\\?\"));

        std::fs::remove_file(path).unwrap();
    }
}

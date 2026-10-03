//! # CSV Export Module (export/csv_export.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Writes matched search items as a UTF-8 CSV file with BOM.
//! Prepends a BOM to prevent garbled characters in Excel, and formats column headers and data rows.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{MatchType, SearchMatch};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

/// ## Description
/// Outputs a slice of search result items to any `Write` stream in CSV format.
/// Attaches a UTF-8 BOM if include_bom is true, then outputs column headers and data rows.
///
/// ## Arguments
/// - `writer`: `&mut W` - Target writable stream
/// - `items`: `&[SearchMatch]` - Slice of search match items to export
/// - `language`: `&str` - Output language (`ja` or `en`)
/// - `catalogs`: `&BTreeMap<String, BTreeMap<String, String>>` - Translation catalogs
/// - `include_bom`: `bool` - Whether to prepend a UTF-8 BOM (true for files, false for stdout)
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if an unsupported language is provided, required translations are missing,
///   or BOM/record writing/flushing fails.
/// - Does not panic.
pub fn write_csv_to_writer<W: Write>(
    writer: &mut W,
    items: &[SearchMatch],
    language: &str,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    include_bom: bool,
) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }

    if include_bom {
        // Prepend UTF-8 BOM to prevent character corruption in Excel
        // Constant reference: crate::constants::CSV_UTF8_BOM
        writer
            .write_all(&crate::constants::CSV_UTF8_BOM)
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_CSV_HEADER_WRITE
                format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
            })?;
    }

    let mut csv_writer = csv::Writer::from_writer(writer);

    // Constant reference: crate::constants::EXPORT_HEADER_KEYS
    let headers = crate::constants::EXPORT_HEADER_KEYS
        .iter()
        .map(|key| {
            crate::i18n::resolve_catalog_text(catalogs, language, key)
                .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    csv_writer.write_record(&headers).map_err(|e| {
        // Constant reference: crate::constants::ERR_CSV_HEADER_WRITE
        format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
    })?;

    // Data rows
    for item in items {
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

        let clean_path = crate::export::normalize_export_path(&item.full_path);
        // Pass array directly to avoid Clippy needless_borrows_for_generic_args
        csv_writer
            .write_record([
                item.id.to_string(),
                item.file_name.clone(),
                clean_path.into_owned(),
                item.sheet_name.clone(),
                item.cell_address.clone(),
                item.shape_name.clone().unwrap_or_default(),
                match_type_str,
                item.full_content.clone(),
                item.formula.clone().unwrap_or_default(),
            ])
            .map_err(|e| {
                // Constant reference: crate::constants::ERR_CSV_RECORD_WRITE
                format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
            })?;
    }

    csv_writer.flush().map_err(|e| {
        // Constant reference: crate::constants::ERR_CSV_RECORD_WRITE
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    Ok(())
}

/// ## Description
/// Outputs a slice of search result items to a UTF-8 CSV file with BOM.
///
/// ## Arguments
/// - `path`: `&str` - Output CSV file path
/// - `items`: `&[SearchMatch]` - Slice of search match items to export
/// - `language`: `&str` - Output language (`ja` or `en`)
/// - `catalogs`: `&BTreeMap<String, BTreeMap<String, String>>` - Translation catalogs
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an English error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if an unsupported language is provided, required translations are missing,
///   or file creation/writing fails.
/// - Does not panic.
pub fn export_to_csv(
    path: &str,
    items: &[SearchMatch],
    language: &str,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }
    let file = File::create(path).map_err(|e| {
        // Constant reference: crate::constants::ERR_CSV_RECORD_WRITE
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    let mut writer = std::io::BufWriter::new(file);
    write_csv_to_writer(&mut writer, items, language, catalogs, true)
}

#[cfg(test)]
mod localization_tests {
    use std::collections::BTreeMap;

    const TEST_OUTPUT_PREFIX: &str = "xlseek-i18n-test";

    /// ## Description
    /// Verifies that CSV column headers use the requested catalog language.
    ///
    /// ## Arguments / Returns
    /// No arguments. Reads output files and compares the second column header.
    ///
    /// ## Errors / Exceptions
    /// Fails if file operations, CSV parsing, or header matching fails.
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
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        japanese.insert(
            "export.header.fileName".to_string(),
            "ファイル名".to_string(),
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
            let path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}-{language}.csv"));
            super::export_to_csv(path.to_str().unwrap(), &[], language, &catalogs).unwrap();
            let mut reader = csv::Reader::from_path(&path).unwrap();
            assert_eq!(reader.headers().unwrap().get(1), Some(expected));
            std::fs::remove_file(path).unwrap();
        }
    }

    /// ## Description
    /// Verifies that CSV export rejects unsupported languages before creating files.
    ///
    /// ## Arguments / Returns
    /// No arguments. Executes assertions.
    ///
    /// ## Errors / Exceptions
    /// Fails on unexpected export result.
    #[test]
    fn rejects_unsupported_language() {
        assert_eq!(
            super::export_to_csv("", &[], "fr", &BTreeMap::new()),
            Err(crate::constants::ERR_INVALID_LANGUAGE.to_string())
        );
    }

    /// ## Description
    /// Verifies that `write_csv_to_writer` respects the BOM inclusion flag.
    ///
    /// ## Arguments / Returns
    /// No arguments. Outputs to memory buffer and inspects leading bytes.
    ///
    /// ## Errors / Exceptions
    /// Fails if catalog resolution, writing, or byte matching fails.
    #[test]
    fn write_csv_to_writer_respects_bom_flag() {
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        let catalogs = BTreeMap::from([(crate::constants::LANGUAGE_JA.to_string(), japanese)]);

        // With BOM
        let mut buffer_with_bom = Vec::new();
        super::write_csv_to_writer(
            &mut buffer_with_bom,
            &[],
            crate::constants::LANGUAGE_JA,
            &catalogs,
            true,
        )
        .unwrap();
        assert!(buffer_with_bom.starts_with(&crate::constants::CSV_UTF8_BOM));

        // Without BOM
        let mut buffer_without_bom = Vec::new();
        super::write_csv_to_writer(
            &mut buffer_without_bom,
            &[],
            crate::constants::LANGUAGE_JA,
            &catalogs,
            false,
        )
        .unwrap();
        assert!(!buffer_without_bom.starts_with(&crate::constants::CSV_UTF8_BOM));
    }

    /// ## Description
    /// Verifies that `write_csv_to_writer` strips Windows verbatim prefixes from full_path in CSV rows.
    ///
    /// ## Arguments / Returns
    /// No arguments. Outputs a test SearchMatch and inspects the resulting CSV text.
    ///
    /// ## Errors / Exceptions
    /// Panics on assertion failure.
    #[test]
    fn write_csv_to_writer_strips_windows_verbatim_paths() {
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        japanese.insert(
            crate::constants::EXPORT_MATCH_VALUE_KEY.to_string(),
            "一致".to_string(),
        );
        let catalogs = BTreeMap::from([(crate::constants::LANGUAGE_JA.to_string(), japanese)]);

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

        let mut buffer = Vec::new();
        super::write_csv_to_writer(
            &mut buffer,
            &[item],
            crate::constants::LANGUAGE_JA,
            &catalogs,
            false,
        )
        .unwrap();

        let output_str = String::from_utf8(buffer).unwrap();
        assert!(output_str.contains(r"C:\Projects\test.xlsx"));
        assert!(!output_str.contains(r"\\?\"));
    }
}

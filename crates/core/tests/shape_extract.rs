//! # Shape Extraction Integration Test
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Generates minimal OOXML workbooks and verifies that shape text is merged into search results.
//! ## Arguments / Returns
//! Creates temporary `.xlsx` files, invokes extractors, and asserts results.
//! ## Errors / Exceptions
//! Fails if ZIP creation, file handling, extraction, or assertions fail.

use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use xlseek_core::constants::*;
use xlseek_core::models::{MatchType, SearchQuery};
use xlseek_core::search::parser::parse_and_search_file;
use xlseek_core::search::shape::extract_shapes;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

const TEST_FILE_NAME: &str = "xlseek-shape-contract.xlsx";
const TEST_UPPERCASE_FILE_NAME: &str = "xlseek-shape-contract-upper.XLSX";
const XLSB_TEST_FILE_NAME: &str = "xlseek-shape-contract.xlsb";
const TEST_FILE_EXTENSION_SEPARATOR: char = '.';
static FIXTURE_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// ## Description
/// Generates a unique temporary fixture path to prevent overwrites during parallel test execution.
/// ## Arguments / Returns
/// Accepts file name constant and returns temporary `PathBuf` containing process ID and sequence number.
/// ## Errors / Exceptions
/// Path generation only; no I/O errors or panics occur.
fn unique_fixture_path(file_name: &str) -> PathBuf {
    let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let (stem, extension) = file_name
        .rsplit_once(TEST_FILE_EXTENSION_SEPARATOR)
        .unwrap_or((file_name, ""));
    std::env::temp_dir().join(format!(
        "{stem}-{}-{sequence}.{extension}",
        std::process::id()
    ))
}

/// ## Description
/// Resolves absolute path to repository root.
/// ## Arguments / Returns
/// No arguments. Returns `PathBuf`.
/// ## Errors / Exceptions
/// Panics if repository root is not found.
fn repo_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("../../tests/fixtures").exists() {
        manifest.join("../..")
    } else {
        manifest
            .parent()
            .expect("repository root must exist")
            .to_path_buf()
    }
}

/// ## Description
/// Generates a temporary OOXML workbook containing a simple DrawingML text box with test text.
/// ## Arguments / Returns
/// No arguments. Returns path to created temporary file.
/// ## Errors / Exceptions
/// Panics if temporary file creation or ZIP writing fails.
fn make_fixture() -> PathBuf {
    let parts = [
        (
            "[Content_Types].xml",
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/></Types>"#,
        ),
        (
            "_rels/.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
        ),
        (
            "xl/workbook.xml",
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
        ),
        (
            "xl/_rels/workbook.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
        ),
        (
            "xl/worksheets/sheet1.xml",
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>cell needle</t></is></c></row></sheetData></worksheet>"#,
        ),
        (
            "xl/worksheets/_rels/sheet1.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="drawing" Target="../drawings/drawing1.xml"/></Relationships>"#,
        ),
        (
            "xl/drawings/drawing1.xml",
            r#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:twoCellAnchor><xdr:from><xdr:col>1</xdr:col><xdr:row>2</xdr:row></xdr:from><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="4" name="Text Box"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>shape needle</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:twoCellAnchor></xdr:wsDr>"#,
        ),
    ];
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, contents) in parts {
        writer
            .start_file(
                name,
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .expect("zip entry must start");
        writer
            .write_all(contents.as_bytes())
            .expect("zip contents must write");
    }
    let path = unique_fixture_path(TEST_FILE_NAME);
    std::fs::write(
        &path,
        writer.finish().expect("zip must finish").into_inner(),
    )
    .expect("fixture must write");
    path
}

/// ## Description
/// Writes a minimal XLSB package containing BrtBundleSh and BrtDrawing to a temporary file.
/// ## Arguments / Returns
/// No arguments. Returns path to created temporary XLSB file.
/// ## Errors / Exceptions
/// Panics if ZIP writing or temporary file output fails.
fn make_xlsb_fixture() -> PathBuf {
    let mut workbook_record = Vec::new();
    let mut bundle = vec![0, 0, 0, 0, 1, 0, 0, 0];
    append_wide_string("rId1", &mut bundle);
    append_wide_string("Sheet1", &mut bundle);
    append_varint(XLSB_BRT_BUNDLE_SH_RECORD_ID, &mut workbook_record);
    append_varint(bundle.len() as u32, &mut workbook_record);
    workbook_record.extend(bundle);

    let mut sheet_record = Vec::new();
    let mut drawing_relation = Vec::new();
    append_wide_string("rId2", &mut drawing_relation);
    append_varint(XLSB_BRT_DRAWING_RECORD_ID, &mut sheet_record);
    append_varint(drawing_relation.len() as u32, &mut sheet_record);
    sheet_record.extend(drawing_relation);

    let parts = [
        (
            "xl/workbook.bin",
            workbook_record,
        ),
        (
            "xl/_rels/workbook.bin.rels",
            br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="worksheet" Target="worksheets/sheet1.bin"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/worksheets/sheet1.bin",
            sheet_record,
        ),
        (
            "xl/worksheets/_rels/sheet1.bin.rels",
            br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId2" Type="drawing" Target="../drawings/drawing1.xml"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/drawings/drawing1.xml",
            br#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="6" name="Binary Shape"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>xlsb needle</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:wsDr>"#.to_vec(),
        ),
    ];
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, contents) in parts {
        writer
            .start_file(
                name,
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .expect("zip entry must start");
        writer
            .write_all(&contents)
            .expect("zip contents must write");
    }
    let path = unique_fixture_path(XLSB_TEST_FILE_NAME);
    std::fs::write(
        &path,
        writer.finish().expect("zip must finish").into_inner(),
    )
    .expect("fixture must write");
    path
}

/// ## Description
/// Appends length-prefixed UTF-16LE string to BIFF12 test record.
/// ## Arguments / Returns
/// `value` is string, `bytes` is target vector. No return value.
/// ## Errors / Exceptions
/// Does not panic for test strings.
fn append_wide_string(value: &str, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&(value.encode_utf16().count() as u32).to_le_bytes());
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
}

/// ## Description
/// Appends `u32` BIFF12 variable-length integer to test record.
/// ## Arguments / Returns
/// `value` is integer, `bytes` is target vector. No return value.
/// ## Errors / Exceptions
/// Does not panic for test integers.
fn append_varint(mut value: u32, bytes: &mut Vec<u8>) {
    while value >= u32::from(XLSB_VARINT_CONTINUATION_MASK) {
        bytes.push((value as u8 & XLSB_VARINT_VALUE_MASK) | XLSB_VARINT_CONTINUATION_MASK);
        value >>= XLSB_VARINT_SHIFT;
    }
    bytes.push(value as u8);
}

/// ## Description
/// Verifies extracting shapes within OOXML workbook with sheet name, shape name, text, and anchor.
/// ## Arguments / Returns
/// No arguments. Fails if extracted values mismatch expectations.
/// ## Errors / Exceptions
/// Fails if extraction fails or temporary file removal fails.
#[test]
fn extracts_text_shape_with_sheet_and_anchor() {
    let path = make_fixture();
    let shapes = extract_shapes(&path, &["Sheet1".to_string()], None)
        .expect("shape extraction must succeed");
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].sheet_name, "Sheet1");
    assert_eq!(shapes[0].shape_name, "Text Box");
    assert_eq!(shapes[0].text, "shape needle");
    assert_eq!(shapes[0].anchor, Some((3, 2)));

    let query = SearchQuery {
        keyword: "needle".to_string(),
        target_dir: path.to_string_lossy().to_string(),
        directory_mode: xlseek_core::models::DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        // Constant reference: xlseek_core::constants::DEFAULT_INCLUDE_VALUE
        include_value: xlseek_core::constants::DEFAULT_INCLUDE_VALUE,
        use_regex: false,
        include_formula: false,
        include_comment: true,
        include_shape: true,
        include_hidden: false,
        extensions: vec![".xlsx".to_string()],
    };
    let results = parse_and_search_file(&path, &query, None, None)
        .expect("search must merge cell and shape results");
    assert_eq!(results.len(), 2);
    assert!(results
        .iter()
        .any(|item| item.match_type == MatchType::CellValue));
    let shape = results
        .iter()
        .find(|item| item.match_type == MatchType::Shape)
        .expect("shape match must exist");
    assert_eq!(shape.shape_name.as_deref(), Some("Text Box"));
    assert_eq!(shape.formula, None);
    assert_eq!(shape.cell_address, "B3");

    let query_without_shapes = SearchQuery {
        include_shape: false,
        ..query
    };
    let cell_only = parse_and_search_file(&path, &query_without_shapes, None, None)
        .expect("cell search must work with Shape search disabled");
    assert_eq!(cell_only.len(), 1);
    assert!(cell_only
        .iter()
        .all(|item| item.match_type != MatchType::Shape));

    std::fs::remove_file(path).expect("fixture must be removed");
}

/// ## Description
/// Verifies that value search cell coordinates reflect real positions in workbooks where ranges start after A1.
/// ## Arguments / Returns
/// No arguments. Matches cell address from known test fixture.
/// ## Errors / Exceptions
/// Fails on workbook read error, search failure, or coordinate mismatch.
#[test]
fn uses_real_cell_coordinates_for_ranges_starting_after_a1() {
    let root = repo_root();
    let path = root.join("tests/fixtures/sample_report.xlsx");
    let query = SearchQuery {
        keyword: "Financial Report Q3".to_string(),
        target_dir: path.to_string_lossy().into_owned(),
        directory_mode: xlseek_core::models::DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        // Constant reference: xlseek_core::constants::DEFAULT_INCLUDE_VALUE
        include_value: xlseek_core::constants::DEFAULT_INCLUDE_VALUE,
        use_regex: false,
        include_formula: false,
        include_comment: false,
        include_shape: false,
        include_hidden: false,
        extensions: xlseek_core::constants::DEFAULT_EXTENSIONS
            .iter()
            .map(|extension| extension.to_string())
            .collect(),
    };
    let results = parse_and_search_file(path, &query, None, None).unwrap();
    assert!(results.iter().any(|item| item.cell_address == "A12"));
}

/// ## Description
/// Verifies correct shape extraction format selection for uppercase extension Excel workbooks.
/// ## Arguments / Returns
/// No arguments. Asserts count of extracted shape text.
/// ## Errors / Exceptions
/// Fails on file error or extraction count mismatch.
#[test]
fn extracts_shapes_when_extension_is_uppercase() {
    let original = make_fixture();
    let uppercase = std::env::temp_dir().join(TEST_UPPERCASE_FILE_NAME);
    std::fs::copy(&original, &uppercase).unwrap();
    let shapes = extract_shapes(&uppercase, &["Sheet1".to_string()], None).unwrap();
    assert_eq!(shapes.len(), 1);
    let _ = std::fs::remove_file(uppercase);
}

/// ## Description
/// Verifies that cell notes in existing OOXML workbooks are returned as comment matches.
/// ## Arguments / Returns
/// No arguments. Compares search results for known note at A12.
/// ## Errors / Exceptions
/// Fails on workbook parse failure, search error, or coordinate mismatch.
#[test]
fn searches_legacy_cell_comments() {
    let root = repo_root();
    let path = root.join("tests/fixtures/sample_report.xlsx");
    let query = SearchQuery {
        keyword: xlseek_core::constants::CLI_TEST_COMMENT_QUERY.to_string(),
        target_dir: path.to_string_lossy().into_owned(),
        directory_mode: xlseek_core::models::DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        // Constant reference: xlseek_core::constants::DEFAULT_INCLUDE_VALUE
        include_value: xlseek_core::constants::DEFAULT_INCLUDE_VALUE,
        use_regex: false,
        include_formula: false,
        include_comment: true,
        include_shape: false,
        include_hidden: false,
        extensions: xlseek_core::constants::DEFAULT_EXTENSIONS
            .iter()
            .map(|extension| extension.to_string())
            .collect(),
    };
    let results = parse_and_search_file(path, &query, None, None).unwrap();
    assert!(results
        .iter()
        .any(|item| { item.match_type == MatchType::Comment && item.cell_address == "A12" }));
}

/// ## Description
/// Verifies traversing XLSB BrtBundleSh -> worksheet relationship -> BrtDrawing -> DrawingML relationships.
/// ## Arguments / Returns
/// No arguments. Verifies shape extraction results via assertions.
/// ## Errors / Exceptions
/// Fails on ZIP read failure or missing sheet/name/text.
#[test]
fn extracts_text_shape_from_xlsb_drawing_relationship() {
    let path = make_xlsb_fixture();
    let shapes = extract_shapes(&path, &["Sheet1".to_string()], None)
        .expect("XLSB shape extraction must succeed");
    std::fs::remove_file(path).expect("fixture must be removed");

    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].sheet_name, "Sheet1");
    assert_eq!(shapes[0].shape_name, "Binary Shape");
    assert_eq!(shapes[0].text, "xlsb needle");
}

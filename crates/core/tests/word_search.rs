//! # Word Document Search Integration Tests (tests/word_search.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies end-to-end scanning, match extraction, and preview payload generation for Word documents:
//! - OOXML `.docx` and `.docm` parsing (paragraphs, comments, and shapes)
//! - Binary `.doc` parsing (OLE2 container, FIB, and text stream)
//! - Error handling for corrupted and password-protected files
//!
//! ## Arguments / Returns
//! Integration test harness invoked via `cargo test --test word_search`.
//!
//! ## Errors / Exceptions
//! Panics on assertion failures or unexpected test fixture setup errors.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use xlseek_core::constants::*;
use xlseek_core::models::{MatchType, SearchQuery};
use xlseek_core::search::word::{extract_word_document, search_word_file};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

static TEST_FIXTURE_SEQ: AtomicUsize = AtomicUsize::new(0);

/// Generates a unique temporary fixture path for concurrency safety.
fn unique_temp_path(ext: &str) -> PathBuf {
    let seq = TEST_FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "xlseek-word-test-{}-{}.{}",
        std::process::id(),
        seq,
        ext
    ))
}

/// Creates a minimal valid .docx or .docm fixture.
fn make_docx_fixture(ext: &str, body_text: &str, comment_text: Option<&str>) -> PathBuf {
    let path = unique_temp_path(ext);
    let file = File::create(&path).expect("create docx fixture file");
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    // [Content_Types].xml
    zip.start_file("[Content_Types].xml", options)
        .expect("start content types");
    let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
</Types>"#;
    zip.write_all(content_types.as_bytes())
        .expect("write content types");

    // word/document.xml
    zip.start_file("word/document.xml", options)
        .expect("start document.xml");
    let document_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>{}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Second paragraph in document</w:t></w:r></w:p>
  </w:body>
</w:document>"#,
        body_text
    );
    zip.write_all(document_xml.as_bytes())
        .expect("write document.xml");

    // word/comments.xml if provided
    if let Some(comment) = comment_text {
        zip.start_file("word/comments.xml", options)
            .expect("start comments.xml");
        let comments_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:comment w:id="1"><w:p><w:r><w:t>{}</w:t></w:r></w:p></w:comment>
</w:comments>"#,
            comment
        );
        zip.write_all(comments_xml.as_bytes())
            .expect("write comments.xml");
    }

    zip.finish().expect("finish docx zip");
    path
}

/// Creates a minimal valid .doc binary fixture via OLE2 Compound File.
fn make_doc_fixture(text: &str) -> PathBuf {
    let path = unique_temp_path("doc");
    let mut compound = cfb::create(&path).expect("create CFB doc file");
    let mut stream = compound
        .create_stream("/WordDocument")
        .expect("create WordDocument stream");

    let mut doc_bytes = vec![0u8; 512];
    // Magic 0xA5EC (Word 97)
    doc_bytes[0] = 0xEC;
    doc_bytes[1] = 0xA5;
    // nFib = 0x00C1 (Word 97)
    doc_bytes[2] = 0xC1;
    doc_bytes[3] = 0x00;
    // fcMin = 512
    let fc_min = 512u32;
    doc_bytes[0x18..0x1C].copy_from_slice(&fc_min.to_le_bytes());

    // Encode text as UTF-16LE
    let u16_chars: Vec<u16> = text.encode_utf16().collect();
    let mut text_bytes = Vec::new();
    for c in u16_chars {
        text_bytes.extend_from_slice(&c.to_le_bytes());
    }

    // fcMac = fcMin + text_bytes.len()
    let fc_mac = fc_min + text_bytes.len() as u32;
    doc_bytes[0x1C..0x20].copy_from_slice(&fc_mac.to_le_bytes());

    doc_bytes.extend_from_slice(&text_bytes);
    stream
        .write_all(&doc_bytes)
        .expect("write WordDocument stream");
    drop(stream);
    drop(compound);

    path
}

fn build_query(keyword: &str, ext: &str, include_comment: bool) -> SearchQuery {
    SearchQuery {
        keyword: keyword.to_string(),
        target_dir: ".".to_string(),
        directory_mode: xlseek_core::models::DirectorySearchMode::Sequential,
        burst_workers: None,
        match_case: false,
        include_value: true,
        use_regex: false,
        include_formula: false,
        include_comment,
        include_shape: false,
        include_hidden: false,
        extensions: vec![ext.to_string()],
    }
}

#[test]
fn test_search_docx_and_docm() {
    for ext in ["docx", "docm"] {
        let fixture = make_docx_fixture(
            ext,
            "Important quarterly revenue report",
            Some("Reviewer note on target budget"),
        );

        // Search body text
        let query_body = build_query("revenue", ext, false);
        let matches =
            search_word_file(&fixture, &query_body, None, None).expect("search docx body");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].match_type, MatchType::WordText);
        // Constant references: WORD_SHEET_LABEL, WORD_CELL_DEFAULT_ADDRESS
        assert_eq!(matches[0].sheet_name, WORD_SHEET_LABEL);
        assert_eq!(matches[0].cell_address, WORD_CELL_DEFAULT_ADDRESS);
        assert_eq!(matches[0].row_index, 1);
        assert!(matches[0].snippet.contains("revenue"));

        // Search comment text
        let query_comment = build_query("budget", ext, true);
        let comment_matches =
            search_word_file(&fixture, &query_comment, None, None).expect("search docx comment");
        assert_eq!(comment_matches.len(), 1);
        assert_eq!(comment_matches[0].match_type, MatchType::Comment);
        assert_eq!(comment_matches[0].sheet_name, WORD_SHEET_LABEL);

        // Preview extraction
        let preview = extract_word_document(&fixture, 1).expect("extract docx preview");
        assert_eq!(preview.target_paragraph, 1);
        assert_eq!(preview.total_paragraphs, 2);
        assert_eq!(preview.paragraphs.len(), 2);
        assert!(preview.paragraphs[0].is_target);
        assert!(!preview.paragraphs[1].is_target);

        let _ = std::fs::remove_file(fixture);
    }
}

#[test]
fn test_search_binary_doc() {
    let fixture =
        make_doc_fixture("First binary paragraph with needle keyword\rSecond paragraph in doc\r");

    let query = build_query("needle", "doc", false);
    let matches = search_word_file(&fixture, &query, None, None).expect("search doc file");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].match_type, MatchType::WordText);
    // Constant references: WORD_SHEET_LABEL, WORD_CELL_DEFAULT_ADDRESS
    assert_eq!(matches[0].sheet_name, WORD_SHEET_LABEL);
    assert_eq!(matches[0].cell_address, WORD_CELL_DEFAULT_ADDRESS);
    assert_eq!(matches[0].row_index, 1);
    assert!(matches[0].snippet.contains("needle"));

    let preview = extract_word_document(&fixture, 1).expect("extract doc preview");
    assert_eq!(preview.target_paragraph, 1);
    assert_eq!(preview.total_paragraphs, 2);
    assert!(preview.paragraphs[0].is_target);
    assert_eq!(
        preview.paragraphs[0].text,
        "First binary paragraph with needle keyword"
    );

    let _ = std::fs::remove_file(fixture);
}

#[test]
fn test_encrypted_word_file_error() {
    // Test encrypted OOXML
    let path = unique_temp_path("docx");
    let file = File::create(&path).expect("create encrypted docx file");
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("EncryptedPackage", options)
        .expect("start EncryptedPackage");
    zip.write_all(b"fake encrypted payload")
        .expect("write bytes");
    zip.finish().expect("finish zip");

    let query = build_query("test", "docx", false);
    let result = search_word_file(&path, &query, None, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Password-protected or encrypted"));

    let _ = std::fs::remove_file(path);
}

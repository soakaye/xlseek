//! # Word OOXML Parser (.docx and .docm) (search/word/docx.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Parses Open Packaging Conventions (OPC) ZIP containers for `.docx` and `.docm` files.
//! Extracts structured paragraph text from main document body (`word/document.xml`),
//! headers, footers, comments (`word/comments.xml`), and drawing shape text boxes.
//! Conforms to Constitution Principle I (English comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle V (Robust error handling).
//!
//! ## Arguments / Returns
//! Exposes functions for scanning matching search items and extracting complete preview paragraphs.
//!
//! ## Errors / Exceptions
//! Returns informative `Err` strings for malformed ZIP structures, missing XML components,
//! or unsupported encrypted packages without crashing.

use crate::models::{MatchType, SearchMatch, SearchQuery, WordParagraph, WordPreviewPayload};
use crate::search::parser::{find_match_position, make_snippet, next_match_id};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use zip::ZipArchive;

/// Entry name for encrypted OOXML documents.
const ENCRYPTED_PACKAGE_ENTRY: &str = "EncryptedPackage";
/// Main document XML part path.
const PART_DOCUMENT_XML: &str = "word/document.xml";
/// Comments XML part path.
const PART_COMMENTS_XML: &str = "word/comments.xml";
/// Header part name prefix.
const PART_HEADER_PREFIX: &str = "word/header";
/// Footer part name prefix.
const PART_FOOTER_PREFIX: &str = "word/footer";
/// XML part suffix.
const PART_XML_SUFFIX: &str = ".xml";

/// Local name for paragraph elements.
const XML_TAG_P: &[u8] = b"p";
/// Local name for text elements.
const XML_TAG_T: &[u8] = b"t";
/// Local name for textbox content elements.
const XML_TAG_TXBX: &[u8] = b"txbxContent";
/// Local name for drawing elements.
const XML_TAG_DRAWING: &[u8] = b"drawing";
/// Local name for comment elements.
const XML_TAG_COMMENT: &[u8] = b"comment";

/// Extracted unit of text with structural metadata.
#[derive(Debug, Clone)]
pub struct ExtractedUnit {
    pub index: u32,
    pub text: String,
    pub match_type: MatchType,
    pub shape_name: Option<String>,
}

/// ## Description
/// Searches an OOXML Word document (.docx or .docm) for occurrences matching the search query.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the `.docx` or `.docm` file on disk.
/// - `query`: `&SearchQuery` - Search configuration, keywords, and matching flags.
/// - `regex_opt`: `Option<&Regex>` - Compiled regular expression when regex mode is enabled.
/// - `cancel_flag`: `Option<&AtomicBool>` - Optional cancellation flag for early termination.
///
/// ## Returns
/// - `Result<Vec<SearchMatch>, String>`: Vector of search matches on success, or error string on failure.
///
/// ## Errors / Exceptions
/// Returns `Err` if the file cannot be opened, is encrypted, or is corrupted. Catches XML errors safely.
pub fn search_docx(
    path: &Path,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    let file =
        File::open(path).map_err(|e| format!("{}: {}", crate::constants::ERR_WORKBOOK_OPEN, e))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("{}: {}", crate::constants::ERR_WORKBOOK_OPEN, e))?;

    // Check for password-protected or encrypted package
    if archive.by_name(ENCRYPTED_PACKAGE_ENTRY).is_ok() {
        return Err("Password-protected or encrypted Word document cannot be scanned".to_string());
    }

    let file_name = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let full_path = path.to_string_lossy().to_string();

    let mut matches = Vec::new();

    // 1. Scan main document body (paragraphs and drawing shapes)
    if let Ok(document_xml) = read_archive_entry(&mut archive, PART_DOCUMENT_XML) {
        let units = parse_xml_paragraphs(&document_xml, query.include_shape);
        for unit in units {
            if let Some(flag) = cancel_flag {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
            }

            if let Some((start, end)) = find_match_position(&unit.text, query, regex_opt) {
                let snippet = make_snippet(&unit.text, start, end);
                matches.push(SearchMatch {
                    id: next_match_id(),
                    file_name: file_name.clone(),
                    full_path: full_path.clone(),
                    // Constant reference: crate::constants::WORD_SHEET_LABEL
                    sheet_name: crate::constants::WORD_SHEET_LABEL.to_string(),
                    // Constant reference: crate::constants::WORD_CELL_DEFAULT_ADDRESS
                    cell_address: crate::constants::WORD_CELL_DEFAULT_ADDRESS.to_string(),
                    row_index: unit.index,
                    col_index: 0,
                    col_name: String::new(),
                    match_type: unit.match_type,
                    shape_name: unit.shape_name,
                    sheet_hidden: false,
                    snippet,
                    full_content: unit.text,
                    formula: None,
                    // Constant reference: crate::constants::WORD_SHEET_LABEL
                    sheets_in_workbook: vec![crate::constants::WORD_SHEET_LABEL.to_string()],
                });
            }
        }
    }

    // 2. Scan comments if enabled
    if query.include_comment {
        if let Ok(comments_xml) = read_archive_entry(&mut archive, PART_COMMENTS_XML) {
            let units = parse_comments_xml(&comments_xml);
            for unit in units {
                if let Some(flag) = cancel_flag {
                    if flag.load(Ordering::Relaxed) {
                        break;
                    }
                }

                if let Some((start, end)) = find_match_position(&unit.text, query, regex_opt) {
                    let snippet = make_snippet(&unit.text, start, end);
                    matches.push(SearchMatch {
                        id: next_match_id(),
                        file_name: file_name.clone(),
                        full_path: full_path.clone(),
                        // Constant reference: crate::constants::WORD_SHEET_LABEL
                        sheet_name: crate::constants::WORD_SHEET_LABEL.to_string(),
                        // Constant reference: crate::constants::WORD_CELL_DEFAULT_ADDRESS
                        cell_address: crate::constants::WORD_CELL_DEFAULT_ADDRESS.to_string(),
                        row_index: unit.index,
                        col_index: 0,
                        col_name: String::new(),
                        match_type: MatchType::Comment,
                        shape_name: None,
                        sheet_hidden: false,
                        snippet,
                        full_content: unit.text,
                        formula: None,
                        // Constant reference: crate::constants::WORD_SHEET_LABEL
                        sheets_in_workbook: vec![crate::constants::WORD_SHEET_LABEL.to_string()],
                    });
                }
            }
        }
    }

    // 3. Scan headers and footers
    let file_names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();

    for name in file_names {
        if (name.starts_with(PART_HEADER_PREFIX) || name.starts_with(PART_FOOTER_PREFIX))
            && name.ends_with(PART_XML_SUFFIX)
        {
            if let Ok(xml_content) = read_archive_entry(&mut archive, &name) {
                let units = parse_xml_paragraphs(&xml_content, query.include_shape);
                for unit in units {
                    if let Some(flag) = cancel_flag {
                        if flag.load(Ordering::Relaxed) {
                            break;
                        }
                    }

                    if let Some((start, end)) = find_match_position(&unit.text, query, regex_opt) {
                        let snippet = make_snippet(&unit.text, start, end);
                        matches.push(SearchMatch {
                            id: next_match_id(),
                            file_name: file_name.clone(),
                            full_path: full_path.clone(),
                            // Constant reference: crate::constants::WORD_SHEET_LABEL
                            sheet_name: crate::constants::WORD_SHEET_LABEL.to_string(),
                            // Constant reference: crate::constants::WORD_CELL_DEFAULT_ADDRESS
                            cell_address: crate::constants::WORD_CELL_DEFAULT_ADDRESS.to_string(),
                            row_index: unit.index,
                            col_index: 0,
                            col_name: String::new(),
                            match_type: unit.match_type,
                            shape_name: unit.shape_name,
                            sheet_hidden: false,
                            snippet,
                            full_content: unit.text,
                            formula: None,
                            // Constant reference: crate::constants::WORD_SHEET_LABEL
                            sheets_in_workbook: vec![crate::constants::WORD_SHEET_LABEL.to_string()],
                        });
                    }
                }
            }
        }
    }

    Ok(matches)
}

/// ## Description
/// Extracts ordered paragraphs from an OOXML Word document (.docx or .docm) for preview display.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the Word document on disk.
/// - `target_paragraph`: `u32` - 1-based paragraph index that was matched in the search.
///
/// ## Returns
/// - `Result<WordPreviewPayload, String>`: Structured preview payload on success, or error string on failure.
///
/// ## Errors / Exceptions
/// Returns `Err` if the file cannot be opened or parsed.
pub fn extract_docx_paragraphs(
    path: &Path,
    target_paragraph: u32,
) -> Result<WordPreviewPayload, String> {
    let file =
        File::open(path).map_err(|e| format!("{}: {}", crate::constants::ERR_WORKBOOK_OPEN, e))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("{}: {}", crate::constants::ERR_WORKBOOK_OPEN, e))?;

    if archive.by_name(ENCRYPTED_PACKAGE_ENTRY).is_ok() {
        return Err(
            "Password-protected or encrypted Word document cannot be previewed".to_string(),
        );
    }

    let file_name = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let full_path = path.to_string_lossy().to_string();

    let document_xml = read_archive_entry(&mut archive, PART_DOCUMENT_XML)
        .map_err(|e| format!("Failed to read {}: {}", PART_DOCUMENT_XML, e))?;

    let units = parse_xml_paragraphs(&document_xml, true);
    let total_paragraphs = units.len();

    let paragraphs = units
        .into_iter()
        .map(|u| WordParagraph {
            index: u.index,
            is_target: u.index == target_paragraph,
            text: u.text,
        })
        .collect();

    Ok(WordPreviewPayload {
        file_path: full_path,
        file_name,
        target_paragraph,
        total_paragraphs,
        paragraphs,
    })
}

/// Helper function to read a full archive entry as a string.
fn read_archive_entry<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    entry_name: &str,
) -> Result<String, String> {
    let mut entry = archive
        .by_name(entry_name)
        .map_err(|e| format!("Entry {} not found: {}", entry_name, e))?;
    let mut content = String::new();
    entry
        .read_to_string(&mut content)
        .map_err(|e| format!("Failed to read entry {}: {}", entry_name, e))?;
    Ok(content)
}

/// Helper function to parse `<w:p>` paragraphs and drawing text from XML string.
fn parse_xml_paragraphs(xml_content: &str, include_shapes: bool) -> Vec<ExtractedUnit> {
    let mut reader = Reader::from_reader(BufReader::new(xml_content.as_bytes()));
    reader.config_mut().trim_text(false);

    let mut units = Vec::new();
    let mut current_text = String::new();
    let mut in_text = false;
    let mut in_drawing = false;
    let mut in_textbox = false;
    let mut paragraph_counter: u32 = 0;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let local = e.local_name();
                if local.as_ref() == XML_TAG_P {
                    current_text.clear();
                } else if local.as_ref() == XML_TAG_T {
                    in_text = true;
                } else if local.as_ref() == XML_TAG_DRAWING {
                    in_drawing = true;
                } else if local.as_ref() == XML_TAG_TXBX {
                    in_textbox = true;
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_text {
                    if let Ok(decoded) = e.decode() {
                        if let Ok(text) = quick_xml::escape::unescape(&decoded) {
                            current_text.push_str(&text);
                        } else {
                            current_text.push_str(&decoded);
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let local = e.local_name();
                if local.as_ref() == XML_TAG_T {
                    in_text = false;
                } else if local.as_ref() == XML_TAG_DRAWING {
                    in_drawing = false;
                } else if local.as_ref() == XML_TAG_TXBX {
                    in_textbox = false;
                } else if local.as_ref() == XML_TAG_P {
                    let trimmed = current_text.trim();
                    if !trimmed.is_empty() {
                        paragraph_counter += 1;
                        let is_shape = in_drawing || in_textbox;
                        if is_shape && !include_shapes {
                            // Skip shape text if disabled
                        } else {
                            units.push(ExtractedUnit {
                                index: paragraph_counter,
                                text: current_text.clone(),
                                match_type: if is_shape {
                                    MatchType::Shape
                                } else {
                                    MatchType::WordText
                                },
                                shape_name: if is_shape {
                                    Some("TextBox".to_string())
                                } else {
                                    None
                                },
                            });
                        }
                    }
                    current_text.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    units
}

/// Helper function to parse `<w:comment>` blocks from comments.xml.
fn parse_comments_xml(xml_content: &str) -> Vec<ExtractedUnit> {
    let mut reader = Reader::from_reader(BufReader::new(xml_content.as_bytes()));
    reader.config_mut().trim_text(false);

    let mut units = Vec::new();
    let mut current_text = String::new();
    let mut in_text = false;
    let mut comment_counter: u32 = 0;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let local = e.local_name();
                if local.as_ref() == XML_TAG_COMMENT {
                    current_text.clear();
                } else if local.as_ref() == XML_TAG_T {
                    in_text = true;
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_text {
                    if let Ok(decoded) = e.decode() {
                        if let Ok(text) = quick_xml::escape::unescape(&decoded) {
                            current_text.push_str(&text);
                        } else {
                            current_text.push_str(&decoded);
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let local = e.local_name();
                if local.as_ref() == XML_TAG_T {
                    in_text = false;
                } else if local.as_ref() == XML_TAG_COMMENT {
                    let trimmed = current_text.trim();
                    if !trimmed.is_empty() {
                        comment_counter += 1;
                        units.push(ExtractedUnit {
                            index: comment_counter,
                            text: current_text.clone(),
                            match_type: MatchType::Comment,
                            shape_name: None,
                        });
                    }
                    current_text.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    units
}

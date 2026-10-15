//! # Word Binary Format Parser (.doc) (search/word/doc.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Parses legacy Microsoft Word binary documents (.doc) stored within OLE2 Compound File (CFB) containers.
//! Reads the `WordDocument` stream and associated `0Table` or `1Table` streams to extract text via
//! the File Information Block (FIB) and Piece Descriptor Table (Clx/Pcdt).
//! Supports both compressed 8-bit text (CP932/Windows-1252) and 16-bit UTF-16LE text.
//! Conforms to Constitution Principle I (English comments), Principle II (Constant references),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust error handling).
//!
//! ## Arguments / Returns
//! Exposes functions for scanning matching search items (`search_doc`) and extracting complete preview paragraphs (`extract_doc_paragraphs`).
//!
//! ## Errors / Exceptions
//! Returns `Err` with descriptive messages on corrupted CFB structures, missing streams,
//! or password-protected/encrypted files without panicking.

use crate::models::{MatchType, SearchMatch, SearchQuery, WordParagraph, WordPreviewPayload};
use crate::search::parser::{find_match_position, make_snippet, next_match_id};
use regex::Regex;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// Stream name for the main Word document stream in OLE2 container.
const STREAM_WORD_DOCUMENT: &str = "/WordDocument";
/// Stream name for the 0Table stream.
const STREAM_0_TABLE: &str = "/0Table";
/// Stream name for the 1Table stream.
const STREAM_1_TABLE: &str = "/1Table";

/// FIB magic identifier for Word 97-2004 documents.
const FIB_MAGIC_WORD97: u16 = 0xA5EC;
/// FIB magic identifier for Word 6.0/95 documents.
const FIB_MAGIC_WORD6: u16 = 0xA5DC;

/// Offset in bytes to the flags2 byte in FibBase.
const FIB_FLAGS2_OFFSET: usize = 0x000B;
/// Bitmask for encryption flag in flags2 (bit 6: fCrypto).
const FIB_FLAG_ENCRYPTED: u8 = 0x40;
/// Bitmask for table stream selector in flags2 (bit 0: fWhichTblStm; 1 = 1Table, 0 = 0Table).
const FIB_FLAG_WHICH_TABLE: u8 = 0x01;

/// Offset to `fcMin` in FibBase (4 bytes, u32).
const FIB_FCMIN_OFFSET: usize = 0x0018;
/// Offset to `fcMac` in FibBase (4 bytes, u32).
const FIB_FCMAC_OFFSET: usize = 0x001C;
/// Offset to `csw` in FIB (2 bytes, u16 count of FibRgW).
const FIB_CSW_OFFSET: usize = 0x0020;

/// Index of fcClx in FibRgFcLcb97 (31st 8-byte entry).
const FIB_CLX_ENTRY_INDEX: usize = 31;

/// Type indicator for Prc entry in Clx.
const CLX_TYPE_PRC: u8 = 0x01;
/// Type indicator for Pcdt entry in Clx.
const CLX_TYPE_PCDT: u8 = 0x02;

/// Byte size of each Piece Descriptor (Pcd).
const PCD_ENTRY_SIZE: usize = 8;
/// Offset to fc inside an 8-byte Pcd entry.
const PCD_FC_OFFSET: usize = 2;
/// Flag indicating compressed 8-bit text (bit 30).
const PCD_COMPRESSED_FLAG: u32 = 0x4000_0000;
/// Mask to extract actual file offset from compressed fc.
const PCD_OFFSET_MASK: u32 = 0x3FFF_FFFF;

/// Error message when opening or reading the CFB file fails.
const ERR_WORD_DOC_READ: &str = "Failed to open or parse Word binary document (.doc)";
/// Error message when a document is password-protected or encrypted.
const ERR_WORD_DOC_ENCRYPTED: &str =
    "Password-protected or encrypted Word document cannot be scanned";
/// Error message when a document cannot be previewed due to encryption.
const ERR_WORD_DOC_PREVIEW_ENCRYPTED: &str =
    "Password-protected or encrypted Word document cannot be previewed";
/// Error message for corrupted or unrecognized Word binary structures.
const ERR_WORD_DOC_CORRUPTED: &str =
    "Word binary document (.doc) structure is corrupted or unrecognized";
/// Maximum stream read limit (100MB) to prevent out-of-memory denial of service.
const MAX_DOC_STREAM_BYTES: u64 = 100 * 1024 * 1024;

/// Paragraph separator in Word text.
const CHAR_CR: char = '\r';
/// Line feed character.
const CHAR_LF: char = '\n';
/// Table cell separator character in Word.
const CHAR_CELL_MARK: char = '\x07';
/// Manual line break character in Word.
const CHAR_LINE_BREAK: char = '\x0B';
/// Page or section break character in Word.
const CHAR_PAGE_BREAK: char = '\x0C';

/// ## Description
/// Scans a binary Microsoft Word document (.doc) for occurrences matching the search query.
///
/// ## Arguments
/// - `path`: `&Path` - Filesystem path to the `.doc` file.
/// - `query`: `&SearchQuery` - User search parameters including keyword, regex mode, and casing options.
/// - `regex_opt`: `Option<&Regex>` - Compiled regular expression if regex mode is active.
/// - `cancel_flag`: `Option<&AtomicBool>` - Optional cancellation signal to terminate scanning early.
///
/// ## Returns
/// - `Result<Vec<SearchMatch>, String>`: List of detected search matches or an error description.
///
/// ## Errors / Exceptions
/// Returns `Err` if the file is not a valid OLE2 compound document, is encrypted, or cannot be read.
pub fn search_doc(
    path: &Path,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    let paragraphs = extract_all_paragraphs(path, ERR_WORD_DOC_ENCRYPTED)?;

    let file_name = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let full_path = path.to_string_lossy().to_string();

    let mut matches = Vec::new();

    for (idx, text) in paragraphs.into_iter().enumerate() {
        if let Some(flag) = cancel_flag {
            if flag.load(Ordering::Relaxed) {
                break;
            }
        }

        let paragraph_index = (idx + 1) as u32;

        if let Some((start, end)) = find_match_position(&text, query, regex_opt) {
            let snippet = make_snippet(&text, start, end);
            matches.push(SearchMatch {
                id: next_match_id(),
                file_name: file_name.clone(),
                full_path: full_path.clone(),
                // Constant reference: crate::constants::WORD_SHEET_LABEL
                sheet_name: crate::constants::WORD_SHEET_LABEL.to_string(),
                // Constant reference: crate::constants::WORD_CELL_DEFAULT_ADDRESS
                cell_address: crate::constants::WORD_CELL_DEFAULT_ADDRESS.to_string(),
                row_index: paragraph_index,
                col_index: 0,
                col_name: String::new(),
                match_type: MatchType::WordText,
                shape_name: None,
                sheet_hidden: false,
                snippet,
                full_content: text,
                formula: None,
                // Constant reference: crate::constants::WORD_SHEET_LABEL
                sheets_in_workbook: vec![crate::constants::WORD_SHEET_LABEL.to_string()],
            });
        }
    }

    Ok(matches)
}

/// ## Description
/// Extracts ordered paragraphs from a binary Word document (.doc) for text preview display.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the `.doc` file on disk.
/// - `target_paragraph`: `u32` - 1-based index of the target paragraph to highlight in preview.
///
/// ## Returns
/// - `Result<WordPreviewPayload, String>`: Preview payload containing all paragraphs and target metadata.
///
/// ## Errors / Exceptions
/// Returns `Err` if the file cannot be opened, is encrypted, or is corrupted.
pub fn extract_doc_paragraphs(
    path: &Path,
    target_paragraph: u32,
) -> Result<WordPreviewPayload, String> {
    let paragraphs_text = extract_all_paragraphs(path, ERR_WORD_DOC_PREVIEW_ENCRYPTED)?;

    let file_name = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_default();
    let full_path = path.to_string_lossy().to_string();

    let total_paragraphs = paragraphs_text.len();
    let paragraphs = paragraphs_text
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            let index = (i + 1) as u32;
            WordParagraph {
                index,
                is_target: index == target_paragraph,
                text,
            }
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

/// Helper function to open an OLE2 container and extract all paragraphs as clean strings.
fn extract_all_paragraphs(path: &Path, encrypted_err_msg: &str) -> Result<Vec<String>, String> {
    let mut compound = cfb::open(path).map_err(|e| format!("{}: {}", ERR_WORD_DOC_READ, e))?;

    if !compound.exists(STREAM_WORD_DOCUMENT) {
        return Err(format!(
            "{}: missing {}",
            ERR_WORD_DOC_CORRUPTED, STREAM_WORD_DOCUMENT
        ));
    }

    let word_doc_bytes = read_stream_bytes(&mut compound, STREAM_WORD_DOCUMENT)?;
    if word_doc_bytes.len() < 32 {
        return Err(ERR_WORD_DOC_CORRUPTED.to_string());
    }

    // Verify magic identifier
    let w_ident = u16::from_le_bytes([word_doc_bytes[0], word_doc_bytes[1]]);
    if w_ident != FIB_MAGIC_WORD97 && w_ident != FIB_MAGIC_WORD6 {
        return Err(format!(
            "{}: unexpected magic 0x{:04X}",
            ERR_WORD_DOC_CORRUPTED, w_ident
        ));
    }

    // Check encryption flag (bit 6 of flags2 at offset 0x000B)
    if word_doc_bytes.len() > FIB_FLAGS2_OFFSET {
        let flags2 = word_doc_bytes[FIB_FLAGS2_OFFSET];
        if (flags2 & FIB_FLAG_ENCRYPTED) != 0 {
            return Err(encrypted_err_msg.to_string());
        }
    }

    // Attempt piece table (Clx) extraction for Word 97-2004 documents
    let extracted_text = if w_ident == FIB_MAGIC_WORD97 {
        match extract_text_from_piece_table(&mut compound, &word_doc_bytes) {
            Ok(Some(text)) if !text.trim().is_empty() => text,
            _ => extract_text_from_fc_range(&word_doc_bytes)?,
        }
    } else {
        extract_text_from_fc_range(&word_doc_bytes)?
    };

    Ok(split_into_paragraphs(&extracted_text))
}

/// Reads all bytes of a named stream from the CFB container up to MAX_DOC_STREAM_BYTES.
fn read_stream_bytes<F: std::io::Seek + Read>(
    compound: &mut cfb::CompoundFile<F>,
    stream_name: &str,
) -> Result<Vec<u8>, String> {
    let stream = compound
        .open_stream(stream_name)
        .map_err(|e| format!("{}: stream {}: {}", ERR_WORD_DOC_READ, stream_name, e))?;

    let mut bytes = Vec::new();
    stream
        .take(MAX_DOC_STREAM_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("{}: read {}: {}", ERR_WORD_DOC_READ, stream_name, e))?;

    if bytes.len() as u64 > MAX_DOC_STREAM_BYTES {
        return Err(format!(
            "Stream {} exceeds maximum allowed size",
            stream_name
        ));
    }

    Ok(bytes)
}

/// Extracts text using the Clx / Piece Descriptor Table (Pcdt) structure from Table stream.
fn extract_text_from_piece_table<F: std::io::Seek + Read>(
    compound: &mut cfb::CompoundFile<F>,
    word_doc_bytes: &[u8],
) -> Result<Option<String>, String> {
    if word_doc_bytes.len() < FIB_CSW_OFFSET + 2 {
        return Ok(None);
    }

    let flags2 = word_doc_bytes[FIB_FLAGS2_OFFSET];
    let table_stream_name = if (flags2 & FIB_FLAG_WHICH_TABLE) != 0 {
        STREAM_1_TABLE
    } else {
        STREAM_0_TABLE
    };

    if !compound.exists(table_stream_name) {
        return Ok(None);
    }

    // Read csw
    let csw = read_u16_le(word_doc_bytes, FIB_CSW_OFFSET)? as usize;
    let offset_cslw = FIB_CSW_OFFSET + 2 + csw * 2;
    if word_doc_bytes.len() < offset_cslw + 2 {
        return Ok(None);
    }

    // Read cslw
    let cslw = read_u16_le(word_doc_bytes, offset_cslw)? as usize;
    let offset_cb_rg_fc_lcb = offset_cslw + 2 + cslw * 4;
    if word_doc_bytes.len() < offset_cb_rg_fc_lcb + 2 {
        return Ok(None);
    }

    // Read cbRgFcLcb
    let cb_rg_fc_lcb = read_u16_le(word_doc_bytes, offset_cb_rg_fc_lcb)? as usize;
    if cb_rg_fc_lcb <= FIB_CLX_ENTRY_INDEX {
        return Ok(None);
    }

    // Read fcClx and lcbClx from entry 31
    let offset_clx_entry = offset_cb_rg_fc_lcb + 2 + FIB_CLX_ENTRY_INDEX * 8;
    if word_doc_bytes.len() < offset_clx_entry + 8 {
        return Ok(None);
    }

    let fc_clx = read_u32_le(word_doc_bytes, offset_clx_entry)? as usize;
    let lcb_clx = read_u32_le(word_doc_bytes, offset_clx_entry + 4)? as usize;

    if lcb_clx == 0 {
        return Ok(None);
    }

    let table_bytes = read_stream_bytes(compound, table_stream_name)?;
    if table_bytes.len() < fc_clx + lcb_clx {
        return Ok(None);
    }

    let clx_slice = &table_bytes[fc_clx..fc_clx + lcb_clx];
    let mut clx_offset = 0;

    // Scan past Prc entries
    while clx_offset < clx_slice.len() {
        let entry_type = clx_slice[clx_offset];
        if entry_type == CLX_TYPE_PRC {
            if clx_offset + 3 > clx_slice.len() {
                return Ok(None);
            }
            let cb_grpprl = read_u16_le(clx_slice, clx_offset + 1)? as usize;
            clx_offset += 3 + cb_grpprl;
        } else if entry_type == CLX_TYPE_PCDT {
            // Found Piece Descriptor Table
            clx_offset += 1;
            if clx_offset + 4 > clx_slice.len() {
                return Ok(None);
            }
            let lcb_pcdt = read_u32_le(clx_slice, clx_offset)? as usize;
            clx_offset += 4;

            if clx_offset + lcb_pcdt > clx_slice.len() || lcb_pcdt < 4 {
                return Ok(None);
            }

            let plcfpcd_slice = &clx_slice[clx_offset..clx_offset + lcb_pcdt];
            // Plcfpcd contains (n + 1) CPs (4 bytes each) and n PCDs (8 bytes each):
            // lcb_pcdt = (n + 1) * 4 + n * 8 = 12 * n + 4
            let n = (lcb_pcdt - 4) / 12;
            if n == 0 {
                return Ok(None);
            }

            let cp_array_len = (n + 1) * 4;
            let pcd_start = cp_array_len;

            let mut full_text = String::new();

            for i in 0..n {
                let cp_curr = read_u32_le(plcfpcd_slice, i * 4)? as usize;
                let cp_next = read_u32_le(plcfpcd_slice, (i + 1) * 4)? as usize;
                if cp_next < cp_curr {
                    continue;
                }
                let char_len = cp_next - cp_curr;
                if char_len == 0 {
                    continue;
                }

                let pcd_offset = pcd_start + i * PCD_ENTRY_SIZE;
                if pcd_offset + PCD_ENTRY_SIZE > plcfpcd_slice.len() {
                    break;
                }

                let fc = read_u32_le(plcfpcd_slice, pcd_offset + PCD_FC_OFFSET)?;
                let is_compressed = (fc & PCD_COMPRESSED_FLAG) != 0;

                if is_compressed {
                    let byte_offset = ((fc & PCD_OFFSET_MASK) / 2) as usize;
                    if byte_offset + char_len <= word_doc_bytes.len() {
                        let raw_bytes = &word_doc_bytes[byte_offset..byte_offset + char_len];
                        let piece_text = decode_8bit_text(raw_bytes);
                        full_text.push_str(&piece_text);
                    }
                } else {
                    let byte_offset = fc as usize;
                    let byte_len = char_len * 2;
                    if byte_offset + byte_len <= word_doc_bytes.len() {
                        let raw_bytes = &word_doc_bytes[byte_offset..byte_offset + byte_len];
                        let piece_text = decode_utf16le_text(raw_bytes);
                        full_text.push_str(&piece_text);
                    }
                }
            }

            return Ok(Some(full_text));
        } else {
            break;
        }
    }

    Ok(None)
}

/// Fallback extraction of text from fcMin..fcMac in WordDocument stream.
fn extract_text_from_fc_range(word_doc_bytes: &[u8]) -> Result<String, String> {
    if word_doc_bytes.len() < FIB_FCMAC_OFFSET + 4 {
        return Err(ERR_WORD_DOC_CORRUPTED.to_string());
    }

    let fc_min = read_u32_le(word_doc_bytes, FIB_FCMIN_OFFSET)? as usize;
    let fc_mac = read_u32_le(word_doc_bytes, FIB_FCMAC_OFFSET)? as usize;

    if fc_min >= fc_mac || fc_mac > word_doc_bytes.len() {
        return Err(ERR_WORD_DOC_CORRUPTED.to_string());
    }

    let raw_slice = &word_doc_bytes[fc_min..fc_mac];
    // Attempt decoding as UTF-16LE first; if invalid or high ASCII, decode as CP932 / Windows-1252
    let text = decode_utf16le_text(raw_slice);
    if text.chars().any(|c| c != '\0' && !c.is_control()) {
        Ok(text)
    } else {
        Ok(decode_8bit_text(raw_slice))
    }
}

/// Decodes 8-bit text trying CP932 / Shift_JIS first, falling back to Windows-1252.
fn decode_8bit_text(bytes: &[u8]) -> String {
    let (cow, _, had_errors) = encoding_rs::SHIFT_JIS.decode(bytes);
    if !had_errors {
        cow.into_owned()
    } else {
        let (cow1252, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
        cow1252.into_owned()
    }
}

/// Decodes byte slice as UTF-16LE characters.
fn decode_utf16le_text(bytes: &[u8]) -> String {
    let (chunks, _) = bytes.as_chunks::<2>();
    let u16_iter = chunks.iter().map(|&[b0, b1]| u16::from_le_bytes([b0, b1]));

    char::decode_utf16(u16_iter)
        .map(|r| r.unwrap_or('\u{FFFD}'))
        .collect()
}

/// Normalizes control characters and splits extracted text into clean, trimmed paragraphs.
fn split_into_paragraphs(raw_text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current = String::new();

    for c in raw_text.chars() {
        match c {
            CHAR_CR | CHAR_LF | CHAR_PAGE_BREAK => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    paragraphs.push(trimmed.to_string());
                }
                current.clear();
            }
            CHAR_CELL_MARK => {
                current.push('\t');
            }
            CHAR_LINE_BREAK => {
                current.push(' ');
            }
            // Strip out non-printable ASCII control characters except tab
            c if c < ' ' && c != '\t' => {}
            c => {
                current.push(c);
            }
        }
    }

    let final_trimmed = current.trim();
    if !final_trimmed.is_empty() {
        paragraphs.push(final_trimmed.to_string());
    }

    paragraphs
}

/// Helper function to read a little-endian u16 from a byte slice safely.
fn read_u16_le(slice: &[u8], offset: usize) -> Result<u16, String> {
    if offset + 2 > slice.len() {
        return Err(ERR_WORD_DOC_CORRUPTED.to_string());
    }
    Ok(u16::from_le_bytes([slice[offset], slice[offset + 1]]))
}

/// Helper function to read a little-endian u32 from a byte slice safely.
fn read_u32_le(slice: &[u8], offset: usize) -> Result<u32, String> {
    if offset + 4 > slice.len() {
        return Err(ERR_WORD_DOC_CORRUPTED.to_string());
    }
    Ok(u32::from_le_bytes([
        slice[offset],
        slice[offset + 1],
        slice[offset + 2],
        slice[offset + 3],
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_paragraphs() {
        let raw = "First Paragraph\rSecond Paragraph\r\x07Table Item\x07\rThird Paragraph\x0C";
        let paragraphs = split_into_paragraphs(raw);
        assert_eq!(paragraphs.len(), 4);
        assert_eq!(paragraphs[0], "First Paragraph");
        assert_eq!(paragraphs[1], "Second Paragraph");
        assert_eq!(paragraphs[2], "Table Item");
        assert_eq!(paragraphs[3], "Third Paragraph");
    }

    #[test]
    fn test_decode_8bit_text() {
        let ascii_bytes = b"Hello, World!";
        let text = decode_8bit_text(ascii_bytes);
        assert_eq!(text, "Hello, World!");
    }

    #[test]
    fn test_decode_utf16le_text() {
        let u16_chars = vec!['T' as u16, 'e' as u16, 's' as u16, 't' as u16];
        let mut bytes = Vec::new();
        for c in u16_chars {
            bytes.extend_from_slice(&c.to_le_bytes());
        }
        let text = decode_utf16le_text(&bytes);
        assert_eq!(text, "Test");
    }
}

//! # BIFF8 / CFB Shape Extractor
//!
//! ## Description
//! Reads Workbook/Book streams, sheet ranges, TxO, and Continue records from CFB to extract text box strings.
//! ## Arguments / Returns
//! Accepts workbook path and sheet name list; returns list of ShapeText items.
//! ## Errors / Exceptions
//! Returns error on corrupted CFB, BIFF records, strings, or exceeded size limits.

use super::ShapeText;
use encoding_rs::WINDOWS_1252;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// ## Description
/// Extracts text objects per sheet from a BIFF workbook in CFB.
/// ## Arguments / Returns
/// `path` is an `.xls` file, `sheet_names` is list of sheet names from reader, `cancel_flag` is optional cancellation flag. Returns list of shapes.
/// ## Errors / Exceptions
/// Returns error on missing CFB stream, exceeded limits, or invalid BIFF structure.
pub(super) fn extract(
    path: &Path,
    sheet_names: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
    if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
        return Ok(Vec::new());
    }
    // Constant reference: SHAPE_MAX_BINARY_BYTES
    let mut compound = cfb::open(path).map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let stream_path = if compound.exists("/Workbook") {
        "/Workbook"
    } else {
        "/Book"
    };
    let stream = compound
        .open_stream(stream_path)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let mut bytes = Vec::new();
    stream
        .take(crate::constants::SHAPE_MAX_BINARY_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    if bytes.len() as u64 > crate::constants::SHAPE_MAX_BINARY_BYTES {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let sheets = parse_bound_sheets(&bytes)?;
    let mut output = Vec::new();
    let mut shape_number = 1_u32;

    for (index, sheet) in sheets.iter().enumerate() {
        if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
            break;
        }
        let Some(sheet_name) = sheet_names.iter().find(|name| **name == sheet.name) else {
            continue;
        };
        let end = sheets
            .get(index + 1)
            .map(|next| next.stream_offset)
            .unwrap_or(bytes.len());
        if sheet.stream_offset >= end || end > bytes.len() {
            continue;
        }
        output.extend(parse_sheet_objects(
            &bytes[sheet.stream_offset..end],
            sheet_name,
            &mut shape_number,
        )?);
    }
    Ok(output)
}

/// ## Description
/// Extracts BoundSheet8 sheet names and stream offsets from workbook global stream.
/// ## Arguments / Returns
/// `bytes` is BIFF workbook stream; returns list of sheet definitions.
/// ## Errors / Exceptions
/// Returns error if BIFF record boundaries or strings are malformed.
fn parse_bound_sheets(bytes: &[u8]) -> Result<Vec<BoundSheet>, String> {
    // Constant reference: XLS_BOUNDSHEET_RECORD_ID and XLS_MAX_BIFF_RECORD_BYTES
    let mut offset = 0;
    let mut sheets = Vec::new();
    while offset < bytes.len() {
        let (record_id, record) = next_biff_record(bytes, &mut offset)?;
        if record_id != crate::constants::XLS_BOUNDSHEET_RECORD_ID || record.len() < 8 {
            continue;
        }
        let stream_offset = u32::from_le_bytes(record[..4].try_into().unwrap_or_default()) as usize;
        let char_count = usize::from(record[6]);
        let is_wide = record[7] & 1 != 0;
        let name_bytes = char_count
            .checked_mul(if is_wide { 2 } else { 1 })
            .and_then(|size| record.get(8..8 + size))
            .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
        let name = decode_chars(name_bytes, is_wide)?;
        sheets.push(BoundSheet {
            name,
            stream_offset,
        });
    }
    sheets.sort_by_key(|sheet| sheet.stream_offset);
    Ok(sheets)
}

/// ## Description
/// Converts TxO records and subsequent Continue records in worksheet BIFF stream into shape text.
/// ## Arguments / Returns
/// `bytes` is single sheet stream slice, `sheet_name` is target sheet name, `shape_number` is sequence counter for unique IDs.
/// ## Errors / Exceptions
/// Returns error on malformed records or incomplete TxO text.
fn parse_sheet_objects(
    bytes: &[u8],
    sheet_name: &str,
    shape_number: &mut u32,
) -> Result<Vec<ShapeText>, String> {
    // Constant reference: XLS_TXO_* / XLS_CONTINUE_RECORD_ID
    let mut offset = 0;
    let mut output = Vec::new();
    while offset < bytes.len() {
        let (record_id, record) = next_biff_record(bytes, &mut offset)?;
        if record_id != crate::constants::XLS_TXO_RECORD_ID {
            continue;
        }
        if record.len() < crate::constants::XLS_TXO_FIXED_HEADER_BYTES {
            continue;
        }
        let text_count_start = crate::constants::XLS_TXO_TEXT_COUNT_OFFSET;
        let text_count = usize::from(u16::from_le_bytes([
            record[text_count_start],
            record[text_count_start + 1],
        ]));
        let run_bytes = usize::from(u16::from_le_bytes([
            record[text_count_start + 2],
            record[text_count_start + 3],
        ]));
        let mut continues = Vec::new();
        while offset < bytes.len() {
            let saved_offset = offset;
            let (next_id, payload) = next_biff_record(bytes, &mut offset)?;
            if next_id != crate::constants::XLS_CONTINUE_RECORD_ID {
                offset = saved_offset;
                break;
            }
            continues.push(payload);
        }
        let text = decode_txo_text(&continues, text_count, run_bytes)?;
        if !text.trim().is_empty() {
            let name = format!(
                "{}{}",
                crate::constants::XLS_SHAPE_NAME_PREFIX,
                *shape_number
            );
            output.push(ShapeText {
                shape_id: shape_number.to_string(),
                shape_name: name,
                text,
                sheet_name: sheet_name.to_string(),
                anchor: None,
            });
            *shape_number = shape_number.saturating_add(1);
        }
    }
    Ok(output)
}

/// ## Description
/// Decodes XLUnicodeStringNoCch from Continue records following TxO up to specified character count.
/// ## Arguments / Returns
/// `continues` is Continue payload list, `text_count` is expected character count, `run_bytes` is format run area size.
/// ## Errors / Exceptions
/// Returns error on missing text, invalid UTF-16, or exceeded format run area limit.
fn decode_txo_text(
    continues: &[&[u8]],
    text_count: usize,
    run_bytes: usize,
) -> Result<String, String> {
    // Constant reference: XLS_MAX_BIFF_RECORD_BYTES
    if text_count == 0 {
        return Ok(String::new());
    }
    if run_bytes > crate::constants::XLS_MAX_BIFF_RECORD_BYTES {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let mut output = String::new();
    let mut remaining = text_count;
    for payload in continues {
        if remaining == 0 {
            break;
        }
        let Some((&options, chars)) = payload.split_first() else {
            continue;
        };
        let wide = options & 1 != 0;
        let unit_size = if wide { 2 } else { 1 };
        let units = chars.len() / unit_size;
        let count = units.min(remaining);
        let byte_count = count * unit_size;
        output.push_str(&decode_chars(&chars[..byte_count], wide)?);
        remaining -= count;
    }
    if remaining > 0 {
        return Err(crate::constants::ERR_SHAPE_READ.to_string());
    }
    Ok(output)
}

/// ## Description
/// Converts BIFF compressed single-byte or UTF-16LE character bytes to a Unicode String.
/// ## Arguments / Returns
/// `bytes` is character byte slice, `wide` indicates UTF-16LE, returns UTF-8 String.
/// ## Errors / Exceptions
/// Returns error on odd-length UTF-16 or invalid surrogates.
fn decode_chars(bytes: &[u8], wide: bool) -> Result<String, String> {
    if wide {
        if !bytes
            .len()
            .is_multiple_of(crate::constants::XLSB_UTF16_UNIT_BYTES)
        {
            return Err(crate::constants::ERR_SHAPE_READ.to_string());
        }
        let units = bytes
            .as_chunks::<{ crate::constants::XLSB_UTF16_UNIT_BYTES }>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes(*pair));
        char::decode_utf16(units)
            .collect::<Result<String, _>>()
            .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())
    } else {
        Ok(WINDOWS_1252
            .decode_without_bom_handling(bytes)
            .0
            .into_owned())
    }
}

/// ## Description
/// Returns BIFF record type, size, and data slice with boundary checking.
/// ## Arguments / Returns
/// Accepts byte slice and mutable offset; returns record ID and payload slice.
/// ## Errors / Exceptions
/// Returns error on missing header, exceeded length limit, or out-of-bounds offset.
fn next_biff_record<'a>(bytes: &'a [u8], offset: &mut usize) -> Result<(u16, &'a [u8]), String> {
    // Constant reference: XLS_BIFF_RECORD_HEADER_BYTES and XLS_MAX_BIFF_RECORD_BYTES
    let header_end = offset
        .checked_add(crate::constants::XLS_BIFF_RECORD_HEADER_BYTES)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
    let record_id = u16::from_le_bytes([bytes[*offset], bytes[*offset + 1]]);
    let size = usize::from(u16::from_le_bytes([bytes[*offset + 2], bytes[*offset + 3]]));
    if size > crate::constants::XLS_MAX_BIFF_RECORD_BYTES {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let end = header_end
        .checked_add(size)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
    let record = &bytes[header_end..end];
    *offset = end;
    Ok((record_id, record))
}

/// ## Description
/// Holds sheet name and workbook stream offset extracted from BoundSheet8.
/// ## Arguments / Returns
/// `name` is sheet name, `stream_offset` is substream start offset.
/// ## Errors / Exceptions
/// Data holder only; does not generate errors or panics.
struct BoundSheet {
    name: String,
    stream_offset: usize,
}

#[cfg(test)]
mod tests {
    use super::{decode_txo_text, extract, parse_sheet_objects};
    use std::io::Write;
    use std::sync::atomic::AtomicBool;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// ## Description
    /// Verifies correctly decoding compressed and UTF-16LE strings split across Continue records.
    /// ## Arguments / Returns
    /// No arguments. Verifies decoded strings via assertions.
    /// ## Errors / Exceptions
    /// Fails if decoded results mismatch expectations.
    #[test]
    fn decodes_split_compressed_and_wide_text() {
        assert_eq!(
            decode_txo_text(&[&[0, b'c', b'a'], &[0, b't']], 3, 16).unwrap(),
            "cat"
        );
        assert_eq!(
            decode_txo_text(&[&[1, b'A', 0, b'B', 0]], 2, 16).unwrap(),
            "AB"
        );
    }

    /// ## Description
    /// Verifies constructing shape text results from TxO and Continue BIFF records.
    /// ## Arguments / Returns
    /// No arguments. Verifies parsed string and sheet name.
    /// ## Errors / Exceptions
    /// Fails if records are malformed or results mismatch expectations.
    #[test]
    fn returns_txo_text_as_sheet_shape() {
        let mut bytes = Vec::new();
        let mut txo = vec![0; crate::constants::XLS_TXO_FIXED_HEADER_BYTES];
        txo[crate::constants::XLS_TXO_TEXT_COUNT_OFFSET
            ..crate::constants::XLS_TXO_TEXT_COUNT_OFFSET + 2]
            .copy_from_slice(&2_u16.to_le_bytes());
        txo[crate::constants::XLS_TXO_TEXT_COUNT_OFFSET + 2
            ..crate::constants::XLS_TXO_TEXT_COUNT_OFFSET + 4]
            .copy_from_slice(&16_u16.to_le_bytes());
        append_record(crate::constants::XLS_TXO_RECORD_ID, &txo, &mut bytes);
        append_record(
            crate::constants::XLS_CONTINUE_RECORD_ID,
            &[0, b'h', b'i'],
            &mut bytes,
        );
        let mut next_shape = 1;
        let shapes = parse_sheet_objects(&bytes, "Sheet1", &mut next_shape).unwrap();

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].text, "hi");
        assert_eq!(shapes[0].sheet_name, "Sheet1");
        assert_eq!(shapes[0].anchor, None);
    }

    /// ## Description
    /// Verifies extracting shape text from CFB Workbook stream via sheet names and TxO.
    /// ## Arguments / Returns
    /// No arguments. Creates temporary `.xls` file and verifies extracted shape results.
    /// ## Errors / Exceptions
    /// Fails if temporary file creation, writing, extraction, or assertions fail.
    #[test]
    fn extracts_txo_text_from_cfb_workbook() {
        let mut sheet = Vec::new();
        let mut txo = vec![0; crate::constants::XLS_TXO_FIXED_HEADER_BYTES];
        txo[crate::constants::XLS_TXO_TEXT_COUNT_OFFSET
            ..crate::constants::XLS_TXO_TEXT_COUNT_OFFSET + 2]
            .copy_from_slice(&2_u16.to_le_bytes());
        append_record(crate::constants::XLS_TXO_RECORD_ID, &txo, &mut sheet);
        append_record(
            crate::constants::XLS_CONTINUE_RECORD_ID,
            &[0, b'o', b'k'],
            &mut sheet,
        );

        let mut workbook = Vec::new();
        let sheet_offset = 4 + 8 + 1;
        let mut bound_sheet = Vec::new();
        bound_sheet.extend_from_slice(&(sheet_offset as u32).to_le_bytes());
        bound_sheet.extend_from_slice(&[0, 0, 1, 0]);
        bound_sheet.push(b'S');
        append_record(
            crate::constants::XLS_BOUNDSHEET_RECORD_ID,
            &bound_sheet,
            &mut workbook,
        );
        workbook.extend_from_slice(&sheet);

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("shape-{}.xls", nonce));
        let mut compound = cfb::create(&path).expect("create CFB test file");
        compound
            .create_stream("/Workbook")
            .expect("create Workbook stream")
            .write_all(&workbook)
            .expect("write Workbook stream");
        drop(compound);

        let sheet_names = ["S".to_string()];
        let shapes = extract(&path, &sheet_names, None).expect("extract Shape text");
        let cancelled = AtomicBool::new(true);
        let cancelled_shapes = extract(&path, &sheet_names, Some(&cancelled))
            .expect("cancelled extraction must return partial results");
        std::fs::remove_file(path).expect("remove CFB test file");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].text, "ok");
        assert!(cancelled_shapes.is_empty());
    }

    /// ## Description
    /// Appends little-endian record ID, length, and payload of a BIFF record to test stream.
    /// ## Arguments / Returns
    /// `record_id` is record type, `payload` is body, `bytes` is destination vector.
    /// ## Errors / Exceptions
    /// Panics if payload exceeds `u16::MAX`.
    fn append_record(record_id: u16, payload: &[u8], bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&record_id.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        bytes.extend_from_slice(payload);
    }
}

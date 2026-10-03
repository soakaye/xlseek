//! # XLSB Shape Extractor
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Obtains relationship IDs from BIFF12 workbook and worksheet records, and reads DrawingML parts.
//! ## Arguments / Returns
//! Accepts workbook path, worksheet name list, and cancellation flag; returns list of ShapeText items.
//! ## Errors / Exceptions
//! Returns error on corrupted ZIP, records, strings, or exceeded security limits.

use super::ooxml;
use super::ShapeText;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use zip::ZipArchive;

/// ## Description
/// Resolves sheet parts and BrtDrawing in XLSB workbook, returning DrawingML shapes.
/// ## Arguments / Returns
/// `path` is an XLSB file, `sheet_names` is list of sheet names, `cancel_flag` is optional cancellation flag.
/// ## Errors / Exceptions
/// Returns error on invalid ZIP or BIFF12 data or exceeded limits; returns partial results on cancellation.
pub(super) fn extract(
    path: &Path,
    sheet_names: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
    let file = File::open(path).map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let mut archive =
        ZipArchive::new(file).map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let mut total_size = 0;
    let workbook_part = "xl/workbook.bin";
    let workbook = read_binary_part(&mut archive, workbook_part, &mut total_size)?;
    let workbook_rels_part = ooxml::relationship_part(workbook_part);
    let workbook_rels = ooxml::read_part(&mut archive, &workbook_rels_part, &mut total_size)?;
    let workbook_rels = ooxml::parse_relationships(&workbook_rels)?;
    let sheets = bundle_sheets(&workbook)?;
    let mut output = Vec::new();

    for sheet in sheets {
        if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
            break;
        }
        let Some(sheet_name) = sheet_names.iter().find(|name| **name == sheet.name) else {
            continue;
        };
        let Some(sheet_target) = workbook_rels.get(&sheet.relation_id) else {
            continue;
        };
        let sheet_part = ooxml::resolve_target(workbook_part, sheet_target);
        let sheet_binary = read_binary_part(&mut archive, &sheet_part, &mut total_size)?;
        let Some(drawing_relation_id) = drawing_relation_id(&sheet_binary)? else {
            continue;
        };
        let sheet_rels_part = ooxml::relationship_part(&sheet_part);
        let sheet_rels = ooxml::read_part(&mut archive, &sheet_rels_part, &mut total_size)?;
        let sheet_rels = ooxml::parse_relationships(&sheet_rels)?;
        let Some(drawing_target) = sheet_rels.get(&drawing_relation_id) else {
            continue;
        };
        let drawing_part = ooxml::resolve_target(&sheet_part, drawing_target);
        let drawing_xml = ooxml::read_part(&mut archive, &drawing_part, &mut total_size)?;
        output.extend(ooxml::parse_drawing(&drawing_xml, sheet_name)?);
    }
    Ok(output)
}

/// ## Description
/// Reads BIFF12 parts within a ZIP archive with size limits.
/// ## Arguments / Returns
/// Accepts ZIP archive, part path, and cumulative size pointer; returns byte vector.
/// ## Errors / Exceptions
/// Returns error on missing part, read failure, or single/cumulative size limit exceeded.
fn read_binary_part(
    archive: &mut ZipArchive<File>,
    part: &str,
    total_size: &mut u64,
) -> Result<Vec<u8>, String> {
    let mut entry = archive
        .by_name(part)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let size = entry.size();
    // Constant reference: SHAPE_MAX_BINARY_BYTES / SHAPE_MAX_TOTAL_XML_BYTES
    if size > crate::constants::SHAPE_MAX_BINARY_BYTES
        || total_size.saturating_add(size) > crate::constants::SHAPE_MAX_TOTAL_XML_BYTES
    {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let mut bytes = Vec::with_capacity(size as usize);
    entry
        .read_to_end(&mut bytes)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    *total_size += size;
    Ok(bytes)
}

/// ## Description
/// Extracts sheet names and workbook relationship IDs from BIFF12 record stream.
/// ## Arguments / Returns
/// `bytes` is entire content of `workbook.bin`; returns list of sheet part mappings.
/// ## Errors / Exceptions
/// Returns error on malformed record length or XLWideString.
fn bundle_sheets(bytes: &[u8]) -> Result<Vec<BundleSheet>, String> {
    let mut position = 0;
    let mut sheets = Vec::new();
    while position < bytes.len() {
        let (record_id, payload) = next_record(bytes, &mut position)?;
        if record_id != crate::constants::XLSB_BRT_BUNDLE_SH_RECORD_ID {
            continue;
        }
        // Constant reference: XLSB_BUNDLE_SHEET_FIXED_BYTES
        let mut cursor = crate::constants::XLSB_BUNDLE_SHEET_FIXED_BYTES;
        let relation_id = read_wide_string(payload, &mut cursor)?;
        let name = read_wide_string(payload, &mut cursor)?;
        if !relation_id.is_empty() && !name.is_empty() {
            sheets.push(BundleSheet { name, relation_id });
        }
    }
    Ok(sheets)
}

/// ## Description
/// Returns relationship ID of BrtDrawing from BIFF12 worksheet records.
/// ## Arguments / Returns
/// `bytes` is worksheet binary content; returns optional drawing relationship ID.
/// ## Errors / Exceptions
/// Returns error on malformed records or drawing string.
fn drawing_relation_id(bytes: &[u8]) -> Result<Option<String>, String> {
    let mut position = 0;
    while position < bytes.len() {
        let (record_id, payload) = next_record(bytes, &mut position)?;
        if record_id == crate::constants::XLSB_BRT_DRAWING_RECORD_ID {
            let mut cursor = 0;
            return read_wide_string(payload, &mut cursor).map(Some);
        }
    }
    Ok(None)
}

/// ## Description
/// Reads variable-length record ID and payload length, validating record boundary in BIFF12.
/// ## Arguments / Returns
/// Accepts `bytes` and mutable `position`; returns record ID and payload slice.
/// ## Errors / Exceptions
/// Returns error on out-of-bounds, length exceeding limit, or invalid varint.
fn next_record<'a>(bytes: &'a [u8], position: &mut usize) -> Result<(u32, &'a [u8]), String> {
    let record_id = read_varint(bytes, position)?;
    let length = read_varint(bytes, position)? as usize;
    if length > crate::constants::SHAPE_MAX_XML_BYTES as usize {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let end = position
        .checked_add(length)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
    let payload = &bytes[*position..end];
    *position = end;
    Ok((record_id, payload))
}

/// ## Description
/// Decodes a 7-bit BIFF12 integer using up to 5 bytes.
/// ## Arguments / Returns
/// Accepts byte slice and mutable read position; returns decoded `u32`.
/// ## Errors / Exceptions
/// Returns error on missing input, excessive continuation bits, or integer overflow.
fn read_varint(bytes: &[u8], position: &mut usize) -> Result<u32, String> {
    let mut value = 0_u32;
    // Constant reference: XLSB_VARINT_*
    for shift in (0..crate::constants::XLSB_VARINT_SHIFT * crate::constants::XLSB_VARINT_MAX_BYTES)
        .step_by(crate::constants::XLSB_VARINT_SHIFT as usize)
    {
        let byte = *bytes
            .get(*position)
            .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
        *position += 1;
        value |= u32::from(byte & crate::constants::XLSB_VARINT_VALUE_MASK)
            .checked_shl(shift)
            .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
        if byte & crate::constants::XLSB_VARINT_CONTINUATION_MASK == 0 {
            return Ok(value);
        }
    }
    Err(crate::constants::ERR_SHAPE_READ.to_string())
}

/// ## Description
/// Reads length-prefixed UTF-16LE string from BIFF12 payload.
/// ## Arguments / Returns
/// Accepts record payload and mutable position; returns decoded string.
/// ## Errors / Exceptions
/// Returns error on length mismatch or invalid UTF-16.
fn read_wide_string(bytes: &[u8], position: &mut usize) -> Result<String, String> {
    // Constant reference: XLSB_STRING_LENGTH_BYTES and XLSB_UTF16_UNIT_BYTES
    let count_bytes = bytes
        .get(*position..position.saturating_add(crate::constants::XLSB_STRING_LENGTH_BYTES))
        .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
    let count = u32::from_le_bytes(count_bytes.try_into().unwrap_or_default());
    *position += crate::constants::XLSB_STRING_LENGTH_BYTES;
    if count == u32::MAX {
        return Ok(String::new());
    }
    let byte_count = usize::try_from(count)
        .ok()
        .and_then(|count| count.checked_mul(crate::constants::XLSB_UTF16_UNIT_BYTES))
        .filter(|size| *size <= crate::constants::SHAPE_MAX_XML_BYTES as usize)
        .ok_or_else(|| crate::constants::ERR_SHAPE_LIMIT.to_string())?;
    let end = position
        .checked_add(byte_count)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| crate::constants::ERR_SHAPE_READ.to_string())?;
    let units = bytes[*position..end]
        .as_chunks::<{ crate::constants::XLSB_UTF16_UNIT_BYTES }>()
        .0
        .iter()
        .map(|value| u16::from_le_bytes(*value));
    let value = char::decode_utf16(units)
        .collect::<Result<String, _>>()
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    *position = end;
    Ok(value)
}

/// ## Description
/// Holds sheet name and relationship ID read from BrtBundleSh.
/// ## Arguments / Returns
/// `name` and `relation_id` identify the worksheet in the workbook.
/// ## Errors / Exceptions
/// Data holder only; does not generate errors or panics.
struct BundleSheet {
    name: String,
    relation_id: String,
}

#[cfg(test)]
mod tests {
    use super::{bundle_sheets, drawing_relation_id};

    /// ## Description
    /// Verifies extracting relationship IDs and sheet names from BrtBundleSh and BrtDrawing records.
    /// ## Arguments / Returns
    /// No arguments. Generates binary record sequence and verifies parse results.
    /// ## Errors / Exceptions
    /// Fails on invalid test data or mismatched parse results.
    #[test]
    fn reads_bundle_sheet_and_drawing_relationships() {
        let mut bundle = vec![0, 0, 0, 0, 1, 0, 0, 0];
        append_wide_string("rId7", &mut bundle);
        append_wide_string("Sheet A", &mut bundle);
        let mut workbook = Vec::new();
        append_varint(
            crate::constants::XLSB_BRT_BUNDLE_SH_RECORD_ID,
            &mut workbook,
        );
        append_varint(bundle.len() as u32, &mut workbook);
        workbook.extend(bundle);
        let sheets = bundle_sheets(&workbook).expect("bundle records must parse");
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[0].name, "Sheet A");
        assert_eq!(sheets[0].relation_id, "rId7");

        let mut drawing = Vec::new();
        append_wide_string("rId9", &mut drawing);
        let mut bytes = Vec::new();
        append_varint(crate::constants::XLSB_BRT_DRAWING_RECORD_ID, &mut bytes);
        append_varint(drawing.len() as u32, &mut bytes);
        bytes.extend(drawing);
        assert_eq!(
            drawing_relation_id(&bytes).expect("drawing record must parse"),
            Some("rId9".to_string())
        );
    }

    /// ## Description
    /// Appends length-prefixed UTF-16LE string to BIFF12 test record.
    /// ## Arguments / Returns
    /// `value` is input string, `bytes` is target vector. No return value.
    /// ## Errors / Exceptions
    /// Panics if character count exceeds `u32::MAX`.
    fn append_wide_string(value: &str, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&(value.encode_utf16().count() as u32).to_le_bytes());
        for unit in value.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
    }

    /// ## Description
    /// Appends `u32` BIFF12 variable-length integer to test byte vector.
    /// ## Arguments / Returns
    /// `value` is unsigned integer, `bytes` is target vector. No return value.
    /// ## Errors / Exceptions
    /// None.
    fn append_varint(mut value: u32, bytes: &mut Vec<u8>) {
        while value >= u32::from(crate::constants::XLSB_VARINT_CONTINUATION_MASK) {
            bytes.push(
                (value as u8 & crate::constants::XLSB_VARINT_VALUE_MASK)
                    | crate::constants::XLSB_VARINT_CONTINUATION_MASK,
            );
            value >>= crate::constants::XLSB_VARINT_SHIFT;
        }
        bytes.push(value as u8);
    }
}

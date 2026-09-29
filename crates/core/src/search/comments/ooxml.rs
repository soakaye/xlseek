//! ## Description
//! Resolves and extracts legacy comment XML from OOXML workbook and sheet relationships.
//! ## Arguments / Returns
//! Accepts workbook path and returns an array of (sheet name, cell coordinates, text).
//! ## Errors / Exceptions
//! Returns Err on missing reference parts, external relationships, corrupted XML, or exceeded limits.

use super::CommentText;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// ## Description
/// Traverses workbook and worksheet relationships to extract comment text and coordinates from comment XML.
/// ## Arguments / Returns
/// Accepts OOXML workbook path and returns a vector of cell comments.
/// ## Errors / Exceptions
/// Returns Err if reading required parts, XML parsing, external references, or cell reference parsing fails.
pub(super) fn extract(path: &Path) -> Result<Vec<CommentText>, String> {
    let file = File::open(path).map_err(|error| error.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|error| error.to_string())?;
    let mut total = 0;
    let workbook_xml = read_part(
        &mut archive,
        crate::constants::OOXML_WORKBOOK_PART,
        &mut total,
    )?;
    let workbook_rels = read_part(
        &mut archive,
        crate::constants::OOXML_WORKBOOK_RELS_PART,
        &mut total,
    )?;
    let sheet_relations = parse_relationships(&workbook_rels)?;
    let sheets = parse_sheets(&workbook_xml)?;
    let mut comments = Vec::new();
    for (sheet_name, relation_id) in sheets {
        let relation = sheet_relations
            .get(&relation_id)
            .ok_or_else(|| crate::constants::ERR_COMMENT_RELATION.to_string())?;
        if relation.external {
            return Err(crate::constants::ERR_COMMENT_EXTERNAL.to_string());
        }
        let worksheet_part =
            resolve_target(crate::constants::OOXML_WORKBOOK_PART, &relation.target);
        let relationships_part = relationship_part(&worksheet_part);
        let Ok(relationships_xml) = read_part(&mut archive, &relationships_part, &mut total) else {
            continue;
        };
        for relation in parse_relationships(&relationships_xml)?.values() {
            if relation.external {
                if relation
                    .kind
                    .ends_with(crate::constants::OOXML_COMMENTS_RELATION_SUFFIX)
                {
                    return Err(crate::constants::ERR_COMMENT_EXTERNAL.to_string());
                }
                continue;
            }
            if relation
                .kind
                .ends_with(crate::constants::OOXML_COMMENTS_RELATION_SUFFIX)
            {
                let comments_part = resolve_target(&worksheet_part, &relation.target);
                let xml = read_part(&mut archive, &comments_part, &mut total)?;
                comments.extend(parse_comments(&xml, &sheet_name)?);
            }
        }
    }
    Ok(comments)
}

/// ## Description
/// Reads XML parts within a ZIP archive with expansion limits.
/// ## Arguments / Returns
/// Accepts ZIP archive, part name, and cumulative size pointer, returning XML bytes.
/// ## Errors / Exceptions
/// Returns Err on missing part, I/O error, or individual/cumulative size limit exceeded.
fn read_part(
    archive: &mut ZipArchive<File>,
    part: &str,
    total: &mut u64,
) -> Result<Vec<u8>, String> {
    let mut entry = archive.by_name(part).map_err(|error| error.to_string())?;
    let size = entry.size();
    if size > crate::constants::SHAPE_MAX_XML_BYTES
        || total.saturating_add(size) > crate::constants::SHAPE_MAX_TOTAL_XML_BYTES
    {
        return Err(crate::constants::ERR_SHAPE_LIMIT.to_string());
    }
    let mut bytes = Vec::with_capacity(size as usize);
    entry
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    *total += size;
    Ok(bytes)
}

/// ## Description
/// Maps relationship parts from Id to Type, Target, and TargetMode.
/// ## Arguments / Returns
/// Accepts XML bytes and returns a HashMap keyed by relationship ID.
/// ## Errors / Exceptions
/// Returns Err if XML syntax or required attributes are invalid.
fn parse_relationships(xml: &[u8]) -> Result<HashMap<String, Relationship>, String> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut relationships = HashMap::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event) | Event::Empty(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_RELATIONSHIP_ELEMENT.as_bytes() =>
            {
                let mut id = None;
                let mut target = None;
                let mut kind = None;
                let mut external = false;
                for attribute in event.attributes() {
                    let attribute = attribute.map_err(|error| error.to_string())?;
                    let key = local_name(attribute.key.as_ref());
                    let value = attribute
                        .decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            reader.decoder(),
                        )
                        .map_err(|error| error.to_string())?
                        .into_owned();
                    match key {
                        key if key
                            == crate::constants::XML_RELATIONSHIP_ID_ATTRIBUTE.as_bytes() =>
                        {
                            id = Some(value)
                        }
                        key if key
                            == crate::constants::XML_RELATIONSHIP_TARGET_ATTRIBUTE.as_bytes() =>
                        {
                            target = Some(value)
                        }
                        key if key
                            == crate::constants::XML_RELATIONSHIP_TYPE_ATTRIBUTE.as_bytes() =>
                        {
                            kind = Some(value)
                        }
                        key if key
                            == crate::constants::XML_RELATIONSHIP_MODE_ATTRIBUTE.as_bytes() =>
                        {
                            external = value == crate::constants::XML_RELATIONSHIP_EXTERNAL_VALUE;
                        }
                        _ => {}
                    }
                }
                let id = id.ok_or_else(|| crate::constants::ERR_COMMENT_RELATION.to_string())?;
                let target =
                    target.ok_or_else(|| crate::constants::ERR_COMMENT_RELATION.to_string())?;
                relationships.insert(
                    id,
                    Relationship {
                        target,
                        kind: kind.unwrap_or_default(),
                        external,
                    },
                );
            }
            Ok(Event::Eof) => break,
            Err(error) => return Err(error.to_string()),
            _ => {}
        }
        buffer.clear();
    }
    Ok(relationships)
}

/// ## Description
/// Extracts sheet names and relationship IDs from workbook XML.
/// ## Arguments / Returns
/// Accepts XML bytes and returns a list of (sheet name, relationship id) pairs.
/// ## Errors / Exceptions
/// Returns Err if XML syntax or required attributes are invalid.
fn parse_sheets(xml: &[u8]) -> Result<Vec<(String, String)>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut sheets = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event) | Event::Empty(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_SHEET_ELEMENT.as_bytes() =>
            {
                let mut name = None;
                let mut relation_id = None;
                for attribute in event.attributes() {
                    let attribute = attribute.map_err(|error| error.to_string())?;
                    let key = local_name(attribute.key.as_ref());
                    let value = attribute
                        .decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            reader.decoder(),
                        )
                        .map_err(|error| error.to_string())?
                        .into_owned();
                    match key {
                        key if key == crate::constants::XML_SHEET_NAME_ATTRIBUTE.as_bytes() => {
                            name = Some(value)
                        }
                        key if key
                            == crate::constants::XML_SHEET_RELATIONSHIP_ID_ATTRIBUTE.as_bytes() =>
                        {
                            relation_id = Some(value)
                        }
                        _ => {}
                    }
                }
                sheets.push((
                    name.ok_or_else(|| crate::constants::ERR_COMMENT_RELATION.to_string())?,
                    relation_id
                        .ok_or_else(|| crate::constants::ERR_COMMENT_RELATION.to_string())?,
                ));
            }
            Ok(Event::Eof) => break,
            Err(error) => return Err(error.to_string()),
            _ => {}
        }
        buffer.clear();
    }
    Ok(sheets)
}

/// ## Description
/// Extracts cell references and multiple run text from comment elements in legacy comment XML.
/// ## Arguments / Returns
/// Accepts XML bytes and sheet name, returning a list of CommentText items.
/// ## Errors / Exceptions
/// Returns Err if XML, cell references, or coordinates are invalid.
fn parse_comments(xml: &[u8], sheet_name: &str) -> Result<Vec<CommentText>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut output = Vec::new();
    let mut active_cell = None;
    let mut text = String::new();
    let mut inside_text = false;
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_COMMENT_ELEMENT.as_bytes() =>
            {
                active_cell = event.attributes().flatten().find_map(|attribute| {
                    (local_name(attribute.key.as_ref())
                        == crate::constants::XML_COMMENT_CELL_ATTRIBUTE.as_bytes())
                    .then(|| {
                        attribute
                            .decoded_and_normalized_value(
                                quick_xml::XmlVersion::Implicit1_0,
                                reader.decoder(),
                            )
                            .ok()
                            .map(|value| value.into_owned())
                    })
                    .flatten()
                });
                text.clear();
            }
            Ok(Event::Empty(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_COMMENT_ELEMENT.as_bytes() =>
            {
                let cell = event.attributes().flatten().find_map(|attribute| {
                    (local_name(attribute.key.as_ref())
                        == crate::constants::XML_COMMENT_CELL_ATTRIBUTE.as_bytes())
                    .then(|| {
                        attribute
                            .decoded_and_normalized_value(
                                quick_xml::XmlVersion::Implicit1_0,
                                reader.decoder(),
                            )
                            .ok()
                            .map(|value| value.into_owned())
                    })
                    .flatten()
                });
                if let Some(cell) = cell {
                    let (row, col) = parse_cell_reference(&cell)?;
                    output.push(CommentText {
                        sheet_name: sheet_name.to_string(),
                        row,
                        col,
                        text: String::new(),
                    });
                }
            }
            Ok(Event::Start(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_TEXT_ELEMENT.as_bytes() =>
            {
                inside_text = true
            }
            Ok(Event::End(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_TEXT_ELEMENT.as_bytes() =>
            {
                inside_text = false
            }
            Ok(Event::Text(event)) if inside_text => {
                let decoded = event.decode().map_err(|error| error.to_string())?;
                text.push_str(
                    &quick_xml::escape::unescape(&decoded).map_err(|error| error.to_string())?,
                );
            }
            Ok(Event::End(event))
                if local_name(event.name().as_ref())
                    == crate::constants::XML_COMMENT_ELEMENT.as_bytes() =>
            {
                if let Some(cell) = active_cell.take() {
                    let (row, col) = parse_cell_reference(&cell)?;
                    output.push(CommentText {
                        sheet_name: sheet_name.to_string(),
                        row,
                        col,
                        text: text.clone(),
                    });
                }
            }
            Ok(Event::Eof) => break,
            Err(error) => return Err(error.to_string()),
            _ => {}
        }
        buffer.clear();
    }
    Ok(output)
}

/// ## Description
/// Converts an A1-style cell reference to 1-based row and column indices.
/// ## Arguments / Returns
/// Accepts cell reference string slice and returns (row, col) u32 tuple.
/// ## Errors / Exceptions
/// Returns Err if reference is empty, lacks column letters/row digits, or overflows u32.
fn parse_cell_reference(reference: &str) -> Result<(u32, u32), String> {
    let split = reference
        .find(|character: char| character.is_ascii_digit())
        .ok_or_else(|| crate::constants::ERR_COMMENT_CELL.to_string())?;
    let (letters, digits) = reference.split_at(split);
    if letters.is_empty()
        || digits.is_empty()
        || !letters.bytes().all(|byte| byte.is_ascii_alphabetic())
    {
        return Err(crate::constants::ERR_COMMENT_CELL.to_string());
    }
    let mut column = 0u32;
    for byte in letters.bytes() {
        let upper = byte.to_ascii_uppercase();
        column = column
            .checked_mul(crate::constants::EXCEL_COLUMN_BASE)
            .and_then(|value| {
                value.checked_add(
                    u32::from(upper - crate::constants::EXCEL_ASCII_A)
                        + crate::constants::EXCEL_ONE_BASED_OFFSET,
                )
            })
            .ok_or_else(|| crate::constants::ERR_COMMENT_CELL.to_string())?;
    }
    let row = digits
        .parse::<u32>()
        .map_err(|_| crate::constants::ERR_COMMENT_CELL.to_string())?;
    if row == crate::constants::EXCEL_ZERO_INDEX || column == crate::constants::EXCEL_ZERO_INDEX {
        return Err(crate::constants::ERR_COMMENT_CELL.to_string());
    }
    Ok((row, column))
}

/// ## Description
/// Returns the local name of an XML tag without namespace prefix.
/// ## Arguments / Returns
/// Accepts XML name byte slice and returns the trailing local name slice.
/// ## Errors / Exceptions
/// Returns full slice if no separator is present; does not panic.
fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|value| *value == crate::constants::XML_NAMESPACE_SEPARATOR)
        .next()
        .unwrap_or(name)
}

/// ## Description
/// Holds Target, kind, and external reference state of an OOXML relationship.
/// ## Arguments / Returns
/// `target` and `kind` are Strings; `external` is a bool.
/// ## Errors / Exceptions
/// Value holder only; does not generate errors.
struct Relationship {
    target: String,
    kind: String,
    external: bool,
}

/// ## Description
/// Normalizes an OOXML relationship target path relative to the source part location.
/// ## Arguments / Returns
/// Accepts source part path and target string, returning normalized ZIP part path.
/// ## Errors / Exceptions
/// Pure string manipulation; no I/O errors occur.
fn resolve_target(source: &str, target: &str) -> String {
    if target.starts_with(crate::constants::OOXML_ABSOLUTE_PATH_PREFIX) {
        return target
            .trim_start_matches(crate::constants::OOXML_ABSOLUTE_PATH_PREFIX)
            .to_string();
    }
    let mut parts = source
        .rsplit_once(crate::constants::PATH_SEPARATOR)
        .map(|(parent, _)| parent)
        .unwrap_or("")
        .split(crate::constants::PATH_SEPARATOR)
        .collect::<Vec<_>>();
    for part in target.split(crate::constants::PATH_SEPARATOR) {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join(crate::constants::PATH_SEPARATOR)
}

/// ## Description
/// Constructs the relationship part path corresponding to a worksheet part.
/// ## Arguments / Returns
/// Accepts worksheet ZIP part path and returns relationship ZIP part path.
/// ## Errors / Exceptions
/// Pure string formatting; does not generate errors.
fn relationship_part(part: &str) -> String {
    let (parent, name) = part
        .rsplit_once(crate::constants::PATH_SEPARATOR)
        .unwrap_or((crate::constants::OOXML_WORKBOOK_DIRECTORY, part));
    format!("{parent}/_rels/{name}.rels")
}

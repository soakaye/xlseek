//! # OOXML Shape Extractor
//!
//! ## Description
//! Traverses worksheet relationships in `.xlsx` and `.xlsm` to read DrawingML shape text and anchors.
//! ## Arguments / Returns
//! Internal functions accept ZIP entry names or XML bytes, returning relationships or shape lists.
//! ## Errors / Exceptions
//! Returns errors on missing ZIP entries, malformed XML, expansion limit exceeded, or depth limit exceeded.

use super::ShapeText;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use zip::ZipArchive;

/// ## Description
/// Traverses OOXML sheets and DrawingML drawings to extract child shapes containing text.
/// ## Arguments / Returns
/// `path` is an `.xlsx` / `.xlsm` file, `cancel_flag` is an optional cancellation flag. Returns a common shape list.
/// ## Errors / Exceptions
/// Returns error on invalid XML, expansion limit exceeded, or ZIP read failure. Returns collected shapes on cancellation.
pub(super) fn extract(
    path: &Path,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
    let file = File::open(path).map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let mut archive =
        ZipArchive::new(file).map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let mut total_size = 0;
    let workbook_xml = read_part(&mut archive, "xl/workbook.xml", &mut total_size)?;
    let workbook_rels = read_part(&mut archive, "xl/_rels/workbook.xml.rels", &mut total_size)?;
    let rels = parse_relationships(&workbook_rels)?;
    let sheets = parse_sheets(&workbook_xml)?;
    let mut shapes = Vec::new();

    for (sheet_name, relation_id) in sheets {
        if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
            break;
        }
        let Some(target) = rels.get(&relation_id) else {
            continue;
        };
        let sheet_part = resolve_target("xl/workbook.xml", target);
        let sheet_rels_part = relationship_part(&sheet_part);
        let Ok(sheet_rels_xml) = read_part(&mut archive, &sheet_rels_part, &mut total_size) else {
            continue;
        };
        let sheet_rels = parse_relationships(&sheet_rels_xml)?;
        let Some(drawing_target) = sheet_rels.values().find(|value| value.contains("drawing"))
        else {
            continue;
        };
        let drawing_part = resolve_target(&sheet_part, drawing_target);
        let Ok(drawing_xml) = read_part(&mut archive, &drawing_part, &mut total_size) else {
            continue;
        };
        shapes.extend(parse_drawing(&drawing_xml, &sheet_name)?);
    }
    Ok(shapes)
}

/// ## Description
/// Reads XML parts within a ZIP archive with limits and tracks cumulative expansion.
/// ## Arguments / Returns
/// `archive` is the ZIP archive, `part` is the internal path, `total_size` is cumulative size. Returns XML bytes.
/// ## Errors / Exceptions
/// Returns error on missing part, read failure, or single/cumulative size limit exceeded.
pub(super) fn read_part(
    archive: &mut ZipArchive<File>,
    part: &str,
    total_size: &mut u64,
) -> Result<Vec<u8>, String> {
    let mut entry = archive
        .by_name(part)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let size = entry.size();
    // Constant reference: SHAPE_MAX_XML_BYTES / SHAPE_MAX_TOTAL_XML_BYTES
    if size > crate::constants::SHAPE_MAX_XML_BYTES
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
/// Extracts relationship IDs and targets from workbook or worksheet relationship XML.
/// ## Arguments / Returns
/// `xml` is the relationship XML. Returns a map from ID to target path.
/// ## Errors / Exceptions
/// Returns error if XML is malformed or exceeds max depth.
pub(super) fn parse_relationships(xml: &[u8]) -> Result<HashMap<String, String>, String> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut relationships = HashMap::new();
    let mut depth = 0;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => {
                depth += 1;
                check_depth(depth)?;
                if local_name(event.name().as_ref()) == b"Relationship" {
                    let attrs = attributes(&reader, &event)?;
                    if let (Some(id), Some(target)) = (attrs.get("Id"), attrs.get("Target")) {
                        relationships.insert(id.clone(), target.clone());
                    }
                }
            }
            Ok(Event::Empty(event)) => {
                if local_name(event.name().as_ref()) == b"Relationship" {
                    let attrs = attributes(&reader, &event)?;
                    if let (Some(id), Some(target)) = (attrs.get("Id"), attrs.get("Target")) {
                        relationships.insert(id.clone(), target.clone());
                    }
                }
            }
            Ok(Event::End(_)) => depth = depth.saturating_sub(1),
            Ok(Event::Eof) => break,
            Err(_) => return Err(crate::constants::ERR_SHAPE_READ.to_string()),
            _ => {}
        }
        buf.clear();
    }
    Ok(relationships)
}

/// ## Description
/// Extracts worksheet names and relationship IDs from `workbook.xml`.
/// ## Arguments / Returns
/// `xml` is the workbook XML. Returns a list of `(sheet_name, relationship_id)` pairs.
/// ## Errors / Exceptions
/// Returns error if XML is malformed or exceeds max depth.
fn parse_sheets(xml: &[u8]) -> Result<Vec<(String, String)>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut buf = Vec::new();
    let mut sheets = Vec::new();
    let mut depth = 0;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => {
                depth += 1;
                check_depth(depth)?;
                if local_name(event.name().as_ref()) == b"sheet" {
                    let attrs = attributes(&reader, &event)?;
                    if let (Some(name), Some(id)) = (attrs.get("name"), attrs.get("id")) {
                        sheets.push((name.clone(), id.clone()));
                    }
                }
            }
            Ok(Event::Empty(event)) => {
                if local_name(event.name().as_ref()) == b"sheet" {
                    let attrs = attributes(&reader, &event)?;
                    if let (Some(name), Some(id)) = (attrs.get("name"), attrs.get("id")) {
                        sheets.push((name.clone(), id.clone()));
                    }
                }
            }
            Ok(Event::End(_)) => depth = depth.saturating_sub(1),
            Ok(Event::Eof) => break,
            Err(_) => return Err(crate::constants::ERR_SHAPE_READ.to_string()),
            _ => {}
        }
        buf.clear();
    }
    Ok(sheets)
}

/// ## Description
/// Collects names, strings, and anchors for regular shapes within DrawingML.
/// ## Arguments / Returns
/// `xml` is drawing XML, `sheet_name` is worksheet name. Returns list of ShapeText items.
/// ## Errors / Exceptions
/// Returns error if XML is malformed or exceeds max depth.
pub(super) fn parse_drawing(xml: &[u8], sheet_name: &str) -> Result<Vec<ShapeText>, String> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut output = Vec::new();
    let mut group_names = Vec::<String>::new();
    let mut shape: Option<ShapeBuilder> = None;
    let mut current_anchor = None;
    let mut anchor_open = false;
    let mut anchor_col: Option<u32> = None;
    let mut anchor_row: Option<u32> = None;
    let mut coordinate = None;
    let mut in_text = false;
    let mut depth = 0;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(event)) => {
                depth += 1;
                check_depth(depth)?;
                let name = local_name(event.name().as_ref()).to_vec();
                match name.as_slice() {
                    b"grpSp" => group_names.push(String::new()),
                    b"sp" => shape = Some(ShapeBuilder::new(&group_names)),
                    b"twoCellAnchor" | b"oneCellAnchor" => {
                        anchor_open = true;
                        anchor_col = None;
                        anchor_row = None;
                    }
                    b"col" if anchor_open => coordinate = Some(false),
                    b"row" if anchor_open => coordinate = Some(true),
                    b"cNvPr" => {
                        let attrs = attributes(&reader, &event)?;
                        if let Some(active) = shape.as_mut() {
                            active.id = attrs.get("id").cloned().unwrap_or_default();
                            active.name = attrs.get("name").cloned().unwrap_or_default();
                        } else if let (Some(group), Some(name)) =
                            (group_names.last_mut(), attrs.get("name"))
                        {
                            *group = name.clone();
                        }
                    }
                    b"t" if shape.is_some() => in_text = true,
                    _ => {}
                }
            }
            Ok(Event::Empty(event)) => match local_name(event.name().as_ref()) {
                b"sp" => {
                    shape = Some(ShapeBuilder::new(&group_names));
                    if let Some(active) = shape.take() {
                        output.push(active.finish(sheet_name, current_anchor));
                    }
                }
                b"cNvPr" => {
                    let attrs = attributes(&reader, &event)?;
                    if let Some(active) = shape.as_mut() {
                        active.id = attrs.get("id").cloned().unwrap_or_default();
                        active.name = attrs.get("name").cloned().unwrap_or_default();
                    } else if let (Some(group), Some(name)) =
                        (group_names.last_mut(), attrs.get("name"))
                    {
                        *group = name.clone();
                    }
                }
                _ => {}
            },
            Ok(Event::Text(event)) if in_text => {
                let decoded = event
                    .decode()
                    .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
                let decoded = quick_xml::escape::unescape(&decoded)
                    .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
                if let Some(active) = shape.as_mut() {
                    active.text.push_str(&decoded);
                }
            }
            Ok(Event::Text(event)) if coordinate.is_some() => {
                let value = event
                    .decode()
                    .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?
                    .parse()
                    .ok();
                if coordinate == Some(true) {
                    anchor_row = value;
                } else {
                    anchor_col = value;
                }
            }
            Ok(Event::End(event)) => {
                let qualified_name = event.name();
                let name = local_name(qualified_name.as_ref());
                match name {
                    b"t" => in_text = false,
                    b"col" | b"row" => coordinate = None,
                    b"p" => {
                        if let Some(active) = shape.as_mut() {
                            active.text.push('\n');
                        }
                    }
                    b"sp" => {
                        if let Some(active) = shape.take() {
                            if !active.text.trim().is_empty() {
                                output.push(active.finish(sheet_name, current_anchor));
                            }
                        }
                    }
                    b"grpSp" => {
                        group_names.pop();
                    }
                    b"from" => {
                        if let (Some(col), Some(row)) = (anchor_col, anchor_row) {
                            current_anchor = Some((row + 1, col + 1));
                        }
                    }
                    b"twoCellAnchor" | b"oneCellAnchor" => {
                        current_anchor = None;
                        anchor_open = false;
                    }
                    _ => {}
                }
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Eof) => break,
            Err(_) => return Err(crate::constants::ERR_SHAPE_READ.to_string()),
            _ => {}
        }
        buf.clear();
    }
    Ok(output)
}

/// ## Description
/// Returns XML attributes stringified by local name.
/// ## Arguments / Returns
/// Accepts XML reader and start element; returns a map of attribute names and values.
/// ## Errors / Exceptions
/// Returns error on malformed attribute or XML value.
fn attributes<R: std::io::BufRead>(
    reader: &Reader<R>,
    element: &BytesStart<'_>,
) -> Result<HashMap<String, String>, String> {
    element
        .attributes()
        .with_checks(false)
        .map(|attribute| {
            let attribute = attribute.map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
            let key = String::from_utf8_lossy(local_name(attribute.key.as_ref())).into_owned();
            let value = attribute
                .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
                .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?
                .into_owned();
            Ok((key, value))
        })
        .collect()
}

/// ## Description
/// Extracts local name byte slice from a qualified XML name.
/// ## Arguments / Returns
/// `name` is qualified name; returns element name after colon.
/// ## Errors / Exceptions
/// Does not panic.
fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|value| *value == b':').next().unwrap_or(name)
}

/// ## Description
/// Normalizes an OOXML relative relationship target into a canonical ZIP part path.
/// ## Arguments / Returns
/// `source` is source part, `target` is relationship target; returns normalized path.
/// ## Errors / Exceptions
/// Pure path normalization; no I/O errors or panics occur.
pub(super) fn resolve_target(source: &str, target: &str) -> String {
    if target.starts_with('/') {
        return target.trim_start_matches('/').to_string();
    }
    let mut components = source
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("")
        .split('/')
        .collect::<Vec<_>>();
    for component in target.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            value => components.push(value),
        }
    }
    components.join("/")
}

/// ## Description
/// Generates relationships part path for a worksheet part.
/// ## Arguments / Returns
/// `sheet_part` is worksheet part path; returns corresponding `.rels` part path.
/// ## Errors / Exceptions
/// Pure string conversion; does not panic.
pub(super) fn relationship_part(sheet_part: &str) -> String {
    let (parent, name) = sheet_part.rsplit_once('/').unwrap_or(("xl", sheet_part));
    format!("{parent}/_rels/{name}.rels")
}

/// ## Description
/// Verifies that XML nesting depth is within allowed limits.
/// ## Arguments / Returns
/// `depth` is current depth; returns `Ok(())` on success.
/// ## Errors / Exceptions
/// Returns error if max depth is exceeded.
fn check_depth(depth: usize) -> Result<(), String> {
    // Constant reference: SHAPE_MAX_XML_DEPTH
    if depth > crate::constants::SHAPE_MAX_XML_DEPTH {
        Err(crate::constants::ERR_SHAPE_LIMIT.to_string())
    } else {
        Ok(())
    }
}

/// ## Description
/// Holds shape name, text, and identifier during construction.
/// ## Arguments / Returns
/// `new` accepts group path; `finish` accepts sheet name and anchor, returning ShapeText.
/// ## Errors / Exceptions
/// String construction only; does not panic.
struct ShapeBuilder {
    id: String,
    name: String,
    text: String,
    group_path: Vec<String>,
}

impl ShapeBuilder {
    fn new(groups: &[String]) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            text: String::new(),
            group_path: groups
                .iter()
                .filter(|name| !name.is_empty())
                .cloned()
                .collect(),
        }
    }

    fn finish(self, sheet_name: &str, anchor: Option<(u32, u32)>) -> ShapeText {
        let shape_name = if self.name.is_empty() {
            self.id.clone()
        } else if self.group_path.is_empty() {
            self.name.clone()
        } else {
            format!("{} / {}", self.group_path.join(" / "), self.name)
        };
        ShapeText {
            shape_id: self.id,
            shape_name,
            text: self.text.trim_end_matches('\n').to_string(),
            sheet_name: sheet_name.to_string(),
            anchor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_drawing;

    /// ## Description
    /// Verifies concatenating text runs in order and extracting shape name and 1-based anchor.
    /// ## Arguments / Returns
    /// No arguments. Verifies shapes from XML string via assertions.
    /// ## Errors / Exceptions
    /// Fails if XML is malformed or extracted values mismatch expectations.
    #[test]
    fn extracts_text_runs_name_and_anchor() {
        let xml = br#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:twoCellAnchor><xdr:from><xdr:col>1</xdr:col><xdr:row>2</xdr:row></xdr:from><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="4" name="Text Box 3"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>find </a:t></a:r><a:r><a:t>shape</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:twoCellAnchor></xdr:wsDr>"#;
        let shapes = parse_drawing(xml, "Sheet1").expect("drawing must parse");

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].shape_id, "4");
        assert_eq!(shapes[0].shape_name, "Text Box 3");
        assert_eq!(shapes[0].text, "find shape");
        assert_eq!(shapes[0].sheet_name, "Sheet1");
        assert_eq!(shapes[0].anchor, Some((3, 2)));
    }

    /// ## Description
    /// Verifies returning child shapes within groups individually and keeping missing anchors as None.
    /// ## Arguments / Returns
    /// No arguments. Verifies extraction results via assertions.
    /// ## Errors / Exceptions
    /// Fails if XML is malformed or shape count, names, or text mismatch.
    #[test]
    fn returns_group_children_separately_without_anchor() {
        let xml = br#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:twoCellAnchor><xdr:from><xdr:col>0</xdr:col><xdr:row>0</xdr:row></xdr:from><xdr:grpSp><xdr:nvGrpSpPr><xdr:cNvPr id="8" name="Group"/></xdr:nvGrpSpPr><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="9" name="Child A"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>first</a:t></a:r></a:p></xdr:txBody></xdr:sp><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="10" name="Child B"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>second</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:grpSp></xdr:twoCellAnchor><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="11" name="No anchor"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>outside</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:wsDr>"#;
        let shapes = parse_drawing(xml, "Sheet1").expect("drawing must parse");

        assert_eq!(shapes.len(), 3);
        assert_eq!(shapes[0].shape_name, "Group / Child A");
        assert_eq!(shapes[1].shape_name, "Group / Child B");
        assert_eq!(shapes[0].text, "first");
        assert_eq!(shapes[1].text, "second");
        assert_eq!(shapes[0].anchor, Some((1, 1)));
        assert_eq!(shapes[2].anchor, None);
    }
}

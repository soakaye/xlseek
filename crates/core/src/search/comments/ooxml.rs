//! ## 処理内容
//! OOXML workbookとsheet relationshipsから従来コメントXMLを解決して抽出する。
//! ## 引数・戻り値
//! ブックパスを受け、sheet名・セル座標・本文の配列を返す。
//! ## エラー
//! 参照パーツ欠落、外部relationship、破損XML、上限超過時にErrを返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): OOXMLメモXML抽出を追加。

use super::CommentText;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// ## 処理内容
/// workbookと各worksheetのrelationshipsを辿り、コメントXMLにある本文・座標を抽出する。
/// ## 引数・戻り値
/// OOXMLブックパスを受け、セルコメントのベクターを返す。
/// ## エラー
/// 必須パーツの読み込み、XML解析、外部参照、セル参照解析に失敗するとErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): OOXMLの既存コメント抽出を追加。
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

/// ## 処理内容
/// ZIP内XMLパーツを展開上限付きで読み込む。
/// ## 引数・戻り値
/// ZIP、パーツ名、累積サイズを受け、XMLバイト列を返す。
/// ## エラー
/// 欠落、I/O、単体・累積サイズ超過でErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): コメントXMLの安全読込を追加。
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

/// ## 処理内容
/// relationshipパーツをIdからType・Target・TargetModeへ対応づける。
/// ## 引数・戻り値
/// XMLバイト列を受け、relationship IDをキーとするHashMapを返す。
/// ## エラー
/// XML構文または必須属性が不正な場合にErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): relationship詳細解析を追加。
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

/// ## 処理内容
/// Workbook XMLからシート名とrelationship IDを抽出する。
/// ## 引数・戻り値
/// XMLバイト列を受け、(sheet name, relation id)の一覧を返す。
/// ## エラー
/// XML構文または必須属性が不正ならErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): ワークシート対応表の解析を追加。
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

/// ## 処理内容
/// 既存コメントXMLのcomment要素からセル参照と複数run本文を抽出する。
/// ## 引数・戻り値
/// XMLバイト列とシート名を受け、CommentText一覧を返す。
/// ## エラー
/// XML・セル参照・座標が不正な場合にErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): コメント本文抽出を追加。
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

/// ## 処理内容
/// A1形式のセル番地をExcel上の1始まり行列へ変換する。
/// ## 引数・戻り値
/// セル参照`&str`を受け、(row, col)のu32対を返す。
/// ## エラー
/// 空、列文字・行数字の欠落、整数オーバーフローでErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): コメント参照座標変換を追加。
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

/// ## 処理内容
/// XMLタグ名からnamespace prefixを除いたローカル名を返す。
/// ## 引数・戻り値
/// XML名のbyte sliceを受け、末尾のローカル名sliceを返す。
/// ## エラー
/// 区切りがなければ入力全体を返しpanicしない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): XMLローカル名処理を追加。
fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|value| *value == crate::constants::XML_NAMESPACE_SEPARATOR)
        .next()
        .unwrap_or(name)
}

/// ## 処理内容
/// OOXML relationshipのTargetと種類・外部参照状態を保持する。
/// ## 引数・戻り値
/// target・kindはString、externalはbool。
/// ## エラー
/// 値保持のみでエラーは発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): OOXML relationship型を追加。
struct Relationship {
    target: String,
    kind: String,
    external: bool,
}

/// ## 処理内容
/// sourceパーツの相対位置からOOXML relationship targetを正規化する。
/// ## 引数・戻り値
/// 参照元とtargetを受け、ZIPパーツ名文字列を返す。
/// ## エラー
/// 純粋な文字列変換でI/Oエラーはない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): relationship target解決を追加。
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

/// ## 処理内容
/// worksheetパーツに対応するrelationshipパーツ名を作成する。
/// ## 引数・戻り値
/// worksheet ZIPパーツ名を受け、relationship ZIPパーツ名を返す。
/// ## エラー
/// 文字列連結のみでエラーは発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): worksheet relationship名生成を追加。
fn relationship_part(part: &str) -> String {
    let (parent, name) = part
        .rsplit_once(crate::constants::PATH_SEPARATOR)
        .unwrap_or((crate::constants::OOXML_WORKBOOK_DIRECTORY, part));
    format!("{parent}/_rels/{name}.rels")
}

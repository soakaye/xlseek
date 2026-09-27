//! # OOXML Shape 抽出器
//!
//! ## 処理内容
//! `.xlsx` と `.xlsm` のワークシート関係を辿り、DrawingML の図形テキストとアンカーを読む。
//! ## 引数・戻り値
//! 内部関数は ZIP エントリ名または XML バイト列を受け取り、関係情報または Shape 一覧を返す。
//! ## エラー / 例外発生条件
//! ZIP エントリ欠損、XML 不正、展開上限超過、深さ上限超過をエラーとして返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): OOXML 描画パーツの基本読取を追加。

use super::ShapeText;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use zip::ZipArchive;

/// ## 処理内容
/// OOXML ブックのシートと DrawingML 描画を辿り、テキストを持つ子 Shape を抽出する。
/// ## 引数・戻り値
/// `path` は `.xlsx` / `.xlsm` ファイル、`cancel_flag` は任意のキャンセル状態。戻り値は共通 Shape 一覧。
/// ## エラー / 例外発生条件
/// 必須 XML が不正、展開上限超過、ZIP 読取失敗時にエラーを返す。キャンセル時は取得済み一覧を返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): OOXML シート関係と DrawingML Shape 抽出を実装。
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

/// ## 処理内容
/// ZIP 内の XML パーツを上限付きで読み込み、累積展開量を確認する。
/// ## 引数・戻り値
/// `archive` は ZIP、`part` は ZIP 内パス、`total_size` は累積サイズ。戻り値は XML バイト列。
/// ## エラー / 例外発生条件
/// パーツ欠損、読み込み失敗、単体または累積サイズ上限超過時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XML パーツ読み込み制限を追加。
pub(super) fn read_part(
    archive: &mut ZipArchive<File>,
    part: &str,
    total_size: &mut u64,
) -> Result<Vec<u8>, String> {
    let mut entry = archive
        .by_name(part)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let size = entry.size();
    // 定数参照: SHAPE_MAX_XML_BYTES / SHAPE_MAX_TOTAL_XML_BYTES を使用。
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

/// ## 処理内容
/// ブック関係またはシート関係の XML から relationship ID とターゲットを取得する。
/// ## 引数・戻り値
/// `xml` は関係 XML、戻り値は ID からターゲットへのマップ。
/// ## エラー / 例外発生条件
/// XML が不正または深さ上限を超えた場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 関係テーブルの読取を追加。
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

/// ## 処理内容
/// `workbook.xml` からワークシート名と relationship ID を抽出する。
/// ## 引数・戻り値
/// `xml` はブック XML、戻り値は `(sheet_name, relationship_id)` の一覧。
/// ## エラー / 例外発生条件
/// XML が不正または深さ上限を超えた場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): シート対応表の読取を追加。
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

/// ## 処理内容
/// DrawingML 内の通常 Shape ごとに名前、文字列、アンカーを収集する。
/// ## 引数・戻り値
/// `xml` は描画 XML、`sheet_name` は実際のシート名、戻り値は Shape 一覧。
/// ## エラー / 例外発生条件
/// XML が不正または深さ上限を超えた場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 図形テキスト・名前・アンカー解析を追加。
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

/// ## 処理内容
/// XML の属性をローカル名で文字列化して返す。
/// ## 引数・戻り値
/// XML リーダーと開始要素を受け取り、属性名と値のマップを返す。
/// ## エラー / 例外発生条件
/// 不正な属性または XML 値の場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XML 属性ユーティリティを追加。
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

/// ## 処理内容
/// XML の修飾名からローカル名のバイト列を取得する。
/// ## 引数・戻り値
/// `name` は修飾名、戻り値はコロン以降の要素名。
/// ## エラー / 例外発生条件
/// 入力保持のみで panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): ローカル名抽出を追加。
fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|value| *value == b':').next().unwrap_or(name)
}

/// ## 処理内容
/// OOXML の相対 relationship target を含む ZIP パーツ名へ正規化する。
/// ## 引数・戻り値
/// `source` は参照元パーツ、`target` は relationship target、戻り値は正規化パス。
/// ## エラー / 例外発生条件
/// パス正規化のみで I/O エラーや panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): relationship target 解決を追加。
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

/// ## 処理内容
/// ワークシートの relationships パーツ名を生成する。
/// ## 引数・戻り値
/// `sheet_part` はワークシートパーツ名、戻り値は対応する `.rels` パーツ名。
/// ## エラー / 例外発生条件
/// 文字列変換のみで panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): シート関係パス生成を追加。
pub(super) fn relationship_part(sheet_part: &str) -> String {
    let (parent, name) = sheet_part.rsplit_once('/').unwrap_or(("xl", sheet_part));
    format!("{parent}/_rels/{name}.rels")
}

/// ## 処理内容
/// XML 入れ子の深さが許容上限内であることを確認する。
/// ## 引数・戻り値
/// `depth` は現在の深さ、戻り値は正常時 `Ok(())`。
/// ## エラー / 例外発生条件
/// 最大深さを超えた場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XML 深さ制限を追加。
fn check_depth(depth: usize) -> Result<(), String> {
    // 定数参照: SHAPE_MAX_XML_DEPTH を使用。
    if depth > crate::constants::SHAPE_MAX_XML_DEPTH {
        Err(crate::constants::ERR_SHAPE_LIMIT.to_string())
    } else {
        Ok(())
    }
}

/// ## 処理内容
/// 図形組み立て中の名前・テキストと識別子を保持する。
/// ## 引数・戻り値
/// `new` はグループ名経路を受け、`finish` はシート名とアンカーを受け ShapeText を返す。
/// ## エラー / 例外発生条件
/// 文字列構築のみで panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): Shape 構築用内部状態を追加。
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

    /// ## 処理内容
    /// 文字列ランを順序どおり結合し、Shape 名と1始まりアンカーを抽出できることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。XML 文字列から得た Shape をアサーションで検証する。
    /// ## エラー / 例外発生条件
    /// XML が不正、または抽出値が異なる場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): DrawingML テキストとアンカーのテストを追加。
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

    /// ## 処理内容
    /// グループ内の子図形を個別に返し、アンカー欠損を `None` で保つことを検証する。
    /// ## 引数・戻り値
    /// 引数なし。抽出結果をアサーションで確認する。
    /// ## エラー / 例外発生条件
    /// XML が不正、子図形数・名前・文字が異なる場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): グループ子図形の分離確認を追加。
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

//! # XLSB Shape 抽出器
//!
//! ## 処理内容
//! BIFF12 ブック・ワークシートレコードからリレーション ID を得て、DrawingML 描画パーツを読む。
//! ## 引数・戻り値
//! ブックパス、ワークシート名一覧、中断状態を受け取り、Shape テキスト一覧を返す。
//! ## エラー / 例外発生条件
//! ZIP・レコード・文字列の破損や安全上限超過時にエラーを返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): BrtBundleSh / BrtDrawing を介した XLSB 描画抽出を追加。

use super::ooxml;
use super::ShapeText;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use zip::ZipArchive;

/// ## 処理内容
/// XLSB ブックのシート部品と BrtDrawing を解決し、DrawingML の Shape を返す。
/// ## 引数・戻り値
/// `path` は XLSB ファイル、`sheet_names` は可視状態等で得たシート名、`cancel_flag` は任意の中断状態。
/// ## エラー / 例外発生条件
/// ZIP・BIFF12 データが不正または上限超過時にエラーを返し、中断時は取得済み結果を返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB 部品関係と描画 XML を結合。
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

/// ## 処理内容
/// ZIP 内の BIFF12 パーツをサイズ上限付きで読み込む。
/// ## 引数・戻り値
/// ZIP アーカイブ、パーツ名、累積サイズを受け取り、バイト列を返す。
/// ## エラー / 例外発生条件
/// パーツ欠損、読取失敗、パーツまたは累積サイズ上限超過時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF12 パーツの上限確認を追加。
fn read_binary_part(
    archive: &mut ZipArchive<File>,
    part: &str,
    total_size: &mut u64,
) -> Result<Vec<u8>, String> {
    let mut entry = archive
        .by_name(part)
        .map_err(|_| crate::constants::ERR_SHAPE_READ.to_string())?;
    let size = entry.size();
    // 定数参照: SHAPE_MAX_BINARY_BYTES / SHAPE_MAX_TOTAL_XML_BYTES を使用。
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

/// ## 処理内容
/// BIFF12 レコード列から各シート名と workbook relationship ID を取り出す。
/// ## 引数・戻り値
/// `bytes` は `workbook.bin` の全内容、戻り値はシート部品対応一覧。
/// ## エラー / 例外発生条件
/// レコード長または XLWideString が不正な場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BrtBundleSh の解析を追加。
fn bundle_sheets(bytes: &[u8]) -> Result<Vec<BundleSheet>, String> {
    let mut position = 0;
    let mut sheets = Vec::new();
    while position < bytes.len() {
        let (record_id, payload) = next_record(bytes, &mut position)?;
        if record_id != crate::constants::XLSB_BRT_BUNDLE_SH_RECORD_ID {
            continue;
        }
        // 定数参照: XLSB_BUNDLE_SHEET_FIXED_BYTES を使用。
        let mut cursor = crate::constants::XLSB_BUNDLE_SHEET_FIXED_BYTES;
        let relation_id = read_wide_string(payload, &mut cursor)?;
        let name = read_wide_string(payload, &mut cursor)?;
        if !relation_id.is_empty() && !name.is_empty() {
            sheets.push(BundleSheet { name, relation_id });
        }
    }
    Ok(sheets)
}

/// ## 処理内容
/// BIFF12 worksheet レコードから BrtDrawing の relationship ID を返す。
/// ## 引数・戻り値
/// `bytes` は worksheet binary の全内容、戻り値は任意の描画 relationship ID。
/// ## エラー / 例外発生条件
/// レコード列または描画文字列が不正の場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BrtDrawing の関係 ID 読取を追加。
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

/// ## 処理内容
/// BIFF12 の可変長 record ID と payload 長を読み、レコード本体の範囲を検証する。
/// ## 引数・戻り値
/// `bytes` と可変の `position` を受け取り、record ID と payload slice を返す。
/// ## エラー / 例外発生条件
/// 範囲外、長さ上限超過、または varint 不正でエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF12 レコード境界検証を追加。
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

/// ## 処理内容
/// 7-bit BIFF12 整数を最大5バイトで復号する。
/// ## 引数・戻り値
/// バイト列と可変の読取位置を受け取り、復号した `u32` を返す。
/// ## エラー / 例外発生条件
/// 入力欠損、過剰な継続ビット、数値オーバーフロー時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 可変長整数読取を追加。
fn read_varint(bytes: &[u8], position: &mut usize) -> Result<u32, String> {
    let mut value = 0_u32;
    // 定数参照: XLSB_VARINT_* を使用して BIFF12 continuation encoding を読む。
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

/// ## 処理内容
/// BIFF12 の長さ付き UTF-16LE 文字列を読み取る。
/// ## 引数・戻り値
/// record payload と可変読取位置を受け取り、デコードした文字列を返す。
/// ## エラー / 例外発生条件
/// 長さ不整合または不正 UTF-16 の場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF12 文字列の境界確認を追加。
fn read_wide_string(bytes: &[u8], position: &mut usize) -> Result<String, String> {
    // 定数参照: XLSB_STRING_LENGTH_BYTES と XLSB_UTF16_UNIT_BYTES を使用。
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

/// ## 処理内容
/// BrtBundleSh から読み込んだシート名と relationship ID を保持する。
/// ## 引数・戻り値
/// `name` と `relation_id` がブック中のシート識別情報となる。
/// ## エラー / 例外発生条件
/// データ保持のみでエラーや panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB シート対応モデルを追加。
struct BundleSheet {
    name: String,
    relation_id: String,
}

#[cfg(test)]
mod tests {
    use super::{bundle_sheets, drawing_relation_id};

    /// ## 処理内容
    /// BrtBundleSh と BrtDrawing のレコードから relationship ID とシート名を取り出せることを確認する。
    /// ## 引数・戻り値
    /// 引数なし。小さなバイナリレコード列を生成して解析結果を検証する。
    /// ## エラー / 例外発生条件
    /// 不正なテスト値や解析結果不一致でテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): XLSB 関係レコード解析テストを追加。
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

    /// ## 処理内容
    /// 長さ付き UTF-16LE 文字列を BIFF12 テストレコードへ追加する。
    /// ## 引数・戻り値
    /// `value` は入力文字列、`bytes` は追記先バイト列。戻り値はない。
    /// ## エラー / 例外発生条件
    /// 文字数が `u32` の範囲を超えると panic するが、固定テスト値のみで呼び出す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): テスト用 BIFF12 文字列生成を追加。
    fn append_wide_string(value: &str, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&(value.encode_utf16().count() as u32).to_le_bytes());
        for unit in value.encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
    }

    /// ## 処理内容
    /// `u32` の BIFF12 可変長整数をテスト用バイト列へ追加する。
    /// ## 引数・戻り値
    /// `value` は符号なし整数、`bytes` は追記先バイト列。戻り値はない。
    /// ## エラー / 例外発生条件
    /// 5バイトで表現できない値は入力型で排除される。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): テスト用 varint 生成を追加。
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

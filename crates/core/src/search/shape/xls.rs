//! # BIFF8 / CFB Shape 抽出器
//!
//! ## 処理内容
//! CFB の Workbook/Book ストリーム、シート範囲、TxO と Continue を読み、テキストボックス文字列を抽出する。
//! ## 引数・戻り値
//! ブックパスとシート名一覧を受け取り、Shape テキスト一覧を返す。
//! ## エラー / 例外発生条件
//! CFB・BIFF レコード・文字列の破損、またはサイズ上限超過時にエラーを返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): BIFF TxO と Continue のテキスト抽出を追加。

use super::ShapeText;
use encoding_rs::WINDOWS_1252;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// ## 処理内容
/// CFB 内の BIFF ワークブックからシートごとのテキストオブジェクトを抽出する。
/// ## 引数・戻り値
/// `path` は `.xls` ファイル、`sheet_names` は Reader が返したシート名、`cancel_flag` は任意の中断フラグ、戻り値は Shape 一覧。
/// ## エラー / 例外発生条件
/// CFB ストリーム欠損、上限超過、BIFF 構造不正時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): CFB/BIFF シート走査を実装。
pub(super) fn extract(
    path: &Path,
    sheet_names: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
    if cancel_flag.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
        return Ok(Vec::new());
    }
    // 定数参照: SHAPE_MAX_BINARY_BYTES で CFB Workbook ストリームを制限する。
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

/// ## 処理内容
/// Workbook グローバル領域から BoundSheet8 のシート名とストリーム位置を取得する。
/// ## 引数・戻り値
/// `bytes` は BIFF Workbook ストリーム、戻り値はシート定義一覧。
/// ## エラー / 例外発生条件
/// BIFF レコード境界または文字列が不正な場合にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BoundSheet8 の読取を追加。
fn parse_bound_sheets(bytes: &[u8]) -> Result<Vec<BoundSheet>, String> {
    // 定数参照: XLS_BOUNDSHEET_RECORD_ID と XLS_MAX_BIFF_RECORD_BYTES を使う。
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

/// ## 処理内容
/// Worksheet BIFF 領域の TxO レコードと直後の Continue 群を Shape テキストへ変換する。
/// ## 引数・戻り値
/// `bytes` は単一シート領域、`sheet_name` は所属シート、`shape_number` は一意な表示 ID の連番。
/// ## エラー / 例外発生条件
/// 不正レコードや不完全な TxO テキスト時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): TxO と Continue のシート走査を実装。
fn parse_sheet_objects(
    bytes: &[u8],
    sheet_name: &str,
    shape_number: &mut u32,
) -> Result<Vec<ShapeText>, String> {
    // 定数参照: XLS_TXO_* / XLS_CONTINUE_RECORD_ID でテキストオブジェクトを識別する。
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

/// ## 処理内容
/// TxO に続く Continue 内の XLUnicodeStringNoCch を、指定された文字数まで復号する。
/// ## 引数・戻り値
/// `continues` は Continue payload 一覧、`text_count` は期待文字数、`run_bytes` は書式領域サイズ。
/// ## エラー / 例外発生条件
/// 文字列が欠損、UTF-16 が不正、または書式領域上限超過時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): TxO 文字列と分割 Continue の復号を追加。
fn decode_txo_text(
    continues: &[&[u8]],
    text_count: usize,
    run_bytes: usize,
) -> Result<String, String> {
    // 定数参照: XLS_MAX_BIFF_RECORD_BYTES で書式ラン領域を制限する。
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

/// ## 処理内容
/// BIFF の compressed single-byte または UTF-16LE 文字列を Unicode 文字列へ変換する。
/// ## 引数・戻り値
/// `bytes` は文字データ、`wide` は UTF-16LE 指定、戻り値は UTF-8 文字列。
/// ## エラー / 例外発生条件
/// 奇数バイト UTF-16 または不正なサロゲート列時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF 文字データ復号を追加。
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

/// ## 処理内容
/// BIFF レコードの type、size、data を境界確認して返す。
/// ## 引数・戻り値
/// バイト列と可変オフセットを受け取り、レコード ID とデータ slice を返す。
/// ## エラー / 例外発生条件
/// ヘッダー欠損、レコード長上限超過、または範囲外時にエラーを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF レコード境界確認を追加。
fn next_biff_record<'a>(bytes: &'a [u8], offset: &mut usize) -> Result<(u16, &'a [u8]), String> {
    // 定数参照: XLS_BIFF_RECORD_HEADER_BYTES と XLS_MAX_BIFF_RECORD_BYTES を使う。
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

/// ## 処理内容
/// BoundSheet8 から抽出したシート名と Workbook ストリーム内位置を保持する。
/// ## 引数・戻り値
/// `name` はシート名、`stream_offset` はサブストリームの開始位置。
/// ## エラー / 例外発生条件
/// データ保持のみでエラーや panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): BIFF シート定義モデルを追加。
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

    /// ## 処理内容
    /// Continue に分割された圧縮文字列と UTF-16LE 文字列を正しく復号することを確認する。
    /// ## 引数・戻り値
    /// 引数なし。異なる文字列エンコードの結果をアサーションで検証する。
    /// ## エラー / 例外発生条件
    /// 復号結果が期待値と一致しない場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): TxO の分割文字列テストを追加。
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

    /// ## 処理内容
    /// TxO と Continue の BIFF レコードから Shape テキスト結果を作ることを確認する。
    /// ## 引数・戻り値
    /// 引数なし。レコード列を解析し、文字列とシート名を検証する。
    /// ## エラー / 例外発生条件
    /// レコードが不正または期待値が異なる場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): BIFF TxO の Shape 結果テストを追加。
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

    /// ## 処理内容
    /// CFB Workbook ストリームからシート名と TxO を経由して Shape テキストを抽出する。
    /// ## 引数・戻り値
    /// 引数なし。仮の `.xls` ファイルを作成し、抽出結果を検証する。
    /// ## エラー / 例外発生条件
    /// 一時ファイル作成、書込、抽出、または期待値検証に失敗するとテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): CFB 統合テストを追加。
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

    /// ## 処理内容
    /// BIFF レコードの little-endian ID、長さ、ペイロードをテスト用ストリームに追加する。
    /// ## 引数・戻り値
    /// `record_id` はレコード型、`payload` は本文、`bytes` は追記先。戻り値はない。
    /// ## エラー / 例外発生条件
    /// ペイロードが `u16` 長を超える場合にテストが panic するが、固定テストデータで呼び出す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-28, Codex): BIFF テストレコード生成を追加。
    fn append_record(record_id: u16, payload: &[u8], bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&record_id.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        bytes.extend_from_slice(payload);
    }
}

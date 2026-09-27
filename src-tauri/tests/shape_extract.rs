//! # Shape 抽出統合テスト
//!
//! ## 処理内容
//! 最小 OOXML ブックを生成し、Shape テキストが検索結果へ合流することを検証する。
//! ## 引数・戻り値
//! テストは一時 `.xlsx` を作成して抽出器を呼び、結果をアサートする。
//! ## エラー / 例外発生条件
//! ZIP 作成、ファイル処理、抽出、または期待値が失敗した場合にテストが失敗する。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): OOXML Shape 検索統合テストを追加。

use exlgrep_lib::constants::*;
use exlgrep_lib::models::{MatchType, SearchQuery};
use exlgrep_lib::search::parser::parse_and_search_file;
use exlgrep_lib::search::shape::extract_shapes;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

const TEST_FILE_NAME: &str = "exlgrep-shape-contract.xlsx";
const XLSB_TEST_FILE_NAME: &str = "exlgrep-shape-contract.xlsb";

/// ## 処理内容
/// 検査用テキストを持つシンプルな DrawingML テキストボックスを含む一時 OOXML ブックを生成する。
/// ## 引数・戻り値
/// 引数なし。生成した一時ファイルのパスを返す。
/// ## エラー / 例外発生条件
/// 一時ファイル生成や ZIP 書き込みに失敗するとテストが panic する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 手製 OOXML テストブックの生成を追加。
fn make_fixture() -> PathBuf {
    let parts = [
        (
            "[Content_Types].xml",
            r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/></Types>"#,
        ),
        (
            "_rels/.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
        ),
        (
            "xl/workbook.xml",
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
        ),
        (
            "xl/_rels/workbook.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
        ),
        (
            "xl/worksheets/sheet1.xml",
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>cell needle</t></is></c></row></sheetData></worksheet>"#,
        ),
        (
            "xl/worksheets/_rels/sheet1.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="drawing" Target="../drawings/drawing1.xml"/></Relationships>"#,
        ),
        (
            "xl/drawings/drawing1.xml",
            r#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:twoCellAnchor><xdr:from><xdr:col>1</xdr:col><xdr:row>2</xdr:row></xdr:from><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="4" name="Text Box"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>shape needle</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:twoCellAnchor></xdr:wsDr>"#,
        ),
    ];
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, contents) in parts {
        writer
            .start_file(
                name,
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .expect("zip entry must start");
        writer
            .write_all(contents.as_bytes())
            .expect("zip contents must write");
    }
    let path = std::env::temp_dir().join(TEST_FILE_NAME);
    std::fs::write(path, writer.finish().expect("zip must finish").into_inner())
        .expect("fixture must write");
    std::env::temp_dir().join(TEST_FILE_NAME)
}

/// ## 処理内容
/// BrtBundleSh と BrtDrawing を持つ最小 XLSB パッケージを一時ファイルへ書き込む。
/// ## 引数・戻り値
/// 引数なし。生成した XLSB ファイルの一時パスを返す。
/// ## エラー / 例外発生条件
/// ZIP または一時ファイル出力に失敗するとテストが panic する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB 描画関係テスト用パッケージを追加。
fn make_xlsb_fixture() -> PathBuf {
    let mut workbook_record = Vec::new();
    let mut bundle = vec![0, 0, 0, 0, 1, 0, 0, 0];
    append_wide_string("rId1", &mut bundle);
    append_wide_string("Sheet1", &mut bundle);
    append_varint(XLSB_BRT_BUNDLE_SH_RECORD_ID, &mut workbook_record);
    append_varint(bundle.len() as u32, &mut workbook_record);
    workbook_record.extend(bundle);

    let mut sheet_record = Vec::new();
    let mut drawing_relation = Vec::new();
    append_wide_string("rId2", &mut drawing_relation);
    append_varint(XLSB_BRT_DRAWING_RECORD_ID, &mut sheet_record);
    append_varint(drawing_relation.len() as u32, &mut sheet_record);
    sheet_record.extend(drawing_relation);

    let parts = [
        (
            "xl/workbook.bin",
            workbook_record,
        ),
        (
            "xl/_rels/workbook.bin.rels",
            br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="worksheet" Target="worksheets/sheet1.bin"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/worksheets/sheet1.bin",
            sheet_record,
        ),
        (
            "xl/worksheets/_rels/sheet1.bin.rels",
            br#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId2" Type="drawing" Target="../drawings/drawing1.xml"/></Relationships>"#.to_vec(),
        ),
        (
            "xl/drawings/drawing1.xml",
            br#"<xdr:wsDr xmlns:xdr="x" xmlns:a="a"><xdr:sp><xdr:nvSpPr><xdr:cNvPr id="6" name="Binary Shape"/></xdr:nvSpPr><xdr:txBody><a:p><a:r><a:t>xlsb needle</a:t></a:r></a:p></xdr:txBody></xdr:sp></xdr:wsDr>"#.to_vec(),
        ),
    ];
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, contents) in parts {
        writer
            .start_file(
                name,
                FileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .expect("zip entry must start");
        writer
            .write_all(&contents)
            .expect("zip contents must write");
    }
    let path = std::env::temp_dir().join(XLSB_TEST_FILE_NAME);
    std::fs::write(path, writer.finish().expect("zip must finish").into_inner())
        .expect("fixture must write");
    std::env::temp_dir().join(XLSB_TEST_FILE_NAME)
}

/// ## 処理内容
/// BIFF12 の長さ付き UTF-16LE 文字列をテスト用レコードへ追加する。
/// ## 引数・戻り値
/// `value` は文字列、`bytes` は追記先。戻り値はない。
/// ## エラー / 例外発生条件
/// 固定テスト文字列のみを使い panic しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB テストデータ文字列生成を追加。
fn append_wide_string(value: &str, bytes: &mut Vec<u8>) {
    bytes.extend_from_slice(&(value.encode_utf16().count() as u32).to_le_bytes());
    for unit in value.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
}

/// ## 処理内容
/// BIFF12 の `u32` 可変長整数をテスト用レコードへ追加する。
/// ## 引数・戻り値
/// `value` は整数、`bytes` は追記先。戻り値はない。
/// ## エラー / 例外発生条件
/// `u32` の範囲内の固定テスト値のみを受け取り panic しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB テストデータ varint 生成を追加。
fn append_varint(mut value: u32, bytes: &mut Vec<u8>) {
    while value >= u32::from(XLSB_VARINT_CONTINUATION_MASK) {
        bytes.push((value as u8 & XLSB_VARINT_VALUE_MASK) | XLSB_VARINT_CONTINUATION_MASK);
        value >>= XLSB_VARINT_SHIFT;
    }
    bytes.push(value as u8);
}

/// ## 処理内容
/// OOXML ブック内の Shape をシート・名前・文字・アンカー付きで抽出できることを確認する。
/// ## 引数・戻り値
/// 引数なし。期待する抽出値が異なる場合にテストが失敗する。
/// ## エラー / 例外発生条件
/// 抽出に失敗した場合、または一時ファイルの削除に失敗した場合にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): Shape 抽出の形式統合テストを追加。
#[test]
fn extracts_text_shape_with_sheet_and_anchor() {
    let path = make_fixture();
    let shapes = extract_shapes(&path, &["Sheet1".to_string()], None)
        .expect("shape extraction must succeed");
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].sheet_name, "Sheet1");
    assert_eq!(shapes[0].shape_name, "Text Box");
    assert_eq!(shapes[0].text, "shape needle");
    assert_eq!(shapes[0].anchor, Some((3, 2)));

    let query = SearchQuery {
        keyword: "needle".to_string(),
        target_dir: path.to_string_lossy().to_string(),
        match_case: false,
        use_regex: false,
        include_formula: false,
        include_comment: true,
        include_shape: true,
        include_hidden: false,
        extensions: vec![".xlsx".to_string()],
    };
    let results = parse_and_search_file(&path, &query, None, None)
        .expect("search must merge cell and shape results");
    assert_eq!(results.len(), 2);
    assert!(results
        .iter()
        .any(|item| item.match_type == MatchType::CellValue));
    let shape = results
        .iter()
        .find(|item| item.match_type == MatchType::Shape)
        .expect("shape match must exist");
    assert_eq!(shape.shape_name.as_deref(), Some("Text Box"));
    assert_eq!(shape.formula, None);
    assert_eq!(shape.cell_address, "B3");

    let query_without_shapes = SearchQuery {
        include_shape: false,
        ..query
    };
    let cell_only = parse_and_search_file(&path, &query_without_shapes, None, None)
        .expect("cell search must work with Shape search disabled");
    assert_eq!(cell_only.len(), 1);
    assert!(cell_only
        .iter()
        .all(|item| item.match_type != MatchType::Shape));

    std::fs::remove_file(path).expect("fixture must be removed");
}

/// ## 処理内容
/// XLSB の BrtBundleSh → worksheet relationship → BrtDrawing → DrawingML 関係を辿れることを検証する。
/// ## 引数・戻り値
/// 引数なし。Shape 抽出結果をアサーションで確認する。
/// ## エラー / 例外発生条件
/// ZIP 読取または期待したシート・名前・文字列が得られない場合にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): XLSB 抽出統合テストを追加。
#[test]
fn extracts_text_shape_from_xlsb_drawing_relationship() {
    let path = make_xlsb_fixture();
    let shapes = extract_shapes(&path, &["Sheet1".to_string()], None)
        .expect("XLSB shape extraction must succeed");
    std::fs::remove_file(path).expect("fixture must be removed");

    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0].sheet_name, "Sheet1");
    assert_eq!(shapes[0].shape_name, "Binary Shape");
    assert_eq!(shapes[0].text, "xlsb needle");
}

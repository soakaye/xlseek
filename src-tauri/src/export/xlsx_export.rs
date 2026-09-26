//! # Excelエクスポートモジュール (export/xlsx_export.rs)
//!
//! ## 処理内容
//! 検索結果アイテムの一覧を整形されたExcel (.xlsx) ブックに出力する。
//! ヘッダーの装飾（背景色・フォント色・罫線）、各列の幅調整、オートフィルターの適用、
//! および英語エラーコードによるエラーハンドリングを提供する。
//! 憲章原則I（日本語出力）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（色・列幅・シート名・メッセージ）、4要素ヘッダコメント付与。

use crate::models::{MatchType, SearchMatch};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};
use std::collections::BTreeMap;

/// ## 処理内容
/// 検索結果アイテムのスライスを整形されたExcel (.xlsx) ファイルに出力する。
/// 列幅の自動設定、ヘッダー書式設定、オートフィルターの設定を行う。
///
/// ## 引数
/// - `path`: `&str` - 出力先Excelファイルのファイルパス
/// - `items`: `&[SearchMatch]` - エクスポート対象の検索一致アイテムスライス
/// - `language`: `&str` - `ja` または `en` の出力言語
/// - `catalogs`: `&BTreeMap<String, BTreeMap<String, String>>` - プラグインから取得した翻訳カタログ
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、言語・翻訳・ワークブック構築・保存失敗時は英語エラーコード
///
/// ## エラー / 例外発生条件
/// - 対応外言語、必要な翻訳の欠落、ワークシート追加、セル書き込み、またはファイル保存時に `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, Codex): 要求言語のカタログからシート名、列見出し、一致種別を取得。
pub fn export_to_xlsx(
    path: &str,
    items: &[SearchMatch],
    language: &str,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 定数参照: constants::EXPORT_SHEET_NAME_KEY を使用。
    let sheet_name = crate::i18n::resolve_catalog_text(
        catalogs,
        language,
        crate::constants::EXPORT_SHEET_NAME_KEY,
    )
    .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())?;
    worksheet.set_name(&sheet_name).map_err(|e| {
        // 定数参照: crate::constants::ERR_XLSX_WORKSHEET を使用
        format!("{}: {}", crate::constants::ERR_XLSX_WORKSHEET, e)
    })?;

    // スタイル定義
    // 定数参照: crate::constants::XLSX_HEADER_BG_COLOR, XLSX_HEADER_FG_COLOR を使用
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(crate::constants::XLSX_HEADER_BG_COLOR))
        .set_font_color(Color::RGB(crate::constants::XLSX_HEADER_FG_COLOR))
        .set_border(FormatBorder::Thin);

    let cell_format = Format::new().set_border(FormatBorder::Thin);

    let id_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    // 定数参照: constants::EXPORT_HEADER_KEYS と XLSX_COLUMN_WIDTHS を使用。
    let headers = crate::constants::EXPORT_HEADER_KEYS
        .iter()
        .map(|key| {
            crate::i18n::resolve_catalog_text(catalogs, language, key)
                .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (col_idx, (header, width)) in headers
        .iter()
        .zip(crate::constants::XLSX_COLUMN_WIDTHS.iter())
        .enumerate()
    {
        worksheet
            .write_string_with_format(0, col_idx as u16, header, &header_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .set_column_width(col_idx as u16, *width)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
    }

    // データ行書き込み
    for (row_idx, item) in items.iter().enumerate() {
        let r = (row_idx + 1) as u32;
        // 定数参照: constants::EXPORT_MATCH_*_KEY を使用。
        let match_key = match item.match_type {
            MatchType::CellValue => crate::constants::EXPORT_MATCH_VALUE_KEY,
            MatchType::Formula => crate::constants::EXPORT_MATCH_FORMULA_KEY,
            MatchType::Comment => crate::constants::EXPORT_MATCH_COMMENT_KEY,
            MatchType::HiddenSheet => crate::constants::EXPORT_MATCH_HIDDEN_SHEET_KEY,
        };
        let match_type_str = crate::i18n::resolve_catalog_text(catalogs, language, match_key)
            .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())?;

        worksheet
            .write_number_with_format(r, 0, item.id as f64, &id_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 1, &item.file_name, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 2, &item.full_path, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 3, &item.sheet_name, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 4, &item.cell_address, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 5, &match_type_str, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 6, &item.full_content, &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
        worksheet
            .write_string_with_format(r, 7, item.formula.as_deref().unwrap_or(""), &cell_format)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
                format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
            })?;
    }

    // オートフィルター有効化（データが存在する場合）
    if !items.is_empty() {
        let last_row = items.len() as u32;
        worksheet.autofilter(0, 0, last_row, 7).map_err(|e| {
            // 定数参照: crate::constants::ERR_XLSX_WRITE を使用
            format!("{}: {}", crate::constants::ERR_XLSX_WRITE, e)
        })?;
    }

    workbook.save(path).map_err(|e| {
        // 定数参照: crate::constants::ERR_XLSX_SAVE を使用
        format!("{}: {}", crate::constants::ERR_XLSX_SAVE, e)
    })?;
    Ok(())
}

#[cfg(test)]
mod localization_tests {
    use calamine::Reader;
    use std::collections::BTreeMap;

    const TEST_OUTPUT_PREFIX: &str = "exlgrep-i18n-test";

    /// ## 処理内容
    /// Excel の列見出しが指定されたカタログ言語になることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。日英の出力ファイルを読み、2列目の見出しを比較する。
    /// ## エラー
    /// ファイル操作、ブック解析、または見出し不一致時にテストが失敗する。
    /// ## 変更履歴
    /// - v1.2.0 (2026-09-26, Codex): カタログによるExcel見出し選択テストを追加。
    #[test]
    fn uses_requested_catalog_for_headers() {
        let mut english = BTreeMap::from([(
            crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
            "Some text could not be translated.".to_string(),
        )]);
        for key in crate::constants::EXPORT_HEADER_KEYS {
            english.insert(key.to_string(), key.to_string());
        }
        english.insert(
            "export.header.fileName".to_string(),
            "File name".to_string(),
        );
        english.insert(
            crate::constants::EXPORT_SHEET_NAME_KEY.to_string(),
            "Search Results".to_string(),
        );
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        japanese.insert(
            "export.header.fileName".to_string(),
            "ファイル名".to_string(),
        );
        japanese.insert(
            crate::constants::EXPORT_SHEET_NAME_KEY.to_string(),
            "検索結果".to_string(),
        );
        let catalogs = BTreeMap::from([
            (crate::constants::LANGUAGE_EN.to_string(), english),
            (crate::constants::LANGUAGE_JA.to_string(), japanese),
        ]);

        for (language, expected) in [
            (crate::constants::LANGUAGE_EN, "File name"),
            (crate::constants::LANGUAGE_JA, "ファイル名"),
        ] {
            // 定数参照: TEST_OUTPUT_PREFIX を使用
            let path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}-{language}.xlsx"));
            super::export_to_xlsx(path.to_str().unwrap(), &[], language, &catalogs).unwrap();
            let mut workbook = calamine::open_workbook_auto(&path).unwrap();
            let sheet = workbook.worksheet_range_at(0).unwrap().unwrap();
            assert_eq!(
                sheet.get_value((0, 1)).map(ToString::to_string).as_deref(),
                Some(expected)
            );
            std::fs::remove_file(path).unwrap();
        }
    }

    /// ## 処理内容
    /// 未対応言語の Excel 出力を、ブック作成前に拒否する。
    /// ## 引数・戻り値
    /// 引数なし。アサーションのみを実行する。
    /// ## エラー
    /// 想定外の出力結果でテストが失敗する。
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): 不正な出力言語の検証を追加。
    #[test]
    fn rejects_unsupported_language() {
        assert_eq!(
            super::export_to_xlsx("", &[], "fr", &BTreeMap::new()),
            Err(crate::constants::ERR_INVALID_LANGUAGE.to_string())
        );
    }
}

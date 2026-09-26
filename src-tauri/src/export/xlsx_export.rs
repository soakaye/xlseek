//! # Excelエクスポートモジュール (export/xlsx_export.rs)
//!
//! ## 処理内容
//! 検索結果アイテムの一覧を整形されたExcel (.xlsx) ブックに出力する。
//! ヘッダーの装飾（背景色・フォント色・罫線）、各列の幅調整、オートフィルターの適用、
//! および日本語メッセージによるエラーハンドリングを提供する。
//! 憲章原則I（日本語出力）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（色・列幅・シート名・メッセージ）、4要素ヘッダコメント付与。

use crate::models::{MatchType, SearchMatch};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};

/// ## 処理内容
/// 検索結果アイテムのスライスを整形されたExcel (.xlsx) ファイルに出力する。
/// 列幅の自動設定、ヘッダー書式設定、オートフィルターの設定を行う。
///
/// ## 引数
/// - `path`: `&str` - 出力先Excelファイルのファイルパス
/// - `items`: `&[SearchMatch]` - エクスポート対象の検索一致アイテムスライス
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、ワークブック構築や保存失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// - ワークシート追加、セル書き込み、またはファイル保存時にエラーが発生した場合 `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（色、シート名、列定義、メッセージ）。
pub fn export_to_xlsx(path: &str, items: &[SearchMatch], language: &str) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 定数参照: crate::constants::EXPORT_DEFAULT_SHEET_NAME を使用
    worksheet
        .set_name(if language == crate::constants::LANGUAGE_JA {
            crate::constants::EXPORT_DEFAULT_SHEET_NAME
        } else {
            crate::constants::EXPORT_DEFAULT_SHEET_NAME_EN
        })
        .map_err(|e| {
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

    // ヘッダー書き込み
    // 定数参照: crate::constants::XLSX_HEADERS_WITH_WIDTH を使用
    let headers = if language == crate::constants::LANGUAGE_JA {
        &crate::constants::XLSX_HEADERS_WITH_WIDTH
    } else {
        &crate::constants::XLSX_HEADERS_WITH_WIDTH_EN
    };
    for (col_idx, (header, width)) in headers.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_idx as u16, *header, &header_format)
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
        // 定数参照: crate::constants::LABEL_MATCH_* を使用
        let match_type_str = match (item.match_type, language) {
            (MatchType::CellValue, crate::constants::LANGUAGE_EN) => {
                crate::constants::LABEL_MATCH_CELL_VALUE_EN
            }
            (MatchType::Formula, crate::constants::LANGUAGE_EN) => {
                crate::constants::LABEL_MATCH_FORMULA_EN
            }
            (MatchType::Comment, crate::constants::LANGUAGE_EN) => {
                crate::constants::LABEL_MATCH_COMMENT_EN
            }
            (MatchType::HiddenSheet, crate::constants::LANGUAGE_EN) => {
                crate::constants::LABEL_MATCH_HIDDEN_SHEET_EN
            }
            (MatchType::CellValue, _) => crate::constants::LABEL_MATCH_CELL_VALUE,
            (MatchType::Formula, _) => crate::constants::LABEL_MATCH_FORMULA,
            (MatchType::Comment, _) => crate::constants::LABEL_MATCH_COMMENT,
            (MatchType::HiddenSheet, _) => crate::constants::LABEL_MATCH_HIDDEN_SHEET,
        };

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
            .write_string_with_format(r, 5, match_type_str, &cell_format)
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
            super::export_to_xlsx("", &[], "fr"),
            Err(crate::constants::ERR_INVALID_LANGUAGE.to_string())
        );
    }
}

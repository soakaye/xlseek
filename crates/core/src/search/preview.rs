//! # セル周辺プレビュー抽出モジュール (search/preview.rs)
//!
//! ## 処理内容
//! 指定されたExcelファイルの特定シート・特定セル番地を中心として、
//! 前後の行・列（バウンディングボックス）のセル値および数式を高速抽出してプレビューグリッドを生成する。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）、原則V（堅牢なエラーハンドリング）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、4要素ヘッダコメント追加、異常系単体テスト追加。

use crate::models::{CellPreviewData, CellValueInfo, PreviewColumn, PreviewRow};
use crate::search::parser::col_to_name;
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use std::collections::HashMap;
use std::path::Path;

/// ## 処理内容
/// 指定セル周辺（定数で定義された前後行・前後列の範囲）のワークシートデータを抽出し、
/// 列ヘッダー、行データ、およびブック内の全シート名を含むプレビュー構造体を生成する。
///
/// ## 引数
/// - `file_path`: `P` - 対象Excelファイルのパス
/// - `sheet_name`: `&str` - プレビュー対象のシート名
/// - `target_row_1based`: `u32` - 1始まりの対象セル行番号
/// - `target_col_1based`: `u32` - 1始まりの対象セル列番号
///
/// ## 戻り値
/// - `Result<CellPreviewData, String>`: 成功時はプレビューグリッドデータ、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// - 指定ファイルが開けない場合、または指定シートが存在しない場合に `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化および日本語エラーメッセージ化。
pub fn extract_cell_preview<P: AsRef<Path>>(
    file_path: P,
    sheet_name: &str,
    target_row_1based: u32,
    target_col_1based: u32,
) -> Result<CellPreviewData, String> {
    let path_ref = file_path.as_ref();
    let mut workbook: Sheets<_> = open_workbook_auto(path_ref).map_err(|e| {
        // 定数参照: crate::constants::ERR_WORKBOOK_OPEN を使用
        format!(
            "{}: {} ({})",
            crate::constants::ERR_WORKBOOK_OPEN,
            path_ref.display(),
            e
        )
    })?;

    let sheets_in_workbook = workbook.sheet_names().to_vec();

    let target_row_0based = target_row_1based.saturating_sub(1);
    let target_col_0based = target_col_1based.saturating_sub(1);

    // 定数参照: crate::constants::PREVIEW_ROW_RADIUS を使用
    let min_row = target_row_0based.saturating_sub(crate::constants::PREVIEW_ROW_RADIUS);
    let max_row = target_row_0based + crate::constants::PREVIEW_ROW_RADIUS;

    // 定数参照: crate::constants::PREVIEW_COL_RADIUS を使用
    let min_col = target_col_0based.saturating_sub(crate::constants::PREVIEW_COL_RADIUS);
    let max_col = target_col_0based + crate::constants::PREVIEW_COL_RADIUS;

    // 列ヘッダー定義
    let mut columns = Vec::new();
    for col_idx in min_col..=max_col {
        let label = col_to_name(col_idx);
        columns.push(PreviewColumn {
            key: label.clone(),
            label,
        });
    }

    // ワークシートのセル値読み込み
    let range = workbook.worksheet_range(sheet_name).map_err(|e| {
        // 定数参照: crate::constants::ERR_SHEET_NOT_FOUND を使用
        format!(
            "{}: '{}' ({})",
            crate::constants::ERR_SHEET_NOT_FOUND,
            sheet_name,
            e
        )
    })?;

    // 数式マップの読み込み (任意)
    let formula_map = if let Ok(f_range) = workbook.worksheet_formula(sheet_name) {
        let mut map: HashMap<(u32, u32), String> = HashMap::new();
        for (r, row) in f_range.rows().enumerate() {
            for (c, formula) in row.iter().enumerate() {
                if !formula.is_empty() {
                    map.insert((r as u32, c as u32), formula.clone());
                }
            }
        }
        Some(map)
    } else {
        None
    };

    let mut rows = Vec::new();

    for r in min_row..=max_row {
        let row_number = r + 1;
        let mut cells = HashMap::new();

        for c in min_col..=max_col {
            let col_name = col_to_name(c);
            let is_target = r == target_row_0based && c == target_col_0based;

            let value_str = if let Some(cell_data) = range.get((r as usize, c as usize)) {
                match cell_data {
                    Data::Empty => String::new(),
                    Data::String(s) => s.clone(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::DateTime(d) => d.to_string(),
                    Data::DateTimeIso(d) => d.clone(),
                    Data::DurationIso(d) => d.clone(),
                    Data::Error(e) => format!("{:?}", e),
                }
            } else {
                String::new()
            };

            let formula = formula_map.as_ref().and_then(|m| m.get(&(r, c)).cloned());

            cells.insert(
                col_name,
                CellValueInfo {
                    value: value_str,
                    is_target,
                    formula,
                },
            );
        }

        rows.push(PreviewRow { row_number, cells });
    }

    Ok(CellPreviewData {
        target_row: target_row_1based,
        target_col: target_col_1based,
        columns,
        rows,
        sheets_in_workbook,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// ## 処理内容
    /// 存在しないファイルパスに対してextract_cell_previewを呼び出した際、
    /// パニックせず日本語エラーメッセージを含むErrが安全に返却されることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版作成（憲章原則V準拠テスト）
    #[test]
    fn test_extract_cell_preview_nonexistent_file() {
        let invalid_path = PathBuf::from("nonexistent_test_workbook_12345.xlsx");
        let result = extract_cell_preview(&invalid_path, "Sheet1", 1, 1);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains(crate::constants::ERR_WORKBOOK_OPEN));
    }
}

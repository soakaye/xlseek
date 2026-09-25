use crate::models::{CellPreviewData, CellValueInfo, PreviewColumn, PreviewRow};
use crate::search::parser::col_to_name;
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use std::collections::HashMap;
use std::path::Path;

/// 指定セル周辺 (前後3行・前後2列) のワークシートデータを高速抽出
pub fn extract_cell_preview<P: AsRef<Path>>(
    file_path: P,
    sheet_name: &str,
    target_row_1based: u32,
    target_col_1based: u32,
) -> Result<CellPreviewData, String> {
    let path_ref = file_path.as_ref();
    let mut workbook: Sheets<_> = open_workbook_auto(path_ref)
        .map_err(|e| format!("ワークブックを開けませんでした {}: {}", path_ref.display(), e))?;

    let sheets_in_workbook = workbook.sheet_names().to_vec();

    let target_row_0based = target_row_1based.saturating_sub(1);
    let target_col_0based = target_col_1based.saturating_sub(1);

    // バウンディングボックスの算出 (前後3行, 前後2列)
    let min_row = target_row_0based.saturating_sub(3);
    let max_row = target_row_0based + 3;
    let min_col = target_col_0based.saturating_sub(2);
    let max_col = target_col_0based + 2;

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
    let range = workbook
        .worksheet_range(sheet_name)
        .map_err(|e| format!("シート '{}' の読み込みに失敗しました: {}", sheet_name, e))?;

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

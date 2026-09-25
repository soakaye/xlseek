use crate::models::{MatchType, SearchMatch};
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook};

/// 検索結果を整形されたExcel (.xlsx) ファイルに出力する
pub fn export_to_xlsx(path: &str, items: &[SearchMatch]) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("検索結果").map_err(|e| format!("シート名設定エラー: {}", e))?;

    // スタイル定義
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x000F766E)) // Teal 700
        .set_font_color(Color::RGB(0x00FFFFFF))
        .set_border(FormatBorder::Thin);

    let cell_format = Format::new()
        .set_border(FormatBorder::Thin);

    let id_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    // ヘッダー書き込み
    let headers = [
        ("ID", 8.0),
        ("ファイル名", 25.0),
        ("フルパス", 40.0),
        ("シート名", 20.0),
        ("セル位置", 12.0),
        ("一致種別", 14.0),
        ("一致内容", 45.0),
        ("数式", 30.0),
    ];

    for (col_idx, (header, width)) in headers.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_idx as u16, *header, &header_format)
            .map_err(|e| format!("ヘッダー書き込みエラー: {}", e))?;
        worksheet.set_column_width(col_idx as u16, *width)
            .map_err(|e| format!("列幅設定エラー: {}", e))?;
    }

    // データ行書き込み
    for (row_idx, item) in items.iter().enumerate() {
        let r = (row_idx + 1) as u32;
        let match_type_str = match item.match_type {
            MatchType::CellValue => "値",
            MatchType::Formula => "数式",
            MatchType::Comment => "コメント",
            MatchType::HiddenSheet => "非表示シート",
        };

        worksheet.write_number_with_format(r, 0, item.id as f64, &id_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 1, &item.file_name, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 2, &item.full_path, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 3, &item.sheet_name, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 4, &item.cell_address, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 5, match_type_str, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 6, &item.full_content, &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
        worksheet.write_string_with_format(r, 7, item.formula.as_deref().unwrap_or(""), &cell_format)
            .map_err(|e| format!("データ書き込みエラー: {}", e))?;
    }

    // オートフィルター有効化（データが存在する場合）
    if !items.is_empty() {
        let last_row = items.len() as u32;
        worksheet.autofilter(0, 0, last_row, 7)
            .map_err(|e| format!("オートフィルタ設定エラー: {}", e))?;
    }

    workbook.save(path).map_err(|e| format!("XLSX保存エラー: {}", e))?;
    Ok(())
}

use std::fs::File;
use std::io::Write;
use crate::models::SearchMatch;

/// 検索結果をBOM付きUTF-8のCSVファイルに出力する
pub fn export_to_csv(path: &str, items: &[SearchMatch]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("CSVファイルの作成に失敗しました: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);

    // Excelで日本語が文字化けしないようにUTF-8 BOMを付与
    writer.write_all(b"\xEF\xBB\xBF").map_err(|e| format!("BOMの書き込みに失敗しました: {}", e))?;

    let mut csv_writer = csv::Writer::from_writer(writer);

    // ヘッダー行
    csv_writer.write_record(&[
        "ID",
        "ファイル名",
        "フルパス",
        "シート名",
        "セル位置",
        "一致種別",
        "一致内容",
        "数式",
    ]).map_err(|e| format!("ヘッダー書き込みエラー: {}", e))?;

    // データ行
    for item in items {
        let match_type_str = match item.match_type {
            crate::models::MatchType::CellValue => "値",
            crate::models::MatchType::Formula => "数式",
            crate::models::MatchType::Comment => "コメント",
            crate::models::MatchType::HiddenSheet => "非表示シート",
        };

        csv_writer.write_record(&[
            item.id.to_string(),
            item.file_name.clone(),
            item.full_path.clone(),
            item.sheet_name.clone(),
            item.cell_address.clone(),
            match_type_str.to_string(),
            item.full_content.clone(),
            item.formula.clone().unwrap_or_default(),
        ]).map_err(|e| format!("データ行書き込みエラー: {}", e))?;
    }

    csv_writer.flush().map_err(|e| format!("フラッシュに失敗しました: {}", e))?;
    Ok(())
}

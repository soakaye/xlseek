//! # CSVエクスポートモジュール (export/csv_export.rs)
//!
//! ## 処理内容
//! 検索一致アイテムの一覧をBOM付きUTF-8のCSVファイルとして書き出す。
//! Excelでの日本語文字化け防止のためBOMを付与し、列ヘッダーおよびデータ行を整形出力する。
//! 憲章原則I（日本語出力）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、Clippy借用指摘修正、4要素ヘッダコメント付与。

use crate::models::{MatchType, SearchMatch};
use std::fs::File;
use std::io::Write;

/// ## 処理内容
/// 検索結果アイテムのスライスをBOM付きUTF-8のCSVファイルに出力する。
///
/// ## 引数
/// - `path`: `&str` - 出力先CSVファイルのファイルパス
/// - `items`: `&[SearchMatch]` - エクスポート対象の検索一致アイテムスライス
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、ファイル作成や書き込み失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// - ファイル作成、BOM書き込み、レコード書き込み、またはフラッシュに失敗した場合に `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化およびClippy対応（`write_record([ ... ])`）。
pub fn export_to_csv(path: &str, items: &[SearchMatch]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    let mut writer = std::io::BufWriter::new(file);

    // Excelで日本語が文字化けしないようにUTF-8 BOMを付与
    writer.write_all(b"\xEF\xBB\xBF").map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_HEADER_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
    })?;

    let mut csv_writer = csv::Writer::from_writer(writer);

    // ヘッダー行書き込み (Clippy: needless_borrows_for_generic_args 回避のため配列を直接渡す)
    // 定数参照: crate::constants::CSV_EXPORT_HEADERS を使用
    csv_writer
        .write_record(crate::constants::CSV_EXPORT_HEADERS)
        .map_err(|e| {
            // 定数参照: crate::constants::ERR_CSV_HEADER_WRITE を使用
            format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
        })?;

    // データ行
    for item in items {
        // 定数参照: crate::constants::LABEL_MATCH_* を使用
        let match_type_str = match item.match_type {
            MatchType::CellValue => crate::constants::LABEL_MATCH_CELL_VALUE,
            MatchType::Formula => crate::constants::LABEL_MATCH_FORMULA,
            MatchType::Comment => crate::constants::LABEL_MATCH_COMMENT,
            MatchType::HiddenSheet => crate::constants::LABEL_MATCH_HIDDEN_SHEET,
        };

        // Clippy: needless_borrows_for_generic_args 回避のため配列を直接渡す
        csv_writer
            .write_record([
                item.id.to_string(),
                item.file_name.clone(),
                item.full_path.clone(),
                item.sheet_name.clone(),
                item.cell_address.clone(),
                match_type_str.to_string(),
                item.full_content.clone(),
                item.formula.clone().unwrap_or_default(),
            ])
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
                format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
            })?;
    }

    csv_writer.flush().map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    Ok(())
}

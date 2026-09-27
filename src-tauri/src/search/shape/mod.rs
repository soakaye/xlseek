//! # Excel Shape 抽出モジュール
//!
//! ## 処理内容
//! OOXML 描画パーツから Shape の名前、テキスト、シート名、アンカーを抽出する。
//! ## 引数・戻り値
//! 各形式アダプターはファイルパスとキャンセル状態を受け取り、共通 Shape テキスト一覧を返す。
//! ## エラー / 例外発生条件
//! 未対応形式、破損 ZIP/XML、上限超過は `Result` のエラーとして返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): Shape 抽出共通型と OOXML アダプターを追加。

mod ooxml;
mod xls;
mod xlsb;

use std::path::Path;
use std::sync::atomic::AtomicBool;

/// ## 処理内容
/// 1つの DrawingML Shape から検索に必要な情報を保持する。
/// ## 引数・戻り値
/// フィールドは図形名、全文、所属シート、任意の1始まりアンカーを格納する。
/// ## エラー / 例外発生条件
/// 値保持のみでエラーや panic は発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 共通 Shape テキスト型を追加。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeText {
    pub shape_id: String,
    pub shape_name: String,
    pub text: String,
    pub sheet_name: String,
    pub anchor: Option<(u32, u32)>,
}

/// ## 処理内容
/// ファイル拡張子に応じた Shape 抽出アダプターを実行する。
/// ## 引数・戻り値
/// `path` は Excel ブック、`cancel_flag` は任意の中断状態、戻り値は Shape 一覧。
/// ## エラー / 例外発生条件
/// ZIP/XML の破損や未対応形式ではエラー文字列を返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 共通 Shape 抽出入口を追加。
pub fn extract_shapes(
    path: &Path,
    sheet_names: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
    match path.extension().and_then(|value| value.to_str()) {
        Some("xlsx" | "xlsm") => ooxml::extract(path, cancel_flag),
        Some("xlsb") => xlsb::extract(path, sheet_names, cancel_flag),
        Some("xls") => xls::extract(path, sheet_names, cancel_flag),
        _ => Ok(Vec::new()),
    }
}

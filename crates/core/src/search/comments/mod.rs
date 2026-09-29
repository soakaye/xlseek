//! ## 処理内容
//! Excel形式ごとのセルコメント抽出と共通コメントモデルを公開する。
//! ## 引数・戻り値
//! 形式別抽出器へブックパスとシート名を渡し、1始まり座標のコメント一覧を返す。
//! ## エラー
//! 参照部品の欠落、外部関係、不正XMLまたは境界違反をErrで返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): コメント抽出モジュールを追加。

mod ooxml;

/// ## 処理内容
/// セルに属するコメント本文とExcel上の座標を表す。
/// ## 引数・戻り値
/// sheet_name・textはString、row・colは1始まりのu32。
/// ## エラー
/// 値保持のみでエラーは発生しない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 共通コメントモデルを追加。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentText {
    pub sheet_name: String,
    pub row: u32,
    pub col: u32,
    pub text: String,
}

/// ## 処理内容
/// OOXMLブックから従来のセルメモを抽出する。
/// ## 引数・戻り値
/// ブックパスを受け、`Result<Vec<CommentText>, String>` を返す。
/// ## エラー
/// ZIP/XML/relationship参照の破損や上限超過はErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): OOXMLコメント抽出入口を追加。
pub fn extract(path: &std::path::Path) -> Result<Vec<CommentText>, String> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| {
            format!(
                "{}{}",
                crate::constants::CLI_EXTENSION_PREFIX,
                value.to_ascii_lowercase()
            )
        });
    match extension.as_deref() {
        Some(crate::constants::EXT_XLSX | crate::constants::EXT_XLSM) => ooxml::extract(path),
        _ => Ok(Vec::new()),
    }
}

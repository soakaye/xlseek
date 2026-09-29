//! # Shape 検索契約テスト
//!
//! ## 処理内容
//! Shape 検索クエリの省略時既定値と Shape 結果モデルのシリアライズ契約を検証する。
//! ## 引数・戻り値
//! 引数なし。Serde の変換結果をアサーションで検証する。
//! ## エラー / 例外発生条件
//! JSON 変換または期待値が一致しない場合にテストが失敗する。
//! ## 変更履歴
//! - v1.0.0 (2026-09-28, Codex): Shape 検索契約テストを追加。

use exlgrep_core::models::{MatchType, SearchMatch, SearchQuery};

/// ## 処理内容
/// include_shape を省略した検索要求が有効として解釈され、Shape 一致種別が安定して直列化されることを確認する。
/// ## 引数・戻り値
/// 引数なし。期待する値が異なる場合にテストが失敗する。
/// ## エラー / 例外発生条件
/// JSON デシリアライズが失敗した場合にテストが失敗する。
/// ## 変更履歴
/// - v1.0.0 (2026-09-28, Codex): 既定値と Shape 種別の検証を追加。
#[test]
fn defaults_shape_search_to_enabled_and_serializes_shape_type() {
    let query: SearchQuery =
        serde_json::from_str(r#"{"keyword":"needle","target_dir":".","include_formula":false}"#)
            .expect("query must deserialize");

    assert!(query.include_shape);
    assert!(!query.include_formula);
    assert_eq!(
        serde_json::to_string(&MatchType::Shape).expect("Shape type must serialize"),
        "\"Shape\""
    );

    let shape_match = SearchMatch {
        id: 0,
        file_name: "book.xlsx".to_string(),
        full_path: "book.xlsx".to_string(),
        sheet_name: "Sheet1".to_string(),
        cell_address: String::new(),
        row_index: 0,
        col_index: 0,
        col_name: String::new(),
        match_type: MatchType::Shape,
        shape_name: Some("TextBox 1".to_string()),
        sheet_hidden: true,
        snippet: "needle".to_string(),
        full_content: "needle".to_string(),
        formula: None,
        sheets_in_workbook: vec!["Sheet1".to_string()],
    };
    let serialized = serde_json::to_value(shape_match).expect("Shape result must serialize");
    assert_eq!(serialized["shape_name"], "TextBox 1");
    assert_eq!(serialized["sheet_hidden"], true);
    assert_eq!(serialized["cell_address"], "");
    assert_eq!(serialized["row_index"], 0);
    assert_eq!(serialized["col_index"], 0);
}

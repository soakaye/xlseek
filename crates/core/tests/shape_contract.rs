//! # Shape Search Contract Test
//!
//! ## Description
//! Verifies default query values and serialization contracts for shape search results.
//! ## Arguments / Returns
//! No arguments. Verifies Serde serialization results via assertions.
//! ## Errors / Exceptions
//! Fails if JSON serialization or deserialization does not match expected output.

use xlseek_core::models::{MatchType, SearchMatch, SearchQuery};

/// ## Description
/// Verifies that omitting include_shape defaults to true and that Shape MatchType serializes stably.
/// ## Arguments / Returns
/// No arguments. Fails if actual results do not match assertions.
/// ## Errors / Exceptions
/// Fails if JSON deserialization fails.
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

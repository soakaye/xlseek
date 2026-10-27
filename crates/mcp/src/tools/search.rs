//! # Tool Handler: xlseek_search (crates/mcp/src/tools/search.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements high-speed cross-document search across Excel and Word documents
//! using the underlying `xlseek-core` search engine.
//! Validates target directory boundaries against the security context,
//! normalizes search paths, maps query parameters, and returns formatted
//! match items to MCP clients.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use crate::protocol::{ToolCallResult, ToolDefinition};
use crate::security::McpSecurityContext;
use serde_json::Value;
use std::path::Path;
use std::sync::{Arc, Mutex};
use xlseek_core::models::{DirectorySearchMode, MatchType, SearchMatch, SearchQuery};
use xlseek_core::search::engine::SearchEngine;

/// Returns the tool definition contract for `xlseek_search`.
///
/// ## Arguments
/// None.
///
/// ## Returns
/// `ToolDefinition` with JSON Schema input properties.
///
/// ## Errors
/// Does not panic.
pub fn tool_definition() -> ToolDefinition {
    // Constant reference: crate::constants::TOOL_NAME_SEARCH
    // Constant reference: crate::constants::TOOL_DESC_SEARCH
    // Constant reference: crate::constants::DEFAULT_MAX_RESULTS
    ToolDefinition {
        name: crate::constants::TOOL_NAME_SEARCH.to_string(),
        description: crate::constants::TOOL_DESC_SEARCH.to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "paths": {
                    "oneOf": [
                        { "type": "string" },
                        { "type": "array", "items": { "type": "string" } }
                    ],
                    "description": "Path or list of paths (directories or files) to search."
                },
                "query": {
                    "type": "string",
                    "description": "Keyword or regular expression to search for."
                },
                "is_regex": {
                    "type": "boolean",
                    "default": false,
                    "description": "Evaluate query as regular expression."
                },
                "match_case": {
                    "type": "boolean",
                    "default": false,
                    "description": "Perform case-sensitive matching."
                },
                "targets": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["values", "formulas", "comments", "shapes"]
                    },
                    "description": "Target elements to search within documents."
                },
                "extensions": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "File extensions to include (e.g. ['xlsx', 'docx'])."
                },
                "ignore_patterns": {
                    "type": "string",
                    "description": "Comma-separated .gitignore-style exclusion patterns (e.g. '~$*, *.tmp')."
                },
                "file_patterns": {
                    "type": "string",
                    "description": "Comma-separated inclusion glob patterns to filter candidate files (e.g. 'budget*, report*.xlsx')."
                },
                "max_results": {
                    "type": "integer",
                    "default": crate::constants::DEFAULT_MAX_RESULTS,
                    "description": "Maximum number of match items to return."
                }
            },
            "required": ["paths", "query"]
        }),
    }
}

/// Formats a single `SearchMatch` into human- and LLM-readable summary text.
///
/// ## Arguments
/// - `m`: `&SearchMatch` - The hit to format.
///
/// ## Returns
/// String representation of the hit.
///
/// ## Errors
/// Does not panic.
fn format_match(m: &SearchMatch) -> String {
    let type_label = match m.match_type {
        MatchType::CellValue => "CellValue",
        MatchType::Formula => "Formula",
        MatchType::Comment => "Comment",
        MatchType::Shape => "Shape",
        MatchType::HiddenSheet => "HiddenSheet",
        MatchType::WordText => "WordText",
    };

    let shape_info = m
        .shape_name
        .as_deref()
        .map(|s| format!("  Shape: {}\n", s))
        .unwrap_or_default();

    let formula_info = m
        .formula
        .as_deref()
        .map(|f| format!("  Formula: {}\n", f))
        .unwrap_or_default();

    format!(
        "- File: {}\n  Sheet: {}\n  Cell: {} (Row {}, Col {})\n  Type: {}\n{}  Content: {}\n  Snippet: {}\n{}",
        m.full_path,
        m.sheet_name,
        m.cell_address,
        m.row_index,
        m.col_index,
        type_label,
        shape_info,
        m.full_content.trim(),
        m.snippet.trim(),
        formula_info,
    )
}

/// Executes the `xlseek_search` tool invocation.
///
/// ## Arguments
/// - `args`: `&Value` - Tool invocation arguments object.
/// - `security`: `&McpSecurityContext` - Active security context for path validation.
///
/// ## Returns
/// `ToolCallResult` containing formatted search matches or error details.
///
/// ## Errors
/// Returns error `ToolCallResult` on invalid arguments, security boundary violations,
/// or core search engine failures without crashing.
pub fn execute(args: &Value, security: &McpSecurityContext) -> ToolCallResult {
    let query_str = match args.get("query").and_then(|q| q.as_str()) {
        Some(q) if !q.is_empty() => q,
        _ => return ToolCallResult::error("Missing or empty required parameter: 'query'"),
    };

    let mut raw_paths = Vec::new();
    if let Some(p) = args.get("paths") {
        if let Some(single_str) = p.as_str() {
            raw_paths.push(single_str.to_string());
        } else if let Some(arr) = p.as_array() {
            for item in arr {
                if let Some(s) = item.as_str() {
                    raw_paths.push(s.to_string());
                }
            }
        }
    } else if let Some(p) = args.get("path").and_then(|p| p.as_str()) {
        raw_paths.push(p.to_string());
    }

    if raw_paths.is_empty() {
        return ToolCallResult::error(
            "Missing or empty parameter: 'paths' (must be string or array of strings)",
        );
    }

    let mut validated_paths = Vec::new();
    for raw in &raw_paths {
        match security.validate_path(raw) {
            Ok(p) => validated_paths.push(p),
            Err(e) => return ToolCallResult::error(e),
        }
    }

    let is_regex = args
        .get("is_regex")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let match_case = args
        .get("match_case")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    // Constant reference: crate::constants::DEFAULT_MAX_RESULTS
    let max_results = args
        .get("max_results")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize)
        .unwrap_or(crate::constants::DEFAULT_MAX_RESULTS);

    let ignore_patterns = args
        .get("ignore_patterns")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let file_patterns = args
        .get("file_patterns")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let (include_val, include_form, include_comm, include_shp) =
        if let Some(targets) = args.get("targets").and_then(|t| t.as_array()) {
            let mut v = false;
            let mut f = false;
            let mut c = false;
            let mut s = false;
            for target in targets {
                if let Some(t_str) = target.as_str() {
                    match t_str {
                        "values" => v = true,
                        "formulas" => f = true,
                        "comments" => c = true,
                        "shapes" => s = true,
                        _ => {}
                    }
                }
            }
            (v, f, c, s)
        } else {
            (true, true, true, true)
        };

    let extensions: Vec<String> =
        if let Some(exts) = args.get("extensions").and_then(|e| e.as_array()) {
            exts.iter()
                .filter_map(|e| e.as_str())
                .map(|s| {
                    if s.starts_with('.') {
                        s.to_lowercase()
                    } else {
                        format!(".{}", s.to_lowercase())
                    }
                })
                .collect()
        } else {
            vec![
                ".xlsx".to_string(),
                ".xlsm".to_string(),
                ".xls".to_string(),
                ".xlsb".to_string(),
                ".docx".to_string(),
                ".doc".to_string(),
            ]
        };

    let collected_matches = Arc::new(Mutex::new(Vec::new()));
    let total_matches_counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    for path in validated_paths {
        let (target_dir, single_file_filter) = if path.is_file() {
            let parent = path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            (parent, Some(path.clone()))
        } else {
            (path.clone(), None)
        };

        let search_query = SearchQuery {
            keyword: query_str.to_string(),
            target_dir: target_dir.display().to_string(),
            directory_mode: DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case,
            include_value: include_val,
            use_regex: is_regex,
            include_formula: include_form,
            include_comment: include_comm,
            include_shape: include_shp,
            include_hidden: false,
            extensions: extensions.clone(),
            ignore_patterns: ignore_patterns.clone(),
            file_patterns: file_patterns.clone(),
        };

        let engine = SearchEngine::new();
        let matches_dest = Arc::clone(&collected_matches);
        let total_counter = Arc::clone(&total_matches_counter);
        let file_filter = single_file_filter.clone();

        let search_result = engine.execute_search(
            search_query,
            move |hit| {
                if let Some(ref required_file) = file_filter {
                    if Path::new(&hit.full_path) != required_file {
                        return;
                    }
                }
                total_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let mut guard = matches_dest.lock().unwrap();
                if guard.len() < max_results {
                    guard.push(hit);
                }
            },
            |_progress| {},
        );

        if let Err(e) = search_result {
            return ToolCallResult::error(format!("Search failed on '{}': {}", path.display(), e));
        }
    }

    let final_matches = collected_matches.lock().unwrap().clone();
    let total_count = total_matches_counter.load(std::sync::atomic::Ordering::Relaxed);

    if final_matches.is_empty() {
        return ToolCallResult::success(format!("No matches found for query: '{query_str}'."));
    }

    let is_truncated = total_count > final_matches.len();
    let header = if is_truncated {
        format!(
            "Found {} matches (truncated to maximum {} results):\n\n",
            total_count,
            final_matches.len()
        )
    } else {
        format!("Found {} match(es):\n\n", final_matches.len())
    };

    let body = final_matches
        .iter()
        .map(format_match)
        .collect::<Vec<_>>()
        .join("\n");

    ToolCallResult::success(format!("{header}{body}"))
}

# Interface Contract: File Search Patterns

**Feature**: `037-file-search-patterns`  
**Date**: 2026-10-27  
**Status**: Complete  

---

## 1. Tauri IPC Command Contract

### 1.1 Command: `start_search`

Initiates an asynchronous search over one or more directories, respecting user-supplied inclusion file search patterns and exclusion ignore patterns.

- **Command Name**: `start_search`
- **Direction**: Frontend (React) -> Backend (Tauri v2 / Rust)

#### Request Payload (`SearchQuery`)
```json
{
  "target_dir": "dummy/path/workbooks",
  "keyword": "dummy_needle",
  "match_case": false,
  "use_regex": false,
  "include_formula": true,
  "include_comment": true,
  "include_shape": false,
  "include_hidden": false,
  "extensions": [".xlsx", ".xlsm", ".xlsb", ".xls"],
  "directory_mode": "sequential",
  "burst_workers": null,
  "ignore_patterns": "~$*, *.tmp, backup/",
  "file_patterns": "*budget*, annual_*"
}
```

#### Field Specifications for `file_patterns`
| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `file_patterns` | `string \| null` | Optional | `null` | Comma-separated inclusion file search patterns (e.g. `*budget*, annual_*`). When empty, null, or whitespace-only, defaults to matching all files matching active `extensions`. Extensionless tokens are automatically complemented with active extensions. |

#### Evaluation Semantics
1. Candidate files are first evaluated against `file_patterns` (or default extensions if `file_patterns` is omitted).
2. Files matching `file_patterns` are subsequently tested against `ignore_patterns`.
3. If an explicit extension is specified in `file_patterns` (e.g., `*.xls`), it takes precedence over unchecked extension toggles for supported document formats.

---

## 2. CLI Interface Contract (`xlseek-cli`)

### 2.1 Command Arguments Specification

The standalone CLI supports specifying inclusion file search patterns via both short and long options:

- **Short Option**: `-g <PATTERNS>`
- **Long Option**: `--file-patterns <PATTERNS>` or `--file-patterns=<PATTERNS>`
- **Description**: Comma-separated inclusion glob patterns to filter scanned files. Patterns without extensions are automatically complemented using active file extension options.
- **Example Invocations**:
  ```bash
  # Search needle across budget spreadsheets in dummy directory
  xlseek-cli -q "dummy_needle" -p "dummy/path" -g "*budget*"

  # Multiple patterns with explicit and extensionless tokens
  xlseek-cli -q "dummy_needle" -p "dummy/path" --file-patterns="*report*, annual.xlsx"

  # Combining inclusion patterns with ignore patterns
  xlseek-cli -q "dummy_needle" -p "dummy/path" -g "*report*" -i "~$*, backup/*"
  ```

---

## 3. MCP Tool Interface Contract (`crates/mcp`)

### 3.1 Tool: `search`

The MCP server exposes the `search` tool with the new `file_patterns` parameter.

#### Tool Definition Schema
```json
{
  "name": "search",
  "description": "Searches for keywords or regular expressions within Excel and Word documents across specified directories or files.",
  "parameters": {
    "type": "object",
    "properties": {
      "paths": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Target directories or file paths to search."
      },
      "query": {
        "type": "string",
        "description": "Search keyword or regular expression pattern."
      },
      "is_regex": {
        "type": "boolean",
        "default": false,
        "description": "Whether to interpret the query as a regular expression."
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
      "file_patterns": {
        "type": "string",
        "description": "Comma-separated inclusion file search patterns (e.g. '*budget*, annual_*'). Extensionless tokens are automatically complemented with active extensions."
      },
      "ignore_patterns": {
        "type": "string",
        "description": "Comma-separated .gitignore-style exclusion patterns (e.g. '~$*, *.tmp')."
      },
      "max_results": {
        "type": "integer",
        "default": 100,
        "description": "Maximum number of match items to return."
      }
    },
    "required": ["paths", "query"]
  }
}
```

---

## 4. Settings & Storage Contract

### 4.1 Settings Configuration Schema (`SearchOptionsConfig`)

Persisted user defaults in application configuration.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `default_file_patterns` (Rust) / `defaultFilePatterns` (TS) | `string` | `""` | Baseline file search patterns loaded when a new search session starts or when search parameters are reset. |

### 4.2 Local Storage History Contract

- **Storage Key**: `xlseek.filePatternHistory`
- **Data Format**: JSON array of strings `["*budget*", "*report*, annual_*"]`
- **Capacity**: Maximum 20 entries (LRU: most recently used entry placed at index 0, duplicate items removed, excess pruned).

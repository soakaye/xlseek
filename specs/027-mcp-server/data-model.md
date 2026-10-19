# Phase 1: Data Model Specification

**Feature**: MCP Server Implementation Based on `xlseek-cli`  
**Feature Branch**: `027-mcp-server`  
**Date**: 2026-10-19  

## 1. JSON-RPC 2.0 & MCP Wire Protocol Models

### JsonRpcRequest
Incoming message received on `stdin`.

| Field | Type | Description |
|:---|:---|:---|
| `jsonrpc` | `String` | Protocol version, always `"2.0"`. |
| `id` | `Option<Value>` | Request ID (string, number, or null for notifications). |
| `method` | `String` | Method name (e.g., `"initialize"`, `"tools/list"`, `"tools/call"`, `"ping"`). |
| `params` | `Option<Value>` | Method-specific parameters object. |

### JsonRpcResponse
Outgoing message sent to `stdout`.

| Field | Type | Description |
|:---|:---|:---|
| `jsonrpc` | `String` | Protocol version, always `"2.0"`. |
| `id` | `Value` | Echoes the request ID. |
| `result` | `Option<Value>` | Successful response payload (mutually exclusive with `error`). |
| `error` | `Option<JsonRpcError>` | Error details on failure. |

### JsonRpcError

| Field | Type | Description |
|:---|:---|:---|
| `code` | `i32` | Standard JSON-RPC error code (e.g., `-32600`, `-32601`, `-32602`, `-32603`). |
| `message` | `String` | Human-readable error description. |
| `data` | `Option<Value>` | Optional structured diagnostic data. |

---

## 2. MCP Core Types

### InitializeResult
Response payload for the `initialize` method.

| Field | Type | Description |
|:---|:---|:---|
| `protocolVersion` | `String` | Protocol version supported by server (e.g., `"2024-11-05"`). |
| `capabilities` | `ServerCapabilities` | Declared capabilities (e.g., `tools: {}`). |
| `serverInfo` | `ServerInfo` | Server identification (`name: "xlseek-mcp"`, `version: "1.0.0"`). |

### ToolDefinition
Exposed in response to `tools/list`.

| Field | Type | Description |
|:---|:---|:---|
| `name` | `String` | Unique tool name (`xlseek_search`, `xlseek_list_sheets`, `xlseek_preview_sheet`). |
| `description` | `String` | Plain English description of what the tool does and when to call it. |
| `inputSchema` | `Value` | JSON Schema describing the expected parameters. |

### ToolCallResult
Payload returned in `tools/call` result.

| Field | Type | Description |
|:---|:---|:---|
| `content` | `Vec<ToolContent>` | Array of content items returned to the LLM. |
| `isError` | `Option<bool>` | Optional boolean flag indicating a business/tool-level error. |

### ToolContent

| Field | Type | Description |
|:---|:---|:---|
| `type` | `String` | Content type, typically `"text"`. |
| `text` | `String` | Formatted payload text (JSON string or Markdown table). |

---

## 3. Tool-Specific Parameter & Response Models

### Tool 1: `xlseek_search`

#### Input Parameters (`XlseekSearchParams`)

| Parameter | Type | Required | Default | Description |
|:---|:---|:---:|:---|:---|
| `paths` | `Vec<String>` or `String` | Yes | - | Single target path or array of paths (files or directories) to search. |
| `query` | `String` | Yes | - | Keyword or regex pattern to search for. |
| `is_regex` | `bool` | No | `false` | Whether `query` is evaluated as a regular expression. |
| `match_case` | `bool` | No | `false` | Case sensitivity of the search. |
| `targets` | `Vec<String>` | No | `["values", "formulas", "comments", "shapes"]` | Target domains to search. |
| `extensions` | `Vec<String>` | No | `["xlsx", "xlsm", "xls", "xlsb", "docx", "doc"]` | File extension filter. |
| `ignore_patterns` | `String` | No | `""` | Comma-separated `.gitignore`-style patterns to exclude (e.g., `"~$*, *.tmp, backup/"`). |
| `max_results` | `usize` | No | `100` | Maximum number of matches to return. |

#### Structured Output Item (`SearchResultItem`)

| Field | Type | Description |
|:---|:---|:---|
| `file_path` | `String` | Normalized path to the containing file. |
| `file_name` | `String` | File name with extension. |
| `sheet_name` | `String` | Worksheet name (or document segment). |
| `cell_address` | `String` | A1 notation coordinate (e.g., `"B12"`). |
| `row` | `u32` | 1-based row index. |
| `col` | `u32` | 1-based column index. |
| `match_type` | `String` | Match category: `"CellValue"`, `"Formula"`, `"Comment"`, `"Shape"`, or `"WordText"`. |
| `value` | `String` | Full text or value of the cell. |
| `snippet` | `String` | Context snippet highlighting the match. |
| `formula` | `Option<String>` | Formula string if applicable. |

---

### Tool 2: `xlseek_list_sheets`

#### Input Parameters (`XlseekListSheetsParams`)

| Parameter | Type | Required | Default | Description |
|:---|:---|:---:|:---|:---|
| `path` | `String` | Yes | - | Path to the Excel workbook file (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`). |

#### Output Structure

| Field | Type | Description |
|:---|:---|:---|
| `file_path` | `String` | Normalized path of the workbook. |
| `sheet_count` | `usize` | Total number of sheets in the workbook. |
| `sheets` | `Vec<String>` | List of all sheet names in tab order. |

---

### Tool 3: `xlseek_preview_sheet`

#### Input Parameters (`XlseekPreviewSheetParams`)

| Parameter | Type | Required | Default | Description |
|:---|:---|:---:|:---|:---|
| `path` | `String` | Yes | - | Path to the Excel workbook file. |
| `sheet_name` | `String` | Yes | - | Name of the worksheet to preview. |
| `start_row` | `u32` | No | `1` | 1-based starting row for the preview window. |
| `max_rows` | `u32` | No | `100` | Maximum number of rows to include in the preview. |

#### Output Structure
Returns a formatted Markdown table string in `content[0].text`:
- Header line: `| Row | A | B | C | ... |`
- Separator line: `|---|---|---|---|---|`
- Data rows: `| 1 | Sales | 2026 | ... |`
Accompanied by metadata:
- `total_rows`: Total non-empty rows in worksheet.
- `total_cols`: Total non-empty columns in worksheet.
- `window_rows`: Number of rows rendered in the current preview slice.

---

## 4. Server Configuration Model (`McpServerConfig`)

| Field | Type | Description |
|:---|:---|:---|
| `allowed_directories` | `Vec<PathBuf>` | List of canonical root directories permitted for search/inspection. Empty = unrestricted user access. |
| `default_max_results` | `usize` | Global ceiling for query matches (default: `100`). |
| `default_preview_rows` | `u32` | Global ceiling for preview rows (default: `100`). |

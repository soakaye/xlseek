# MCP Tool Contracts & JSON Schema Definitions

**Feature**: MCP Server Implementation Based on `xlseek-cli`  
**Feature Branch**: `027-mcp-server`  
**Protocol Version**: `2024-11-05`  

## 1. Protocol Handshake Contracts

### Method: `initialize`

#### Client Request
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "Claude Desktop",
      "version": "1.0.0"
    }
  }
}
```

#### Server Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "xlseek-mcp",
      "version": "1.0.2"
    }
  }
}
```

---

## 2. Tools Discovery: `tools/list`

#### Client Request
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

#### Server Response Schema
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "tools": [
      {
        "name": "xlseek_search",
        "description": "High-speed cross-document search across Excel (.xlsx, .xlsm, .xls, .xlsb) and Word (.docx, .doc) files. Returns matching cells, formulas, comments, and drawing shape text.",
        "inputSchema": {
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
            "max_results": {
              "type": "integer",
              "default": 100,
              "description": "Maximum number of match items to return."
            }
          },
          "required": ["paths", "query"]
        }
      },
      {
        "name": "xlseek_list_sheets",
        "description": "List all worksheet names contained within a specific Excel workbook file (.xlsx, .xlsm, .xls, .xlsb).",
        "inputSchema": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string",
              "description": "Absolute or relative path to the Excel file."
            }
          },
          "required": ["path"]
        }
      },
      {
        "name": "xlseek_preview_sheet",
        "description": "Preview rows and columns of an Excel worksheet formatted as a clean Markdown table.",
        "inputSchema": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string",
              "description": "Path to the Excel file."
            },
            "sheet_name": {
              "type": "string",
              "description": "Name of the sheet to preview."
            },
            "start_row": {
              "type": "integer",
              "default": 1,
              "description": "1-based starting row index."
            },
            "max_rows": {
              "type": "integer",
              "default": 100,
              "description": "Maximum number of rows to return in the preview window."
            }
          },
          "required": ["path", "sheet_name"]
        }
      }
    ]
  }
}
```

---

## 3. Tool Execution: `tools/call`

### Call 1: `xlseek_search`

#### Request
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "xlseek_search",
    "arguments": {
      "paths": ["C:/Projects/Finance"],
      "query": "Revenue Q3",
      "match_case": false,
      "max_results": 2
    }
  }
}
```

#### Response
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Found 2 matches across 1 file:\n\n- File: C:/Projects/Finance/2026_Report.xlsx\n  Sheet: Summary\n  Cell: C14 (Row 14, Col 3)\n  Type: CellValue\n  Content: Total Revenue Q3: $1,450,000\n  Snippet: Total [Revenue Q3]: $1,450,000\n\n- File: C:/Projects/Finance/2026_Report.xlsx\n  Sheet: Details\n  Cell: B4 (Row 4, Col 2)\n  Type: CellValue\n  Content: Revenue Q3 Breakdown\n  Snippet: [Revenue Q3] Breakdown"
      }
    ]
  }
}
```

---

### Call 2: `xlseek_preview_sheet`

#### Request
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "xlseek_preview_sheet",
    "arguments": {
      "path": "C:/Projects/Finance/2026_Report.xlsx",
      "sheet_name": "Summary",
      "start_row": 1,
      "max_rows": 3
    }
  }
}
```

#### Response
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Preview of 'Summary' in C:/Projects/Finance/2026_Report.xlsx (Rows 1-3 of 45, 6 columns):\n\n| Row | A | B | C | D | E | F |\n|---|---|---|---|---|---|---|\n| 1 | Category | Q1 | Q2 | Q3 | Q4 | Total |\n| 2 | Revenue | 1200000 | 1350000 | 1450000 | 0 | 4000000 |\n| 3 | Operating Expenses | 800000 | 850000 | 900000 | 0 | 2550000 |"
      }
    ]
  }
}
```

---

## 4. Error Handling Contracts

When a tool encounters a domain or runtime error (such as a missing file, disallowed path, or corrupt archive), the server responds with a tool-level error payload:

```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Access denied: Path 'D:/Secret/Keys.xlsx' is outside the permitted directories list [C:/Projects/Finance]."
      }
    ],
    "isError": true
  }
}
```

For invalid JSON-RPC protocol messages (e.g. invalid JSON, unknown method), the standard JSON-RPC error format is used:

```json
{
  "jsonrpc": "2.0",
  "id": null,
  "error": {
    "code": -32601,
    "message": "Method not found: invalid_method"
  }
}
```

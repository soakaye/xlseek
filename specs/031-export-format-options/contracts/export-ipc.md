# Tauri IPC Contract: `export_results`

**Feature**: `031-export-format-options`  
**Command Name**: `export_results` (`COMMANDS.EXPORT_RESULTS`)

## 1. Request Payload (`ExportRequest`)

```json
{
  "request": {
    "format": "xlsx | csv | md | json",
    "output_path": "/path/to/synthetic/xlseek_export_2026-10-24.md",
    "language": "ja | en",
    "items": [
      {
        "id": 1,
        "file_name": "sample.xlsx",
        "full_path": "/path/to/synthetic/sample.xlsx",
        "sheet_name": "Sheet1",
        "cell_address": "B2",
        "row_index": 2,
        "col_index": 2,
        "match_type": "CellValue",
        "snippet": "dummy value",
        "full_content": "dummy value",
        "formula": null,
        "shape_name": null
      }
    ],
    "llm_options": {
      "column_window_size": 5,
      "copy_type": "table",
      "word_extraction_mode": "headings_and_paragraph"
    }
  }
}
```

## 2. Response

- **Success**: `null` (`Ok(())`)
- **Error**: `CommandError`
  - `{ "code": "internal_error" }` — when `language` is neither `"ja"` nor `"en"`
  - `{ "code": "export_failed" }` — when file writing, context extraction, or background task execution fails

## 3. Frontend Dialog & Notification Contract

| Format | Save Dialog Extension | Filter Translation Key | Saved Toast Translation Key |
| :--- | :--- | :--- | :--- |
| `"xlsx"` | `["xlsx"]` | `ui.EXPORT_XLSX_FILTER_NAME` | `ui.EXPORT_XLSX_SAVED` |
| `"csv"` | `["csv"]` | `ui.EXPORT_CSV_FILTER_NAME` | `ui.EXPORT_CSV_SAVED` |
| `"md"` | `["md"]` | `ui.EXPORT_MD_FILTER_NAME` | `ui.EXPORT_MD_SAVED` |
| `"json"` | `["json"]` | `ui.EXPORT_JSON_FILTER_NAME` | `ui.EXPORT_JSON_SAVED` |

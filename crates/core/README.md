<!-- Description: Overview and documentation for xlseek-core shared library. Arguments & Returns: Documents modules, parsing engines, directory traversal, exports, and tests within crates/core. Errors / Exceptions: Outlines error types, cancellation handling, and catalog validation. -->
# xlseek-core

Shared core library for **Excel Seek (`xlseek`)**, providing the document parsing engine, multi-threaded recursive directory scanning, `.gitignore`-style ignore pattern filtering, search execution, result exporting, and multilingual catalog management.

Used by both the Tauri desktop application (`src-tauri`) and the standalone CLI tool (`crates/cli`).

---

## Architecture & Modules

| Module | Location | Description |
| :--- | :--- | :--- |
| **`models`** | `src/models/` | Domain structs and enums (`SearchQuery`, `SearchResult`, `CellPosition`, `ExportFormat`, `DirectorySearchMode`, etc.) |
| **`search`** | `src/search/` | Core search engine (`engine.rs`), Excel parsing (`excel/`), Word document parsing (`word/`), shape text extraction, and cell preview extraction |
| **`export`** | `src/export/` | Formatted export generators for CSV (`csv_export.rs`) and Excel (`xlsx_export.rs`) with atomic temp-file safety |
| **`constants`** | `src/constants.rs` | Centralized constants (field keys, default limits, regex limits, error messages) |
| **`i18n`** | `src/i18n.rs` | Embedded YAML translation catalog management (`locales/ja.yml`, `locales/en.yml`) |

---

## Supported Formats

- **Excel Workbooks**: `.xlsx`, `.xlsm`, `.xlsb`, `.xls`
  - Cell values and formulas via `calamine`
  - Comments / notes (`.xlsx`, `.xlsm`)
  - Drawing shapes and textboxes (`.xlsx`, `.xlsm`)
  - Hidden worksheets inclusion toggle
- **Word Documents**: `.docx`, `.docm`, `.doc`
  - Paragraphs and tables via OOXML (`quick-xml` + `zip`) and compound binary (`cfb`)
  - Headers, footers, comments, and drawing shape text
  - Formatted text paragraph extraction for preview display

---

## Directory Traversal & Ignore Filtering

- **Sequential Mode**: Depth-first directory traversal (default).
- **Burst Mode**: Concurrently traverses subdirectories using bounded worker threads (auto or 2..32 workers) via `rayon`.
- **Ignore Pattern Matching**: Matches paths and directory names against standard `.gitignore` glob patterns via the `ignore` / `globset` crates. Matching directories are skipped immediately during traversal.

---

## Verification & Testing

```bash
# Run unit and integration tests
cargo test -p xlseek-core

# Run Clippy checks
cargo clippy -p xlseek-core --all-targets -- -D warnings
```

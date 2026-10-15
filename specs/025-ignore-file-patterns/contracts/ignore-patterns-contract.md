# Interface Contract: Ignore File Patterns

**Feature**: `025-ignore-file-patterns`  
**Date**: 2026-10-15  
**Status**: Complete

---

## 1. Tauri IPC Command Contract

### 1.1 Command: `start_search`

Initiates an asynchronous multi-threaded search over one or more directories, respecting user-supplied ignore patterns.

- **Command Name**: `start_search`
- **Direction**: Frontend (React) -> Backend (Tauri v2 / Rust)

#### Request Payload (`SearchQuery`)
```json
{
  "target_dir": "C:\\Projects\\Accounting",
  "keyword": "Total Revenue",
  "match_case": false,
  "use_regex": false,
  "include_formula": true,
  "include_comment": true,
  "include_shape": false,
  "include_hidden": false,
  "extensions": [".xlsx", ".xlsm", ".xlsb", ".xls"],
  "directory_mode": "sequential",
  "burst_workers": null,
  "ignore_patterns": "~$*, *.tmp, backup/"
}
```

#### Field Specifications for `ignore_patterns`
| Field | Type | Required | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `ignore_patterns` | `string \| null` | Optional | `null` | Comma-separated gitignore-style wildcard expressions. If `null`, omitted, or empty/whitespace, no ignore filtering is performed. |

#### Response / Return
- Returns: `Result<(), CommandError>`
  - `Ok(())`: Search accepted and started in background worker threads.
  - `Err({ code: "search_failed" })`: Target paths invalid or fatal initialization failure.
  - `Err({ code: "invalid_regex" })`: Keyword is an invalid regular expression.
- *Note*: Invalid ignore pattern syntax logs a warning issue (`SearchIssue` with code `read_failed`) and continues scanning non-ignored items without crashing the application.

---

## 2. CLI Interface Contract (`xlseek-cli`)

### 2.1 Command Arguments Specification

The CLI supports passing ignore patterns via both long and short options:

- **Long Option**: `--ignore-patterns <PATTERNS>` or `--ignore-patterns=<PATTERNS>`
- **Short Option**: `-i <PATTERNS>`
- **Description**: Comma-separated gitignore-style wildcard patterns to exclude files and folders from directory scanning.
- **Example Usage**:
  ```bash
  # Exclude lock files and backup folders
  xlseek-cli -p "C:\Documents" -q "Q4 Budget" -i "~$*, backup/*"

  # Exclude temporary files and node_modules using long option
  xlseek-cli --path="." --query="Revenue" --ignore-patterns="*.tmp, node_modules/" -o results.xlsx
  ```

#### CLI Error Handling
- Supplying `-i` without an argument: Produces CLI syntax error `Missing argument for option: -i`.
- Duplicate `-i` or `--ignore-patterns`: Produces CLI error `Duplicate option specified: --ignore-patterns`.

---

## 3. Persistent LocalStorage Contracts

### 3.1 Ignore Pattern History Contract
- **Storage Key**: `xlseek.ignorePatternHistory`
- **Data Type**: JSON array of non-empty trimmed strings
- **Constraints**:
  - Maximum 20 entries
  - Most recently used (MRU) at index `0`
  - Deduplicated case-sensitively
  - Does not store empty or whitespace-only patterns

#### Example LocalStorage Value:
```json
[
  "~$*",
  "~$*, *.tmp",
  "backup/, draft/*",
  "node_modules/, *.bak"
]
```

### 3.2 Default Search Options Settings Contract
- **Storage Key**: `xlseek.default_search_options`
- **Data Type**: JSON object extending existing `DefaultSearchOptions`
- **Field Added**: `ignore_patterns: string` (default: `"~$*"`)

#### Example LocalStorage Value:
```json
{
  "match_case": false,
  "use_regex": false,
  "include_formula": true,
  "include_comment": true,
  "include_shape": false,
  "include_hidden": false,
  "extensions": [".xlsx", ".xlsm", ".xlsb", ".xls"],
  "directory_mode": "sequential",
  "burst_workers": null,
  "enable_word_search": false,
  "ignore_patterns": "~$*"
}
```

---

## 4. UI Layout & Accessibility Contract

### 4.1 Search Bar Component Placement
- Located in `src/components/search/SearchBar.tsx`:
  - **Row 1**: Target directory input with Browse button and Path History dropdown.
  - **Row 2 (NEW)**: Ignore patterns input field:
    - Left icon: `Filter` / `EyeOff` / `FileX` with localized title.
    - Text input with placeholder: `ui.IGNORE_PATTERNS_PLACEHOLDER` (e.g. `~$*, *.tmp, backup/`).
    - History button: `History` icon opening the history dropdown list.
    - Clear button: `X` icon shown when text is non-empty.
  - **Row 3**: Keyword input, Regex/Case toggles, Search button.
  - **Row 4**: File extension checkboxes (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`, `.docx`).

### 4.2 Accessibility & Keyboard Navigation
- Ignore pattern input has `aria-label` bound to `ui.IGNORE_PATTERNS_LABEL`.
- Pressing `Enter` in the ignore patterns input triggers search execution.
- Pressing `ArrowDown` opens the history dropdown if closed, or focuses the first entry if open.
- In history dropdown:
  - `ArrowDown` / `ArrowUp`: Highlights items in cyclic rotation.
  - `Enter`: Selects highlighted item, fills the input, and closes dropdown.
  - `Escape`: Closes dropdown and returns focus to input.

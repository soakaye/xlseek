# Feature Specification: Search Results Export Format Options and UI Reorganization

**Feature Branch**: `031-export-format-options`

**Created**: 2026-10-24

**Status**: Draft

**Input**: User description: "ファイルへの出力機能を整理する。現在、csv、excelのみだが、AI用text/md, xlseek-mcpの様なjson出力を選択できるように出力ボタンの機能を整理する。"

## Clarifications

### Session 2026-10-24

- Q: 「AI用 text/md」としてファイル出力する際、出力内容の形式と拡張子はどの方式にしますか？ (FR-002, FR-005) → A: Option A (AIコピー共通のMarkdown形式 `.md`: 既存の「AIコピー」と同じ抽出エンジン `extract_llm_context` とユーザー設定を用いて、周辺セルや見出し文脈を含む `.md` ファイルを出力する)
- Q: 「`xlseek-mcp` のようなJSON出力（`.json`）」のスキーマ構造はどの形式にしますか？ (FR-006) → A: Option A (メタデータ付き構造化JSONオブジェクト: トップレベルに `total_matches` 等のメタデータと `matches` 配列を持ち、各要素に正規化済みパス・シート名・セル座標・種別・内容・スニペット・数式・図形名を含む `SearchMatch` 属性を格納する)
- Q: ステータスバーの「出力（エクスポート）」ボタンのUI操作方式はどれにしますか？ (FR-001) → A: Option B (スプリットボタン方式: メインボタンのクリックで現在選択されている形式の保存ダイアログを即座に開き、右端の「▾」ボタンでフォーマット選択メニューを開いて形式を切り替える)
- Q: スプリットボタンの「▾」メニューから別のフォーマットを選択した際、保存ダイアログの起動タイミングはどう動作させますか？ (FR-001, FR-003) → A: Option A (形式切り替え＋即座に保存ダイアログを開く: メニュー項目を選択するとメインボタンの選択形式が切り替わり次回以降用に記憶されると同時に、そのまま保存ダイアログが開く。初期デフォルト形式はExcel `.xlsx`)
- Q: 新設する `md`（AI用Markdown）および `json` 出力形式は、GUIだけでなくCLI（`xlseek-cli` の `--format` オプションや出力ファイル拡張子推論）でも対応しますか？ (FR-002) → A: Option A (GUIとCLIの両方で対応: `xlseek-core` のエクスポート基盤を拡張し、GUIのスプリットボタンだけでなく `xlseek-cli` の `-f, --format <csv|xlsx|md|json>` および `-o` 拡張子自動判定でも利用可能にする)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consolidated Export Split Button with Format Menu (Priority: P1) 🎯 MVP

As an Excel Seek user reviewing search results,
I want a consolidated export split button in the status bar where the main action exports in the active format and the dropdown toggle (`▾`) lets me choose between CSV, Excel, AI Markdown, and JSON,
So that I can export with a single click in my preferred format while keeping the status bar clean and having instant access to all other export formats.

**Why this priority**:
Currently, separate buttons exist in the status bar for CSV and Excel. As new export targets (Markdown for AI and JSON for integrations) are introduced, adding more individual buttons would overcrowd the status bar, degrade layout balance, and scale poorly. A unified split button provides one-click convenience for the active format while housing all four formats cleanly in a toggle menu.

**Independent Test**:
Perform a search yielding one or more results. Click the main part of the Export split button in the status bar to immediately open the file save dialog for the active format. Click the `▾` toggle to open the format selection menu (CSV, Excel, AI Markdown, JSON) and switch the active format.

**Acceptance Scenarios**:

1. **Given** a completed search with results, **When** the user clicks the main action area of the Export split button, **Then** the operating system file save dialog opens immediately for the currently active format.
2. **Given** a completed search with results, **When** the user clicks the dropdown toggle (`▾`) of the Export split button, **Then** an options menu opens displaying format choices: CSV (`.csv`), Excel (`.xlsx`), AI Markdown (`.md`), and JSON (`.json`).
3. **Given** zero search results (idle state or empty search), **When** viewing the status bar, **Then** the entire Export split button (both main action and `▾` toggle) is disabled and cannot be activated.
4. **Given** the export format menu is open, **When** the user clicks outside the menu or presses the Escape key, **Then** the menu closes without triggering an export action.
5. **Given** the user selects a format from the `▾` menu, **When** the format item is clicked, **Then** the menu closes, that format becomes the new active format displayed on the split button (persisted for subsequent exports, with Excel `.xlsx` as the initial default), and the operating system file save dialog opens immediately for that format.

---

### User Story 2 - Export Search Results as AI-Ready Markdown / Text (Priority: P2)

As a researcher or knowledge worker analyzing spreadsheet data with AI assistants,
I want to export search results into a structured Markdown (`.md`) file,
So that I can directly supply formatted file, sheet, cell coordinates, and surrounding context into LLM chat windows, agents, or personal knowledge bases.

**Why this priority**:
Users frequently need to analyze search hits using AI models. While "AI Copy" allows copying individual items or selections to the clipboard, exporting an entire search result set into an AI-ready Markdown file enables offline prompt preparation, archiving, and batch context ingestion for large language models.

**Independent Test**:
Perform a search with matching cells. Choose "AI Markdown" from the export menu, choose a destination file, and verify that the saved `.md` file contains clearly structured workbook paths, sheet tabs, cell addresses, matched values, and contextual content formatted for AI consumption.

**Acceptance Scenarios**:

1. **Given** search matches across one or more documents, **When** the user selects "AI Markdown" export and saves the file, **Then** a `.md` file is generated containing structured sections with document paths, worksheet tab names, cell/paragraph coordinates, match types, and hit values.
2. **Given** search matches containing multiline cell values or markdown-sensitive characters, **When** exported to Markdown, **Then** table cells and code fences are properly escaped or structured to prevent markdown layout breakage.
3. **Given** search matches from Word documents or Excel shapes, **When** exported to Markdown, **Then** paragraph numbers or shape names are clearly noted alongside the text snippet.

---

### User Story 3 - Export Search Results as Structured JSON Aligned with MCP Schema (Priority: P3)

As a developer or automation engineer building scripts or AI agent pipelines,
I want to export search results to a clean, structured JSON (`.json`) file matching the schema used by `xlseek-mcp`,
So that I can programmatically consume, transform, or inspect search findings using automated scripts or MCP-compatible tooling without custom text parsing.

**Why this priority**:
`xlseek-mcp` exposes structured search results for AI assistant integrations. Providing an export format that follows this exact JSON structure allows GUI users to save machine-readable search runs for batch evaluation, regression testing, or ingestion into external tools.

**Independent Test**:
Perform a search, choose "JSON" export from the export menu, save the file as `.json`, and validate that the output is syntactically valid JSON conforming to the search match schema (containing query metadata, total match count, and an array of match objects with path, sheet, address, value, match type, and context attributes).

**Acceptance Scenarios**:

1. **Given** search matches, **When** the user exports to JSON, **Then** the produced file contains valid UTF-8 JSON with top-level search metadata (query keyword, match count, export timestamp) and a `matches` array of detailed match objects.
2. **Given** matches with special characters, quotes, or unicode characters, **When** exported to JSON, **Then** all strings are strictly valid JSON-escaped strings.
3. **Given** a requirement for human readability, **When** the JSON file is written, **Then** it is indented (pretty-printed) with 2 spaces.

---

### User Story 4 - Preserve Existing CSV and Excel Export Reliability (Priority: P4)

As an existing user who routinely exports search results for spreadsheet reporting in CSV or Excel,
I want the CSV and XLSX export workflows to function identically through the new consolidated export control,
So that existing data export workflows, column structures, and workbook formatting remain completely uninterrupted.

**Why this priority**:
CSV and Excel exports are core features already relied upon by users. Consolidating the UI controls must preserve 100% backward compatibility for file generation logic, column headers, atomic writing safety, and notification handling.

**Independent Test**:
Export a search result set to CSV and Excel via the new menu. Verify that the resulting `.csv` and `.xlsx` files have identical column layouts, headers, and values compared to previous releases.

**Acceptance Scenarios**:

1. **Given** search results, **When** selecting "CSV" from the export menu, **Then** the CSV file is generated with standard localized header rows and accurate match data.
2. **Given** search results, **When** selecting "Excel" from the export menu, **Then** an `.xlsx` workbook is created with formatted column headers and autofitted content.
3. **Given** successful export of any format, **When** file write completes, **Then** a localized toast notification confirms the saved path.

---

### Edge Cases

- **Zero Search Results**: When no results are available, the export button is disabled. If invoked programmatically, an error or informative notification indicates no results exist to export.
- **Save Dialog Cancellation**: If the user cancels the OS file save dialog, the export operation terminates silently without raising an error toast or writing partial files.
- **Write Permission Denied / File Lock**: When saving to a path with insufficient permissions or locked by another application, an explicit user-friendly error notification is displayed, and temporary staging files are cleaned up.
- **Large Result Set Export**: Exporting thousands of matches does not freeze the UI; processing runs asynchronously in the background.
- **File Name Conflicts and Extension Normalization**: If the user omits the extension in the save dialog, the chosen format's extension is automatically appended. Overwrites are prompted by the OS file save dialog.
- **Synthetic Data Compliance**: All test fixtures, documentation examples, and specifications strictly use synthetic dummy data (e.g., `sample.xlsx`, `Sheet1`, `A1`, `dummy search keyword`).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST consolidate search result export triggers in the GUI status bar into a single unified Split Button, where clicking the primary button immediately opens the save dialog for the currently selected format, and clicking the adjacent chevron (`▾`) toggle opens the format selection menu. Selecting an item from the menu MUST update and persist the active format (initial default: Excel `.xlsx`) and immediately open the file save dialog for that format.
- **FR-002**: System MUST support four distinct export file formats across both GUI and CLI (`xlseek-cli`):
  - Comma-Separated Values (`csv` / `.csv`)
  - Microsoft Excel Workbook (`xlsx` / `.xlsx`)
  - AI Markdown Document (`md` / `.md`)
  - Structured JSON (`json` / `.json`)
- **FR-003**: System MUST open the native operating system file save dialog when an export is triggered in the GUI, pre-populating a default filename matching `export_YYYY-MM-DD.<ext>` (or localized equivalent prefix) with the matching extension filter.
- **FR-004**: System MUST disable the entire export split button (both primary button and `▾` toggle) whenever the search result list contains zero items.
- **FR-005**: Markdown export MUST reuse the existing AI Copy context extraction logic and active user AI Copy settings (e.g., table mode vs. header-only mode, column window size, Word extraction mode) to generate well-structured `.md` files containing document file paths, worksheet tab names, cell/paragraph coordinates, match types, hit values, and surrounding context.
- **FR-006**: JSON export MUST produce a well-formed, 2-space pretty-printed UTF-8 JSON object containing top-level summary metadata (`total_matches`) and a `matches` array where each item contains normalized file path (`full_path`, `file_name`), location (`sheet_name`, `cell_address`, `row_index`, `col_index`), `match_type`, `full_content`, `snippet`, and optional `formula` and `shape_name`.
- **FR-007**: System MUST perform file writing safely through temporary file staging or atomic creation, preventing corrupted output if writing is interrupted.
- **FR-008**: System MUST provide localized user feedback (success toast with file path upon completion, or error toast upon failure) in both Japanese and English.
- **FR-009**: System MUST allow closing the format selection menu by clicking outside the dropdown or pressing the `Escape` key without initiating an export.
- **FR-010**: All UI button labels, menu item texts, tooltips, and toast notifications MUST use centralized constants and support multilingual localization.
- **FR-011**: The standalone CLI (`xlseek-cli`) MUST accept `md` and `json` in `-f, --format <csv|xlsx|md|json>`, automatically infer `md` and `json` formats from `.md` and `.json` output file extensions (`-o`), and support streaming `md` and `json` to standard output (`stdout`) when no `-o` path is given.

### Key Entities *(include if feature involves data)*

- **ExportFormat**: Enumeration representing supported export formats (`csv`, `xlsx`, `md`, `json`).
- **ExportRequest**: Data structure conveying the chosen `ExportFormat`, target destination file path, collection of `SearchMatch` records, current UI language, and optional `LlmCopyOptions` for Markdown context formatting.
- **ExportArtifact**: The generated file residing on the filesystem, formatted according to the rules of the selected `ExportFormat`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can initiate export in the active format with 1 click (Primary Split Button) or switch to and export in any of the four formats within 2 clicks (`▾` Toggle → Format Item).
- **SC-002**: Status bar control footprint is reduced by replacing multiple format buttons with a single unified split button control, preserving UI space for search progress and status messages.
- **SC-003**: 100% of exported files across all four formats in both GUI and CLI pass programmatic validation (valid CSV syntax, valid XLSX workbook structure, valid AI Copy Markdown structure, and valid JSON syntax parseable by standard JSON parsers).
- **SC-004**: Exporting up to 1,000 search matches completes without causing UI responsiveness lag (asynchronous background execution).
- **SC-005**: Zero regression in existing CSV and Excel export behaviors and test suites across GUI and CLI.

## Assumptions

- Markdown export in GUI uses the user's persisted `LlmCopyOptions` (from settings), and in CLI uses `LlmCopyOptions::default()`.
- JSON export is encoded in UTF-8 with 2-space indentation and includes a top-level object `{ "total_matches": <number>, "matches": [ ... ] }` with normalized paths (`normalize_export_path`).
- Default export filenames continue using the date-stamped convention (`xlseek_export_YYYY-MM-DD.<ext>`) localized per language catalog, with `.xlsx` as the initial default format for the split button.
- Format menu styling conforms to the dark theme styling of existing menus and dialog triggers in the application, and the standalone HTML prototype (`design/mainui/index.html`) will be kept in sync.

# Feature Specification: Explicit Sheet Tab Name in AI Copy Target Cell Metadata

**Feature Branch**: `030-ai-copy-tab-name`

**Created**: 2026-10-23

**Status**: Draft

**Input**: User description: "AIコピーでコピーしたテキストのタブ情報が分かりにくい。検索結果Target Cellにタブ名を明示する。"

## Clarifications

### Session 2026-10-23

- Q: What format should be used to display the worksheet tab name in the `Target Cell` metadata line? → A: Option A: Bracketed prefix format `[SheetName] CellAddress` (e.g. `- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)`).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Explicit Sheet Tab Name in AI Copy Output (Priority: P1) 🎯 MVP

As an Excel search user copying search results via "AI Copy" (Copy for LLM),
I want the `Target Cell` line in the copied prompt text to explicitly display the worksheet tab name alongside the cell address using bracketed notation `[SheetName] CellAddress`,
So that when I paste the snippet into an AI assistant or notes, I can immediately identify which worksheet tab the cell belongs to without ambiguity.

**Why this priority**:
Currently, the AI Copy output generates a metadata entry such as `- Target Cell: B2 (Column: Status, Row: Item 1)`. While the top-level section header includes the sheet name (`### SampleWorkbook.xlsx - Sheet1`), the specific `Target Cell` bullet point omits the tab name. When copying multiple results, long sheets, or sharing prompt snippets, users and LLMs struggle to quickly correlate the cell coordinate with its specific worksheet tab. Adding the bracketed tab name directly to the `Target Cell` descriptor immediately resolves this ambiguity.

**Independent Test**:
Perform an Excel search, click "AI Copy" (or use right-click context menu "Copy for LLM"), and paste the clipboard content. Verify that the `Target Cell` line includes the worksheet tab name in bracketed notation (e.g. `- Target Cell: [Sheet1] B2 (Column: ..., Row: ...)`).

**Acceptance Scenarios**:

1. **Given** an Excel search match in worksheet tab "Sheet1" at cell address "B2", **When** the user copies LLM context via AI Copy (in either Table or Header-only mode), **Then** the generated metadata line explicitly displays the tab name within the `Target Cell` entry as `- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)`.
2. **Given** multiple search matches from different worksheet tabs copied together in a single action, **When** formatted as LLM context, **Then** each individual `Target Cell` line unambiguously designates its corresponding worksheet tab using bracketed notation `[SheetName] CellAddress`.
3. **Given** an Excel worksheet tab whose name contains spaces or special characters (e.g. `FY2026 Q3 (Final)`), **When** AI context is generated, **Then** the tab name is rendered cleanly enclosed in brackets without syntax corruption or string truncation (e.g. `[FY2026 Q3 (Final)] A1`).

---

### User Story 2 - Consistent Metadata Formatting Across Context Modes (Priority: P2)

As a user switching between "Table" format and "Header Only" context copy modes,
I want both formatting modes to present the worksheet tab name in the `Target Cell` line consistently,
So that the prompt metadata format remains uniform regardless of the selected copy mode.

**Why this priority**:
Users can configure whether AI Copy outputs a full Markdown table or a compact header-only bullet list. Ensuring both code paths format the `Target Cell` line identically prevents inconsistencies.

**Independent Test**:
Configure AI Copy options to "Header Only", copy a match, and verify the `Target Cell` line format matches the one produced in "Table" mode.

**Acceptance Scenarios**:

1. **Given** the user has configured copy mode to "Header Only" (or "Table"), **When** AI Copy is executed, **Then** the `Target Cell` line includes the worksheet tab name in the same standardized `[SheetName] CellAddress` structure across both modes.

---

### Edge Cases

- **Sheet Names with Special Characters**: Worksheet names containing spaces, Japanese characters, hyphens, parentheses, or single quotes (e.g. `'Summary (2026)'`) must render cleanly inside the bracketed prefix.
- **Maximum Length Sheet Names**: Excel allows sheet names up to 31 characters. The metadata line must format the full 31-character name without truncation.
- **Word Document Matches**: Word matches do not have worksheet tabs; they represent document sections and paragraphs. Word context generation must remain unaffected and not output empty or broken sheet prefixes.
- **Empty or Missing Sheet Name**: If a sheet name is unexpectedly empty, the system must fall back gracefully to the standard cell address format (e.g. `- Target Cell: A16 (Column: ..., Row: ...)`) without crashing or producing empty brackets `[]`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST include the worksheet tab name in the `Target Cell` metadata line of the generated AI Copy text for Excel search matches.
- **FR-002**: The sheet tab name inclusion MUST apply consistently to both Table context mode (`format_table_excel_context`) and Header-only context mode (`format_header_only_excel_context`).
- **FR-003**: The system MUST format the Target Cell line using bracketed sheet notation `- Target Cell: [SheetName] CellAddress (Column: ColHeader, Row: RowKey)` when a sheet name is present, while preserving column header and row key metadata.
- **FR-004**: The system MUST support Excel worksheet names up to the maximum 31 characters, including alphanumeric, Japanese, spaces, and symbolic characters.
- **FR-005**: The system MUST leave non-spreadsheet document context formatting (Word documents) unaffected while maintaining document-level section and paragraph identification.
- **FR-006**: When copying multiple search matches from different worksheets in a single batch, the system MUST identify the correct worksheet tab for each respective target cell.
- **FR-007**: If the sheet name is empty or unavailable, the system MUST omit the bracketed prefix and output `- Target Cell: CellAddress (Column: ColHeader, Row: RowKey)`.

### Key Entities

- **AI Copy Context (LLM Context)**: The structured Markdown text generated for clipboard copying, intended for consumption by AI assistants.
- **Target Cell Descriptor**: The metadata line identifying the exact location, sheet, coordinates, column header, and row key of a matched cell.
- **Worksheet Tab (Sheet Name)**: The name of the specific Excel worksheet tab in which the matched cell resides.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of Excel search matches copied via AI Copy explicitly contain the worksheet tab name in the bracketed `[SheetName] CellAddress` format in the `Target Cell` line.
- **SC-002**: 100% of automated unit tests for LLM context generation pass with the updated `Target Cell` format.
- **SC-003**: When AI Copy text is pasted into an LLM, the LLM can identify the exact worksheet tab for each target cell from the `Target Cell` line alone, without cross-referencing outer section titles.
- **SC-004**: Zero regressions in Word document context generation or existing search and export capabilities.

## Assumptions

- The representation of the sheet tab in the `Target Cell` line uses the bracketed format `[SheetName] CellAddress` (e.g. `[Sheet1] B2`), providing high clarity for both human readers and LLMs without requiring special quotation escaping.
- The change is in the text generation formatting in `xlseek-core` (`llm_context.rs`) and does not alter the underlying IPC data models or UI buttons.

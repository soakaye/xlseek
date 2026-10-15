# Feature Specification: Search MS Word Documents (.doc, .docx, .docm)

**Feature Branch**: `024-word-doc-search`

**Created**: 2026-10-15

**Status**: Draft

**Input**: User description: "MS-WORD の .doc, .docx  も検索対象とする。.doc, .docx 検索は設定画面からのオプション選択で指定。検索結果プレビューは、テキスト形式表示とします。追加指示: .docm も対象にします"

## Clarifications

### Session 2026-10-15

- Q: Should Word document search be enabled or disabled by default upon initial installation? → A: Disabled by default (opt-in) so existing Excel search behavior and performance are preserved unless the user enables Word search in Settings.
- Q: How should the Sheet and Cell columns in the search results table represent Word document matches? → A: Display "Word" in the Sheet column, and display the corresponding line/paragraph number or "-" in the Cell column.
- Q: What text extraction scope should be searched within Word documents? → A: Comprehensively cover body paragraphs and tables, headers/footers, comments, and drawing shape text (providing full parity with Excel cell, comment, and shape search).
- Q: When Word document search is enabled in Settings, how should the extension toggle buttons on the main search bar interact? → A: Dynamically add `.docx`, `.docm`, and `.doc` toggle buttons to the main search bar extension list when Word search is enabled in Settings, allowing per-search on/off toggling.
- Q: In the search result preview, how should Word document text display and hit navigation behave? → A: Display the document's full extracted text with line/paragraph numbers, automatically scrolling to the selected match location and highlighting keyword occurrences.
- Q: How should existing search detail options (comments, shape text, etc.) apply during Word document searches? → A: Align "include_comment" and "include_shape" toggles to Word comments and shape/textbox text search, while Excel-specific options ("include_formula", "include_hidden") are gracefully ignored for Word files.
- Q: Does Word document search include macro-enabled documents (`.docm`)? → A: Yes, `.docm` is included alongside `.docx` and `.doc` and shares the same OOXML text extraction pipeline as `.docx`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Configure and Search MS Word Documents (.docx, .docm, .doc) (Priority: P1)

As an xlseek user working with mixed office documents,
I want to enable MS Word (.doc, .docx, .docm) search in the application settings so that the search engine scans Word documents alongside or instead of Excel workbooks when searching across folders.

**Why this priority**:
Core functionality requested by the user. Enabling Word document search expands xlseek from a pure spreadsheet search tool into a unified office document finder while allowing users to opt in or out via settings.

**Independent Test**:
In the Settings dialog, toggle on the MS Word document search option. Select a folder containing `.docx`, `.docm`, and `.doc` files, enter a search term present in those files, and execute the search. Verify that matching Word documents appear in the search results.

**Acceptance Scenarios**:

1. **Given** MS Word search is enabled in Settings,
   **When** the user searches a directory containing `.docx`, `.docm`, and `.doc` files,
   **Then** the search scans all Word formats and lists all matching files with matching snippet text in the results table.
2. **Given** MS Word search is disabled in Settings,
   **When** the user searches a directory containing `.docx`, `.docm`, and `.doc` files,
   **Then** Word files are ignored and only configured spreadsheet files are scanned.
3. **Given** the user toggles the Word search option in Settings,
   **When** the application restarts or runs subsequent searches,
   **Then** the preference persists across sessions.
4. **Given** MS Word search is enabled in Settings,
   **When** the user inspects the main search bar,
   **Then** `.docx`, `.docm`, and `.doc` toggle buttons appear dynamically alongside spreadsheet extensions, allowing per-search on/off toggling.

---

### User Story 2 - Text-Based Preview for Word Document Search Results (Priority: P1)

As a user inspecting a search hit inside a Word document,
I want the preview pane to display the surrounding text in a readable text format rather than a spreadsheet cell grid,
So that I can read the context of the hit as natural prose or paragraphs with matching terms highlighted.

**Why this priority**:
User explicitly specified: "検索結果プレビューは、テキスト形式表示とします。" (Search result preview shall be displayed in text format). Since Word documents do not have sheets or cell coordinates, a text-based preview mode is essential for usability and prevents rendering errors or nonsensical spreadsheet grids.

**Independent Test**:
Click on a search result from a `.docx`, `.docm`, or `.doc` file. Verify that the preview area switches to a text reader view showing the surrounding text/paragraphs with the search keywords highlighted.

**Acceptance Scenarios**:

1. **Given** a selected search result originating from a `.docx`, `.docm`, or `.doc` file,
   **When** the user selects the result row in the results table,
   **Then** the preview area renders the document's full extracted text in a scrollable viewer with line/paragraph numbers, automatically scrolls to bring the matched hit into view, and highlights all matching keywords.
2. **Given** the user alternates between an Excel result row and a Word result row,
   **When** the selection changes,
   **Then** the preview pane seamlessly switches between the spreadsheet grid view (for Excel) and the text view (for Word) without lag or display corruption.
3. **Given** a Word document containing multiple paragraphs or tables,
   **When** viewed in the text preview,
   **Then** paragraphs and table text are presented in readable reading order.

---

### User Story 3 - Word Document Result Navigation and Action Parity (Priority: P2)

As a user reviewing search results,
I want double-clicking or invoking actions on a Word search result to open the document in the default OS word processor or open its enclosing folder,
So that the workflow matches the existing Excel file interaction experience.

**Why this priority**:
Consistency with existing desktop integration (open file with associated app, open containing folder) ensures seamless user experience.

**Independent Test**:
Double-click a Word document search hit, and verify that the operating system launches the default application associated with `.docx` / `.docm` / `.doc` (e.g., Microsoft Word, LibreOffice Writer).

**Acceptance Scenarios**:

1. **Given** a search result for a `.docx`, `.docm`, or `.doc` file,
   **When** the user double-clicks the result row or clicks "Open File",
   **Then** the system launches the file in the default registered word processing application.
2. **Given** a search result for a Word file,
   **When** the user triggers "Open Containing Folder",
   **Then** the OS file manager opens to the folder enclosing the Word file.

---

### Edge Cases

- **Password-protected or encrypted Word documents**: The search engine must gracefully skip encrypted `.docx`, `.docm`, and `.doc` files without crashing or aborting the overall search, recording a non-fatal warning or skipping quietly.
- **Corrupted or unreadable files**: Malformed zip containers (`.docx`, `.docm`) or corrupted OLE compound structures (`.doc`) must be caught and reported as non-fatal errors or ignored.
- **Empty documents or documents with only images**: Files with no text content must complete scanning without matching and without error.
- **Very large documents**: Documents with hundreds of pages or thousands of paragraphs must be parsed efficiently using streaming or chunked reading to prevent UI hangs or excessive memory spikes.
- **Legacy binary `.doc` format variations**: Different revisions of Word97-2004 binary formats and Japanese codepages (CP932/Shift_JIS, UTF-16LE) must decode text accurately without garbled characters.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Settings view MUST provide an option to enable or disable MS Word document (`.doc`, `.docx`, `.docm`) searching.
- **FR-002**: The system MUST persist the Word search setting across application launches.
- **FR-003**: System default for Word document search MUST be disabled (opt-in) upon initial installation to maintain default Excel search focus and performance.
- **FR-004**: When Word document search is enabled, the main search bar MUST display `.docx`, `.docm`, and `.doc` toggle buttons alongside default extensions, and the search engine MUST queue enabled Word extensions for scanning.
- **FR-005**: The search engine MUST parse OOXML `.docx` and `.docm` files by extracting textual content from paragraphs and tables.
- **FR-006**: The search engine MUST parse legacy binary `.doc` (OLE2 Compound Document) files by extracting text from the WordDocument stream.
- **FR-007**: Scope of Word document text extraction MUST cover body paragraphs and tables by default; comments and shape/textbox text MUST be included when the respective "include_comment" and "include_shape" query options are enabled; headers and footers MUST be searched; Excel-specific options (formulas, hidden sheets) MUST be safely ignored during Word scanning.
- **FR-008**: In the search results table, Word document hits MUST display "Word" in the Sheet column, and display the corresponding line/paragraph index or "-" in the Cell column.
- **FR-009**: When a Word document result is selected, the preview component MUST render a text-formatted display showing the document's extracted text with line/paragraph numbers, automatically scroll to bring the matched hit into view, and highlight matching keywords.
- **FR-010**: Double-clicking or selecting "Open File" on a Word search hit MUST invoke the OS default application handler for the file type.
- **FR-011**: Exporting search results (CSV or XLSX) MUST support Word hit entries without corrupting export structure.

### Key Entities

- **WordSearchConfig**: Represents user configuration for Word document scanning (enabled/disabled, default extensions: `.docx`, `.docm`, `.doc`).
- **WordSearchResult**: A specialized or polymorphic variant of search result representing a match in a Word document, containing file path, text snippet, match offsets, and document location identifier (e.g., paragraph index or section).
- **TextPreviewPayload**: Data transfer representation containing extracted paragraph text, total character length, and highlight ranges for rendering in the text preview component.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can toggle MS Word search on or off in the Settings dialog in under 3 clicks or 5 seconds.
- **SC-002**: Scanning 100 Word documents (.doc, .docx, and .docm) completes with zero crashes, unhandled errors, or thread deadlocks.
- **SC-003**: Selecting a Word search result renders the text preview with highlighted matching keywords in under 300 milliseconds.
- **SC-004**: Text extraction from Japanese `.docx`, `.docm`, and legacy `.doc` files produces 0% mojibake (garbled characters) for standard Shift_JIS/CP932 and UTF-16 encoded documents.
- **SC-005**: User interface remains responsive (60fps) during active Word document scanning.

---

## Assumptions

- Users who search Word documents prioritize fast keyword location over complex layout rendering; plain text preview with clear line/paragraph separation is preferred.
- Macro-enabled Word documents (`.docm`) share the Open Packaging Conventions (OPC) zip container structure with `.docx`, using the identical `word/document.xml` text payload structure.
- Legacy `.doc` files are assumed to be Word 97-2003 OLE2 Compound File Binary format (older Word 6.0/95 formats may have best-effort extraction or require standard OLE2 parsing).
- The existing spreadsheet search flow remains unchanged and unaltered when Word document searching is disabled.
- In CLI mode, Word search can be controlled via a dedicated flag or included when Word extensions are specified in `--ext`.

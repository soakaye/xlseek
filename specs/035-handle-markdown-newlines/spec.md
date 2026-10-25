# Feature Specification: Handle Multiline Text in Markdown and CSV Search Results Export

**Feature Branch**: `035-handle-markdown-newlines`

**Created**: 2026-10-25

**Status**: Ready for Planning

**Input**: User description: "検索結果markdown 出力時の検索結果文字列内の改行の処理を行う。CSVにも同様の問題がある"

## Clarifications

### Session 2026-10-25

- **Q1: Markdown出力時の改行の表現形式 (Newline Tag Format)** (FR-001)
  - Selected: **Option A** (`<br>` タグへ変換)
  - Details: Markdownの表（Table）および見出し・セルのみ（Header & Cell Only）形式において、セル内改行（`\r\n`, `\n`, `\r`）を標準的な `<br>` タグに置換して行構造を維持する。
- **Q2: 連続する空行およびセル外側の余白の処理方針 (Consecutive Newlines & Outer Whitespace)** (FR-004)
  - Selected: **Option A** (先頭・末尾をトリムし、内部の連続改行はそのまま保持)
  - Details: セル先頭および末尾の不要な改行・空白文字はトリムし、セル内部の連続改行（例: `\n\n`）は忠実に `<br><br>` として保持する。
- **Q3: CSV出力時のセル内改行の処理方針 (CSV Newline Handling)** (FR-007)
  - Selected: **Option A** (半角スペースに置換)
  - Details: CSV出力時（GUIファイル保存およびCLI stdoutストリーミング）、セル値（`full_content`）や数式等に含まれる改行（`\r\n`, `\n`, `\r`）の先頭・末尾をトリムし、内部の改行を半角スペース `" "` に置換する。これにより、1検索結果＝厳格に1行（1レコード）を保証し、行指向ツール（grep, awk, head）やExcelでの一覧性・安全性を最大化する。

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Preserve Multiline Content in Markdown Table Export (Priority: P1) 🎯 MVP

As an Excel Seek user or analyst exporting search results to a Markdown document or copying AI context,
I want cells and matched texts containing line breaks to be preserved and readable within Markdown tables without breaking the table row layout or splitting columns across lines,
So that I can view multiline cell contents seamlessly in Markdown viewers and supply clean, structured tabular context to LLMs.

**Why this priority**:
Currently, line breaks in cells exported as Markdown tables are flattened into single spaces, causing multiline data (such as addresses, notes, bullet points within a cell, or logs) to lose their line breaks. Converting newlines to standard Markdown inline breaks (`<br>`) keeps table syntax intact while retaining multiline structure.

**Independent Test**:
Perform a search on sample files containing multiline cell values (e.g., cell content with multiple lines separated by `\n` or `\r\n`). Export the results to a Markdown (`.md`) file or copy table context for LLM. Open the generated Markdown in a standard Markdown previewer and verify that:
1. The table row remains a single valid table row (pipes `|` aligned, no row breakage).
2. The multiline cell displays line breaks clearly between its lines.

**Acceptance Scenarios**:

1. **Given** an Excel cell with multiline content (e.g., `"Line 1\nLine 2"`), **When** search results are exported to Markdown in table format, **Then** the cell content is rendered inside the table cell as `Line 1<br>Line 2`, preserving the line break without creating separate table rows.
2. **Given** cell content containing both pipe characters and newlines (e.g., `"Col A | Val\nCol B | Val"`), **When** exported to Markdown, **Then** the pipes are escaped (`\|`) and newlines are converted (`<br>`), producing a syntactically valid table row.
3. **Given** an Excel cell with Windows line endings (`\r\n`) or Unix line endings (`\n`), **When** exported to Markdown, **Then** both line ending variants are uniformly handled without leftover carriage return characters (`\r`).

---

### User Story 2 - Single-Line Guarantee in CSV Export (Priority: P2)

As a developer, sysadmin, or data analyst exporting search results to CSV for command-line processing or spreadsheet review,
I want cell values containing newlines to be flattened into single-line strings with spaces replacing newlines,
So that every search result strictly occupies exactly one physical line in the CSV file, preventing line-splitting issues when piping into `grep`, `awk`, `wc -l`, or simple CSV readers.

**Why this priority**:
In standard CSV, multiline cells generate multi-line quoted records. While compliant with RFC 4180, this severely breaks line-oriented pipelines (`grep`, `awk`, `while read line`) and makes reviewing search results in text viewers confusing. Replacing newlines with single spaces guarantees that `total_lines = 1 (header) + match_count`.

**Independent Test**:
Search a workbook with multiline cells (e.g., cell containing 3 lines). Export to CSV via GUI or stream to stdout via CLI (`xlseek-cli -p ./test -q "keyword"`). Count lines using a line counter and inspect the output: verify that the multiline cell is formatted on a single line with internal newlines replaced by single spaces.

**Acceptance Scenarios**:

1. **Given** an Excel cell with multiline content (e.g., `"Item A\nItem B"`), **When** exported to CSV (GUI file export or CLI stdout), **Then** the cell value in the CSV record is formatted as `"Item A Item B"` on a single physical line.
2. **Given** a search result set of N matches where some matches contain multiline cells, **When** exported to CSV, **Then** the resulting CSV output consists of exactly N + 1 lines (1 header line + N data rows).
3. **Given** a cell value with leading or trailing newlines/whitespace, **When** exported to CSV, **Then** the leading and trailing whitespace/newlines are trimmed prior to space substitution.

---

### User Story 3 - Clean Multiline Formatting in Header & Cell Only / List Mode (Priority: P3)

As a user who prefers the compact "Header & Cell Only" Markdown format for AI prompt ingestion,
I want multiline cell values in list items to be formatted without breaking Markdown bullet indentation or splitting bold formatting tags,
So that the list structure remains clean, readable, and easy for AI models to parse.

**Why this priority**:
In "Header & Cell Only" mode, matched values are formatted as bullet list entries with bold highlights (e.g., `- Matched Value: **{value}**`). If `{value}` contains unhandled raw newlines, subsequent lines break out of the bullet indent and split the bold `**` markers across lines, corrupting the document hierarchy.

**Independent Test**:
Export or copy search matches with multiline values using "Header & Cell Only" format. Verify that the output retains a clean bullet list hierarchy and bold tags close properly on each entry.

**Acceptance Scenarios**:

1. **Given** a search match with a multiline value in Header & Cell Only mode, **When** the Markdown is generated, **Then** the multiline value formats cleanly within the list item entry using `<br>` without breaking list indentation or leaving unclosed bold tags.
2. **Given** multiple search matches across different sheets or documents with multiline text, **When** exported, **Then** each match entry maintains consistent spacing and hierarchy.

---

### User Story 4 - Line Ending Normalization and Whitespace Hygiene (Priority: P4)

As a user working with heterogeneous spreadsheets created across different operating systems,
I want redundant outer blank lines and mixed line ending characters to be cleaned up automatically during Markdown and CSV exports,
So that exported documents remain tidy without unsightly empty lines at the top or bottom of table cells or records.

**Why this priority**:
Spreadsheet cells often accumulate unintentional leading/trailing newlines or mixed carriage returns. Trimming outer whitespace before generating Markdown or CSV ensures professional document presentation and minimizes token waste when providing context to LLMs, while preserving intentional internal blank lines in Markdown.

**Independent Test**:
Search a workbook with cells containing leading/trailing blank lines (e.g., `"\n\nHeader\n\nValue\n\n"`). Verify that the exported Markdown trims outer blank lines while preserving inner line breaks (`"Header<br><br>Value"`), and the exported CSV trims outer whitespace and replaces inner breaks with spaces (`"Header  Value"`).

**Acceptance Scenarios**:

1. **Given** a cell value with leading or trailing newlines or whitespace, **When** exported to Markdown or CSV, **Then** the leading and trailing whitespace/newlines are trimmed so that no empty `<br>` tags or leading spaces appear at the start or end of the value.
2. **Given** a cell value containing consecutive blank lines (e.g., `\n\n`), **When** exported to Markdown, **Then** internal separation is preserved cleanly as corresponding sequential line breaks (`<br><br>`).

---

### Edge Cases

- **Empty cell or cell with only newlines/whitespace**: Cell produces an empty string or standard placeholder without emitting orphaned `<br>` tags in Markdown or unnecessary spacing in CSV.
- **Single-line cell without newlines**: Standard cell text escaping applies without inserting `<br>` in Markdown, and plain text is output in CSV.
- **Cell containing commas, quotes, and newlines in CSV**: Standard CSV escaping (quoting fields with commas or quotes, doubling inner quotes) is preserved while newlines are converted to spaces.
- **Very long multiline content**: Multiple lines with large text blocks format cleanly without memory spikes or performance degradation.
- **Word document paragraphs and Shape text**: If Word paragraphs or Shape matches contain internal line breaks, they are normalized consistently with cell text formatting in both Markdown and CSV.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST replace newline characters (`\r\n`, `\n`, `\r`) in matched cell texts and search results during Markdown export and LLM context generation with standard `<br>` tags, preserving multiline structure within table cells and list entries without breaking Markdown syntax.
- **FR-002**: System MUST escape pipe characters (`|`) in Markdown table cells independently of newline processing so that table column delimiters are preserved.
- **FR-003**: System MUST normalize all newline conventions (`\r\n`, `\r`, `\n`) consistently across all supported document types (Excel cells, Shape text, Word paragraphs) in both Markdown and CSV export pipelines.
- **FR-004**: System MUST trim leading and trailing newlines and whitespace from matched cell contents in Markdown export while faithfully preserving internal consecutive line breaks as corresponding sequential `<br>` tags (e.g., `\n\n` -> `<br><br>`).
- **FR-005**: In Header & Cell Only Markdown mode, system MUST format multiline matched values using `<br>` so that Markdown bullet list structure and bold highlights remain syntactically valid and visually consistent.
- **FR-006**: System MUST apply identical multiline newline handling consistently across all Markdown generation channels (GUI `.md` file export, GUI AI Context copying to clipboard, and CLI `-f md` / stdout streaming).
- **FR-007**: System MUST normalize newline characters (`\r\n`, `\n`, `\r`) in CSV export fields (including cell `full_content` and `formula`) by trimming leading/trailing newlines/whitespace and replacing internal newlines with a single space (`" "`), strictly guaranteeing that each exported search match occupies exactly one physical line in the CSV output across both GUI file export and CLI stdout streaming.

---

### Key Entities *(include if feature involves data)*

- **Search Match Content**: The textual content of a matched cell, shape, or document paragraph, which may contain single-line or multi-line text with varying line-ending conventions.
- **Markdown Table Cell**: A single cell entry enclosed within pipe delimiters (`| ... |`) in a GitHub Flavored Markdown table, using `<br>` for inline multiline rendering.
- **CSV Data Record**: A single row of comma-separated values in an exported CSV file, strictly confined to a single physical line per search match.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of exported Markdown tables containing multiline cells render as syntactically valid tables without row splintering or column misalignment in standard Markdown parsers.
- **SC-002**: Multiline cell text retains its intended line separation in rendered Markdown output (lines are visually separated with `<br>` rather than flattened into a single space-separated string).
- **SC-003**: 100% of list-based Markdown outputs containing multiline cells maintain valid bullet indentation and properly closed bold styling tags.
- **SC-004**: 100% of exported CSV records for search matches with multiline cell contents occupy exactly one physical line (total lines in the CSV file strictly equals 1 header line + total match count).
- **SC-005**: Zero syntax corruption or unescaped character leakage when cell text contains combinations of pipes (`|`), commas (`,`), quotes (`"`), and newlines.
- **SC-006**: Performance overhead of newline processing during Markdown and CSV export remains undetectable (< 5% difference in export duration for 1,000 matches).

---

## Assumptions

- Standard GitHub Flavored Markdown (GFM) and CommonMark table parsers universally interpret `<br>` as an inline line break within table cells.
- In CSV export, converting internal newlines to spaces to enforce 1 record = 1 line is preferred for command-line tool compatibility (`grep`, `awk`) and tabular clarity over RFC 4180 multiline quoted fields.
- Trimming leading and trailing newlines/whitespace from cell content improves readability and token efficiency without altering the semantic meaning of the cell data.
- Existing Excel (`.xlsx`) and JSON export formats are unaffected by this Markdown and CSV newline enhancement.

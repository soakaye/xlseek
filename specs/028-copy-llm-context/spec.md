# Feature Specification: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context`

**Created**: 2026-10-19

**Status**: Ready for Planning

**Input**: User description: "「見出し自動付与」AI コンテキスト抽出 / プロンプトコピー\n\n課題:\nExcel で G42 セルの「未対応」や「150,000」がヒットしても、1行目の「列見出し」や A列の「項目名（行見出し）」が離れていると、周辺数セルのプレビューだけでは何のデータか判断しにくい。\n\nアイデア:\nヒットしたセルに対して、そのシートのヘッダー行（列見出し）と左端のキー列（行見出し）を自動で特定して組み合わせ、AI が理解しやすい Markdown テーブルや構造化プロンプト形式に整形する。\n\n利点:\nまずは「AI 向けに文脈付きでコピー（Copy for LLM）」ボタンとして実装すれば、API キー設定や外部通信機能をアプリに持たせなくても、普段使っている ChatGPT / Claude / Copilot に貼り付けるだけで即座に活用できる。将来的なアプリ内 AI 要約機能の基盤にもそのまま直結する。\n\n追加要件: 横長シート（多列）の列範囲切り出しは、オプションにて列数を指定可能にする。Word ドキュメントの文脈抽出もオプションで章・節タイトルとファイル名・ページ/段落番号および該当段落テキストを選択可能にする。右クリックコンテキストメニューでコピー種別を選択。"

## Clarifications

### Session 2026-10-19

- Q: クリップボードにコピーされる出力テキストの構造（フォーマット）はどうあるべきか？ → A: Option A (純粋な構造化コンテキストとして、ファイル名・シート名・セル番地・見出し情報およびMarkdownテーブルのみをコピーし、ユーザーが自身のプロンプト文脈に自由に挿入できるようにする)。
- Q: 「AI向けに文脈付きでコピー」のUI操作導線はどこに配置すべきか？ → A: Option A (プレビュー画面のツールバー、および検索結果テーブルの右クリックコンテキストメニューの両方に配置し、一覧からもプレビュー詳細からも即座にコピー可能とする)。
- Q: 検索結果で複数行を選択した場合のコピー挙動はどうあるべきか？ → A: Option B (複数選択時は同一シートのヒット行をまとめた単一の統合Markdownテーブルとしてコピーし、異なるシート/ブックの場合はシート単位でグループ化して出力する)。
- Q: 生成される Markdown テーブル内でヒットした対象セルを LLM が一目で識別できるようにどのような強調表示を施すべきですか？ → A: Option A (テーブル内のヒットセル値を Markdown 太字記法 `**値**` で強調し、テーブル上部のメタデータ部にも対象列名・番地・値を明記する)。
- Q: 横長シート（数十〜百列以上）において、該当行の Markdown テーブルにはどの列範囲を含めてコピーすべきですか？ → A: Option A (先頭キー列を必ず保持しつつ、ヒットセルの前後各5列、最大計約15列を抽出し、省略箇所は `...` 列で表す)。
- Q: Word 文書（.docx, .docm, .doc）の検索ヒットに対して「AI 向けに文脈付きでコピー」を実行した場合、どのような形式でコンテキストを出力すべきですか？ → A: Option A (直近の上位見出しと該当段落および前後文脈を Markdown 見出し・引用ブロック形式で整形して出力する)。
- Q: 横長シートの列切り出し数、Word 文書の抽出形式、および右クリックメニューの動作について追加のカスタマイズ要件はありますか？ → A: (1) 横長シートの列範囲切り出しはオプションにて列数を指定可能にする。(2) Word 文書の文脈抽出もオプションで「章・節タイトル付き」と「ファイル名・ページ/段落番号および該当段落テキスト」を選択可能にする。(3) 検索結果一覧の右クリックコンテキストメニューからコピー種別（表形式、見出しのみ、Word形式等）を直接選択可能にする。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - One-Click "Copy for LLM" with Header Context (Priority: P1)

As a user inspecting Excel search results,
I want to copy a matched cell along with its automatically identified column header (e.g. Row 1 "Status" or "Amount") and row header/key (e.g. Column A "Task ID" or "Client Name") with a single action,
So that I can paste it directly into an AI assistant (ChatGPT, Claude, Copilot) and immediately receive accurate analysis without manually tracing row and column headers across distant cells.

**Why this priority**:
This solves the primary pain point where isolated cell matches (such as cell G42 containing "未対応" or "150,000") lose their semantic meaning when detached from header rows and item columns.

**Independent Test**:
Perform a search that hits cell G42 in a test spreadsheet. Trigger "Copy for LLM" from either the preview toolbar or the search results context menu. Paste the clipboard contents into a text editor and verify that the column header from the top of column G and the row key from column A of row 42 are accurately included alongside the cell coordinate, value, sheet name, and file path.

**Acceptance Scenarios**:

1. **Given** a search result located in an arbitrary row/column (e.g., cell G42),
   **When** the user clicks "Copy for LLM" in the preview toolbar or selects it from the context menu,
   **Then** the clipboard is populated with pure structured context containing document metadata (file path, sheet, cell address, column name, row key, matched value), and a Markdown table with the hit cell highlighted in bold (`**value**`).
2. **Given** the copy operation completes successfully,
   **When** clipboard data is transferred,
   **Then** a brief non-intrusive toast notification confirms that the context has been copied for LLM use.
3. **Given** a user pastes the copied text into an LLM chat window,
   **When** submitted to the AI assistant,
   **Then** the assistant can unambiguously identify what data the cell represents based on the included headers without needing additional manual explanation.

---

### User Story 2 - Markdown Table Row Context Extraction and Multi-Selection Bundling (Priority: P2)

As a user needing broader situational context for an AI prompt or comparing multiple search hits,
I want the copied context to format the hit row as a Markdown table with column headers, and when multiple results from the same sheet are selected, consolidate them into a single unified Markdown table,
So that the AI assistant can analyze all relevant attributes and multiple matched records collectively.

**Why this priority**:
A single cell value often requires surrounding row attributes (e.g., date, category, status, amount) for the LLM to understand the record, and users frequently need to analyze or summarize several search results at once.

**Independent Test**:
Select multiple search results in the search results table from the same worksheet and trigger "Copy for LLM". Verify that the clipboard contains a single consolidated Markdown table sharing the sheet's column headers with each hit row included.

**Acceptance Scenarios**:

1. **Given** a single search hit within a multi-column worksheet,
   **When** context extraction executes,
   **Then** a Markdown table snippet representing the row aligned with its column headers is generated cleanly.
2. **Given** multiple search hits selected within the same worksheet,
   **When** "Copy for LLM" is invoked,
   **Then** all selected rows are consolidated into a single Markdown table sharing the header row, with hit cells highlighted in bold (`**value**`).
3. **Given** multiple search hits selected across different worksheets or workbooks,
   **When** "Copy for LLM" is invoked,
   **Then** the output groups hits by workbook and sheet, providing a distinct Markdown table section for each group.
4. **Given** rows containing empty cells, numbers, formulas, or special characters (`|`, line breaks),
   **When** formatted into the Markdown table,
   **Then** empty cells remain empty table cells and special characters are escaped to ensure valid Markdown syntax.

---

### User Story 3 - Robust Header & Key Column Heuristics (Priority: P3)

As a user working with real-world Excel workbooks that do not start neatly at A1,
I want the system to intelligently identify the true header row and leftmost key column even when worksheets have title banners, blank margin rows, or merged cells,
So that the copied context remains accurate across diverse spreadsheet layouts.

**Why this priority**:
Real-world enterprise spreadsheets frequently place report titles, company logos, or metadata in rows 1-3 before the actual tabular header row begins.

**Independent Test**:
Test on an Excel sheet where rows 1-2 contain title banners and row 3 contains column headers ("No", "Item", "Price"). Verify that searching a cell in row 10 correctly identifies row 3 as the header row rather than row 1.

**Acceptance Scenarios**:

1. **Given** a sheet where the data table starts below row 1 (e.g. row 3),
   **When** header detection runs,
   **Then** the system detects the populated header row above the hit cell and pairs the correct column title.
2. **Given** a sheet where column A is empty or contains row numbers, and column B contains the descriptive item name,
   **When** key column detection runs,
   **Then** the system identifies the primary descriptive key column or leftmost filled column.
3. **Given** an ambiguous or unstructured sheet where no clear header pattern is found,
   **When** detection fails or confidence is low,
   **Then** the system gracefully falls back to top-row/leftmost cell values without failing or throwing errors.

---

### User Story 4 - Context Menu Selection & Customization Options (Priority: P2)

As a user reviewing search results with diverse formatting needs,
I want to right-click any match item in the search results table to open a context menu that lets me choose the specific copy type directly, and configure column window limits and Word extraction modes in settings,
So that I have direct control over output granularity and prompt density for various LLM workflows.

**Why this priority**:
Different LLM tasks demand different prompt sizes (e.g. quick cell identification vs. full table context), and users need flexibility without having to edit copied text manually.

**Independent Test**:
Right-click on an Excel search result row. Verify that the context menu offers options to copy as "Table Context" or "Header & Cell Only". Right-click on a Word match item and verify that Word-specific copy choices appear. Open Settings and adjust the column window size; verify that subsequent copies adhere to the new column limit.

**Acceptance Scenarios**:

1. **Given** a user right-clicks an Excel match row in the search results table,
   **When** the context menu opens,
   **Then** options for copying context appear, including "Copy for LLM (Table Context)" and "Copy for LLM (Header & Cell Only)".
2. **Given** a user right-clicks a Word match row in the search results table,
   **When** the context menu opens,
   **Then** context menu options provide choices for "Copy for LLM (With Headings & Context)" and "Copy for LLM (Paragraph Only)".
3. **Given** a user configures the column window limit in application settings (e.g., set to 3 adjacent columns),
   **When** copying a hit from a wide sheet,
   **Then** the output Markdown table constrains columns to the leading key column plus 3 columns before and after the hit column.
4. **Given** a user selects a preferred default Word extraction mode in settings,
   **When** triggering the standard "Copy for LLM" shortcut or preview toolbar button on a Word hit,
   **Then** the system uses the configured default mode.

---

### Edge Cases

- **Hit cell is itself in the header row or column A**: System handles this gracefully without duplicating headers or self-referencing.
- **Merged header cells**: If the column header spans merged cells (e.g., grouped category headers across columns B through D), the merged title or combined path (e.g., "Category > Subcategory") is resolved where possible.
- **Extremely wide sheets (e.g., 50+ columns)**: System retains the primary key column (e.g., Column A) and extracts up to the user-configured number of adjacent columns before and after the hit column (default: 5 columns), using ellipsis columns (`...`) to denote omitted column ranges so that readability and token efficiency are preserved.
- **Non-Excel documents (e.g., Word documents)**: For Word document hits, the system supports extracting either (a) the nearest preceding heading/section title with the matched paragraph formatted in Markdown heading and blockquote syntax (`### Heading` + `> paragraph`), or (b) the file name, page/paragraph number, and paragraph text, according to user selection or settings.
- **Empty or whitespace-only cells in headers**: System traverses upward/leftward to find meaningful labels or falls back to column letters (e.g., "Column G") when headers are blank.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST identify the column header label corresponding to any matched Excel cell by locating the table header row above the cell.
- **FR-002**: System MUST identify the row header or key identifier corresponding to any matched Excel cell by locating the leftmost descriptive column of that row.
- **FR-003**: System MUST assemble the file path, sheet name, cell coordinates, header labels, and cell value into a structured text payload.
- **FR-004**: System MUST format the extracted row and column headers as a clean, valid Markdown table compatible with standard LLMs.
- **FR-005**: System MUST provide a dedicated action to trigger context copying to the system clipboard and show a success confirmation toast.
- **FR-006**: System MUST format the copied snippet as pure structured context containing document metadata (file path, sheet name, cell address), detected header labels, and a clean Markdown table representing the matched row(s), without injecting prescriptive analysis prompts.
- **FR-007**: System MUST provide the "Copy for LLM" trigger in both the preview panel toolbar and the search results table right-click context menu.
- **FR-008**: System MUST support multiple selected search results by bundling hits belonging to the same sheet into a single consolidated Markdown table with shared column headers, and grouping results cleanly by sheet/workbook when multiple sheets or workbooks are selected.
- **FR-009**: System MUST execute all context extraction, parsing, and formatting locally on the user's machine without transmitting file content or queries to external network services.
- **FR-010**: System MUST handle sheets with non-standard offsets (headers starting on rows 2-5) through heuristic detection with a safe fallback to row 1 / column A.
- **FR-011**: System MUST visually emphasize matched cell values within the generated Markdown table using bold Markdown formatting (`**value**`), and explicitly enumerate the target column name, cell address, and matched value in the preceding metadata block.
- **FR-012**: System MUST allow configuring the column window size (number of adjacent columns before/after the hit cell, default: 5) via application settings, preserving the leading key column and denoting pruned columns with ellipsis markers (`...`).
- **FR-013**: System MUST allow configuring Word document context extraction via options, supporting selectable modes: (a) section/chapter headings with paragraph blockquote, or (b) file name, page/paragraph number, and paragraph text.
- **FR-014**: System MUST provide a right-click context menu on search result rows allowing users to directly choose the desired LLM copy type (e.g., Structured Table Context, Header & Cell Only, or Word Context variants).
- **FR-015**: System MUST persist the user's configurable LLM copy options (column window size, default copy type, Word extraction mode) across application sessions in settings.

### Key Entities *(include if feature involves data)*

- **CellMatchContext**: Represents the target match including file path, sheet name, cell address (e.g., `G42`), raw value, formula (if any), and match type.
- **HeaderContext**: Represents the resolved context surrounding the target cell, including `column_header` (name, coordinate, row index), `row_header` (name, coordinate, column index), and `table_range` (detected bounding box of the table).
- **PromptSnippet**: The generated plain-text payload containing metadata and formatted Markdown table representation ready for clipboard transfer.
- **LlmCopyOptions**: Represents user configuration for context extraction: `column_window_size` (integer, default: 5), `word_extraction_mode` (`"headings_and_paragraph"` | `"paragraph_only"`), and `default_copy_type` (`"table"` | `"header_cell_only"`).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can copy complete LLM-ready context for any search result with a single click in under 200 milliseconds.
- **SC-002**: In over 90% of standard tabular spreadsheets, the system correctly pairs the matched cell with its intended column header and row identifier.
- **SC-003**: 100% of copied Markdown table snippets parse cleanly without broken table pipes or misaligned columns when pasted into standard LLM interfaces (ChatGPT, Claude, Copilot).
- **SC-004**: 100% of operations execute completely offline with zero external network transmission, strictly honoring privacy and security policies.
- **SC-005**: Right-click context menu opens immediately (<50ms) on any search result row, providing direct selection of copy types without blocking the UI thread.
- **SC-006**: User-configured column window sizes and Word extraction modes persist reliably across app restarts.

## Assumptions

- Users intend to paste the copied snippet into external AI chatbots or desktop LLM clients rather than requiring integrated in-app API keys.
- Spreadsheets predominantly follow standard vertical or tabular conventions where column names reside above data cells and row identifiers reside to the left.
- In-memory cell caching or fast sheet inspection from `xlseek-core` / `calamine` can retrieve header cells efficiently without noticeable UI lag.
- Settings for LLM copy options integrate smoothly into the existing settings dialog architecture (`DefaultSearchOptions` / `src/types/defaultOptions.ts`).

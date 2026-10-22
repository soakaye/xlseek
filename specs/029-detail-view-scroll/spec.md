# Feature Specification: Scrollable Match Detail Area in Search Preview

**Feature Branch**: `029-detail-view-scroll`

**Created**: 2026-10-22

**Status**: Ready for Planning

**Input**: User description: "Excel の検索結果の一致の詳細情報領域にスクロール機能を付ける。検索結果セルの文字列が長いとExcelのシート部が表示されず見えにくい。スクロール機能追加によりセル部と詳細情報領域の視認性を高める。 C:\Users\asuzuki.SSL\AppData\Local\Temp\orca-paste-1792661155807-1a4e29df-2a69-4c4d-92eb-33e72f7d9bc4.png"

## Clarifications

### Session 2026-10-22

- Q: 一致の詳細情報領域（セルメタ情報カード）の高さ制限とスクロールは、どのような制御方式にするべきですか？ (FR-001) → A: Option A (固定の最大高さ上限［max-height 約140px〜160px / 7〜8行分］を設定し、それを超える場合に自動で縦スクロール［overflow-y: auto］を表示する方式。スプリッター等の追加複雑性を避け、周辺セルプレビュー領域を確実に保護する)。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Long Cell Content Scrolling and Sheet Preview Preservation (Priority: P1)

As an Excel search user inspecting search results with long cell text (such as long sentences, paragraphs, or multi-line notes),
I want the match detail area to have an upper height limit with vertical scrolling,
So that the surrounding cell preview grid remains adequately sized and visible while I can still inspect the full matched text by scrolling.

**Why this priority**:
In the current application, when a matched cell contains dozens of lines or hundreds of characters, the match detail card expands vertically without bound. Because the preview pane has a fixed overall height, this unbounded expansion crushes the surrounding cell preview grid into near-zero height, preventing users from seeing the spreadsheet rows and sheet context. Constraining the detail card height and adding vertical scrolling directly resolves this visibility defect.

**Independent Test**:
Select an Excel search match containing over 10 lines of text (or long multi-line paragraphs). Verify that:
1. The surrounding cell grid retains sufficient visible height (at least 3 rows visible).
2. The match detail content box is capped at a maximum height and provides an interactive vertical scrollbar.
3. The user can scroll through the entire text content.

**Acceptance Scenarios**:

1. **Given** an Excel search result with long cell content (e.g., 5+ lines or over 200 characters), **When** the user clicks/selects the result in the results table, **Then** the spreadsheet preview grid preserves its visible display of surrounding cells, and the match detail content box displays an interactive vertical scrollbar without expanding into the grid area.
2. **Given** an Excel search result with short cell content (e.g., 1-2 lines or a short number), **When** the user selects the result, **Then** the match detail content box sizes naturally to fit the text without displaying unnecessary vertical scrollbars or blank vertical space.
3. **Given** a user is viewing long content within the match detail box, **When** the user scrolls using the mouse wheel, trackpad gesture, or drags the scrollbar, **Then** the detail text scrolls smoothly from beginning to end without causing outer layout shifting.

---

### User Story 2 - Keyboard Navigation and Text Selection in Detail View (Priority: P2)

As a keyboard-centric or accessibility user reviewing search results,
I want to navigate into the detail area, scroll through the text, and select/copy text portions,
So that I can verify and extract long text easily without relying entirely on precise mouse movements.

**Why this priority**:
Users frequently need to read through specific sections of long text and copy key portions. Enabling standard keyboard navigation and smooth selection within the scrollable container ensures productivity and accessibility.

**Independent Test**:
Focus or click within the scrollable detail box for a long cell match, and verify that Up/Down arrow keys or Page Up/Page Down keys scroll the text, and standard copy shortcuts (Ctrl+C / Cmd+C) copy the selected text.

**Acceptance Scenarios**:

1. **Given** the user has clicked or focused within the scrollable detail content box, **When** they press the Up/Down arrow keys or Page Up/Page Down keys, **Then** the content scrolls up or down accordingly.
2. **Given** the user selects text across lines within the scrollable detail area, **When** they execute a copy action (Ctrl+C or Cmd+C), **Then** the exact selected text is copied to the system clipboard.

---

### User Story 3 - Visual Distinction and Overflow Indicators (Priority: P3)

As a user glancing at the match detail area,
I want clear visual feedback (such as styled scrollbars or overflow hints) when text exceeds the visible container,
So that I immediately recognize that more content is available to scroll through rather than assuming the text was cut off.

**Why this priority**:
Without clear visual feedback, users might mistake a scrolled or bounded view for a truncated string. Visible scroll cues prevent confusion.

**Independent Test**:
Inspect a match with text exceeding the max height, and confirm that the scrollbar is visually apparent according to the application's theme.

**Acceptance Scenarios**:

1. **Given** a search result with text exceeding the visible boundary of the detail box, **When** rendered on screen, **Then** a visible scrollbar matching the dark theme is displayed.
2. **Given** a search result with text that completely fits within the visible boundary, **When** rendered on screen, **Then** no scrollbar is shown.

---

### Edge Cases

- **Extremely Large Text**: A single cell contains several thousand characters or thousands of lines (e.g., copied CSV/XML dumps). The scrollable container must render smoothly without UI freeze, clipping, or breaking parent flex layouts.
- **Explicit Newlines and Whitespace**: Text containing `\r\n` or `\n` line breaks must preserve its formatting and wrap lines appropriately within the scrollable container (`whitespace-pre-wrap` behavior).
- **Narrow or Short Window Heights**: When the desktop application window is resized to minimum dimensions, the preview layout must maintain proportional or minimum heights for both the spreadsheet grid and the detail card, preventing overlaps with window borders or the status bar.
- **Different Match Types**: Long text appearing in other match types (e.g. comments, shape text, or word documents) must consistently follow the same scrollable max-height rules when displayed in the metadata card.
- **Empty or Whitespace-Only Content**: When cell content is empty or contains only whitespace, the detail card renders the standard empty placeholder without broken scrollbars.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST constrain the height of the match detail content container to a fixed maximum boundary (approximately 140px to 160px / 7 to 8 lines) when displaying text that exceeds the default view height.
- **FR-002**: The system MUST provide interactive vertical scrolling (`overflow-y: auto`) within the match detail content container whenever text content overflows its maximum boundary.
- **FR-003**: The system MUST ensure that the spreadsheet preview grid (`周辺セルプレビュー`) maintains adequate visible vertical height to display surrounding rows and columns regardless of the length of the selected match's text.
- **FR-004**: The system MUST adaptively fit the height of the match detail container to short text so that no scrollbars or excessive empty whitespace are rendered when the text fits within the boundary.
- **FR-005**: The system MUST preserve line breaks and whitespace formatting (`whitespace-pre-wrap`) for multi-line cell values, formulas, comments, and shape text inside the scrollable content container.
- **FR-006**: The system MUST allow users to highlight, select, and copy text within the scrollable detail container.
- **FR-007**: The system MUST keep header metadata (match title, match type) and footer metadata (row index, column index, sheet visibility status) statically visible outside the scrollable text viewport.
- **FR-008**: The system MUST prevent scrolling inside the match detail container from causing accidental horizontal scrolling or parent window layout shifting.

### Key Entities

- **Match Detail View (一致の詳細情報領域)**: The preview pane section presenting the full text, match type, coordinate metadata, and hidden status of the selected search result.
- **Spreadsheet Grid Preview (周辺セルプレビュー)**: The interactive grid showing surrounding cells of the active search match.
- **Detail Content Viewport (詳細テキスト表示領域)**: The specific scrollable box inside the match detail card that renders the formatted string content.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of search result selections involving multi-line or long cell content (over 5 lines or 200 characters), the spreadsheet cell preview grid remains visible with at least 3 rows visible to the user.
- **SC-002**: Users can read 100% of long text content within the match detail area via standard mouse wheel, trackpad scrolling, or scrollbar dragging.
- **SC-003**: For single-line or short text (1-2 lines), the detail area renders without vertical scrollbars and with zero redundant empty space.
- **SC-004**: Zero layout clipping, zero horizontal overflow, and zero overlap with the window frame or status bar across all supported desktop window sizes (minimum supported window resolution 1024x768 and above).

## Assumptions

- The maximum height for the match detail text box is set to a fixed upper bound of approximately 140px to 160px (e.g. `max-h-36` to `max-h-40`), ensuring that the spreadsheet grid preview retains dominant visual focus with at least 3 rows visible while the detail text is comfortably readable.
- Standard desktop scrolling physics, mouse wheel interactions, and OS clipboard operations apply.
- Dark theme visual styling is maintained, consistent with existing borders (`border-zinc-800`), backgrounds (`bg-zinc-950/80`), and text colors (`text-zinc-300`).

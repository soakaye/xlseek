# Feature Specification: Clarify Search Results Save Button in Status Bar

**Feature Branch**: `034-clarify-save-results-button`

**Created**: 2026-10-25

**Status**: Draft

**Input**: User description: "検索結果の保存ボタンがわかりにくいので、わかりやすくする。現在出力フォーマットのみが表示され何のボタンかわからない。検索結果の保存であることを明示する。"

## Clarifications

### Session 2026-10-25

- **Q1: ボタン表示ラベルとデザインの形式 (Button Label & Visual Presentation)** (FR-001)
  - Question: 現在ステータスバーの保存ボタンにはフォーマット名（例: `Excel (.xlsx)`）のみが表示されています。「検索結果の保存」であることを明示するにあたり、どのラベル・アイコンの表示スタイルを採用しますか？
  - Selected: **Option A** (`[ファイルアイコン] 結果を保存: Excel (.xlsx)` / `Save Results: Excel (.xlsx)`)
  - Details:
    - Japanese label: `結果を保存: {フォーマット名}` (例: `結果を保存: Excel (.xlsx)`, `結果を保存: CSV (.csv)`, `結果を保存: AI用 Markdown (.md)`, `結果を保存: JSON (.json)`)
    - English label: `Save Results: {Format}` (e.g., `Save Results: Excel (.xlsx)`, `Save Results: CSV (.csv)`, `Save Results: AI Markdown (.md)`, `Save Results: JSON (.json)`)
    - Icon: 既存の形式別ファイルアイコン（Excel: `FileSpreadsheet`, CSV: `FileText`, Markdown: `FileCode`, JSON: `Braces`）を維持
    - Tooltip & Accessible description: 検索結果の保存であることを明記（JA: `検索結果を保存 (Excel)` / EN: `Save search results (Excel)`）

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Explicit Save Action on Results Export Button (Priority: P1) 🎯 MVP

As an Excel Seek user reviewing search hits in the results table,
I want the primary export button in the status bar to explicitly display "結果を保存: {Format}" (or "Save Results: {Format}" in English) along with the format icon,
So that I immediately understand that clicking the button will save the search results to a file without any ambiguity.

**Why this priority**:
Currently, the split button displays only the format name (e.g., `Excel (.xlsx)`). Users perceive this as a passive format tag rather than a clickable save button. Adding the explicit action prefix `結果を保存: ` / `Save Results: ` directly addresses user feedback while keeping the status bar compact and readable.

**Independent Test**:
Perform a search yielding one or more matches. Look at the bottom status bar and verify that the button displays the format icon followed by `結果を保存: Excel (.xlsx)` (or `Save Results: Excel (.xlsx)`). Click the button to confirm that it immediately opens the native file save dialog for the selected format.

**Acceptance Scenarios**:

1. **Given** search results are displayed, **When** the user views the bottom status bar, **Then** the primary action button explicitly displays the format icon and `結果を保存: {Format}` (in Japanese) or `Save Results: {Format}` (in English), clearly communicating that clicking it will save the search results.
2. **Given** a completed search with results, **When** the user clicks the primary action button, **Then** the operating system file save dialog opens immediately for the active format (preserving existing export behavior).
3. **Given** zero search results (idle state or search producing 0 matches), **When** viewing the status bar, **Then** the button clearly displays the save action label while rendered in its disabled visual state.

---

### User Story 2 - Clear Format Switching and Accessible Descriptions (Priority: P2)

As an Excel Seek user who wants to export results into different formats,
I want the dropdown format selection menu, tooltips, and accessibility attributes to clearly reinforce that each option saves the search results in the chosen format,
So that switching formats and keyboard/screen-reader navigation remain completely clear and intuitive.

**Why this priority**:
Users who need CSV, AI Markdown, or JSON need to know that choosing an option in the menu sets the format for saving search results and triggers the save workflow.

**Independent Test**:
Click the dropdown arrow (`▾`) to open the format menu. Verify that the menu items and the button's tooltip / `aria-label` clearly state that search results will be saved in the chosen format. Select a format and verify that the button label updates immediately to show the new active format with the explicit `結果を保存: ` prefix.

**Acceptance Scenarios**:

1. **Given** search results are present, **When** the user hovers over or focuses the primary button or format dropdown toggle (`▾`), **Then** the tooltip and `aria-label` explicitly describe saving the search results (e.g., "検索結果を保存 (Excel)" / "Save search results (Excel)").
2. **Given** the format dropdown menu is opened, **When** the user selects an alternative format (e.g., CSV, AI Markdown, JSON), **Then** the menu closes, the primary button label updates immediately (e.g., to `結果を保存: CSV (.csv)`), the choice is persisted in local storage, and the save dialog opens for that format.

---

### User Story 3 - Responsive and Uncluttered Status Bar Layout (Priority: P3)

As an Excel Seek user working on screens of various sizes and resolutions,
I want the updated, more explicit export button to fit cleanly into the status bar alongside the progress indicators, status text, and settings button,
So that no visual clipping, awkward wrapping, or layout distortion occurs at any supported window size.

**Why this priority**:
Adding text to the button increases its width. The status bar must accommodate this gracefully without disrupting the progress bar or causing the status bar height to expand.

**Independent Test**:
Resize the application window to its minimum supported width (800px). Verify that the status bar maintains its fixed height (40px / `h-10`), the export split button is fully visible and clickable without wrapping, and the status text truncates cleanly with ellipsis (`...`).

**Acceptance Scenarios**:

1. **Given** a narrow application window (800px) with active search results, **When** the status bar renders, **Then** the status bar retains its single-line height (`h-10`), the export split button remains fully visible on the right, and variable-length status text truncates cleanly with ellipsis.
2. **Given** switching between English and Japanese languages, **When** the status bar is rendered in either language, **Then** button labels fit comfortably without overflowing their containers or overlapping neighboring elements.

---

### Edge Cases

- **Zero Search Results**: When no results are available, the button is disabled (`opacity-40`, `cursor-not-allowed`) but the explicit save label and active format remain readable so users know where the export action will be once results are found.
- **Format Switch Persistence**: When switching from Excel to CSV, Markdown, or JSON, the new format is stored in local storage and correctly restored on subsequent application launches, displaying the explicit save label with the persisted format.
- **Rapid Window Resizing**: The flex layout in the status bar prevents the export button from being pushed out of the viewport; the flex-shrink-0 container guarantees button accessibility at all times.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The primary export button in the status bar MUST explicitly indicate that its action is saving/exporting search results. Specifically, the button label MUST display an action prefix followed by the active format:
  - In Japanese: `結果を保存: {Format}` (e.g., `結果を保存: Excel (.xlsx)`, `結果を保存: CSV (.csv)`, `結果を保存: AI用 Markdown (.md)`, `結果を保存: JSON (.json)`)
  - In English: `Save Results: {Format}` (e.g., `Save Results: Excel (.xlsx)`, `Save Results: CSV (.csv)`, `Save Results: AI Markdown (.md)`, `Save Results: JSON (.json)`)
  - Visual icon: Retain the format-specific file icons (`FileSpreadsheet`, `FileText`, `FileCode`, `Braces`).
- **FR-002**: The button MUST continue to indicate the currently active export format alongside the save action indicator.
- **FR-003**: The split button structure MUST be preserved: clicking the primary button triggers immediate export in the active format; clicking the dropdown toggle (`▾`) opens the format selection menu.
- **FR-004**: When an alternative format is selected from the menu, the primary button's label MUST update immediately to show the new format with the explicit save action, and save the choice to persistent storage.
- **FR-005**: All button text, menu labels, tooltips, and accessible labels (`aria-label`) MUST be managed centrally through the localization catalogs (`crates/core/locales/ja.yml`, `crates/core/locales/en.yml`) and centralized constants (`src/constants/index.ts`), with zero hardcoded strings.
- **FR-006**: The standalone HTML mockup (`design/mainui/index.html`) MUST be updated in synchronization with the component to reflect the improved button design.

### Key Entities *(include if feature involves data)*

- **ExportSplitButton**: Status bar UI element comprising a primary action button and a secondary format selector toggle.
- **ExportFormatOption**: Data entity defining the export format (`xlsx`, `csv`, `md`, `json`), localized label keys, filter keys, and associated icon.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of users viewing the status bar can immediately identify the button as saving search results without having to hover over it or click it to discover its function.
- **SC-002**: Zero regressions in export functionality: clicking the primary button opens the file save dialog in under 200ms; all 4 formats (Excel, CSV, Markdown, JSON) continue to export accurately.
- **SC-003**: Status bar remains strictly single-line at fixed 40px height with zero layout shifts or wrapping across window widths from 800px upward.
- **SC-004**: All verification checks pass with zero warnings (`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `npm run build`).

## Assumptions

- The existing split button mechanism (primary button for direct export, chevron button for dropdown format selection) is well-liked and should be retained; only the button's labeling/visual presentation needs clarification.
- The button is positioned in the status bar footer, which has sufficient horizontal space to accommodate the expanded label (e.g. "結果を保存: Excel (.xlsx)" or "Save Results: Excel (.xlsx)").
- Excel (`.xlsx`) remains the default export format for fresh installations.

# Research & Architecture Decisions: Scrollable Match Detail Area in Search Preview

**Feature Branch**: `029-detail-view-scroll` | **Date**: 2026-10-22 | **Spec**: [spec.md](./spec.md)

This document records the research findings, design tradeoffs, and technical decisions made during Phase 0 for Feature 029 (Scrollable Match Detail Area and Sheet Preview Preservation).

---

## 1. Match Detail Height Constraint and Scroll Mechanism

### Problem
In `MetaInfoCard.tsx`, the cell content display container (`full_content` viewport) previously only specified `break-words` without any upper height limit (`max-height`) or vertical scroll setting (`overflow-y`). Consequently, when a search match contains long multi-line sentences (e.g. 20+ lines or hundreds of characters), the card expands vertically without bound.
Because the parent preview pane has a fixed vertical height constrained by the application window, this unbounded expansion crushes the `SpreadsheetGrid` (surrounding cells preview) located above it, leaving only the header row `# A B` visible (near-zero rows displayed) and defeating the purpose of the cell preview.

### Decision
Apply a fixed maximum height boundary (`max-height: 9rem` / `144px`, Tailwind class `max-h-36`) to the cell content viewport in `MetaInfoCard.tsx`, enabling automatic vertical scrolling (`overflow-y-auto`) when content exceeds the boundary:
1. **Max Height and Scrolling**: `max-h-36 overflow-y-auto` (Tailwind CSS)
2. **Whitespace and Line Breaks**: Add `whitespace-pre-wrap` so that explicit line breaks (`\r\n`, `\n`) and whitespace inside Excel cells are preserved and properly wrapped.
3. **Scroll Chaining Prevention**: Add `overscroll-contain` to prevent mouse wheel scrolling inside the detail box from chaining to the parent pane or window when reaching boundaries.
4. **Centralized Constants**: Define `META_DETAIL_MAX_HEIGHT_CLASS: "max-h-36"` and `META_DETAIL_MAX_HEIGHT_PX: 144` in `LAYOUT_CONSTANTS` in `src/constants/index.ts`, eliminating magic constants per Constitution Principle II.

### Rationale
- **Guaranteed Spreadsheet Grid Visibility**: By capping the total height of the metadata card to approximately 200px (header + 144px content + coordinates), the surrounding spreadsheet preview (`SpreadsheetGrid`) is guaranteed ample vertical display space (at least 3 to 5 rows, typically 250px–450px) across standard desktop screen resolutions (1024x768 and up).
- **Simplicity and Robustness**: Introducing a draggable splitter pane would require mouse drag listeners, min/max clamp bounds, resize persistence, and substantial edge-case testing. A CSS-driven fixed `max-h` approach resolves the visibility problem natively without UI state overhead or regression risk (as agreed in Clarification Session Q1, Option A).
- **Natural Fit for Short Content**: Because `max-h-36` represents an upper limit, short 1–2 line cell values size down naturally without empty whitespace or unnecessary scrollbars.

### Alternatives Considered
- **Alternative A: Draggable Splitter Bar**:
  - *Pros*: Users could manually adjust pane heights on the fly.
  - *Cons*: Significant state management and drag-listener complexity, risk of layout glitch during window resize, and unnecessary friction for a straightforward display issue (violates YAGNI).
- **Alternative B: Percentage-based Max Height (e.g. `max-h-[35%]`)*:
  - *Pros*: Dynamically scales with window height.
  - *Cons*: When the window is resized to small heights, the detail box shrinks excessively to 1–2 lines, hurting readability. A fixed `max-h-36` provides predictable readability (~7–8 lines) while keeping the grid visible.

---

## 2. Line Break Preservation and Word Wrapping

### Problem
Excel cells often contain newline characters (`Alt+Enter`, formatted as `\n` or `\r\n`). With only `break-words`, multi-line formatting can be collapsed or rendered unpredictably.

### Decision
Apply `whitespace-pre-wrap` alongside `break-words`.
- `whitespace-pre-wrap`: Preserves sequences of whitespace and newline characters while wrapping lines at boundary edges.
- `break-words`: Ensures long unbroken alphanumeric strings do not overflow horizontally.

### Rationale
The screenshot provided by the user exhibits itemized bullet points and QA format (`1) 確認: ... 回答: ...`). Preserving explicit line breaks restores the formatting intended in the Excel document.

---

## 3. Mockup Synchronization (HTML Prototype)

### Requirement
Per `AGENTS.md` Section 5 ("Keep the UI Mockup in Sync"), any modification to component structure or visual styles in `src/` must be mirrored in the standalone HTML prototype at `design/mainui/index.html`.

### Decision
Update the `#metaFullContent` element in `design/mainui/index.html` with the identical class list (`max-h-36 overflow-y-auto overscroll-contain whitespace-pre-wrap`).

---

## 4. Test Strategy

### Decision
Create `tests/meta-info-card.test.tsx` using Vitest and `@testing-library/react` to verify:
1. **Short Text Rendering**: Content renders correctly with `max-h-36`, `overflow-y-auto`, and `whitespace-pre-wrap`.
2. **Multi-line / Long Text Rendering**: Multi-line strings render without crashing, safely contained within the scrollable viewport.
3. **Empty Content Fallback**: An empty string falls back cleanly to `ui.EMPTY_CONTENT`.
4. **Null Match Safety**: Passing `match={null}` safely returns `null` without throwing errors.
5. **Metadata Preservation**: Coordinate labels (row index, column index, hidden sheet status) render statically outside the scrollable text box.

Complete verification against all workspace quality gates (`npm run build`, `npm test`, `cargo test --workspace`, `cargo clippy`, `cargo fmt`).

# Feature Specification: Horizontal Side-by-Side Pattern Inputs Layout

**Feature Branch**: `038-horizontal-pattern-inputs`

**Created**: 2026-10-28

**Status**: Draft

**Input**: User description: "検索ファイルパターンと無視パターンが立て(Row)に並んでいるが、指定数が多くは無いので横に並べる"

## Clarifications

### Session 2026-10-28

- Q: Should the horizontal side-by-side layout be applied exclusively to the main search bar, or should the Settings dialog also place the default pattern options side-by-side? → A: Main search bar and UI mockup only; the Settings dialog retains its full-width card layout for readability of descriptive help text.
- Q: How should the pattern input controls adapt on narrow window widths (responsive behavior)? → A: Use responsive threshold `md:` (768px): side-by-side 2-column layout (`grid-cols-1 md:grid-cols-2`) at >= 768px, falling back cleanly to stacked 1-column layout when viewport width is below 768px.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Side-by-Side Alignment of File Search and Ignore Patterns (Priority: P1)

Users currently see the File Search Patterns input and the Ignore Patterns input placed in two separate vertical rows beneath the search directory row. Because file and ignore pattern entries are usually short (e.g. `*budget*, *.xlsx` or `~$*, *.tmp`), dedicating two full-width rows consumes excessive vertical screen space in the search header. Users want both controls placed horizontally side-by-side in a single row, maximizing vertical room for search results and previews.

**Why this priority**: Directly implements the requested layout optimization, reducing the search bar's vertical footprint while keeping both pattern filters readily accessible at a glance.

**Independent Test**: Can be tested by launching the application or inspecting the search bar: verify that the File Search Patterns input and the Ignore Patterns input share a single horizontal row on standard desktop resolutions.

**Acceptance Scenarios**:

1. **Given** the application is open at standard desktop window dimensions, **When** the user views the search panel, **Then** the File Search Patterns control appears on the left half and the Ignore Patterns control appears on the right half within the same horizontal row.
2. **Given** both pattern controls are placed side-by-side, **When** the user reads the row from left to right, **Then** the sequence reflects the filtering execution order (inclusion search patterns first, exclusion ignore patterns second).

---

### User Story 2 - Full Functionality and History Dropdown Usability (Priority: P1)

Users frequently interact with each pattern input by typing patterns, clearing text with the inline `X` button, selecting recent patterns from the history dropdown, or pressing Enter to execute a search. These controls must retain all existing interactivity and accessible controls when laid out side-by-side.

**Why this priority**: Changing the layout must not break or degrade any existing features, keyboard shortcuts, history selection, or clear actions.

**Independent Test**: Can be tested by typing values into both inputs, clicking the clear buttons, opening each history dropdown, and selecting an item from history to confirm seamless operation.

**Acceptance Scenarios**:

1. **Given** text is entered into either the file patterns or ignore patterns input, **When** the user clicks the respective clear (`X`) button, **Then** only that specific input is cleared.
2. **Given** recent pattern history exists, **When** the user clicks the history button or presses the history toggle shortcut on either input, **Then** that input's dropdown opens directly below its own container with appropriate z-index layering without obscuring or conflicting with the adjacent control.
3. **Given** focus is inside either pattern input, **When** the user presses Enter, **Then** the search is triggered immediately just as before.

---

### User Story 3 - Responsive Layout on Narrow Windows (Priority: P2)

When the application window is resized or opened on a compact display where half-width columns might cause input fields or labels to become excessively cramped, the layout should handle narrow viewport widths cleanly.

**Why this priority**: Ensures robust visual appearance and usability across various monitor sizes and split-screen desktop arrangements.

**Independent Test**: Can be tested by resizing the window to narrow dimensions (< 768px) and confirming that controls stack vertically without overlapping or breaking out of bounds.

**Acceptance Scenarios**:

1. **Given** the window is resized to a narrow width below 768px (Tailwind `md:`), **When** viewport space is constrained, **Then** the pattern controls stack vertically in single columns, preventing button clipping or cramped input widths.
2. **Given** the window width is 768px or greater, **When** rendered, **Then** the pattern controls display side-by-side in 2 equal-width columns.

---

### Edge Cases

- **Long pattern strings**: When a user pastes a long comma-separated list of patterns, the input field must scroll horizontally inside its input box, maintaining the visibility and clickability of the trailing action buttons (clear `X`, history dropdown, `Enter` badge).
- **Both history dropdowns toggled**: If a user clicks the history button of one input while the other's dropdown is open, the previous dropdown must close and the newly clicked one must open smoothly without layout jitter.
- **IME Composition**: During Japanese / CJK IME composition in either pattern input field, pressing Enter to commit candidate characters must not trigger search or premature history selection.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The search interface MUST display the File Search Patterns control and the Ignore Patterns control horizontally side-by-side in a single row container on standard desktop window sizes (`md:` breakpoint >= 768px). This layout applies to the main search bar and standalone HTML UI mockup; the Settings dialog retains its full-width card layout.
- **FR-002**: The layout MUST place the File Search Patterns control on the left and the Ignore Patterns control on the right, mirroring the logical execution order of inclusion filtering followed by ignore filtering.
- **FR-003**: Each pattern control MUST preserve its complete set of interface elements:
  - Identifying icon (FileText for search patterns, Filter for ignore patterns)
  - Label text (localized via translation catalogs)
  - Text input field with localized placeholder and tooltip
  - Clear (`X`) button visible when input is non-empty
  - History dropdown trigger button
  - Keyboard hint badge (`Enter`)
- **FR-004**: Each history dropdown overlay MUST be anchored to its respective input container, opening downwards with proper stacking context (`z-index`) and closing upon blur, item selection, or outside click.
- **FR-005**: The layout MUST allocate balanced width (e.g., 50% / 50% via `grid-cols-2`) to each control, providing sufficient horizontal room to display typical patterns comfortably.
- **FR-006**: On constrained viewport widths below the 768px threshold (Tailwind `md:`), the pattern controls MUST stack vertically in single columns to preserve full button visibility and comfortable input widths without truncation or overflow.
- **FR-007**: The standalone HTML UI mockup (`design/mainui/index.html`) MUST be updated to mirror the horizontal side-by-side pattern layout.

### Key Entities

- **SearchQuery**: Carries search parameters including `file_patterns` (inclusion string) and `ignore_patterns` (exclusion string).
- **PatternFilterControl**: Visual and interactive container encapsulating label, input, clear button, history dropdown trigger, and history dropdown overlay.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Vertical height dedicated to pattern filtering controls in the search header is reduced by 1 row (approximately 36-44 pixels saved on standard desktop displays), increasing visible space for search results.
- **SC-002**: 100% of interactive capabilities (typing, clearing, history dropdown selection, Enter-to-search, IME composition) continue to function identically to the previous implementation.
- **SC-003**: Both pattern inputs maintain at least 150-200 pixels of usable text input width at standard default window width (1000px+), allowing users to read common patterns without cramped styling.
- **SC-004**: Zero TypeScript type errors, zero Clippy warnings, and all automated tests across frontend and backend pass.

## Assumptions

- Standard desktop window width for xlseek is typically 900px or greater, where a 50/50 two-column split provides ample space for pattern inputs.
- Users specify concise patterns (such as `*budget*, *.xlsx` and `~$*, *.tmp`), making full-width single rows unnecessarily spacious.
- Backend filtering logic, CLI behavior, settings defaults, and MCP tool interfaces are completely unaffected by this frontend layout refinement.

# Research: Horizontal Side-by-Side Pattern Inputs Layout

**Feature Branch**: `038-horizontal-pattern-inputs`  
**Date**: 2026-10-28  
**Spec Reference**: [spec.md](./spec.md)

---

## 1. Grid & Responsive Layout Architecture

### Decision
Use Tailwind CSS responsive grid classes on a single container row:
```html
<div className="grid grid-cols-1 md:grid-cols-2 gap-3 items-center">
  <!-- Left: File Search Patterns Column -->
  <div ref={filePatternInputRef} onBlur={...} className="relative">
    ...
  </div>
  <!-- Right: Ignore Patterns Column -->
  <div ref={ignoreInputRef} onBlur={...} className="relative">
    ...
  </div>
</div>
```

### Rationale
- **Vertical space efficiency**: Merges two consecutive 12-column grid rows into a single row, immediately saving ~36-44px of vertical screen height on desktop viewports.
- **Natural processing flow**: In English/Japanese LTR reading, Left-to-Right reflects the execution pipeline: inclusion file search patterns (`file_patterns`) are evaluated first, followed by exclusion ignore patterns (`ignore_patterns`).
- **Responsive fallback**: On desktop and tablet viewports (window width $\ge 768$px, Tailwind `md:`), both controls sit side-by-side in equal-width columns (`md:grid-cols-2`). On narrow viewports ($< 768$px), the grid automatically falls back to single columns (`grid-cols-1`), preventing button overlap or squeezed input boxes.
- **Alternatives considered**:
  - *Flexbox (`flex flex-wrap`)*: Flexbox requires explicit `flex-1 min-w-[...]` and manual wrap margin adjustments. CSS grid (`grid-cols-1 md:grid-cols-2 gap-3`) provides cleaner, symmetric 50/50 column distribution with uniform gap management.
  - *Keep as separate rows (`grid-cols-12` each)*: Rejected by user request; pattern inputs are typically short (e.g. `*budget*, *.xlsx` and `~$*, *.tmp`), making full-width single rows an inefficient use of vertical space.

---

## 2. Dropdown Stacking Context and Layering (`z-index`)

### Decision
Anchor each history dropdown overlay (`FilePatternHistoryDropdown` and `IgnoreHistoryDropdown`) inside its respective column container using `relative` container positioning and `absolute top-full mt-1 w-full z-30`:

```html
<div className="relative flex-1">
  <input ... />
  <div className="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
    ...buttons...
  </div>
  <FilePatternHistoryDropdown isOpen={openList === "file-pattern-history"} ... />
</div>
```

### Rationale
- Anchoring the absolute dropdown to `div.relative.flex-1` ensures the dropdown width matches the input field exactly, aligning neatly below the input box without extending under the label.
- A `z-index` of `z-20` or `z-30` guarantees the open dropdown displays cleanly above subsequent search options rows, radio buttons, and the results table.
- When one history dropdown is opened, the existing `openList` state in `SearchBar.tsx` can only hold one active dropdown identifier (`"file-pattern-history"`, `"ignore-history"`, or `"path"`), ensuring mutual exclusivity and zero visual overlap between the two adjacent pattern dropdowns.
- **Alternatives considered**:
  - *Fixed positioning / Portal*: Unnecessary complexity; the search container has no clipping `overflow: hidden` on its outer wrapper that would cut off the dropdown menu.

---

## 3. Label and Action Button Space Allocation

### Decision
Preserve identical inner structure for both controls:
1. Identifying Icon + Label (`flex-shrink-0 select-none`):
   - Left: `FileText` icon + `t("ui.FILE_PATTERNS_LABEL")` (`"検索パターン:"` / `"Search patterns:"`)
   - Right: `Filter` icon + `t("ui.IGNORE_PATTERNS_LABEL")` (`"除外パターン:"` / `"Ignore patterns:"`)
2. Flexible Input Box (`flex-1 relative`):
   - Text input with `pl-3 pr-20 py-1.5 font-mono text-xs`
   - Inline action tray:
     - Clear (`X`) button (conditionally rendered when input length > 0)
     - History trigger button (`History` icon)
     - Enter shortcut badge (`Enter`)

### Rationale
- At default desktop window widths (1000px+), each 50% column has approximately 460px.
- The label occupies ~110px. The remaining ~350px provides ample room for the input box and trailing 3 action icons (occupying ~70px of right padding `pr-20`), leaving ~280px for text (accommodating ~30-35 monospace characters without scrolling).
- If pattern strings exceed the visible area, native input horizontal scrolling handles overflow automatically.
- **Alternatives considered**:
  - *Shorten or omit labels (icons only)*: Rejected because clear text labels provide superior accessibility and immediate recognition without relying on tooltips.

---

## 4. UI Prototype Synchronization

### Decision
Update `design/mainui/index.html` to replace the two separate pattern rows (`filePatternControl` row and `ignorePatternControl` row) with a single responsive grid row:
```html
<!-- 中段: ファイル検索パターン & 無視パターン指定 (横並び) -->
<div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-center">
  <div id="filePatternControl" class="relative"> ... </div>
  <div id="ignorePatternControl" class="relative"> ... </div>
</div>
```

### Rationale
- Complies with AGENTS.md rule: "Keep the UI Mockup in Sync: If you change component structure, visual styles, or UI options under `src/`, use the syncing-mainui-mock skill to keep the standalone HTML prototype at `design/mainui/index.html` consistent."

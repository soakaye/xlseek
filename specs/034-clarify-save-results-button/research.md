# Research: Clarify Search Results Save Button in Status Bar

**Feature**: `034-clarify-save-results-button`
**Date**: 2026-10-25

## 1. Research Questions & Decisions

### Decision 1: Label Construction Pattern and Localization

- **Problem**: How to dynamically construct the primary button label with the action prefix `結果を保存: ` / `Save Results: ` while ensuring natural localized strings, easy extensibility, and compliance with Constitution Principle II (No Hardcoded Constants)?
- **Options Considered**:
  1. *Composite Template String with Parameter Interpolation*: Define a format template key `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT` in `ja.yml` (`"結果を保存: {format}"`) and `en.yml` (`"Save Results: {format}"`), evaluated via `t(EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT, { format: t(activeOption.labelKey) })`.
  2. *Individual Static Translation Keys per Format*: Define separate keys like `ui.EXPORT_SAVE_XLSX`, `ui.EXPORT_SAVE_CSV`, etc.
  3. *Hardcoded Concatenation in Component*: Concatenate localized prefix with format string in JSX.
- **Decision**: **Option 1 (Composite Template String with Parameter Interpolation)**.
- **Rationale**:
  - `src/i18n.ts` already has built-in parameter replacement support: `t(key, { format: ... })`.
  - Avoids duplicating 4 format labels across 2 languages (8 keys).
  - Keeps format names consistent between the dropdown menu (which displays `{t(opt.labelKey)}`) and the primary button.
  - Fully compliant with Principle II: constant defined centrally as `EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT`.

---

### Decision 2: Accessible Names and Tooltip Phrasing

- **Problem**: How to update `title` and `aria-label` attributes to ensure screen readers and mouse hovers convey the exact action and active format?
- **Options Considered**:
  1. *Update `EXPORT_BUTTON_TOOLTIP` with Format Parameter*: Change `ui.EXPORT_BUTTON_TOOLTIP` to support `{format}` or add `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT` with `"検索結果を保存 ({format})"` / `"Save search results ({format})"`.
  2. *Static Tooltip*: Retain `"検索結果を保存"` / `"Save search results"` without format indication.
- **Decision**: **Option 1 (`ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT`)**.
- **Rationale**:
  - When screen readers announce the button, hearing "Save search results (Excel (.xlsx))" or "検索結果を保存 (Excel (.xlsx))" provides unambiguous context.
  - Adding `EXPORT_CONSTANTS.BUTTON_TOOLTIP_WITH_FORMAT` leaves existing fallback keys intact for backward compatibility.

---

### Decision 3: Icon Retention and Sizing within Fixed Status Bar Height

- **Problem**: Does the wider button text fit comfortably in the 40px (`h-10`) status bar without causing layout wrapping or pushing controls out on 800px minimum viewport width?
- **Options Considered**:
  1. *Retain existing format-specific icons (`FileSpreadsheet`, `FileText`, `FileCode`, `Braces`)*: Display icon at `w-3.5 h-3.5` alongside the expanded text.
  2. *Replace icon with generic Download icon*: Lose visual file type distinction.
  3. *Remove icon entirely*: Decrease visual polish and visual anchors.
- **Decision**: **Option 1 (Retain existing format-specific file icons)**.
- **Rationale**:
  - The format-specific icon provides immediate visual reinforcement of the target file type (Excel green spreadsheet, CSV document icon, etc.).
  - The right-hand controls container uses `flex-shrink-0`, and the left-hand status text uses `truncate flex-1`. Even on an 800px window width, the status bar has ample horizontal room (approx 350px available for right-hand controls, which need only ~180px).
  - Single-line height (`h-10`) is strictly preserved with zero vertical growth.

---

### Decision 4: Standalone Mockup Synchronization

- **Problem**: Keeping `design/mainui/index.html` synchronized with the updated component per Constitution & development guidelines.
- **Decision**: Update `design/mainui/index.html` lines 595-604 to reflect:
  - Button text: `<span id="exportActiveLabel">結果を保存: Excel (.xlsx)</span>`
  - Tooltip: `title="検索結果を保存 (Excel (.xlsx))"`
  - `aria-label="検索結果を保存 (Excel (.xlsx))"`

---

## 2. Summary of Decisions

| Item | Resolution |
| :--- | :--- |
| Primary Button Label | `t(EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT, { format: t(activeOption.labelKey) })` |
| Primary Button Tooltip | `t(EXPORT_CONSTANTS.BUTTON_TOOLTIP_WITH_FORMAT, { format: t(activeOption.labelKey) })` |
| Dropdown Menu Items | Keep concise format names (`Excel (.xlsx)`, `CSV (.csv)`, `AI用 Markdown (.md)`, `JSON (.json)`) |
| Icon | Retain format-specific file icons (`FileSpreadsheet`, `FileText`, `FileCode`, `Braces`) |
| Accessibility | Explicit `aria-label` matching tooltip description |
| Constants & Locales | Centrally registered in `src/constants/index.ts`, `crates/core/src/constants.rs`, `ja.yml`, `en.yml` |

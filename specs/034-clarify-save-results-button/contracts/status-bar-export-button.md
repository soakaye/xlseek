# UI & Localization Contract: Status Bar Export Save Button

**Feature**: `034-clarify-save-results-button`
**Date**: 2026-10-25

## 1. Scope & Interface Boundary

This document specifies the UI rendering contract, localization keys, and accessibility attributes between `StatusBar.tsx`, centralized constants, and localization catalogs (`ja.yml`, `en.yml`).

---

## 2. Localization Catalog Contract

### 2.1 Japanese Catalog (`crates/core/locales/ja.yml`)

```yaml
# Save Results Button Clarification (Feature 034)
ui.EXPORT_BUTTON_LABEL_WITH_FORMAT: "結果を保存: {format}"
ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT: "検索結果を保存 ({format})"
```

### 2.2 English Catalog (`crates/core/locales/en.yml`)

```yaml
# Save Results Button Clarification (Feature 034)
ui.EXPORT_BUTTON_LABEL_WITH_FORMAT: "Save Results: {format}"
ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT: "Save search results ({format})"
```

---

## 3. Centralized Constants Contract

### 3.1 Frontend Constants (`src/constants/index.ts`)

```typescript
export const EXPORT_CONSTANTS = {
  // Existing constants ...
  FORMAT_XLSX: "xlsx",
  FORMAT_CSV: "csv",
  FORMAT_MD: "md",
  FORMAT_JSON: "json",
  DEFAULT_FORMAT: "xlsx",
  STORAGE_KEY_FORMAT: "xlseek.exportFormat",
  FILTER_XLSX: "ui.EXPORT_XLSX_FILTER_NAME",
  FILTER_CSV: "ui.EXPORT_CSV_FILTER_NAME",
  FILTER_MD: "ui.EXPORT_MD_FILTER_NAME",
  FILTER_JSON: "ui.EXPORT_JSON_FILTER_NAME",
  LABEL_XLSX: "ui.EXPORT_FORMAT_XLSX",
  LABEL_CSV: "ui.EXPORT_FORMAT_CSV",
  LABEL_MD: "ui.EXPORT_FORMAT_MD",
  LABEL_JSON: "ui.EXPORT_FORMAT_JSON",
  BUTTON_LABEL: "ui.EXPORT_BUTTON_LABEL",
  BUTTON_TOOLTIP: "ui.EXPORT_BUTTON_TOOLTIP",
  MENU_TOOLTIP: "ui.EXPORT_MENU_TOOLTIP",
  DEFAULT_FILENAME_PREFIX: "ui.EXPORT_DEFAULT_FILENAME_PREFIX",
  NO_RESULTS_MSG: "ui.EXPORT_NO_RESULTS_MSG",
  EXPORT_FAILED: "ui.EXPORT_FAILED",
  SAVED_XLSX: "ui.EXPORT_XLSX_SAVED",
  SAVED_CSV: "ui.EXPORT_CSV_SAVED",
  SAVED_MD: "ui.EXPORT_MD_SAVED",
  SAVED_JSON: "ui.EXPORT_JSON_SAVED",

  // NEW Constants for Feature 034:
  BUTTON_LABEL_WITH_FORMAT: "ui.EXPORT_BUTTON_LABEL_WITH_FORMAT",
  BUTTON_TOOLTIP_WITH_FORMAT: "ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT",
} as const;
```

### 3.2 Rust Backend Constants (`crates/core/src/constants.rs` & `src-tauri/src/constants.rs`)

```rust
// Export button composite label and tooltip keys
pub const EXPORT_BUTTON_LABEL_WITH_FORMAT_KEY: &str = "ui.EXPORT_BUTTON_LABEL_WITH_FORMAT";
pub const EXPORT_BUTTON_TOOLTIP_WITH_FORMAT_KEY: &str = "ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT";
```

---

## 4. UI DOM & Accessibility Contract

### 4.1 Primary Export Button Markup Contract

```html
<button
  type="button"
  disabled="{items.length === 0}"
  title="{tooltipText}"
  aria-label="{tooltipText}"
  class="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-200 hover:text-white rounded-l border border-zinc-700 flex items-center gap-1.5 transition text-xs font-medium cursor-pointer"
>
  <!-- Lucide Icon with format color -->
  <svg class="w-3.5 h-3.5 {activeOption.iconColor}" ...></svg>
  <!-- Explicit Action Label -->
  <span>{buttonLabel}</span>
</button>
```

Where:
- `buttonLabel` = `t(EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT, { format: t(activeOption.labelKey) })`
- `tooltipText` = `t(EXPORT_CONSTANTS.BUTTON_TOOLTIP_WITH_FORMAT, { format: t(activeOption.labelKey) })`

### 4.2 Dropdown Chevron Button Markup Contract

```html
<button
  type="button"
  disabled="{items.length === 0}"
  title="{t(EXPORT_CONSTANTS.MENU_TOOLTIP)}"
  aria-label="{t(EXPORT_CONSTANTS.MENU_TOOLTIP)}"
  aria-haspopup="menu"
  aria-expanded="{isMenuOpen}"
  class="px-1.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-400 hover:text-zinc-200 rounded-r border-y border-r border-zinc-700 flex items-center justify-center transition cursor-pointer border-l-0"
>
  <svg class="w-3 h-3 transition-transform {isMenuOpen ? 'rotate-180' : ''}" ...></svg>
</button>
```

### 4.3 Format Menu Contract

The upward format menu renders each of the 4 supported formats (`xlsx`, `csv`, `md`, `json`):
- Displays the concise format name `{t(opt.labelKey)}` (e.g. `Excel (.xlsx)`)
- Selected format includes an active indicator dot
- Keyboard navigation: `Escape` closes the menu without exporting
- Outside click closes the menu without exporting

---

## 5. Verification Contract

| Attribute | Contract Expectation | Test Verification Method |
| :--- | :--- | :--- |
| English Primary Label | Text content contains `"Save Results: Excel (.xlsx)"` | `getByRole("button", { name: "Save search results (Excel (.xlsx))" })` |
| Japanese Primary Label | Text content contains `"結果を保存: Excel (.xlsx)"` | Tested under `<LocaleProvider value="ja">` |
| Empty Results State | `disabled === true` on both split button elements | Checked in `StatusBar.test.tsx` |
| Format Switching | Updates button text to `"Save Results: CSV (.csv)"` when CSV selected | Event fire + assertion in `StatusBar.test.tsx` |

# Data Model: Clarify Search Results Save Button in Status Bar

**Feature**: `034-clarify-save-results-button`
**Date**: 2026-10-25

## 1. Overview

This document specifies the data entities, localization models, and UI state models governing the search results save split button in the application status bar.

*Note*: In strict compliance with Constitution Principle III, all examples and data schemas in this document use synthetic dummy data only (e.g., `dummy_keyword`, `sample.xlsx`, `Sheet1`).

---

## 2. Entities & Data Structures

### 2.1 ExportFormat

The discrete enumeration of supported export formats:

```typescript
export type ExportFormat = "xlsx" | "csv" | "md" | "json";
```

| Format Value | Target Extension | Target MIME / MIME Description | Default Status |
| :--- | :--- | :--- | :--- |
| `xlsx` | `.xlsx` | Office Open XML Spreadsheet | Default for initial install |
| `csv` | `.csv` | Comma-Separated Values (UTF-8 BOM) | Alternative tabular |
| `md` | `.md` | AI-Ready Markdown with Context | Optimized for LLM prompts |
| `json` | `.json` | Structured JSON matching MCP schema | Optimized for scripts / automation |

---

### 2.2 ExportFormatOption

Frontend visual representation model for an export format item:

```typescript
export interface ExportFormatOption {
  /** Target format identifier matching ExportFormat */
  format: ExportFormat;
  /** Translation key for format label (e.g. "ui.EXPORT_FORMAT_XLSX") */
  labelKey: TranslationKey;
  /** Translation key for OS file dialog filter name */
  filterKey: TranslationKey;
  /** Translation key for completion toast notification */
  savedKey: TranslationKey;
  /** Lucide icon component representing the format */
  icon: React.ComponentType<{ className?: string }>;
  /** Tailwind CSS color class for the icon */
  iconColor: string;
}
```

---

### 2.3 Localized Label Template Model

Model for generating the composite primary button label and accessible description:

```typescript
export interface ExportButtonLabelContext {
  /** Format name resolved from activeOption.labelKey (e.g. "Excel (.xlsx)") */
  format: string;
}
```

#### Interpolated Label Values

| Language | Template Key | Template Value | Formatted Example (Synthetic) |
| :--- | :--- | :--- | :--- |
| `ja` | `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT` | `"結果を保存: {format}"` | `結果を保存: Excel (.xlsx)` |
| `en` | `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT` | `"Save Results: {format}"` | `Save Results: Excel (.xlsx)` |
| `ja` | `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT` | `"検索結果を保存 ({format})"` | `検索結果を保存 (Excel (.xlsx))` |
| `en` | `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT` | `"Save search results ({format})"` | `Save search results (Excel (.xlsx))` |

---

### 2.4 StatusBar Local State Model

The reactive state model encapsulated within `StatusBar.tsx`:

```typescript
export interface StatusBarExportState {
  /** Currently active export format */
  activeFormat: ExportFormat;
  /** Whether the upward format dropdown menu is visible */
  isMenuOpen: boolean;
  /** Number of matching search items (disables button when 0) */
  itemCount: number;
}
```

#### State Transitions

```mermaid
stateDiagram-v2
    [*] --> IdleEmpty: App Launch (0 results)
    [*] --> IdleWithResults: Search Complete (>0 results)

    state IdleEmpty {
        note right of IdleEmpty: Button rendered as disabled\nLabel displays "結果を保存: {activeFormat}"\nMenu toggle disabled
    }

    state IdleWithResults {
        note right of IdleWithResults: Button rendered as enabled\nLabel displays "結果を保存: {activeFormat}"\nMenu toggle enabled
    }

    IdleEmpty --> IdleWithResults: Search yields matches
    IdleWithResults --> IdleEmpty: New search starts / clear

    state MenuOpen {
        note right of MenuOpen: Dropdown shows 4 format choices\nEsc or outside click closes menu
    }

    IdleWithResults --> MenuOpen: Click format toggle (▾)
    MenuOpen --> IdleWithResults: Click outside / Press Esc

    MenuOpen --> Exporting: Select alternative format
    IdleWithResults --> Exporting: Click primary save button

    state Exporting {
        note right of Exporting: Save dialog opens\nNew format persisted to localStorage
    }

    Exporting --> IdleWithResults: Export completes or cancelled
```

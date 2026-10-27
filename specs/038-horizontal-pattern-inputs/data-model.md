# Data Model: Horizontal Side-by-Side Pattern Inputs Layout

**Feature Branch**: `038-horizontal-pattern-inputs`  
**Date**: 2026-10-28  
**Spec Reference**: [spec.md](./spec.md)

---

## 1. Component State & Data Structures

This feature reorganizes the layout of existing pattern filter controls in the search interface. The underlying data model and types remain stable and unchanged.

### 1.1 SearchQuery Model

Represents the active search parameters manipulated by the user in `SearchBar.tsx`:

```typescript
export interface SearchQuery {
  /** Root directory paths targeted for scanning */
  paths: string[];
  /** Search keyword or regex */
  keyword: string;
  /** Whether keyword is evaluated as a regular expression */
  is_regex: boolean;
  /** Whether search is case-sensitive */
  is_case_sensitive: boolean;
  /** Target file extensions (e.g. [".xlsx", ".xlsm"]) */
  extensions: string[];
  /** Optional inclusion file search patterns (comma-separated, e.g. "*sample*, quarterly_*") */
  file_patterns?: string;
  /** Optional exclusion ignore patterns (comma-separated, e.g. "~$*, *.tmp") */
  ignore_patterns?: string;
  /** Target workbook content scopes (values, formulas, comments, etc.) */
  content_types: {
    values: boolean;
    formulas: boolean;
    comments: boolean;
    shapes: boolean;
    hidden_sheets: boolean;
  };
}
```

### 1.2 UI Layout Grid Hierarchy

```text
SearchBar (Root Container)
└── Search Options Form (grid gap-3)
    ├── Row 1: Search Directory Row (paths input + browse + history + search button)
    ├── Row 2: Pattern Filters Row (grid grid-cols-1 md:grid-cols-2 gap-3 items-center)
    │   ├── Column 1 (Left): File Search Patterns Control
    │   │   ├── Icon: FileText
    │   │   ├── Label: ui.FILE_PATTERNS_LABEL
    │   │   ├── Input: search-file-patterns-input (query.file_patterns)
    │   │   ├── Clear Button: (query.file_patterns.length > 0)
    │   │   ├── History Trigger Button: History icon
    │   │   ├── Shortcut Badge: Enter
    │   │   └── Overlay: FilePatternHistoryDropdown (openList === "file-pattern-history")
    │   └── Column 2 (Right): Ignore Patterns Control
    │       ├── Icon: Filter
    │       ├── Label: ui.IGNORE_PATTERNS_LABEL
    │       ├── Input: ignore-patterns-input (query.ignore_patterns)
    │       ├── Clear Button: (query.ignore_patterns.length > 0)
    │       ├── History Trigger Button: History icon
    │       ├── Shortcut Badge: Enter
    │       └── Overlay: IgnoreHistoryDropdown (openList === "ignore-history")
    ├── Row 3: Keyword Input Row (keyword + regex toggle + case sensitive)
    └── Row 4: Extension & Content Scope Toggles (xlsx, xlsm, docx, values, formulas, etc.)
```

### 1.3 State Management

| State Property | Type | Managing Component | Description |
| :--- | :--- | :--- | :--- |
| `query.file_patterns` | `string` | `App.tsx` (via `useSearch`) | Bound to the left input control; updated on change or clear. |
| `query.ignore_patterns` | `string` | `App.tsx` (via `useSearch`) | Bound to the right input control; updated on change or clear. |
| `openList` | `string \| null` | `SearchBar.tsx` | Mutually exclusive dropdown identifier: `"file-pattern-history"`, `"ignore-history"`, or `"path"`. |
| `filePatternHistory` | `string[]` | `SearchBar.tsx` | Array of recent file search pattern strings (max 20 entries) loaded from storage. |
| `ignoreHistory` | `string[]` | `SearchBar.tsx` | Array of recent ignore pattern strings (max 20 entries) loaded from storage. |

### 1.4 Dummy Data Examples (Constitution Principle III)

- **Synthetic Inclusion Patterns**: `*budget*, sample_report*.xlsx, summary_*`
- **Synthetic Exclusion Patterns**: `~$*, *.tmp, backup/*`
- **Synthetic File Targets**: `sample_data.xlsx`, `test_workbook.xlsm`

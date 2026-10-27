# UI Layout Contract: Horizontal Side-by-Side Pattern Inputs

**Feature Branch**: `038-horizontal-pattern-inputs`  
**Date**: 2026-10-28  
**Spec Reference**: [spec.md](../spec.md)

---

## 1. Scope & Interface Boundary

This document specifies the structural contract for placing the File Search Patterns control and the Ignore Patterns control side-by-side within the search interface.

Target components:
- `src/components/search/SearchBar.tsx`
- `design/mainui/index.html` (Standalone HTML prototype)

---

## 2. DOM & CSS Grid Contract

### 2.1 Unified Pattern Row Container

```html
<div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-center">
  <!-- Column 1: File Search Patterns -->
  <div ref={filePatternInputRef} onBlur={handleFieldBlur(..., "file-pattern-history")} class="relative">
    <div class="flex items-center gap-2">
      <span class="text-zinc-400 text-xs font-medium flex items-center gap-1.5 flex-shrink-0 select-none">
        <FileText class="w-3.5 h-3.5 text-zinc-400" />
        <span>{t("ui.FILE_PATTERNS_LABEL")}</span>
      </span>
      <div class="relative flex-1">
        <input
          id={FILE_PATTERNS_CONSTANTS.INPUT_ID}
          type="text"
          value={query.file_patterns ?? ""}
          placeholder={t("ui.FILE_PATTERNS_PLACEHOLDER")}
          title={t("ui.FILE_PATTERNS_TOOLTIP")}
          aria-label={t("ui.FILE_PATTERNS_LABEL")}
          class="w-full pl-3 pr-20 py-1.5 bg-[#202024] border border-zinc-700 rounded-lg text-xs font-mono text-zinc-300 placeholder-zinc-500 focus:outline-none focus:border-excel-light focus:ring-1 focus:ring-excel-light transition"
        />
        <div class="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
          <!-- Clear Button (when value length > 0) -->
          <!-- History Dropdown Trigger Button -->
          <!-- Enter Shortcut Badge -->
        </div>
        <!-- FilePatternHistoryDropdown -->
      </div>
    </div>
  </div>

  <!-- Column 2: Ignore Patterns -->
  <div ref={ignoreInputRef} onBlur={handleFieldBlur(..., "ignore-history")} class="relative">
    <div class="flex items-center gap-2">
      <span class="text-zinc-400 text-xs font-medium flex items-center gap-1.5 flex-shrink-0 select-none">
        <Filter class="w-3.5 h-3.5 text-zinc-400" />
        <span>{t("ui.IGNORE_PATTERNS_LABEL")}</span>
      </span>
      <div class="relative flex-1">
        <input
          id={IGNORE_PATTERNS_CONSTANTS.INPUT_ID}
          type="text"
          value={query.ignore_patterns ?? ""}
          placeholder={t("ui.IGNORE_PATTERNS_PLACEHOLDER")}
          title={t("ui.IGNORE_PATTERNS_TOOLTIP")}
          aria-label={t("ui.IGNORE_PATTERNS_LABEL")}
          class="w-full pl-3 pr-20 py-1.5 bg-[#202024] border border-zinc-700 rounded-lg text-xs font-mono text-zinc-300 placeholder-zinc-500 focus:outline-none focus:border-excel-light focus:ring-1 focus:ring-excel-light transition"
        />
        <div class="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
          <!-- Clear Button (when value length > 0) -->
          <!-- History Dropdown Trigger Button -->
          <!-- Enter Shortcut Badge -->
        </div>
        <!-- IgnoreHistoryDropdown -->
      </div>
    </div>
  </div>
</div>
```

### 2.2 Breakpoint and Sizing Rules

| Viewport Width | Grid Class | Visual Behavior |
| :--- | :--- | :--- |
| $\ge 768\text{px}$ (`md:`) | `md:grid-cols-2` | 2 columns side-by-side, each 50% width minus gap ($12\text{px}$). |
| $< 768\text{px}$ | `grid-cols-1` | 1 column stacked vertically: File Search Patterns on top, Ignore Patterns below. |

---

## 3. Dropdown Overlay Positioning Contract

1. Each dropdown overlay MUST be positioned `absolute top-full mt-1 w-full`.
2. The dropdown overlay's containing block MUST be `div.relative.flex-1` (the input box wrapper), ensuring the dropdown matches the width of the input field exactly.
3. The dropdown overlay MUST specify `z-30` or higher to stack cleanly above subsequent rows.
4. Only one dropdown may be open at any given moment (`openList` equality check).

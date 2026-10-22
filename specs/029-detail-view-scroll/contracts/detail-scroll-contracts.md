# UI Component & Layout Contracts: Scrollable Match Detail Area in Search Preview

**Feature Branch**: `029-detail-view-scroll` | **Date**: 2026-10-22 | **Spec**: [spec.md](./spec.md)

This document defines the interface contracts, DOM structures, test identifiers, and constant references governing Feature 029.

---

## 1. Component Contract (`MetaInfoCard.tsx`)

### 1.1 Props Specification
```typescript
export interface MetaInfoCardProps {
  /**
   * The active search match selected in the search results table.
   * If null, the component renders nothing and safely returns null.
   */
  match: SearchMatch | null;
}
```

### 1.2 Output DOM Specification
When `match` is provided, the component MUST render a container adhering to the following DOM structure and attributes:

```html
<div class="mt-3 p-3 bg-zinc-900/90 rounded-lg border border-zinc-800 text-xs space-y-2 mr-2 flex-shrink-0">
  <!-- Card Header -->
  <div class="flex items-center justify-between text-zinc-400">
    <span class="font-medium text-zinc-300 flex items-center gap-1.5">
      <svg ... class="w-3.5 h-3.5 text-emerald-400" />
      Match Details
    </span>
    <span class="text-[11px] text-zinc-500">
      Type: <strong class="text-zinc-300">{matchTypeLabel}</strong>
    </span>
  </div>

  <!-- Scrollable Content Viewport -->
  <div
    data-testid="meta-info-content"
    class="text-zinc-300 bg-zinc-950/80 p-2 rounded border border-zinc-800/80 font-mono text-[11px] select-text break-words whitespace-pre-wrap max-h-36 overflow-y-auto overscroll-contain"
  >
    {match.full_content || t("ui.EMPTY_CONTENT")}
  </div>

  <!-- Shape name (rendered only when match.match_type === 'Shape') -->
  { ... }

  <!-- Card Footer -->
  <div class="flex items-center gap-4 text-[11px] text-zinc-400 pt-1">
    <span>Row: <strong class="text-zinc-200">{row_index}</strong></span>
    <span>Col: <strong class="text-zinc-200">{col_index} ({col_name})</strong></span>
    <span>Hidden: <strong class="text-zinc-200">{sheet_hidden}</strong></span>
  </div>
</div>
```

---

## 2. Centralized Constants Contract (`src/constants/index.ts`)

In compliance with Constitution Principle II, layout utility classes and pixel dimensions MUST be centralized:

```typescript
export const LAYOUT_CONSTANTS = {
  // Existing layout constants...
  
  /** Match detail content container max height Tailwind utility class */
  META_DETAIL_MAX_HEIGHT_CLASS: "max-h-36",
  /** Match detail content container max height in pixels (144px) */
  META_DETAIL_MAX_HEIGHT_PX: 144,
  /** Test identifier attribute for the scrollable match detail content element */
  META_DETAIL_TEST_ID: "meta-info-content",
} as const;
```

---

## 3. UI Prototype Contract (`design/mainui/index.html`)

The standalone HTML prototype's `#metaFullContent` element MUST match the classes and styling applied in `MetaInfoCard.tsx`:

```html
<div
  id="metaFullContent"
  class="text-zinc-300 bg-zinc-950/80 p-2 rounded border border-zinc-800/80 font-mono text-[11px] select-text break-words whitespace-pre-wrap max-h-36 overflow-y-auto overscroll-contain"
>
  Financial Report Q3 (FY2026-Q3-Final-Audit)
</div>
```

---

## 4. Test Contract (`tests/meta-info-card.test.tsx`)

The test suite MUST implement and pass the following unit verification cases:

| Test Case ID | Input Condition | Expected Result |
|---|---|---|
| `TC-001: Short Text Rendering` | `full_content: "Short text"` | Viewport renders containing `max-h-36`, `overflow-y-auto`, and `whitespace-pre-wrap` classes |
| `TC-002: Multi-line Long Text` | Long text with 20+ newline characters | Content renders in full without crash, properly contained within the scrollable viewport |
| `TC-003: Empty Content Fallback` | `full_content: ""` | Displays localized placeholder matching `ui.EMPTY_CONTENT` |
| `TC-004: Null Match Safety` | `match: null` | Safely renders nothing (`container.firstChild === null`) without exceptions |
| `TC-005: Metadata Visibility` | Row 16, Col 1(A), Visible: true | Row, Column, and Visibility status labels render accurately outside the scrollable text box |

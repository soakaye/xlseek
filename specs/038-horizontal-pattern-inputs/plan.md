# Implementation Plan: Horizontal Side-by-Side Pattern Inputs Layout

**Branch**: `038-horizontal-pattern-inputs` | **Date**: 2026-10-28 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/038-horizontal-pattern-inputs/spec.md`

---

## Summary

This feature rearranges the File Search Patterns control and the Ignore Patterns control in the search header from two vertically stacked rows into a single, compact horizontal row with equal-width columns (`grid-cols-1 md:grid-cols-2 gap-3`).

Because users typically enter short pattern strings (such as `*budget*, *.xlsx` or `~$*, *.tmp`), dedicating two full-width rows consumes unnecessary vertical real estate in the search header. Merging them into a single side-by-side row reduces the vertical height by approximately 36-44px on desktop resolutions, allowing more search results and preview data to be visible without scrolling.

All existing functional capabilities—including text editing, clear (`X`) buttons, history dropdowns with LRU storage, Enter key shortcuts, tooltip explanations, and Japanese/English translations—are 100% preserved. On window viewports narrower than 768px (Tailwind `md:`), the row gracefully stacks into single columns, preventing any button clipping or cramped input widths. The standalone HTML prototype at `design/mainui/index.html` is kept strictly synchronized.

---

## Technical Context

**Language/Version**: TypeScript 5.x, React 18, Tailwind CSS, HTML5  
**Primary Dependencies**:
- Frontend: `@tauri-apps/api` v2, `lucide-react`, `tailwindcss`  
**Storage**: Client-side `localStorage` (`xlseek.filePatternHistory`, `xlseek.ignoreHistory`)  
**Testing**: `npm test` (Vitest component & catalog tests), `npm run build` (tsc + vite bundle), `cargo test --workspace`  
**Target Platform**: Windows, macOS, Linux (desktop GUI application)  
**Project Type**: Desktop GUI application (`src/components/search/SearchBar.tsx`, `design/mainui/index.html`)  
**Performance Goals**:
- Zero layout shift (CLS) or rendering jitter when toggling history dropdowns.
- Instantaneous responsive adaptation upon window resizing across the 768px (`md:`) boundary.  
**Constraints**:
- Strict conformance to Project Constitution (Zero hardcoded constants, comprehensive header comments, modular single responsibility, zero warnings/errors).
- Clean stacking context (`z-index: 20` or higher) ensuring history dropdown overlays do not obscure or conflict with adjacent controls.
- Synthetic dummy data only in documentation (Principle III).  
**Scale/Scope**:
- 1 main search React component (`SearchBar.tsx`).
- 1 standalone HTML UI prototype (`design/mainui/index.html`).
- Frontend unit/integration test suites in `tests/`.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Conformance / Verification | Status |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English default for code comments, spec docs; natural UI translations in catalogs. | All specification and plan documents written in clear English. UI labels and placeholders reuse existing centralized catalog keys (`ui.FILE_PATTERNS_*`, `ui.IGNORE_PATTERNS_*`). | PASS |
| **II. No Hardcoded Constants** | No magic numbers or strings (except `0` and `""`). All constants centralized with reference comments. | Reuses existing constants (`FILE_PATTERNS_CONSTANTS`, `IGNORE_PATTERNS_CONSTANTS`) in `src/constants/index.ts` with explicit constant reference comments. | PASS |
| **III. Comprehensive Header Comments & Documentation Hygiene** | Every file, module, struct, function, and method must have a header comment with (1) description, (2) arguments/returns, and (3) error/panic behavior. Zero real search data in documents. | `SearchBar.tsx` and modified components maintain comprehensive 3-element header comments. All documentation and test scenarios use synthetic dummy data only. | PASS |
| **IV. Modular Design & Code Standards** | Single responsibility, standard style guides, zero TypeScript errors (`tsc --noEmit`), zero ESLint warnings, formatted code (`cargo fmt --check`). | Layout adjustments cleanly encapsulate each pattern filter inside its own column within the grid row. Zero type errors and clean linting guaranteed. | PASS |
| **V. Robust Error Handling & Testing** | No unhandled exceptions; safe blur/focus handling; comprehensive unit and integration tests. | Dropdown opening/closing and field blur handling preserved; regression test suite verifies layout structure and input rendering. | PASS |

---

## Project Structure

### Documentation (this feature)

```text
specs/038-horizontal-pattern-inputs/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0: Layout & dropdown stacking research
├── data-model.md        # Phase 1: Component hierarchy & state model
├── quickstart.md        # Phase 1: Quickstart & verification guide
├── contracts/
│   └── ui-layout-contract.md  # Phase 1: DOM & grid layout contract
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2: Actionable tasks (created by /speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── components/
│   └── search/
│       ├── SearchBar.tsx                  # Update: merge Row 2 and Row 3 into side-by-side grid
│       ├── FilePatternHistoryDropdown.tsx # Maintain: anchored dropdown overlay
│       └── IgnoreHistoryDropdown.tsx      # Maintain: anchored dropdown overlay
└── constants/
    └── index.ts                           # Maintain: centralized constants and element IDs

design/
└── mainui/
    └── index.html                         # Update: sync mockup with side-by-side pattern row

tests/
├── components/
│   └── SearchBar.test.tsx                 # Verify / add test for side-by-side pattern layout
└── catalog.test.ts                        # Existing catalog verification
```

**Structure Decision**: Web/desktop GUI frontend modification modifying `SearchBar.tsx` and `design/mainui/index.html`. Backend engines, CLI, and MCP servers remain entirely untouched.

---

## Complexity Tracking

> *No Constitution violations identified. No complexity exceptions required.*

# Implementation Plan: Scrollable Match Detail Area in Search Preview

**Branch**: `029-detail-view-scroll` | **Date**: 2026-10-22 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/029-detail-view-scroll/spec.md`

---

## Summary

In the Excel search result preview pane, when a matched cell contains long multi-line text (e.g. dozens of lines or paragraphs), the match detail card expands vertically without bound, squashing the surrounding cell preview grid (`SpreadsheetGrid`) down to near-zero height.
This plan resolves the visibility defect by applying a fixed maximum height boundary (`max-h-36` / 144px), automatic vertical scrolling (`overflow-y-auto`), explicit newline preservation (`whitespace-pre-wrap`), and scroll containment (`overscroll-contain`) to the cell content container in `MetaInfoCard.tsx`. It also establishes centralized constants in `src/constants/index.ts`, synchronizes the standalone mockup in `design/mainui/index.html`, and implements comprehensive unit tests in `tests/meta-info-card.test.tsx`.

---

## Technical Context

**Language/Version**: TypeScript 5.x / React 18, Rust 2021 edition (1.75+)

**Primary Dependencies**:
- Frontend: React 18, Tailwind CSS v4, Lucide React, `@tanstack/react-virtual`
- Backend: Tauri v2, `calamine`, `rayon` (no backend changes needed for this UI-level enhancement)

**Storage**: N/A (local UI presentation only)

**Testing**:
- Frontend: Vitest (`npm test`), `@testing-library/react`, TypeScript checking (`npm run build`)
- Rust: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`

**Target Platform**: Windows, macOS, Linux (Tauri v2 Desktop GUI)

**Project Type**: Desktop application (Tauri v2 + React 18 GUI)

**Performance Goals**:
- Detail view scroll rendering: 60fps (zero jank, zero layout shifts)
- Result selection latency: 0ms added overhead

**Constraints**:
- Strict adherence to the project constitution (`.specify/memory/constitution.md`):
  - Principle I: Documentation, comments, and internal error strings in natural English; UI localized via catalogs.
  - Principle II: No hardcoded magic strings or numbers outside centralized constants (`LAYOUT_CONSTANTS.META_DETAIL_MAX_HEIGHT_CLASS`, etc.).
  - Principle III: Comprehensive 3-part header comments on all functions and components.
  - Principle IV: Clean modular architecture conforming to `clippy`, `rustfmt`, and ESLint zero-warning gates.
  - Principle V: Robust error handling, non-panicking null checks, and complete unit test coverage.
- UI Mockup Synchronization: Comply with `AGENTS.md` Section 5 by keeping `design/mainui/index.html` in sync.

**Scale/Scope**:
- Frontend files:
  - `src/constants/index.ts` (add layout constants)
  - `src/components/preview/MetaInfoCard.tsx` (apply scroll classes, whitespace handling, test IDs)
  - `design/mainui/index.html` (synchronize standalone HTML prototype)
  - `tests/meta-info-card.test.tsx` (new component unit test suite)

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS — All documentation artifacts (`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `tasks.md`), source code comments, and error strings are written in natural, accurate English.
- **Principle II (No Hardcoded Constants)**: PASS — The max-height class (`"max-h-36"`), pixel height (`144`), and test ID (`"meta-info-content"`) are extracted to `LAYOUT_CONSTANTS` in `src/constants/index.ts` with explicit `// Constant reference:` comments.
- **Principle III (Comprehensive Header Comments)**: PASS — `MetaInfoCard.tsx` and `tests/meta-info-card.test.tsx` include comprehensive 3-part header comments (Description, Arguments/Returns, Errors/Exceptions) without per-file change histories.
- **Principle IV (Modular Design & Code Standards)**: PASS — Minimal targeted modifications to the preview presentation layer without touching unrelated files. Verified with zero TypeScript errors and zero lint warnings.
- **Principle V (Robust Error Handling & Testing)**: PASS — Safe handling of null match props and empty content fallbacks, validated with dedicated Vitest tests.

---

## Project Structure

### Documentation (this feature)

```text
specs/029-detail-view-scroll/
├── spec.md                     # Feature specification with clarifications
├── plan.md                     # This implementation plan (/speckit-plan output)
├── research.md                 # Phase 0: Research and architectural decisions
├── data-model.md               # Phase 1: Data entities and UI layout definitions
├── quickstart.md               # Phase 1: Verification and test run guide
├── contracts/                  # Phase 1: Interface contracts
│   └── detail-scroll-contracts.md # UI component and layout contracts
├── checklists/
│   └── requirements.md         # Spec quality checklist (100% pass)
└── tasks.md                    # Phase 2: Actionable task list (/speckit-tasks output)
```

### Source Code (repository root)

```text
src/
├── constants/
│   └── index.ts                # Adds META_DETAIL_MAX_HEIGHT_CLASS to LAYOUT_CONSTANTS
├── components/
│   └── preview/
│       └── MetaInfoCard.tsx    # Applies max-h-36 overflow-y-auto overscroll-contain whitespace-pre-wrap
design/
└── mainui/
    └── index.html              # Synchronizes #metaFullContent classes in HTML prototype
tests/
└── meta-info-card.test.tsx     # New component unit tests for MetaInfoCard
```

**Structure Decision**: Single desktop application project (React 18 + Tailwind CSS frontend). No new workspace crates or external libraries required.

---

## Complexity Tracking

> No constitution violations or unjustified architectural complexities. Standard CSS utility classes and centralized constants provide a clean, zero-overhead solution.

| Item | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| None | Fully compliant | N/A |

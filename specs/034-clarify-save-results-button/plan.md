# Implementation Plan: Clarify Search Results Save Button in Status Bar

**Branch**: `034-clarify-save-results-button` | **Date**: 2026-10-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/034-clarify-save-results-button/spec.md`

## Summary

Clarify the search results export split button in the status bar across frontend components, localization catalogs, constants, and the UI prototype:
1. **Localization & Constants**: Add composite format template keys `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT` (`"結果を保存: {format}"` / `"Save Results: {format}"`) and `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT` (`"検索結果を保存 ({format})"` / `"Save search results ({format})"`) to `ja.yml`, `en.yml`, `crates/core/src/constants.rs`, and `src/constants/index.ts`.
2. **StatusBar UI Component (`StatusBar.tsx`)**: Update the primary split button to interpolate the active format into the composite label template, retaining the format-specific Lucide icon, while updating tooltips and `aria-label` to provide unambiguous context.
3. **Automated Testing (`StatusBar.test.tsx`)**: Update unit tests to verify the explicit save label in disabled and active states, format switching assertions, and accessible names.
4. **Mockup Synchronization (`design/mainui/index.html`)**: Update the HTML prototype button label and tooltip to match the production implementation.

## Technical Context

**Language/Version**: TypeScript 5.x / React 18, Rust 2021 Edition (1.75+)

**Primary Dependencies**: React 18, Tailwind CSS, Lucide React (`FileSpreadsheet`, `FileText`, `FileCode`, `Braces`, `ChevronDown`), Tauri v2 (`@tauri-apps/api`, `tauri-plugin-dialog`)

**Storage**: Browser `localStorage` (`xlseek.exportFormat` for persisting active format selection)

**Testing**: `npm test -- src/components/common/__tests__/StatusBar.test.tsx --run` (Vitest), `npm run build`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`

**Target Platform**: Desktop GUI (Windows, macOS, Linux via Tauri v2)

**Project Type**: Desktop GUI Application within Cargo Workspace

**Performance Goals**: Instant UI rendering (0ms overhead); fixed 40px (`h-10`) footer height with zero layout shift or vertical expansion.

**Constraints**: Zero hardcoded strings (Principle II); comprehensive 3-part header comments (Principle III); synthetic dummy data only in all tests and documentation (Principle III).

**Scale/Scope**: 1 UI component (`StatusBar.tsx`), 1 test suite (`StatusBar.test.tsx`), 2 localization catalogs (`ja.yml`, `en.yml`), 2 constants files, and 1 HTML prototype.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate Requirement | Pre-Design Status | Post-Design Status |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English source comments/docs; bilingual catalog entries (`ja.yml`, `en.yml`); zero garbled tokens. | PASS | PASS |
| **II. No Hardcoded Constants** | All format templates, translation keys, and tooltip keys extracted to centralized constants (`EXPORT_CONSTANTS`) with explicit `// Constant reference:` comments. | PASS | PASS |
| **III. Comprehensive Header Comments & Documentation Hygiene** | Every modified component, function, and test file includes Description, Arguments/Returns, and Errors/Panics; zero real user data/real search results in documentation (synthetic data only). | PASS | PASS |
| **IV. Modular Design & Code Standards** | Component responsibilities cleanly separated; zero `clippy`, `rustfmt`, or `tsc` warnings/errors. | PASS | PASS |
| **V. Robust Error Handling & Testing** | Disabled states handled gracefully; unit tests updated in `StatusBar.test.tsx` verifying label updates and accessibility. | PASS | PASS |

## Project Structure

### Documentation (this feature)

```text
specs/034-clarify-save-results-button/
├── spec.md              # Feature specification with Option A clarification
├── plan.md              # This implementation plan
├── research.md          # Phase 0 decisions on label composition and layout
├── data-model.md        # Phase 1 data entities and UI state models
├── quickstart.md        # Phase 1 manual and automated validation guide
├── contracts/           # Phase 1 UI and localization contracts
│   └── status-bar-export-button.md
└── checklists/
    └── requirements.md  # Quality validation checklist
```

### Source Code (repository root)

```text
crates/
└── core/
    ├── locales/
    │   ├── en.yml                               # English translation keys for composite button label & tooltip
    │   └── ja.yml                               # Japanese translation keys for composite button label & tooltip
    └── src/
        └── constants.rs                         # Rust constant definitions for new translation keys

src/
├── constants/
│   └── index.ts                                 # EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT & BUTTON_TOOLTIP_WITH_FORMAT
└── components/
    └── common/
        ├── StatusBar.tsx                        # Update split button label & tooltip with format interpolation
        └── __tests__/
            └── StatusBar.test.tsx               # Unit tests verifying new labels, tooltips, and format switching

design/
└── mainui/
    └── index.html                               # Standalone HTML mockup synchronized with new button text
```

**Structure Decision**: This feature modifies existing frontend UI components, localization resource files, centralized constants, and the HTML prototype. No new crates or architectural layers are introduced.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*No violations. All principles and constraints fully satisfied.*

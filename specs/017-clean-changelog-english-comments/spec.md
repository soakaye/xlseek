# Feature Specification: Clean Changelog Headers and English Source Comments

**Feature Branch**: `017-clean-changelog-english-comments`

**Created**: 2026-09-30

**Status**: Draft

**Input**: User description: "改定したConstitutionに基づきソースコードヘッダの改定履歴の削除及び日本語言語ファイルメッセージ以外のコメントを英語にしてください。"

## Clarifications

### Session 2026-09-30

- Q: Should `design/mainui/index.html` (HTML/JS prototype) also have its changelog headers removed and comments converted to English? → A: Include `design/mainui/index.html` along with source code to keep complete project consistency.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Remove In-File Changelogs from Source and Prototype Headers (Priority: P1)

Developers and maintainers inspecting source code files and UI prototypes across the codebase should see clean docstrings and header comments that describe functionality, arguments, return values, and errors/panics, without redundant, easily outdated in-file changelog sections (version, author, creation date, modification log), adhering to Constitution Principle III (v3.0.0).

**Why this priority**: Directly enforces the amended Constitution Principle III by eliminating in-file maintenance overhead and ensuring file headers are concise and current while relying on git history for change tracking.

**Independent Test**: Can be validated by scanning all source files (`.rs`, `.ts`, `.tsx`) and `design/mainui/index.html` to confirm no in-file change history / revision blocks (`変更履歴`, `Change History`, `Revision`) remain in headers or item doc comments.

**Acceptance Scenarios**:

1. **Given** any existing Rust source file (in `crates/` or `src-tauri/`), TypeScript/TSX file (in `src/`), or `design/mainui/index.html`, **When** reviewing the file-level, struct-level, and function-level doc comments, **Then** all change history / changelog blocks are completely removed while retaining descriptions, arguments, returns, and error conditions.
2. **Given** any newly generated or updated component or function, **When** documentation comments are inspected, **Then** no revision history headers are generated.

---

### User Story 2 - English Translation of In-Code Comments (Priority: P2)

International developers and AI coding agents inspecting the codebase need comments, docstrings, and inline notes to be in natural, accurate English (per Constitution Principle I v3.0.0), improving readability and cross-tool compatibility, while preserving Japanese strings exclusively inside Japanese localization files.

**Why this priority**: Unifies codebase documentation into English as the default project language, fulfilling Constitution Principle I while keeping end-user Japanese localization intact.

**Independent Test**: Can be validated by inspecting source code files and design prototypes to verify that all code comments (doc comments, inline comments, constant reference annotations) are in English, and confirming Japanese UI/log messages in `crates/core/locales/ja.yml` remain untouched.

**Acceptance Scenarios**:

1. **Given** source files in `crates/`, `src-tauri/`, `src/`, and `design/mainui/index.html`, **When** comments (line comments `//`, block comments `/* */`, and doc comments `///`, `//!`, `/** */`, HTML comments `<!-- -->`) are inspected, **Then** their explanatory text is written in natural, accurate English.
2. **Given** localization resources such as `crates/core/locales/ja.yml` and language catalog keys, **When** inspecting translation entries, **Then** the Japanese localization messages remain unaltered and functional.
3. **Given** constant reference comments (e.g., `// 定数参照: ...`), **When** reviewed, **Then** they are standardized to English (e.g., `// Constant reference: ...`).

---

### Edge Cases

- **Localization files (`ja.yml`, `en.yml`)**: Japanese translation strings in `crates/core/locales/ja.yml` are resource definitions, not source comments; they must not be converted to English.
- **License / Third-party attribution text**: Legal notices or bundled third-party licenses (e.g., `licenses.json`) must remain in their original legal wording.
- **Constant reference annotations**: Constitution Principle II requires referencing constants where used; comment tags like `// 定数参照:` must be translated cleanly to English (`// Constant reference:`) without breaking lint or style standards.
- **Header 3-element compliance**: Removing the 4th element (revision history) must preserve the 3 required elements (purpose/description, parameters/returns, errors/panics) in full detail.
- **Design Prototype HTML (`design/mainui/index.html`)**: Comments inside embedded scripts and HTML blocks must be translated to English without altering mock dataset values that represent sample Japanese Excel file contents.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST eliminate all in-file changelog sections (e.g., `変更履歴`, `Revision History`, `Version / Date / Author / Changes` blocks) from all source files in `crates/`, `src-tauri/`, `src/`, and `design/mainui/index.html`.
- **FR-002**: File headers, module docstrings, type definitions, and function doc comments MUST retain the 3 mandatory documentation elements:
  1. Detailed description of the purpose / behavior
  2. Arguments and return types with explanations
  3. Possible errors, `Result`/`Option` handling, and panic/exception conditions
- **FR-003**: All explanatory code comments (inline comments, block comments, doc comments, TODOs, constant reference annotations) in `crates/`, `src-tauri/`, `src/`, and `design/mainui/index.html` MUST be written in natural, accurate English.
- **FR-004**: Japanese localization catalogs and translation values (specifically `crates/core/locales/ja.yml`) MUST remain in Japanese to ensure end-user Japanese language support is preserved.
- **FR-005**: All existing functionality, tests, and build gates MUST remain green:
  - `npm run build` succeeds with zero errors.
  - `cargo test --workspace` passes 100%.
  - `cargo clippy --workspace --all-targets -- -D warnings` reports zero warnings.
  - `cargo fmt --check` passes cleanly.

### Key Entities

- **Source Code Comments**: Explanatory text blocks (`//`, `///`, `//!`, `/* */`, `/** */`, `<!-- -->`) associated with modules, files, types, functions, and inline code logic.
- **Localization Catalog**: User-facing resource files containing multi-language UI strings (`crates/core/locales/*.yml`), exempt from comment translation rules for their localized values.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of in-file changelog / revision history comment blocks in the codebase and `design/mainui/index.html` are removed (0 occurrences of in-file revision metadata headers remaining in source files).
- **SC-002**: 100% of source code comments outside localization catalog values are written in English.
- **SC-003**: 0 regression in all automated test suites and quality gates (`cargo test`, `cargo clippy`, `cargo fmt`, `npm run build`).
- **SC-004**: End-user Japanese UI text continues to display correctly without truncation or missing translation keys.

## Assumptions

- Version control history (Git commit logs, PR descriptions, tags) serves as the single source of truth for file changes and author attribution.
- English comments should follow concise, standard technical English conventions.
- Project source code files (`crates/`, `src-tauri/`, `src/`), developer guidelines (`AGENTS.md`), and UI prototype (`design/mainui/index.html`) are covered under this cleanup; third-party bundled licenses and user documentation like `README.md` retain their intended content.

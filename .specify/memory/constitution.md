<!-- Sync Impact Report
Version change: 3.1.0 -> 3.2.0
List of modified principles:
- Principle III, Comprehensive Header Comments & Documentation Hygiene: Added mandatory rule that generated and persisted documents (such as data-model.md, spec.md, plan.md) MUST NOT use real data, real search results, or real file paths/content, and MUST use synthetic dummy data.
- Technical Constraints (Section 2.3, Security & Privacy): Explicitly prohibited inclusion of real search results, file paths, and business data in persisted documents.
- Development Workflow & Quality Gates (Section 3.3, AI Generation Quality Assurance): Added verification requirement to inspect and guarantee zero real data in generated artifacts.
Added sections: None
Removed sections: None
Follow-up TODOs: None
-->
# Excel Grep Constitution

## Core Principles

### I. Language-Directed Quality
English MUST be the primary language of this project. Unless another language is explicitly specified, responses to users, commit messages, generated documents, source code comments, UI text, error messages, and logs MUST be written in natural, accurate English.
When a user instruction or approved specification explicitly specifies a language, responses to users, commit messages, generated documents, source code comments, UI text, error messages, and logs MUST be written in that specified language.
If the specified language is not English, English translations MUST NOT be forced.
Before delivering artifacts, verify that there is no garbled text or unwanted/unexplained special tokens such as `<PAD>` and `<pad>`. If detected, they MUST be corrected using natural expressions appropriate to the specified language.
- **Rationale**: To maintain international standards and English-by-default documentation and communication quality, while respecting explicitly requested languages and preventing confusion caused by unwanted tokens or unnatural language mixing.

### II. No Hardcoded Constants
Except for `0` and the empty string (`""`), constant values (numbers, magic numbers, fixed strings, etc.) MUST NOT be hardcoded directly into code. All constant values MUST be extracted into centralized constants definition files such as `src/constants/index.ts` (frontend) or `src-tauri/src/constants.rs` (Rust backend), and every usage site MUST include an explicit comment identifying the referenced constant (e.g., `// Constant reference: constants::MENU_ITEM_ABOUT_ID`). Even for automatically generated code, constant extraction and constant reference comments MUST be applied in the same manner.
- **Rationale**: Magic numbers and hardcoded fixed values significantly impair maintainability and lead to missed updates and inconsistencies during specification changes. Centralizing constants and explicitly marking their usage ensures ease of modification.

### III. Comprehensive Header Comments & Documentation Hygiene
Every file, module, struct, class, function, and method MUST have a header comment. Header comments MUST comprehensively include all of the following:
1. A detailed description of what it does
2. The types and descriptions of its arguments and return value
3. Possible errors, handling of `Result`/`Option`, or conditions that cause a panic or exception
Per-file change histories (version, creation date, author, modification details) MUST NOT be recorded or generated in code header comments; version control systems (such as Git commit history) MUST be used for that purpose. Even for automatically generated code, header comments MUST be applied in the same manner.
Furthermore, across all generated, persisted, and saved documents (such as `data-model.md`, specifications, plans, etc.), actual search results or real user data (real file names, real paths, actual cell contents, etc.) MUST NOT be used; synthetic dummy data MUST always be used.
- **Rationale**: To enforce code self-documentation while eliminating the maintenance overhead of in-file change logs that quickly become obsolete and duplicate Git history, minimizing cognitive load during multi-developer and AI agent collaboration. In addition, to prevent leakage of confidential or personal information and permanently guarantee the safety and reproducibility of documents.

### IV. Modular Design & Code Standards
Implementations MUST follow standard formatting for the target language and maintain consistent, correct indentation. Implementations MUST fully comply with the official style guides and best practices of the target languages (e.g., Rust's `clippy` and `rustfmt`, TypeScript's zero type errors and zero ESLint warnings). Code MUST be appropriately divided into modules according to classes, functions, and units of responsibility, using clear file names that accurately convey their responsibilities and contents.
- **Rationale**: To adhere to the Single Responsibility Principle (SRP), improve codebase clarity and reusability, and ensure uniform quality through automated validation with official tooling.

### V. Robust Error Handling & Testing
Unexpected panics and unhandled exceptions MUST be eliminated; robust error handling MUST be performed using `Result` or appropriate error types. Modules and functions MUST be structured to be independently unit-testable, with test cases created to verify their correctness.
- **Rationale**: In desktop applications, crashes and abnormal terminations directly result in user data and work loss. Robust error handling and continuous test verification permanently guarantee application reliability.

## Technical Constraints & Quality Standards

### 1. Architecture & Technology Stack
- **GUI Layer**: Tauri v2, React, TypeScript, Tailwind CSS
- **Backend Layer**: Rust (2021 edition)
- **Excel Parsing Engine**: `calamine`, `rust_xlsxwriter`, `csv`
- **Parallel Scanning**: High-speed multithreaded scanning via `rayon`
- **Pattern Matching**: `regex`

### 2. Performance & Resource Management
- Even when searching large or numerous Excel files, asynchronous and parallel processing MUST be maintained to prevent excessive memory consumption and avoid blocking the UI thread.
- Full scans and duplicate reads SHOULD be avoided in favor of efficient streaming and iterative scanning.

### 3. Security & Privacy
- Search operations MUST be entirely self-contained within the local environment; file data and search queries MUST NOT be transmitted to external networks without explicit user consent.
- File access permissions and OS API calls MUST adhere to the principle of least privilege.
- **No Real Data in Persisted Artifacts**:
  Across all generated, persisted, and saved documents (such as `data-model.md`, `spec.md`, `plan.md`, `contracts/`, etc.), real user file names, real file paths, actual search results, or actual business data (data potentially containing confidential or personally identifiable information) MUST NOT be included or quoted. When describing data structures, schemas, and example outputs, entirely synthetic dummy data (e.g., `sample.xlsx`, `Sheet1`, `dummy value`) or abstract placeholders MUST always be used.

## Development Workflow & Quality Gates

### 1. Test-Driven Development & Verification
- For all new features and bug fixes, corresponding unit tests (Rust: `cargo test` / TypeScript: unit tests) MUST be created to verify behavior.
- Test cases MUST cover edge cases (empty files, corrupted workbooks, password-protected files, minimal/maximal cell values, etc.).

### 2. Code Quality & Static Analysis
- Rust code MUST pass `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`.
- TypeScript code MUST maintain zero type errors (`tsc --noEmit`) and zero linter warnings.
- Code reviews MUST verify that there are no missing header comments or remaining hardcoded constants.

### 3. AI-Generated Code & Artifact Quality Assurance
- Code and documents generated by AI agents MUST be inspected for garbled text or unwanted tokens such as `<PAD>`, and any defects MUST be self-corrected before submission.
- Even for automatically generated code, header comment requirements and constant extraction standards MUST be satisfied identically to manual implementations.
- All design artifacts generated and saved by AI agents (such as `data-model.md`, `contracts/`, `plan.md`, `spec.md`, etc.) MUST be inspected and guaranteed to contain zero real search results or actual user data.

## Governance

- **Supremacy**: This Constitution MUST supersede all other design documents, implementation rules, and operational guidelines within the project.
- **Compliance Verification**: Strict compliance with the principles defined in this Constitution MUST be a prerequisite for all changes (Pull Requests, reviews, and AI agent code generation).
- **Amendment Process**: Amendments to this Constitution MUST be made deliberately, requiring explicit statements of rationale, stakeholder review, and presentation of backward compatibility or migration plans.
- **Versioning Policy**: Semantic Versioning (MAJOR.MINOR.PATCH) MUST be strictly followed:
  - **MAJOR**: Removal of principles, non-backward-compatible policy changes, or comprehensive overhaul of governance rules
  - **MINOR**: Addition of new principles or sections, or substantive expansion of the scope of existing guidelines
  - **PATCH**: Clarification of wording, typo/formatting corrections, or non-semantic explanatory additions
- **Runtime Guidance**: Specification files under `.specify/` and this Constitution MUST be continuously referenced during development and specification drafting to maintain project-wide consistency.

**Version**: 3.2.0 | **Ratified**: 2026-09-25 | **Last Amended**: 2026-10-23

# AGENTS.md — Excel Seek Development Guidelines & Agent Code of Conduct

This file defines the highest-level conduct rules, coding standards, architecture, and development procedures that all AI agents working in the Excel Seek repository (including Antigravity, Claude, Codex, and Cursor) must follow.

---

## 1. Project Overview

- **Name**: Excel Seek (`xlseek`)
- **Purpose**: A desktop application that quickly and securely searches cells, sheets, and comments across large numbers of Excel files (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`) for specified keywords or regular expressions.
- **Architecture**:
  - **GUI / frontend**: React 18, TypeScript, Tailwind CSS, Lucide React, Tauri v2 API
  - **Backend / core engine**: Rust (2021 edition), `calamine` (fast Excel parsing), `rayon` (parallel multithreaded scanning), `rust_xlsxwriter` (Excel output), `regex` (regular expressions)
  - **Desktop platform**: Tauri v2 (`@tauri-apps/api` v2, `tauri-plugin-dialog`, `tauri-plugin-shell`)

---

## 2. Core Principles (In Accordance With the Project Constitution)

All agents **MUST** strictly follow the five principles in `.specify/memory/constitution.md` (the project constitution), without exception.

### Principle I. Language-Directed Quality
- Write responses to users, commit messages, generated documents, UI text, error messages, and logs in the language explicitly specified by the user or an approved specification.
- Use natural, accurate English by default for responses to users, commit messages, generated documents, source code comments, UI text, error messages, and logs when no language is specified. Do not force English translations where another language is specified, such as in multilingual resources including `crates/core/locales/ja.yml`.
- Before delivery, check for garbled text and unwanted or unexplained special tokens such as `<PAD>` and `<pad>`. Correct them using natural language appropriate to the specified language.

### Principle II. No Hardcoded Constants
- **MUST NOT** hardcode constant values (numbers, magic numbers, fixed strings, event names, command names, UI labels, etc.) directly in code, except `0` and the empty string (`""`).
- **Frontend constants**: Define and manage them centrally in `src/constants/index.ts`.
- **Rust backend constants**: Define and manage them centrally in `src-tauri/src/constants.rs`.
- Wherever a constant is used, include a comment explicitly identifying the referenced constant (for example, `// Constant reference: constants::MENU_ITEM_ABOUT_ID`).

### Principle III. Comprehensive Header Comments
Every file, module, struct, class, function, and method **MUST** have a header comment containing all three of the following:
1. **A detailed description of what it does**
2. **The types and descriptions of its arguments and return value**
3. **Possible errors, handling of `Result`/`Option`, or conditions that cause a panic or exception**

**MUST NOT** record or generate per-file change histories (version, creation date, author, or changes) in code header comments. Use version control, such as the Git commit history, for that purpose.

### Principle IV. Modular Design & Code Standards
- Follow the single-responsibility principle (SRP) and divide modules according to their responsibilities.
- Rust: Fully comply with the official style guide, `clippy`, and `rustfmt`.
- TypeScript: Maintain zero type errors (`tsc --noEmit`) and zero ESLint warnings.

### Principle V. Robust Error Handling & Testing
- Prevent unexpected panics and unhandled exceptions. Handle errors safely with `Result` or appropriate error types.
- Whenever a module or function is added or changed, create or update corresponding unit tests (`cargo test` / frontend tests) and verify its behavior.

---

## 3. Directory Structure and Responsibilities

```text
xlseek/
├── AGENTS.md                  # This file (AI agent code of conduct)
├── README.md                  # Project overview and feature descriptions
├── Cargo.toml                 # Cargo workspace definition (crates/core, crates/cli, src-tauri)
├── package.json               # Frontend dependencies and npm scripts
├── crates/                    # Shared backend core and standalone CLI crate
│   ├── core/                  # Search engine, parsers, exports, shared models (xlseek-core)
│   │   ├── Cargo.toml
│   │   ├── locales/           # Source translation catalogs (ja.yml, en.yml)
│   │   ├── src/               # lib.rs, constants.rs, models/, search/, export/, i18n.rs
│   │   └── tests/             # shape_*.rs integration tests
│   └── cli/                   # Standalone CLI binary (xlseek-cli)
│       ├── Cargo.toml
│       ├── src/               # main.rs, lib.rs, constants.rs, args.rs, output.rs
│       └── tests/             # cli_search.rs, cli_output.rs integration tests
├── src/                       # Frontend source code (React / TypeScript)
│   ├── App.tsx                # Root component (state binding and dialog integration)
│   ├── main.tsx               # Entry point
│   ├── constants/             # Centralized constants module (index.ts, licenses.json)
│   ├── types/                 # TypeScript type definitions (search.ts, license.ts)
│   ├── hooks/                 # Custom hooks (useSearch.ts, etc.)
│   └── components/            # UI components
│       ├── about/             # About and OSS license dialogs (AboutDialog.tsx, etc.)
│       ├── common/            # Shared components (StatusBar.tsx, Toast.tsx)
│       ├── layout/            # Window frame and drag region (WindowFrame.tsx)
│       ├── preview/           # Excel cell preview grid and formula bar
│       ├── results/           # Search results table (virtual scrolling)
│       └── search/            # Search input bar, extension toggles, and controls
├── src-tauri/                 # Tauri v2 desktop GUI application (xlseek)
│   ├── Cargo.toml             # Rust crate dependencies (depends on xlseek-core)
│   ├── tauri.conf.json        # Tauri configuration (window settings, permissions)
│   ├── capabilities/          # Tauri v2 security capabilities (default.json)
│   └── src/
│       ├── lib.rs             # App initialization, menu construction, event handlers
│       ├── main.rs            # Executable entry point
│       ├── constants.rs       # Centralized GUI-specific constants and unit tests
│       └── commands/          # Tauri IPC command handlers (search, preview, export, system)
├── specs/                     # Spec Kit feature specs, plans, and task tracking
├── design/                    # UI mockups and standalone HTML prototypes
└── .specify/                  # Spec Kit workflow infrastructure
    ├── memory/constitution.md # Project constitution (highest-priority governance document)
    ├── bugs/                  # Bug assessment, fix, and verification reports
    └── extensions/            # Spec Kit extensions (git, bug, etc.)
```

---

## 4. Required Verification Commands (Quality Gates)

After changing or fixing code, **MUST** run all of the following verification commands before reporting completion and confirm that there are no errors or warnings.

| Layer | Verification command | Purpose and passing criteria |
| :--- | :--- | :--- |
| **Frontend** | `npm run build` | TypeScript type checking (`tsc`) and the Vite bundle complete successfully with exit code 0 |
| **Rust Test** | `cargo test --workspace` | All unit and integration tests in every crate pass with no failures |
| **Rust Clippy** | `cargo clippy --workspace --all-targets -- -D warnings` | No compiler or Clippy warnings in any crate |
| **Rust Format** | `cargo fmt --check` | All workspace code fully conforms to the official Rust formatting rules |

---

## 5. Spec Kit & Bug Workflow

This project uses the Spec Kit toolchain and related skills.

### Bug Triage & Fix Workflow
1. **Bug assessment (`/speckit-bug-assess`)**:
   - Gather symptoms, identify the code path, develop a root-cause hypothesis, and propose a fix.
   - Deliverable: `.specify/bugs/<slug>/assessment.md`
2. **Bug fix (`/speckit-bug-fix`)**:
   - Make minimal, constitution-compliant code changes and add tests based on `assessment.md`.
   - Deliverable: `.specify/bugs/<slug>/fix.md`
3. **Bug verification (`/speckit-bug-test`)**:
   - Run automated tests and build checks, confirm the bug does not recur, and record a verification report.
   - Deliverable: `.specify/bugs/<slug>/test.md`
4. **Commit (`/speckit-git-commit`)**:
   - Commit changes through an automatic commit hook or command.

### Keep the UI Mockup in Sync
- If you change component structure, visual styles, or UI options under `src/`, use the `syncing-mainui-mock` skill to keep the standalone HTML prototype at `design/mainui/index.html` consistent.

---

## 6. Prohibited Coding Practices for Agents (Anti-Patterns)

- ❌ **Hardcoded magic numbers or strings**: Extract all constants other than `0` and `""` into the appropriate constants file.
- ❌ **Missing header comments or unnecessary change histories**: Never omit any of the three required header-comment elements, even for generated code or small fixes. Do not include change histories in file headers.
- ❌ **Reporting completion without type checks or tests**: Do not claim success without evidence from `npm run build` and `cargo test`.
- ❌ **Excessive refactoring**: Do not change unrelated code outside the scope of a bug fix or feature implementation (YAGNI principle).

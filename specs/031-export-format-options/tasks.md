# Tasks: Search Results Export Format Options and UI Reorganization

**Input**: Design documents from `/specs/031-export-format-options/` (`plan.md`, `spec.md`, `data-model.md`, `contracts/`, `research.md`, `quickstart.md`)

**Prerequisites**: `plan.md`, `spec.md`, `data-model.md`, `contracts/export-ipc.md`, `contracts/cli-export.md`

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Includes exact file paths in every task description

---

## Phase 1: Setup (Constants & Shared Infrastructure)

**Purpose**: Establish centralized constants, file extensions, and translation keys across core, backend, CLI, and frontend.

- [X] T001 [P] Define export format identifiers (`EXT_MD`, `EXT_JSON`, `CLI_FORMAT_MD`, `CLI_FORMAT_JSON`), default export format constant (`DEFAULT_EXPORT_FORMAT = "xlsx"`), and translation key constants in `crates/core/src/constants.rs`
- [X] T002 [P] Define UI translation keys for Markdown/JSON export buttons, dialog filters, and completion toast notifications in `crates/core/locales/en.yml` and `crates/core/locales/ja.yml`
- [X] T003 [P] Re-export new format constants in `src-tauri/src/constants.rs` and `crates/cli/src/constants.rs`
- [X] T004 [P] Define frontend constants (`EXPORT_FORMATS`, `EXPORT_FORMAT_STORAGE_KEY`, `DEFAULT_EXPORT_FORMAT`) in `src/constants/index.ts`

---

## Phase 2: Foundational (Type Models & Shared Crate Infrastructure)

**Purpose**: Core model updates that all user stories and export dispatchers depend on.

**⚠️ CRITICAL**: Must be completed before User Story implementation begins.

- [X] T005 [P] Extend `ExportFormat` enum (`Csv`, `Xlsx`, `Md`, `Json`) with serde lowercase serialization and update `ExportRequest` with optional `llm_options: Option<LlmCopyOptions>` in `crates/core/src/models/mod.rs`
- [X] T006 [P] Update frontend `ExportFormat` union type (`"csv" | "xlsx" | "md" | "json"`) and `ExportRequest` interface with `llm_options?: LlmCopyOptions` in `src/types/search.ts`
- [X] T007 Re-export `md_export` and `json_export` submodules in `crates/core/src/export/mod.rs`

**Checkpoint**: Core data models and constants ready - user story implementation can proceed.

---

## Phase 3: User Story 1 - Consolidated Export Split Button with Format Menu (Priority: P1) 🎯 MVP

**Goal**: Deliver a unified Split Button in the status bar where the main button exports in the active format and the `▾` toggle opens a format picker (CSV, Excel, AI Markdown, JSON) that remembers the active format and opens the save dialog immediately.

**Independent Test**: Perform a search yielding results. Click the primary button to open the save dialog for the active format. Click the `▾` toggle to open the menu, select another format, and verify the button label/icon updates, persists to `localStorage`, and opens the save dialog for that format. Verify button is disabled when results list is empty.

### Implementation for User Story 1

- [X] T008 [P] [US1] Create frontend unit tests for Export Split Button rendering, format switching, menu open/close, and disabled state in `src/components/common/__tests__/StatusBar.test.tsx`
- [X] T009 [US1] Implement Export Split Button component with upward format dropdown menu, `Escape` key and outside-click dismissal, and `localStorage` format persistence in `src/components/common/StatusBar.tsx`
- [X] T010 [US1] Pass defaultOptions-derived `LlmCopyOptions` (`column_window_size`, `copy_type`, `word_extraction_mode`) into `StatusBar` in `src/App.tsx`
- [X] T011 [US1] Synchronize the standalone HTML UI mockup status bar export split button and upward format menu in `design/mainui/index.html`

**Checkpoint**: User Story 1 (MVP) is fully functional in the UI; format switching and dialog triggers operate cleanly.

---

## Phase 4: User Story 2 - Export Search Results as AI-Ready Markdown / Text (Priority: P2)

**Goal**: Enable exporting search results to structured `.md` files using `extract_llm_context` and active user AI Copy preferences.

**Independent Test**: Export search matches to a `.md` file. Verify the file contains valid Markdown with document paths, sheet names, cell addresses, matched content, and surrounding table/header context matching the existing AI Copy output format.

### Implementation for User Story 2

- [X] T012 [P] [US2] Write unit tests for Markdown export generation, workbook grouping, and empty match slice handling in `crates/core/tests/export_md_test.rs`
- [X] T013 [US2] Implement `write_md_to_writer` and `export_to_md` reusing `extract_llm_context` with atomic temp-file staging in `crates/core/src/export/md_export.rs`
- [X] T014 [US2] Wire `ExportFormat::Md` handling and `llm_options` extraction in Tauri command handler `src-tauri/src/commands/export_cmd.rs`

**Checkpoint**: User Stories 1 and 2 functional; AI Markdown files can be exported from GUI.

---

## Phase 5: User Story 3 - Export Search Results as Structured JSON Aligned with MCP Schema (Priority: P3)

**Goal**: Enable exporting search results to structured, 2-space pretty-printed `.json` files containing top-level `total_matches` metadata and a `matches` array with normalized paths.

**Independent Test**: Export search matches to a `.json` file. Validate that the file is syntactically valid JSON with `total_matches` and `matches` array containing normalized paths, sheet names, cell coordinates, and match details.

### Implementation for User Story 3

- [X] T015 [P] [US3] Write unit tests for JSON export serialization, schema fields, and path normalization in `crates/core/tests/export_json_test.rs`
- [X] T016 [US3] Implement `JsonExportPayload`, `JsonExportMatch`, `write_json_to_writer`, and `export_to_json` with atomic temp-file staging in `crates/core/src/export/json_export.rs`
- [X] T017 [US3] Wire `ExportFormat::Json` handling in Tauri command handler `src-tauri/src/commands/export_cmd.rs`

**Checkpoint**: User Stories 1, 2, and 3 functional; JSON export works seamlessly alongside Markdown, CSV, and XLSX.

---

## Phase 6: User Story 4 - Preserve Existing CSV/Excel Export & Extend CLI Integration (Priority: P4)

**Goal**: Preserve 100% backward compatibility for CSV and XLSX export while extending `xlseek-cli` to support `-f <csv|xlsx|md|json>`, `.md`/`.json` file extension inference, and `stdout` streaming.

**Independent Test**: Run `xlseek-cli` with `-f md`, `-f json`, and file output (`-o file.md`, `-o file.json`). Verify stdout streaming and file outputs match expected formats, and verify existing CSV/XLSX export tests pass without regressions.

### Implementation for User Story 4

- [X] T018 [P] [US4] Write integration tests for CLI `-f md`, `-f json`, `-o file.md`, `-o file.json`, and stdout streaming in `crates/cli/tests/cli_search.rs`
- [X] T019 [US4] Update CLI argument parsing for `-f <csv|xlsx|md|json>` and `.md`/`.json` extension inference in `crates/cli/src/args.rs`
- [X] T020 [US4] Update CLI result publication to support `ExportFormat::Md` and `ExportFormat::Json` via temporary directory staging in `crates/cli/src/output.rs`
- [X] T021 [US4] Update CLI stdout streaming for Markdown and JSON output in `crates/cli/src/lib.rs`

**Checkpoint**: All 4 user stories complete across GUI, CLI, and core engine.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation updates, quality gate verification, and end-to-end scenario validation.

- [X] T022 [P] Update CLI documentation and command-line reference in `crates/cli/README.md` and workspace `README.md` / `README_ja.md`
- [X] T023 Run all workspace quality gates (`npm run build`, `npm test -- --run`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`)
- [X] T024 Validate end-to-end scenarios per `specs/031-export-format-options/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - BLOCKS all User Stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 (Foundational).
- **User Story 2 (Phase 4)**: Depends on Phase 2 (Foundational). Can proceed in parallel with US1.
- **User Story 3 (Phase 5)**: Depends on Phase 2 (Foundational). Can proceed in parallel with US1 and US2.
- **User Story 4 (Phase 6)**: Depends on Phase 4 (US2) and Phase 5 (US3) export generators.
- **Polish (Phase 7)**: Depends on all User Story phases being complete.

### Parallel Opportunities

- **Phase 1 (Setup)**: Tasks T001, T002, T003, and T004 can all execute in parallel.
- **Phase 2 (Foundational)**: Tasks T005 and T006 can execute in parallel.
- **Phase 4 & 5 (US2 & US3)**: Core `md_export.rs` (T012, T013) and `json_export.rs` (T015, T016) can be developed in parallel as independent modules.

---

## Parallel Example: User Stories 2 & 3

```bash
# Developer / Agent A: Markdown Export
Task: "Write unit tests for Markdown export in crates/core/tests/export_md_test.rs"
Task: "Implement write_md_to_writer in crates/core/src/export/md_export.rs"

# Developer / Agent B: JSON Export
Task: "Write unit tests for JSON export in crates/core/tests/export_json_test.rs"
Task: "Implement JsonExportPayload and export_to_json in crates/core/src/export/json_export.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Complete Phase 3 (User Story 1).
3. **STOP and VALIDATE**: Verify status bar split button renders, switches formats, toggles menu, and invokes export dialog.

### Incremental Delivery

1. Setup + Foundational → Core models ready.
2. User Story 1 → Split Button UI integrated in GUI (MVP).
3. User Story 2 → AI Markdown export backend and GUI integration.
4. User Story 3 → Structured JSON export backend and GUI integration.
5. User Story 4 → CLI `--format md|json` and stdout streaming.
6. Polish → Full test suite, clippy, formatting, and documentation.

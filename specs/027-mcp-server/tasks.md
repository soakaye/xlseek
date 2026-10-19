# Implementation Tasks: MCP Server Implementation Based on xlseek-cli

**Feature**: MCP Server Implementation Based on `xlseek-cli`  
**Feature Branch**: `027-mcp-server`  
**Date**: 2026-10-19  
**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)  

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize the new `crates/mcp` crate in the Cargo workspace and establish shared constants and manifest configurations.

- [ ] T001 Register `crates/mcp` as a workspace member in root `Cargo.toml`
- [ ] T002 Create package manifest and crate overview for `xlseek-mcp` in `crates/mcp/Cargo.toml` and `crates/mcp/README.md`
- [ ] T003 [P] Define centralized constants (protocol version, tool names, default row limits, error codes) and constant reference unit tests in `crates/mcp/src/constants.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core JSON-RPC 2.0 framing, stdio communication loop, error models, and path security that MUST be complete before tool handlers can be implemented.

**⚠️ CRITICAL**: No user story work can begin until this foundational phase is complete.

- [ ] T004 Implement JSON-RPC 2.0 error structures and standard error code mappings in `crates/mcp/src/protocol/error.rs`
- [ ] T005 [P] Implement JSON-RPC 2.0 request and response data models with Serde serialization in `crates/mcp/src/protocol/messages.rs`
- [ ] T006 [P] Implement directory whitelist and path sandboxing logic with canonical path prefix verification in `crates/mcp/src/security.rs`
- [ ] T007 Expose protocol types and validation modules in `crates/mcp/src/protocol/mod.rs`
- [ ] T008 Implement the core stdio event loop and protocol handshake handlers (`initialize`, `notifications/initialized`, `ping`) in `crates/mcp/src/lib.rs`
- [ ] T009 Implement executable entry point and CLI option parsing (`--allowed-dir`, `--help`, `--version`) in `crates/mcp/src/main.rs`
- [ ] T010 [P] Add protocol handshake and lifecycle integration tests in `crates/mcp/tests/protocol_test.rs`

**Checkpoint**: Foundation ready — JSON-RPC 2.0 stdio dispatcher and lifecycle communication verified. User story tool implementation can now begin.

---

## Phase 3: User Story 1 - Fast Cross-Document Search via MCP (Priority: P1) 🎯 MVP

**Goal**: Enable AI assistants (Claude Desktop, Cursor, Antigravity) to perform high-speed cross-document searches across Excel (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`) and Word (`.docx`, `.doc`) documents via the `xlseek_search` tool over stdio.

**Independent Test**: Pipe JSON-RPC `tools/call` for `xlseek_search` into the server over stdio with a target directory and query. Verify structured search hits with file paths, sheet names, cell coordinates, and matched snippets are returned without process crashes.

### Tests for User Story 1

- [ ] T011 [P] [US1] Create integration tests for `xlseek_search` covering multi-path search, regex, case sensitivity, extension filters, and truncation in `crates/mcp/tests/tools_search_test.rs`

### Implementation for User Story 1

- [ ] T012 [P] [US1] Implement `xlseek_search` parameter extraction, `xlseek_core::models::SearchQuery` mapping, and structured result formatting in `crates/mcp/src/tools/search.rs`
- [ ] T013 [US1] Implement tool catalog registry exposing `xlseek_search` JSON Schema in `tools/list` and dispatching in `crates/mcp/src/tools/mod.rs`
- [ ] T014 [US1] Connect `tools/list` and `tools/call` for `xlseek_search` to the stdio message dispatcher in `crates/mcp/src/lib.rs`

**Checkpoint**: User Story 1 (MVP) is fully functional and testable independently. AI assistants can discover and call `xlseek_search`.

---

## Phase 4: User Story 2 - Worksheet Listing and Markdown Grid Cell Preview (Priority: P2)

**Goal**: Allow AI assistants to list worksheets within an Excel workbook and retrieve a tabular cell preview formatted as a clean Markdown table for specific sheets.

**Independent Test**: Call `xlseek_list_sheets` on a test workbook to verify sheet names, then call `xlseek_preview_sheet` to verify the returned Markdown table matches cell data within requested row limits (`start_row` and `max_rows`).

### Tests for User Story 2

- [ ] T015 [P] [US2] Create integration tests for worksheet listing and Markdown preview table generation in `crates/mcp/tests/tools_preview_test.rs`

### Implementation for User Story 2

- [ ] T016 [P] [US2] Implement `xlseek_list_sheets` parameter parsing and workbook sheet extraction using `calamine` in `crates/mcp/src/tools/list_sheets.rs`
- [ ] T017 [P] [US2] Implement `xlseek_preview_sheet` parameter parsing, cell range extraction, and Markdown table rendering in `crates/mcp/src/tools/preview.rs`
- [ ] T018 [US2] Register `xlseek_list_sheets` and `xlseek_preview_sheet` schemas and call handlers in `crates/mcp/src/tools/mod.rs` and `crates/mcp/src/lib.rs`

**Checkpoint**: User Stories 1 and 2 are functional and independently testable. AI assistants can search documents and preview specific sheet structures.

---

## Phase 5: User Story 3 - Robust and Secure Stdio MCP Connection with Path Sandboxing (Priority: P3)

**Goal**: Guarantee directory whitelist boundary enforcement, strict `stdout`/`stderr` channel separation, and 0% crash rate on corrupted files, missing paths, and permission errors.

**Independent Test**: Attempt tool calls against paths outside `--allowed-dir`, invalid regular expressions, and corrupt archives. Verify well-formed tool errors (`isError: true`) are returned, `stdout` remains pristine JSON-RPC, and the process remains running.

### Tests for User Story 3

- [ ] T019 [P] [US3] Create integration tests for directory whitelist rejection, corrupt file resilience, and invalid argument handling in `crates/mcp/tests/sandboxing_test.rs`

### Implementation for User Story 3

- [ ] T020 [US3] Integrate `crates/mcp/src/security.rs` path validation checks into all tool execution pipelines in `crates/mcp/src/tools/mod.rs`
- [ ] T021 [US3] Enforce non-panicking error conversions and strict stderr-only diagnostic logging across `crates/mcp/src/lib.rs`

**Checkpoint**: All three user stories are complete, resilient, and securely bounded.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation updates, workspace integration, constitution compliance verification, and end-to-end quickstart validation.

- [ ] T022 [P] Update workspace documentation in `README.md` and `README_ja.md` with `xlseek-mcp` configuration and usage examples
- [ ] T023 Run full constitution quality verification (`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`)
- [ ] T024 Validate end-to-end scenarios per `specs/027-mcp-server/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately.
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS all user stories.
- **User Story 1 (Phase 3 - MVP)**: Depends on Phase 2 completion.
- **User Story 2 (Phase 4)**: Depends on Phase 2 completion (can proceed independently of or following US1).
- **User Story 3 (Phase 5)**: Depends on Phase 2, Phase 3, and Phase 4 completion (secures and hardens all tool pipelines).
- **Polish (Phase 6)**: Depends on all user story phases being complete.

### User Story Dependencies

```text
[Phase 1: Setup]
       │
       ▼
[Phase 2: Foundational (JSON-RPC & Stdio Dispatcher)]
       │
   ┌───┴───────────────────────┐
   ▼                           ▼
[Phase 3: US1 - Search]    [Phase 4: US2 - Preview & Sheets]
   │                           │
   └───┬───────────────────────┘
       ▼
[Phase 5: US3 - Sandboxing & Hardening]
       │
       ▼
[Phase 6: Polish & Verification]
```

### Parallel Opportunities

- **Phase 1**: T003 can be authored in parallel with T002.
- **Phase 2**: T005, T006, and T010 can be developed in parallel once T004 is established.
- **Phase 3 (US1)**: Test T011 and parameter parsing T012 can run in parallel.
- **Phase 4 (US2)**: T015 (tests), T016 (`list_sheets`), and T017 (`preview_sheet`) can run in parallel.
- **Phase 5 (US3)**: Test T019 can run in parallel with security integration.
- **Phase 6**: T022 (docs) can run in parallel with verification tasks.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1: Setup (T001 - T003).
2. Complete Phase 2: Foundational (T004 - T010).
3. Complete Phase 3: User Story 1 (T011 - T014).
4. **STOP and VALIDATE**: Test `xlseek_search` independently with `crates/mcp/tests/tools_search_test.rs` and stdio piping.
5. Demonstrate working MVP for cross-document Excel/Word search via MCP.

### Incremental Delivery
1. Foundation & Handshake (Phases 1-2) → Server boots, registers with MCP clients, responds to `initialize` and `tools/list`.
2. MVP (Phase 3) → Adds `xlseek_search`, enabling core file search capabilities.
3. Workbook Inspection (Phase 4) → Adds `xlseek_list_sheets` and Markdown `xlseek_preview_sheet`.
4. Sandboxing & Resilience (Phase 5) → Enforces directory boundaries and zero-panic error handling.
5. Verification & Release (Phase 6) → Quality gates pass and documentation updated.

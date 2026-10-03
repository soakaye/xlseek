# Tasks: Fix Open Folder Functionality on Ubuntu

**Feature**: `023-fix-ubuntu-open-folder`
**Plan**: [specs/023-fix-ubuntu-open-folder/plan.md](plan.md)
**Spec**: [specs/023-fix-ubuntu-open-folder/spec.md](spec.md)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Define Linux external desktop command constants and URI helpers in centralized constants module.

- [X] T001 [P] Define Linux desktop command constants in `src-tauri/src/constants.rs` (`CMD_DBUS_SEND`, `CMD_GIO`, `CMD_XDG_OPEN`, `DBUS_DEST_FILEMANAGER`, `DBUS_PATH_FILEMANAGER`, `DBUS_METHOD_SHOW_ITEMS`, `FILE_URI_PREFIX`)
- [X] T002 [P] Add unit tests for newly added constants in `src-tauri/src/constants.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core URI formatting and path handling utilities required by the Linux file manager integration.

**⚠️ CRITICAL**: Must complete before user story implementation begins.

- [X] T003 Implement path-to-file-URI conversion helper function with percent-encoding in `src-tauri/src/commands/system_cmd.rs`
- [X] T004 Add comprehensive unit tests for path-to-file-URI conversion (testing spaces, non-ASCII characters, symbols) in `src-tauri/src/commands/system_cmd.rs`

**Checkpoint**: Foundation ready - URI encoding and constants are in place.

---

## Phase 3: User Story 1 - Reveal Found File in Default File Manager on Linux / Ubuntu (Priority: P1) 🎯 MVP

**Goal**: On Ubuntu / Linux, clicking "Open in Folder" invokes the desktop file manager via DBus `org.freedesktop.FileManager1.ShowItems` to highlight the target file, falling back cleanly to parent folder opening if highlighting is unsupported.

**Independent Test**: On Linux, invoking `open_in_folder` with an existing file path launches the file manager highlighting the file (or opening its parent directory) and returns `Ok(())` without error.

### Implementation for User Story 1

- [X] T005 [US1] Implement Linux `open_file_in_folder_linux` helper function executing DBus `ShowItems` via `std::process::Command` in `src-tauri/src/commands/system_cmd.rs`
- [X] T006 [US1] Integrate `open_file_in_folder_linux` into `open_in_folder` under `#[cfg(target_os = "linux")]` in `src-tauri/src/commands/system_cmd.rs`
- [X] T007 [US1] Add unit tests verifying `open_in_folder` returns `ErrorCode::PathNotFound` for non-existent files on Linux in `src-tauri/src/commands/system_cmd.rs`

**Checkpoint**: At this point, User Story 1 delivers full MVP functionality on Ubuntu with default desktop integration.

---

## Phase 4: User Story 2 - Resilient Fallback and Clear Error Feedback on Linux (Priority: P2)

**Goal**: Provide resilient fallbacks for lightweight or non-DBus Linux environments (`gio open <parent>` -> `xdg-open <parent>` -> `open::that(<parent>)`) and clear error handling.

**Independent Test**: On a Linux environment where DBus `FileManager1` fails or is absent, invoking `open_in_folder` successfully falls back to opening the parent folder via `gio` or `xdg-open`, or returns `ErrorCode::FolderOpenFailed` if all openers fail.

### Implementation for User Story 2

- [X] T008 [US2] Implement sequential fallback pipeline (`gio open <parent>` -> `xdg-open <parent>` -> `open::that(parent)`) in `src-tauri/src/commands/system_cmd.rs`
- [X] T009 [US2] Ensure all fallback child processes properly capture output, avoid zombie processes, and map exhausted opener failures to `ErrorCode::FolderOpenFailed` in `src-tauri/src/commands/system_cmd.rs`
- [X] T010 [US2] Add unit tests validating fallback dispatch and error conversion in `src-tauri/src/commands/system_cmd.rs`

**Checkpoint**: User Stories 1 AND 2 are fully implemented and resilient across all Linux desktop configurations.

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates, static analysis, formatting, and documentation compliance.

- [X] T011 [P] Verify full compliance with Constitution Principle I, II (constant references), and III (Rustdoc headers with descriptions, arguments, returns, errors) across all modified files
- [X] T012 Run automated verification suite (`cargo test --workspace`, `npm run build`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`)
- [X] T013 Execute manual validation scenarios described in `specs/023-fix-ubuntu-open-folder/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately (T001, T002 in parallel)
- **Foundational (Phase 2)**: Depends on Phase 1 completion (T003, T004)
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion (T005 → T006 → T007)
- **User Story 2 (Phase 4)**: Depends on Phase 3 completion (T008 → T009 → T010)
- **Polish (Phase 5)**: Depends on Phase 4 completion (T011, T012, T013)

### Parallel Opportunities

- T001 and T002 can be designed/reviewed together.
- T011 and T013 can run in parallel with T012 verification.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Constants) + Phase 2 (Foundational URI helpers).
2. Complete Phase 3 (User Story 1: DBus file highlight & parent folder opening).
3. Validate User Story 1: Proves primary Ubuntu Nautilus integration works.

### Incremental Delivery
1. Add Phase 4 (User Story 2: GIO, XDG, and open crate fallbacks).
2. Complete Phase 5 (Quality gates and Constitution compliance).

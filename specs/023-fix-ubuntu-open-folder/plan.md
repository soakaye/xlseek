# Implementation Plan: Fix Open Folder Functionality on Ubuntu

**Branch**: `023-fix-ubuntu-open-folder` | **Date**: 2026-10-02 | **Spec**: [specs/023-fix-ubuntu-open-folder/spec.md](spec.md)

**Input**: Feature specification from `specs/023-fix-ubuntu-open-folder/spec.md`

## Summary

This feature resolves the issue where the "Open in Folder" button in the file preview pane fails to open the enclosing folder on Ubuntu and other Linux desktop distributions. On Linux, we implement a priority execution pipeline:
1. Attempt to reveal and highlight the specific file in the desktop file manager using the Freedesktop `org.freedesktop.FileManager1.ShowItems` DBus interface (`dbus-send`).
2. If DBus is unavailable or fails, fall back to opening the parent directory using GNOME `gio open <parent>`.
3. If `gio` is unavailable or fails, fall back to `xdg-open <parent>`.
4. As a final fallback, invoke the cross-platform `open` crate on the parent directory.

All constants are centrally managed in `src-tauri/src/constants.rs`, with zero magic strings and full compliance with Constitution principles.

## Technical Context

**Language/Version**: Rust 2021 edition (backend `src-tauri` / `crates/core`), TypeScript 5.x / React 18 (frontend)

**Primary Dependencies**: Tauri v2, `open` crate (v5.4.4), `std::process::Command`

**Storage**: Local filesystem paths

**Testing**: `cargo test --workspace`, `npm run build`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`

**Target Platform**: Linux (Ubuntu 20.04+, GNOME / XFCE / KDE), macOS, Windows

**Project Type**: Desktop GUI Application (Tauri v2 + React)

**Performance Goals**: File manager launches in < 1.5 seconds from user click; zero UI freezes or hangs; instantaneous error notification (< 500ms) for non-existent files.

**Constraints**: Local execution only; no external network requests; zero unhandled panics; strict adherence to Constitution Principles I-V.

**Scale/Scope**: Impacts `src-tauri/src/commands/system_cmd.rs` and `src-tauri/src/constants.rs`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS. All documentation, comments, and commit messages are in English.
- **Principle II (No Hardcoded Constants)**: PASS. All command names (`dbus-send`, `gio`, `xdg-open`), arguments, and interface names are defined in `src-tauri/src/constants.rs` with explicit `// Constant reference:` comments.
- **Principle III (Comprehensive Header Comments)**: PASS. Every module, function, and helper includes complete description, arguments, returns, and error handling headers without per-file change history.
- **Principle IV (Modular Design & Code Standards)**: PASS. Changes are confined to `src-tauri/src/commands/system_cmd.rs` and `src-tauri/src/constants.rs`. Passes `cargo clippy` and `cargo fmt`.
- **Principle V (Robust Error Handling & Testing)**: PASS. Safely handles missing files, spaces in filenames, non-zero process exits, and missing system tools with `Result` and `ErrorCode`. Unit tests added for path-to-URI conversion and validation.

## Project Structure

### Documentation (this feature)

```text
specs/023-fix-ubuntu-open-folder/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── open_in_folder.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── commands/
│   │   ├── system_cmd.rs    # open_in_folder implementation & Linux pipeline
│   │   └── mod.rs
│   ├── constants.rs         # Centralized constants (Linux command names & arguments)
│   └── lib.rs
crates/
└── core/                    # Unmodified core data structures and error models
src/
└── components/preview/      # Existing UI invoking open_in_folder (unchanged)
```

**Structure Decision**: Standard Tauri v2 desktop application layout. Modifications are confined to the backend command layer in `src-tauri`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*(No violations. Design strictly complies with all Constitution principles.)*

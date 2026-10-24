# Research & Technical Decisions: Search Results Export Format Options and UI Reorganization

**Feature**: `031-export-format-options`  
**Date**: 2026-10-24

## 1. AI Markdown Export Implementation (`md_export.rs`)

- **Decision**: Reuse `xlseek_core::search::llm_context::extract_llm_context` inside a new `crates/core/src/export/md_export.rs` module exposing `write_md_to_writer` and `export_to_md`.
- **Rationale**:
  - Guarantees 100% consistency between clipboard-based "AI Copy" and file-based "AI Markdown (`.md`)" export (same worksheet grouping, header/row-key detection, column windowing, and Word paragraph extraction).
  - Avoids duplicating workbook/document context extraction logic.
  - Accepts `&LlmCopyOptions` (populated from the user's configured AI Copy preferences in GUI, or `LlmCopyOptions::default()` in CLI).
  - For empty `items` slices (`items.is_empty()`), `write_md_to_writer` writes nothing (`Ok(())`) so CLI `stdout` / file export with 0 matches succeeds cleanly just like CSV and XLSX exports.
- **Alternatives Considered**:
  - *Flat summary text format without re-reading workbooks*: Rejected during clarification (Session 2026-10-24 Q1 -> Option A) because users expect the same rich table/header context produced by AI Copy.

## 2. Structured JSON Export Schema (`json_export.rs`)

- **Decision**: Create `crates/core/src/export/json_export.rs` exposing `write_json_to_writer` and `export_to_json` using `serde_json::to_writer_pretty` (2-space indentation, UTF-8) with a top-level `JsonExportPayload` struct (`total_matches` and `matches: Vec<JsonExportMatch>`).
- **Rationale**:
  - Directly satisfies Clarification Q2 (Option A: metadata + structured `matches` array).
  - Applies `crate::export::normalize_export_path` to `full_path` on every serialized match record so Windows verbatim prefixes (`\\?\`, `\\?\UNC\`) are stripped identically to CSV and XLSX exports.
  - `serde_json` is already a workspace dependency in `crates/core`, requiring zero new third-party crates.
- **Alternatives Considered**:
  - *Raw top-level JSON array `[...]`*: Rejected in Q2 because top-level metadata (`total_matches`) makes programmatic consumption and MCP tool alignment cleaner.
  - *MCP `ToolCallResult` JSON-RPC envelope*: Rejected in Q2 because wrapping structured data inside a single escaped string inside `content[0].text` forces consumers to parse text rather than native JSON fields.

## 3. Status Bar Export Split Button & Format Persistence (`StatusBar.tsx`)

- **Decision**: Replace the two separate `CSV` and `Excel` buttons in `src/components/common/StatusBar.tsx` with a single accessible Split Button component:
  - **Primary button (left segment)**: Displays the icon and label of the currently active `ExportFormat` (initial default: `"xlsx"`). Clicking it immediately invokes `handleExport(activeFormat)`.
  - **Dropdown toggle (right segment `▾`)**: Opens an upward-positioned menu (`bottom-full right-0 mb-1.5`) listing all four formats (`xlsx`, `csv`, `md`, `json`).
  - **Menu selection behavior**: Selecting any format from the menu updates `activeFormat`, persists it to `localStorage` under a centralized constant key (`EXPORT_FORMAT_STORAGE_KEY`), closes the menu, and immediately opens the OS file save dialog for that format (Clarification Q4 -> Option A).
  - **Dismissal**: Clicking outside (via `mousedown` listener on `document`) or pressing `Escape` closes the menu without triggering an export.
- **Rationale**:
  - Reduces horizontal width in the status bar while supporting 4 formats instead of 2.
  - Preserves 1-click export for users who repeatedly export in the same format, and 2-click export when switching formats.
  - Persisting `activeFormat` in a dedicated `localStorage` key avoids altering the `SavedSettings` schema validation contract while surviving app restarts.
- **Alternatives Considered**:
  - *Single dropdown button (always 2 clicks)*: Rejected in Q3 in favor of Split Button (Option B).
  - *OS Save Dialog filter selection only*: Rejected in Q3 because Tauri's native `save` dialog does not reliably return which filter dropdown entry the user picked across all OS platforms if the user does not manually type an extension.

## 4. Unified `ExportFormat` Across Core, GUI, and CLI

- **Decision**: Extend `xlseek_core::models::ExportFormat` to `{ Csv, Xlsx, Md, Json }` (serialized lowercase: `"csv"`, `"xlsx"`, `"md"`, `"json"`), and update `xlseek-cli` argument parsing (`-f, --format <csv|xlsx|md|json>` and `-o` extension inference for `.csv`, `.xlsx`, `.md`, `.json`) as well as `stdout` streaming for `csv`, `md`, and `json`.
- **Rationale**:
  - Approved in Clarification Q5 (Option A).
  - Keeps `ExportFormat` as a single shared enum across `xlseek-core`, `xlseek-cli`, and `src-tauri` without divergent types or `unreachable!()` branches.
  - Enables batch CLI workflows to pipe JSON (`xlseek -f json "keyword" ./dir | jq ...`) or AI Markdown (`xlseek -f md "keyword" ./dir > context.md`) directly.
- **Alternatives Considered**:
  - *GUI-only support*: Rejected in Q5 because `xlseek-core` is shared with `xlseek-cli` and supporting CLI requires minimal argument parser additions.

## 5. Standalone UI Mockup Synchronization (`design/mainui/index.html`)

- **Decision**: Update `design/mainui/index.html` status bar markup to replace the separate CSV/Excel buttons with the consolidated Export Split Button and upward format menu.
- **Rationale**:
  - Mandated by `AGENTS.md` Section 5 ("If you change component structure, visual styles, or UI options under `src/`, use the `syncing-mainui-mock` skill to keep the standalone HTML prototype at `design/mainui/index.html` consistent").

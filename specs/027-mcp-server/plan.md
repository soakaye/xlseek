# Implementation Plan: MCP Server Implementation Based on xlseek-cli

**Branch**: `027-mcp-server` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/027-mcp-server/spec.md`

## Summary

Implement a Model Context Protocol (MCP) compliant server as an independent binary crate `crates/mcp` (executable: `xlseek-mcp`). The server communicates over standard I/O (stdio) using newline-delimited JSON-RPC 2.0 messages, exposes three dedicated tools (`xlseek_search`, `xlseek_list_sheets`, `xlseek_preview_sheet`), and reuses high-performance parallel traversal and parsing logic from `xlseek-core`. It guarantees clean stdio separation (diagnostics strictly on `stderr`), robust error handling (0% crash rate), and optional directory whitelisting for sandboxed filesystem access.

## Technical Context

**Language/Version**: Rust 2021 edition (1.75+)

**Primary Dependencies**:
- `xlseek-core` (path: `../core`, parallel traversal, Excel/Word parsing)
- `serde` (version: `1.0`, derive)
- `serde_json` (version: `1.0`)
- `std::io` (stdio communication)
- `same-file` (version: `1.0.6`)
- `walkdir` (version: `2.5`)
- `regex` (version: `1.10`)
- `calamine` (version: `0.36`)

**Storage**: Local filesystem (read-only file inspection, traversal, and cell extraction)

**Testing**: `cargo test --package xlseek-mcp`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`

**Target Platform**: Windows, macOS, Linux (Desktop environments integrating with Claude Desktop, Cursor, Antigravity, Cline)

**Project Type**: Dedicated MCP server binary crate (`xlseek-mcp`) within the existing Cargo workspace, sharing `xlseek-core` with `xlseek-cli` and `xlseek` (GUI)

**Performance Goals**:
- Query response time < 5 seconds across 1,000 Excel cells.
- Preview generation < 100ms for sheets up to 100 rows.
- Zero extraneous allocations on idle stdio listening loop.

**Constraints**:
- Strict adherence to the project constitution:
  - Principle I: Documentation, comments, and protocol messages in accurate English.
  - Principle II: No hardcoded constants outside `constants.rs`.
  - Principle III: Comprehensive 3-part header comments on all functions and modules.
  - Principle IV: Clean modular architecture conforming to `clippy` and `rustfmt`.
  - Principle V: Robust error handling, non-panicking fallbacks, and 100% test coverage for negative paths.
- Pristine `stdout` stream: all logging, debug statements, and warnings routed strictly to `stderr`.

**Scale/Scope**:
- Workspace manifest update (`Cargo.toml`).
- New crate `crates/mcp` (`Cargo.toml`, `src/lib.rs`, `src/main.rs`, `src/constants.rs`, `src/protocol/`, `src/tools/`, `src/security.rs`).
- Unit and integration tests covering protocol handshake, tool execution, truncation, and directory sandboxing.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS — All specification artifacts (`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`), source code comments, tool descriptions, and error messages are written in accurate English. User chat responses are provided in Japanese as requested.
- **Principle II (No Hardcoded Constants)**: PASS — All protocol version strings (`"2024-11-05"`), JSON-RPC error codes (`-32601`, `-32602`), tool names (`"xlseek_search"`, `"xlseek_list_sheets"`, `"xlseek_preview_sheet"`), default parameters (`100`), and CLI option strings are centralized in `crates/mcp/src/constants.rs` and referenced with `// Constant reference:` comments.
- **Principle III (Comprehensive Header Comments)**: PASS — Every module, struct, and function in `crates/mcp` will include the mandatory 3-part comment block (Description, Arguments/Returns, Errors/Exceptions) without per-file change histories.
- **Principle IV (Modular Design & Code Standards)**: PASS — Protocol parsing (`protocol/`), tool execution (`tools/`), security sandboxing (`security.rs`), and constants (`constants.rs`) are strictly decoupled. Complies with `clippy` with zero warnings.
- **Principle V (Robust Error Handling & Testing)**: PASS — Uses `Result<T, E>` throughout, wraps tool errors in `ToolCallResult` with `isError: true` rather than terminating the process, and includes integration tests simulating bad JSON, missing files, and whitelist violations.

---

## Project Structure

### Documentation (this feature)

```text
specs/027-mcp-server/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   └── mcp-tools.md     # Protocol and tool schema definitions
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

```text
Cargo.toml               # Workspace manifest (add "crates/mcp")
crates/
├── core/                # Shared search engine & data models (xlseek-core)
├── cli/                 # Standalone CLI binary (xlseek-cli)
└── mcp/                 # Dedicated MCP server binary crate (xlseek-mcp)
    ├── Cargo.toml
    ├── README.md
    ├── src/
    │   ├── main.rs      # Executable entry point (CLI argument parsing)
    │   ├── lib.rs       # Stdio loop & dispatcher orchestration
    │   ├── constants.rs # Centralized constants & reference tests
    │   ├── security.rs  # Directory whitelist & path sandboxing logic
    │   ├── protocol/    # JSON-RPC 2.0 & MCP message serialization
    │   │   ├── mod.rs
    │   │   ├── messages.rs
    │   │   └── error.rs
    │   └── tools/       # Tool execution handlers
    │       ├── mod.rs
    │       ├── search.rs
    │       ├── list_sheets.rs
    │       └── preview.rs
    └── tests/
        ├── protocol_test.rs   # Handshake, tool list, invalid method tests
        ├── tools_test.rs      # Search, sheet list, preview extraction tests
        └── sandboxing_test.rs # Directory whitelist restriction tests
```

**Structure Decision**: Option C (Dedicated crate `crates/mcp` within workspace) was selected to provide clean binary separation, avoid mixing CLI argument logic with long-running stdio event loops, and allow independent deployment of `xlseek-mcp`.

---

## Complexity Tracking

> *No constitutional violations. Table left blank.*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|:---|:---|:---|
| None | N/A | N/A |

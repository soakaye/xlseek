# Phase 0: Research & Architecture Decisions

**Feature**: MCP Server Implementation Based on `xlseek-cli`  
**Feature Branch**: `027-mcp-server`  
**Date**: 2026-10-19  

## Research Questions & Decisions

### 1. MCP Protocol Implementation Strategy (SDK vs Lightweight stdio Dispatcher)

- **Context**: The Model Context Protocol (MCP) requires a JSON-RPC 2.0 compliant server running over standard input/output (stdio).
- **Decision**: Implement a purpose-built, lightweight, zero-panic JSON-RPC 2.0 dispatcher using `serde`, `serde_json`, and `std::io` within the new `crates/mcp` crate, rather than pulling in external third-party MCP frameworks (such as `rmcp`).
- **Rationale**:
  - `serde` and `serde_json` are already workspace dependencies with zero additional footprint.
  - The MCP specification over stdio consists of a clean set of JSON-RPC 2.0 messages: `initialize`, `notifications/initialized`, `ping`, `tools/list`, and `tools/call`.
  - External MCP crates in the Rust ecosystem are fast-moving, often introduce heavyweight async runtimes (Tokio), and complicate strict compliance with the project constitution (Principle II: externalized constants, Principle III: comprehensive 3-part header comments, Principle V: zero-panic error handling).
  - A lightweight synchronous stdio read-eval-write loop with graceful deserialization matches the performance, deterministic execution, and simplicity of `xlseek-cli`.
- **Alternatives Considered**:
  - *Third-party `rmcp` crate*: Adds heavy asynchronous dependencies, rapid API churn, and less control over protocol edge cases and constant extraction.
  - *Extending `xlseek-cli` binary*: Mixing CLI argument parsing and long-running stdio MCP event loops in one crate increases binary complexity; Option C (separate `crates/mcp` crate with binary `xlseek-mcp`) was selected for clean separation of concerns.

---

### 2. Standard Output Discipline and Logging

- **Context**: In stdio MCP servers, any stray text or progress output written to `stdout` breaks JSON-RPC message framing and causes MCP clients (Claude Desktop, Cursor) to fail immediately.
- **Decision**: Strictly restrict `stdout` to valid JSON-RPC response lines delimited by newline (`\n`). Route all diagnostics, initialization messages, search warnings, and errors exclusively to `stderr`.
- **Rationale**: Keeps the protocol communication channel pristine while preserving visibility of errors and debug logs in client host log files.
- **Alternatives Considered**:
  - *Content-Length framing*: MCP stdio standard uses newline-delimited JSON-RPC messages; Content-Length headers are optional and less commonly expected by desktop MCP clients. Newline-delimited JSON is cleaner and less error-prone.

---

### 3. Integration with `xlseek-core` for Search & Extraction

- **Context**: The server needs to perform parallel cross-document searches across Excel and Word files.
- **Decision**: Directly reuse `xlseek_core::search::engine::search_directory` and `xlseek_core::search::path` normalization. Convert tool invocation arguments into `xlseek_core::models::SearchQuery` and map results into structured `SearchMatch` summaries.
- **Rationale**: Avoids duplicating file discovery (`walkdir`, `ignore`), Excel workbook parsing (`calamine`), shape text extraction, Word document extraction, or ignore pattern matching (`.gitignore` syntax).
- **Alternatives Considered**:
  - *Calling `xlseek-cli` as a subprocess*: Inefficient, adds OS process creation overhead per search, and prevents streaming results or fine-grained error control.

---

### 4. Excel Preview Formatting (Markdown Table vs Raw JSON)

- **Context**: AI models need worksheet context to understand tabular relationships around search hits.
- **Decision**: Format `xlseek_preview_sheet` cell grids as clean, GitHub-flavored Markdown tables with column letter headers (e.g., `| A | B | C |`) and row number indices, capped at `max_rows` (default: 100, configurable via `start_row` and `max_rows`).
- **Rationale**: Markdown tables consume far fewer tokens than deep JSON object structures while allowing LLMs to intuitively grasp row/column alignment, headers, and cell relationships.
- **Alternatives Considered**:
  - *2D JSON Array*: Requires the model to compute column index mappings mentally.
  - *CSV text*: Less readable for models when cells contain commas or multiline strings.

---

### 5. Path Access Security & Sandboxing (Whitelist Mode)

- **Context**: MCP servers can be invoked with arbitrary local filesystem paths. A mechanism is needed to enforce directory boundaries when desired.
- **Decision**: Support optional whitelist root paths configured via `--allowed-dir <PATH>` CLI arguments or the `XLSEEK_ALLOWED_DIRS` environment variable. By default (when no whitelist is supplied), permit access to any path accessible by the current OS user. When a whitelist is active, verify that all resolved canonical target paths start with an allowed directory prefix.
- **Rationale**: Gives regular desktop users frictionless access out of the box, while granting enterprise or security-conscious users the ability to lock down tool execution to approved project folders.
- **Alternatives Considered**:
  - *Mandatory whitelist*: Causes friction for everyday desktop use where users search arbitrary folders.
  - *No whitelist support*: Leaves no path validation mechanism for sandboxed environments.

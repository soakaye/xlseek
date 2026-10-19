# Feature Specification: MCP Server Implementation Based on xlseek-cli

**Feature Branch**: `027-mcp-server`

**Created**: 2026-10-19

**Status**: Ready for Planning

**Input**: User description: "MCP サーバー化を行う。既存の xlseek-cli を土台にして作成する。"

## Clarifications

### Session 2026-10-19

- Q: MCP サーバーの起動方法およびバイナリ構成はどうあるべきか？ → A: Option C (Create an independent crate `crates/mcp` within the workspace and provide a dedicated binary `xlseek-mcp`. Reuse the scanning and search foundations from `xlseek-core` similarly to `xlseek-cli`, isolating MCP JSON-RPC protocol handling and dependencies into an independent module).
- Q: MCP サーバーが提供する検索ツールは、Excel と Word を統合した単一の検索ツールとすべきですか、それともドキュメント種別ごとにツールを分離すべきですか？また他サーバーとの競合を避ける命名はどうあるべきですか？ → A: Option B (To prevent tool name collisions with other MCP servers like the standard filesystem server and maximize discoverability, adopt the `xlseek_` prefix for the unified search tool `xlseek_search`, as well as `xlseek_list_sheets` and `xlseek_preview_sheet`. Support flexible filtering of extensions and targets via arguments).
- Q: プレビューツール (`xlseek_preview_sheet`) は、シートのセルデータをどのようなフォーマットおよび行数制限で AI クライアントに返却すべきですか？ → A: Option A (Return cell data primarily as a Markdown table formatted string that LLMs can naturally interpret, with a safe default limit of the first 100 rows. Allow clients to control range using `start_row` and `max_rows` arguments).
- Q: MCP サーバーがアクセス可能な検索・閲覧対象のファイルパスについて、セキュリティ上のアクセス制限（許可ディレクトリの制限）をどのように設けるべきですか？ → A: Option A (Allow arbitrary paths accessible by the OS execution user by default for desktop usability, while offering an optional whitelist restriction via command-line arguments or environment variables to restrict operations to permitted root directories).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fast Cross-Document Search via MCP (Priority: P1)

As a user leveraging AI assistants (e.g., Claude Desktop, Cursor, Antigravity, or other MCP clients),
I want the AI assistant to perform high-speed cross-document searches across Excel (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`) and Word (`.docx`, `.doc`) files in local directories using keywords or regular expressions via the Model Context Protocol,
So that the AI assistant can accurately retrieve cell values, formulas, comments, and drawing shape text to answer questions and analyze local documents.

**Why this priority**:
This is the core value proposition of the feature. Enabling AI assistants to search and inspect local documents directly without manual file opening or copy-pasting is the primary objective.

**Independent Test**:
Call the `xlseek_search` tool from an MCP client or test JSON-RPC client over stdio with a target directory and query. Verify that matching cell coordinates, sheet names, file paths, values, and match targets are returned as structured data.

**Acceptance Scenarios**:

1. **Given** a directory containing multiple Excel and Word documents,
   **When** an MCP client calls `xlseek_search` with a query string (e.g., `"Financial Report"`),
   **Then** a structured list of matches containing file paths, sheet names, cell addresses (e.g., `B12`), cell values, matched substrings, and target types (Value, Formula, Comment, Shape, WordText) is returned.
2. **Given** optional parameters specified (e.g., regex matching, case sensitivity, target type filters such as values-only or formulas-only, or file extension filters),
   **When** `xlseek_search` executes,
   **Then** only matches satisfying all specified criteria are returned.
3. **Given** a search query with no matching files or cells,
   **When** `xlseek_search` executes,
   **Then** an empty match list is returned with a count of 0 without raising an error.
4. **Given** a search that yields a large number of hits,
   **When** `xlseek_search` executes,
   **Then** results are truncated up to the specified `max_results` (default: 100) with a flag indicating whether truncation occurred.

---

### User Story 2 - Worksheet Listing and Markdown Grid Cell Preview (Priority: P2)

As a user interacting with an AI assistant,
I want the AI assistant to inspect the available worksheets in an Excel workbook and retrieve a preview grid of cell contents for a specific sheet in Markdown table format,
So that the AI assistant can understand the overall layout, column headers, and tabular context surrounding matched cells.

**Why this priority**:
Inspecting table structure (header rows, total rows, adjacent columns) dramatically improves the accuracy of AI answers and summaries.

**Independent Test**:
Call `xlseek_list_sheets` on a multi-sheet workbook to verify all sheet names are listed. Then call `xlseek_preview_sheet` on a sheet and verify that a Markdown table representing the cells is returned within the requested row bounds.

**Acceptance Scenarios**:

1. **Given** an Excel workbook containing multiple worksheets,
   **When** an MCP client calls `xlseek_list_sheets` with the workbook file path,
   **Then** an array of all worksheet names in the workbook is returned.
2. **Given** a valid Excel workbook and sheet name,
   **When** an MCP client calls `xlseek_preview_sheet` with pagination parameters (`start_row` and `max_rows`, default: 100),
   **Then** a Markdown table string representing cell coordinates and formatted text values is returned along with total row and column dimensions.

---

### User Story 3 - Robust and Secure Stdio MCP Connection with Path Sandboxing (Priority: P3)

As a system administrator or developer,
I want the MCP server to communicate reliably over standard input/output (stdio) according to the MCP specification, keep standard output strictly reserved for JSON-RPC messages, and gracefully handle invalid paths, corrupt files, and security restrictions without crashing.

**Why this priority**:
A crash in an MCP server terminates the entire AI tool session. Robust error handling, clean stdio channel discipline, and directory boundary enforcement are critical for stability and security.

**Independent Test**:
Send invalid JSON-RPC requests, query non-existent directories, attempt to access paths outside a configured whitelist, and query corrupt workbooks. Verify that the server returns well-formed error responses or tool error results without crashing, and that no diagnostic logs leak into stdout.

**Acceptance Scenarios**:

1. **Given** an MCP client initializing a connection,
   **When** the client sends a `tools/list` request,
   **Then** the server responds with definitions and JSON schemas for `xlseek_search`, `xlseek_list_sheets`, and `xlseek_preview_sheet`.
2. **Given** a request specifying a non-existent path or a corrupt file,
   **When** the tool executes,
   **Then** the server returns an informative error result rather than terminating the process.
3. **Given** active server execution,
   **When** diagnostic logging or warnings are generated,
   **Then** all log messages are written exclusively to `stderr`, leaving `stdout` uncorrupted for JSON-RPC framing.
4. **Given** a server started with an allowed directory whitelist,
   **When** a client requests a path outside the whitelist,
   **Then** the request is safely rejected with a clear access denial error.

---

### Edge Cases

- **Corrupted or Password-Protected Files**: The server logs a partial warning and skips the unreadable file while continuing to search remaining files.
- **Empty Directories or No Matching Files**: The server returns an empty result set (0 matches) with status success.
- **Malformed Regular Expressions**: Returns an explicit regex syntax error message in the tool response without process termination.
- **Massive Hit Counts**: Truncates results to the configured `max_results` limit (default 100) to prevent memory exhaustion and context window overflow.
- **Office Lock Files and Gitignore**: Temporary office lock files (e.g., `~$*`) and patterns matching `.gitignore` are excluded by default during directory traversal.
- **Path Whitelist Violations**: When directory restriction is active, any attempt to resolve paths outside the whitelist is rejected before filesystem access.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a Model Context Protocol (MCP) compliant server communicating over standard I/O (stdio) using JSON-RPC 2.0 framing.
- **FR-002**: System MUST return tool definitions, human-readable descriptions, and complete JSON Schema parameter definitions for all available tools in response to `tools/list`.
- **FR-003**: System MUST provide a unified search tool named `xlseek_search` capable of scanning Excel (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`) and Word (`.docx`, `.doc`) documents within specified directory or file paths.
- **FR-004**: `xlseek_search` MUST accept the following parameters:
  - `path` or `paths`: Target directory or file path(s) to search.
  - `query`: Search keyword or regular expression string.
  - `is_regex` (boolean): Whether `query` is evaluated as a regular expression (default: false).
  - `match_case` (boolean): Whether matching is case-sensitive (default: false).
  - `targets` (array of strings): Search target domains (e.g., `values`, `formulas`, `comments`, `shapes`).
  - `extensions` (array of strings): Target file extensions to include (default: all supported formats).
  - `ignore_patterns` (array or comma-separated string): Exclusion patterns in `.gitignore` syntax.
  - `max_results` (integer): Maximum number of match items to return (default: 100).
- **FR-005**: `xlseek_search` results MUST be returned as structured data containing file path, sheet name, cell coordinates (e.g., `B12`), cell text/value, matched substring, and match target category for each hit.
- **FR-006**: System MUST provide a worksheet listing tool named `xlseek_list_sheets` that returns all sheet names within a specified workbook.
- **FR-007**: System MUST provide a cell grid preview tool named `xlseek_preview_sheet` that outputs tabular cell data primarily as a Markdown table string, accepting `start_row` and `max_rows` (default: 100) for row windowing.
- **FR-008**: System MUST dedicate `stdout` exclusively to JSON-RPC protocol messages and MUST NOT write diagnostics, progress indicators, or trace logs to `stdout` (all logging directed to `stderr`).
- **FR-009**: System MUST NOT panic or crash on missing files, permission errors, corrupted archives, or invalid arguments, returning descriptive error payloads to the client.
- **FR-010**: System MUST be implemented as an independent crate `crates/mcp` providing a dedicated binary executable named `xlseek-mcp`, reusing core scanning and extraction logic from `xlseek-core`.
- **FR-011**: System MUST permit access to any path accessible by the operating system user by default, while supporting an optional directory whitelist (via CLI option or environment variable) that strictly restricts file operations to designated root folders.

### Key Entities

- **SearchMatch**: Represents a single search hit. Contains file path, sheet name, row and column index (1-based), cell coordinate (A1 notation), cell content, matched snippet, and target category (Value, Formula, Comment, Shape, WordText).
- **ToolDefinition**: MCP tool specification exposed to clients. Contains tool name (`xlseek_search`, `xlseek_list_sheets`, `xlseek_preview_sheet`), description, and JSON Schema input properties.
- **SheetInfo**: Metadata for a single worksheet within an Excel workbook (index, name, visibility).
- **SheetPreview**: Structured preview payload containing Markdown table string, active row range (`start_row`, `row_count`), and total sheet dimensions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Clients (Claude Desktop, Cursor, Antigravity) can connect and register the server as a single native binary without external runtime dependencies (such as Node.js or Python).
- **SC-002**: A search across a directory tree containing 1,000 Excel cells completes and returns structured MCP responses within 5 seconds under standard desktop conditions.
- **SC-003**: In an automated test suite containing at least 10 negative test cases (corrupt files, password locks, missing paths, invalid regex, out-of-bound coordinates), the server maintains a 0% crash/panic rate and returns valid MCP error responses.
- **SC-004**: 100% of returned cell addresses and file paths in test fixtures accurately locate the exact matching cell coordinates when validated against raw file contents.

## Assumptions

- **Transport**: Standard I/O (stdio) is the primary transport mechanism. HTTP/SSE transports are out of scope for the initial release.
- **Security Boundary**: The server executes locally and does not perform network downloads or external web requests. File access is bounded by OS user privileges, with optional root directory whitelisting.
- **Core Engine Reuse**: Multithreaded directory traversal (`rayon`), Excel parsing (`calamine`), and text extraction rely directly on `xlseek-core`.
- **CLI Backward Compatibility**: The standalone `xlseek-cli` binary and its CLI arguments remain completely unchanged and independently functional.

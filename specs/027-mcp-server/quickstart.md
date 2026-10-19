# Quickstart & Verification Guide: `xlseek-mcp`

**Feature**: MCP Server Implementation Based on `xlseek-cli`  
**Feature Branch**: `027-mcp-server`  
**Date**: 2026-10-19  

## 1. Prerequisites & Build

### Prerequisites
- Rust 2021 edition toolchain (1.75+)
- Git

### Build the MCP Binary
Build the dedicated binary crate `xlseek-mcp`:

```bash
# Debug build
cargo build --package xlseek-mcp --bin xlseek-mcp

# Release build (recommended for performance)
cargo build --release --package xlseek-mcp --bin xlseek-mcp
```

The resulting executable will be located at `target/release/xlseek-mcp` (or `target/release/xlseek-mcp.exe` on Windows).

---

## 2. Client Configuration

### Claude Desktop Configuration
Add `xlseek-mcp` to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "xlseek": {
      "command": "F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/xlseek-mcp/target/release/xlseek-mcp.exe",
      "args": [
        "--allowed-dir", "C:/Users/username/Documents/ExcelReports"
      ]
    }
  }
}
```

*Note*: If `--allowed-dir` is omitted, the server operates in user-privileged mode without folder restrictions.

### Cursor / Cline Configuration
Configure the stdio command:
- **Command**: `xlseek-mcp` (or full path to binary)
- **Args**: `[]` (or `["--allowed-dir", "/path/to/workbooks"]`)
- **Transport**: `stdio`

---

## 3. Manual Verification Scenarios (via Command Line)

You can verify the MCP server directly by piping JSON-RPC lines into its `stdin`:

### Scenario 1: Protocol Handshake (`initialize`)

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0"}}}' | ./target/release/xlseek-mcp
```

**Expected Output (on stdout)**:
```json
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"xlseek-mcp","version":"1.0.2"}}}
```

---

### Scenario 2: Tool Discovery (`tools/list`)

```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/release/xlseek-mcp
```

**Expected Output**:
A JSON-RPC result containing tool definitions for `xlseek_search`, `xlseek_list_sheets`, and `xlseek_preview_sheet` with valid inputSchemas.

---

### Scenario 3: Cross-Workbook Search (`xlseek_search`)

```bash
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"xlseek_search","arguments":{"paths":"crates/core/tests/fixtures","query":"test"}}}' | ./target/release/xlseek-mcp
```

**Expected Output**:
A JSON-RPC result containing formatted match items with file names, sheet names, cell coordinates, and matched snippets.

---

### Scenario 4: Worksheet Listing (`xlseek_list_sheets`)

```bash
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"xlseek_list_sheets","arguments":{"path":"crates/core/tests/fixtures/test.xlsx"}}}' | ./target/release/xlseek-mcp
```

**Expected Output**:
A JSON-RPC result listing all sheet names in the workbook.

---

### Scenario 5: Markdown Sheet Preview (`xlseek_preview_sheet`)

```bash
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"xlseek_preview_sheet","arguments":{"path":"crates/core/tests/fixtures/test.xlsx","sheet_name":"Sheet1","max_rows":10}}}' | ./target/release/xlseek-mcp
```

**Expected Output**:
A JSON-RPC result with `content[0].text` containing a formatted Markdown table (`| Row | A | B | ... |`).

---

### Scenario 6: Sandbox Enforcement

Start the server restricting paths to `C:/Permitted`, then request a file under `D:/Restricted`:

```bash
echo '{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"xlseek_search","arguments":{"paths":"D:/Restricted","query":"secret"}}}' | ./target/release/xlseek-mcp --allowed-dir "C:/Permitted"
```

**Expected Output**:
A tool-level error (`"isError": true`) stating access is denied outside permitted directories. Process continues running without crashing.

---

## 4. Automated Testing Commands

Verify code quality and test coverage before committing:

```bash
# Run unit and integration tests across all crates
cargo test --workspace

# Run MCP crate tests specifically
cargo test --package xlseek-mcp

# Verify strict compiler warnings and Clippy
cargo clippy --workspace --all-targets -- -D warnings

# Verify official formatting
cargo fmt --check
```

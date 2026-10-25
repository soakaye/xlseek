# xlseek-mcp

Standalone Model Context Protocol (MCP) server for Excel Seek (`xlseek`).

## Overview

`xlseek-mcp` exposes the high-performance parallel Excel and Word search and inspection engine of `xlseek-core` to AI assistants (including Claude Desktop, Cursor, and Antigravity) over standard input/output (stdio) using the Model Context Protocol (JSON-RPC 2.0).

## Exposed Tools

1. **`xlseek_search`**: Cross-document search across Excel (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`) and Word (`.docx`, `.doc`) files. Searches cell values, formulas, comments, and drawing shape text with support for regular expressions, case sensitivity, target filters, and exclusion patterns.
2. **`xlseek_list_sheets`**: Lists all worksheet names within a specified workbook file.
3. **`xlseek_preview_sheet`**: Extracts a window of cells from a specified worksheet and formats them as a clean Markdown table with row and column headers.

## Installation & Downloads

### Standalone Pre-built Binaries (GitHub Releases)

Pre-built binary archives are available on [GitHub Releases](https://github.com/soakaye/xlseek/releases):

- **macOS (Apple Silicon)**: `xlseek-mcp-<tag>-aarch64-apple-darwin.tar.gz`
- **Linux (x86_64)**: `xlseek-mcp-<tag>-x86_64-unknown-linux-gnu.tar.gz`
- **Windows (x86_64)**: `xlseek-mcp-<tag>-x86_64-pc-windows-msvc.zip`

Extract the executable (`xlseek-mcp` or `xlseek-mcp.exe`) and place it in your PATH or tools directory. No Rust toolchain or desktop GUI application is required.

### Building from Source

```bash
cargo build --release -p xlseek-mcp
```
The compiled binary will be located at `target/release/xlseek-mcp` (or `target/release/xlseek-mcp.exe`).

## Configuration Example (Claude Desktop)

Add the server to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "xlseek": {
      "command": "path/to/xlseek-mcp",
      "args": [
        "--allowed-dir", "C:/Projects/Spreadsheets"
      ]
    }
  }
}
```

## Security & Whitelist Mode

By default, `xlseek-mcp` allows access to any file readable by the operating system user. You can restrict file access by supplying one or more `--allowed-dir <PATH>` arguments (or setting the `XLSEEK_ALLOWED_DIRS` environment variable with comma-separated paths). Any attempt to search or preview files outside permitted directories will be rejected with an access denial error.

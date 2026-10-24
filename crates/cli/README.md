<!-- Description: Overview and command line reference for xlseek-cli standalone tool. Arguments & Returns: Documents options, positional arguments, stdout streaming, export formats, ignore patterns, and exit statuses. Errors / Exceptions: Explains error validation, overwrite rules, and exit status codes 0, 1, and 2. -->
# xlseek-cli

Standalone command-line interface for **Excel Seek (`xlseek`)**. Enables fast batch search across Excel workbooks and document files without launching a graphical environment. Search results can be streamed as CSV, Markdown, or JSON to standard output (`stdout`) or exported directly to `.csv`, `.xlsx`, `.md`, or `.json` files.

---

## Building

```bash
# Build standalone executable for current platform
cargo build --release -p xlseek-cli --bin xlseek-cli

# Binary output:
# - Windows: target/release/xlseek-cli.exe
# - Linux/macOS: target/release/xlseek-cli
```

---

## Usage Syntax

```bash
xlseek-cli [OPTIONS] [QUERY] [PATH]...
```

- **Positional Arguments**:
  - `1st argument`: Search query string or regular expression.
  - `2nd+ arguments`: One or more target file or directory paths.
- **Flag Options**:
  - `-q, --query <TEXT>`: Search query string or regular expression.
  - `-p, --path <PATH>`: Search target directory or file (comma-separated or quoted paths supported).
  - `-o, --output <PATH>`: Destination output file (`.csv`, `.xlsx`, `.md`, `.json`). When omitted, results stream to `stdout` (`csv`, `md`, or `json`).
  - `-f, --format <csv|xlsx|md|json>`: Export format. Inferred from file extension if `-o` is provided; defaults to `csv` for `stdout` (`xlsx` cannot be streamed to `stdout`).
  - `-i, --ignore-patterns <LIST>`: Comma-separated glob patterns using `.gitignore` syntax (e.g. `~$*, *.tmp, backup/*`).
  - `-c, --match-case <BOOL>`: Case-sensitive matching (default: `false`).
  - `-r, --regex <BOOL>`: Treat query as regular expression (default: `false`).
  - `-w, --overwrite`: Allow overwriting an existing output destination file.
  - `-l, --language <ja|en>`: Output language for CSV/Excel headers and error messages (default: `ja`).
  - `-e, --extensions <LIST>`: Target file extensions (default: `.xlsx,.xlsm,.xlsb,.xls`).
  - `--directory-mode <sequential|burst>`: Directory traversal mode (default: `sequential`).
  - `--burst-workers <COUNT>`: Number of Burst worker threads (2..32 or automatic).
  - Scope options: `--values <BOOL>`, `--formulas <BOOL>`, `--comments <BOOL>`, `--shapes <BOOL>`, `--hidden-sheets <BOOL>`.
  - `-h, --help`: Display usage and options.

---

## Usage Examples

```bash
# Quick positional search piped to grep or head via stdout (CSV)
./target/release/xlseek-cli "Revenue" ./reports

# Stream structured JSON search results to stdout (matching xlseek-mcp format)
./target/release/xlseek-cli "Revenue" ./reports -f json

# Stream AI-ready Markdown table context to stdout
./target/release/xlseek-cli "Revenue" ./reports -f md

# Export search to formatted XLSX file
./target/release/xlseek-cli -p ./reports -q "Revenue" -o ./results.xlsx

# Export search to AI-ready Markdown file (format inferred from .md extension)
./target/release/xlseek-cli -p ./reports -q "Revenue" -o ./results.md

# Export search to structured JSON file (format inferred from .json extension)
./target/release/xlseek-cli -p ./reports -q "Revenue" -o ./results.json

# Search across multiple directories with ignore patterns and English column headers
./target/release/xlseek-cli -p "./reports, ./archive" -q "Revenue" \
  -i "~$*, *.tmp, backup/*" -f csv -o ./results.csv -l en

# Run Burst parallel traversal with 8 directory workers
./target/release/xlseek-cli "Revenue" ./large_archive --burst-workers 8
```

---

## Exit Status Codes

| Code | Meaning |
| :--- | :--- |
| `0` | **Success**: Search completed (includes 0 matches found). |
| `1` | **Partial Failure**: One or more files could not be read or parsed. |
| `2` | **Fatal Error**: Invalid arguments, input/output conflict, output file write error, or fatal root directory error. |

---

## Verification & Testing

```bash
# Run unit and integration tests
cargo test -p xlseek-cli

# Run Clippy checks
cargo clippy -p xlseek-cli --all-targets -- -D warnings
```

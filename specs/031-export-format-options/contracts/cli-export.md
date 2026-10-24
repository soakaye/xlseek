# CLI Export Contract: `xlseek-cli`

**Feature**: `031-export-format-options`  
**Binary**: `xlseek-cli`

## 1. Command-Line Options

- `-f, --format <csv|xlsx|md|json>`:
  - Explicitly specifies the output format.
  - When `-o, --output <PATH>` is provided:
    - If `-f` is omitted, the format is inferred from the output file extension (`.csv` → `Csv`, `.xlsx` → `Xlsx`, `.md` → `Md`, `.json` → `Json`, case-insensitive).
    - If `-f` is specified, the output file extension MUST match the specified format (`csv` ↔ `.csv`, `xlsx` ↔ `.xlsx`, `md` ↔ `.md`, `json` ↔ `.json`); mismatches return `ERR_CLI_FORMAT`.
  - When `-o, --output` is omitted (`stdout` streaming mode):
    - Defaults to `csv` when `-f` is omitted.
    - Supports `-f csv`, `-f md`, and `-f json` written directly to standard output (`stdout`).
    - Rejects `-f xlsx` on `stdout` with `ERR_CLI_FORMAT` (binary workbook cannot be streamed to text `stdout`).

## 2. File Staging & Atomic Publication (`OutputGuard::publish`)

All four formats (`Csv`, `Xlsx`, `Md`, `Json`) write to a staged file inside a temporary directory in the destination's parent directory and atomically publish to the destination path after verifying no concurrent conflict occurred.

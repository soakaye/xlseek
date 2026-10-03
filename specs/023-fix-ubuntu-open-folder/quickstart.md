# Quickstart Validation Guide: Fix Open Folder Functionality on Ubuntu

**Feature**: `023-fix-ubuntu-open-folder`
**Date**: 2026-10-02

## 1. Prerequisites

- Linux desktop environment (e.g. Ubuntu with GNOME / Nautilus, or Linux CLI with `gio` / `dbus-send`).
- Rust toolchain and Node.js installed.

## 2. Automated Quality Gates

Run the required quality gates from repository root:

```bash
# 1. Frontend typecheck and build
npm run build

# 2. Rust unit and integration tests
cargo test --workspace

# 3. Rust Clippy verification (zero warnings allowed)
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust code formatting check
cargo fmt --check
```

## 3. Targeted Unit & Integration Testing

Verify unit tests for the Linux folder opening logic:

```bash
# Run unit tests in src-tauri
cargo test -p xlseek --lib commands::system_cmd::tests
```

Key test cases:
1. **URI Encoding**: Test that paths with spaces and non-ASCII characters (e.g. `/tmp/test folder/日本語.xlsx`) are correctly converted to standard `file://` URIs.
2. **Missing Path Handling**: Test that non-existent paths return `ErrorCode::PathNotFound`.
3. **Execution Pipeline**: Test that Linux opening logic handles DBus, gio, and xdg-open execution cleanly without panicking.

## 4. Manual Verification on Linux Desktop

1. Launch application in dev mode:
   ```bash
   npm run tauri dev
   ```
2. In the search box, search for a known Excel file (e.g. `test.xlsx`).
3. Click on a search result to display the preview panel on the right.
4. Click the **Folder** ("Open in Folder") icon button in the preview header.
5. **Expected Outcome**:
   - The file manager (e.g., Nautilus) opens displaying the directory containing the file.
   - If Nautilus or a compatible file manager is running with DBus support, `test.xlsx` is highlighted.
   - No error toast appears.

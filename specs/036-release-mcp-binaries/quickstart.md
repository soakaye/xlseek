# Quickstart: Standalone Multi-Platform xlseek-mcp Binary Release Validation

**Feature**: `036-release-mcp-binaries`
**Date**: 2026-10-25

## Purpose

This guide outlines runnable local and CI validation steps to verify that `xlseek-mcp` compiles as a standalone executable, packages correctly into platform archives, and runs in headless MCP mode without desktop GUI dependencies.

---

## 1. Local Build & Packaging Verification

### Step 1: Compile Standalone xlseek-mcp in Release Mode

```bash
cargo build --release -p xlseek-mcp
```

**Expected Outcome**:
- Build completes with exit code 0.
- Binary produced at `target/release/xlseek-mcp` (or `target/release/xlseek-mcp.exe` on Windows).

### Step 2: Validate Headless Execution

Test the binary with the `--help` flag to verify it starts and displays its usage information:

```bash
./target/release/xlseek-mcp --help
```

**Expected Outcome**:
```text
Model Context Protocol (MCP) server for xlseek Excel search

Usage: xlseek-mcp [OPTIONS]
```

### Step 3: Verify Archive Packaging Simulation

On macOS / Linux:
```bash
DUMMY_TAG="v1.0.0"
TARGET_TRIPLE="aarch64-apple-darwin"
mkdir -p "target/test-dist"
cp "target/release/xlseek-mcp" "target/test-dist/"
chmod +x "target/test-dist/xlseek-mcp"
tar -czvf "xlseek-mcp-${DUMMY_TAG}-${TARGET_TRIPLE}.tar.gz" -C "target/test-dist" xlseek-mcp
tar -tzvf "xlseek-mcp-${DUMMY_TAG}-${TARGET_TRIPLE}.tar.gz"
```

**Expected Outcome**:
- Output tarball contains `xlseek-mcp` with executable permissions (`-rwxr-xr-x`).
- Extracted binary runs cleanly.

---

## 2. GitHub Actions Workflow Syntax Verification

Check that `.github/workflows/publish-to-auto-release.yml` passes YAML syntax validation and GitHub Actions schema checks.

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/publish-to-auto-release.yml'))"
```

**Expected Outcome**:
- Python `yaml.safe_load` exits with code 0 (valid YAML structure).

---

## 3. End-to-End Release Test Scenario

1. Push a test tag to a fork or CI test repository:
   ```bash
   git tag v9.9.9-test
   git push origin v9.9.9-test
   ```
2. Verify GitHub Actions dashboard for workflow `publish`:
   - Both `publish-mcp-binaries` and `publish-tauri` jobs appear.
   - Matrix runners execute for `macos-latest`, `ubuntu-24.04`, and `windows-latest`.
   - Each runner compiles `xlseek-mcp` and uploads the corresponding `.tar.gz` / `.zip` archive.
3. Verify GitHub Releases page for tag `v9.9.9-test`:
   - Contains:
     - `xlseek-mcp-v9.9.9-test-aarch64-apple-darwin.tar.gz`
     - `xlseek-mcp-v9.9.9-test-x86_64-unknown-linux-gnu.tar.gz`
     - `xlseek-mcp-v9.9.9-test-x86_64-pc-windows-msvc.zip`

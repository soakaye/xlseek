# Workflow Contract: publish-to-auto-release.yml

**Feature**: `036-release-mcp-binaries`
**Date**: 2026-10-25

## 1. Trigger Specification

```yaml
on:
  push:
    tags:
      - 'v*'
```

- **Trigger Event**: Push of a version tag matching glob `v*` (e.g. `v1.0.0`, `v2.1.3`).
- **Context Variables**:
  - `github.ref_name`: Version tag string (e.g., `v1.0.0`).
  - `secrets.GITHUB_TOKEN`: GitHub workflow token with `contents: write`.

---

## 2. Job Contracts

### Job 1: `publish-mcp-binaries`

- **Purpose**: Builds the headless `xlseek-mcp` executable across operating systems and attaches individual `.tar.gz` and `.zip` archives to the GitHub Release.
- **Dependencies**: None (`publish-mcp-binaries` runs independently of `publish-tauri`).
- **Runner Strategy**:
  - `fail-fast: false`
  - Matrix entries:
    - Platform: `macos-latest`, Target: `aarch64-apple-darwin`, Binary: `xlseek-mcp`, Extension: `tar.gz`
    - Platform: `ubuntu-24.04`, Target: `x86_64-unknown-linux-gnu`, Binary: `xlseek-mcp`, Extension: `tar.gz`
    - Platform: `windows-latest`, Target: `x86_64-pc-windows-msvc`, Binary: `xlseek-mcp.exe`, Extension: `zip`

#### Execution Steps Contract:

1. **Checkout**: `actions/checkout@v7`
2. **Setup Rust Toolchain**: `dtolnay/rust-toolchain@stable` with `targets: ${{ matrix.target }}`
3. **Build Binary**:
   - Command: `cargo build --release -p xlseek-mcp --target ${{ matrix.target }}`
   - Expected Output File: `target/${{ matrix.target }}/release/${{ matrix.binary_name }}`
4. **Archive Packaging**:
   - Unix (`matrix.archive_ext == 'tar.gz'`):
     - Ensure execution bit: `chmod +x target/${{ matrix.target }}/release/xlseek-mcp`
     - Package command: `tar -czvf xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.tar.gz -C target/${{ matrix.target }}/release xlseek-mcp`
   - Windows (`matrix.archive_ext == 'zip'`):
     - Package command: `Compress-Archive -Path target/${{ matrix.target }}/release/xlseek-mcp.exe -DestinationPath xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.zip -Force`
5. **Upload to Release**:
   - Action: `softprops/action-gh-release@v2`
   - Parameters:
     - `files`: `xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.${{ matrix.archive_ext }}`
     - `tag_name`: `${{ github.ref_name }}`
     - `draft`: `true`
   - Environment: `GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}`

---

### Job 2: `publish-tauri`

- **Purpose**: Builds the desktop GUI application and installer bundles (`.dmg`, `.msi`, `.deb`).
- **Permissions**: `contents: write`
- **Output**: Desktop installer assets attached to the draft release.

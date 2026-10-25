# Research: Standalone Multi-Platform xlseek-mcp Binary Release

**Feature**: `036-release-mcp-binaries`
**Date**: 2026-10-25

## Research Topics & Findings

### 1. Release Asset Upload Mechanism in GitHub Actions

**Question**: How should `publish-mcp-binaries` upload assets to the GitHub Release reliably and concurrently with `publish-tauri`?

- **Option A**: GitHub CLI (`gh release upload` with fallback create)
  - Using native GitHub CLI `gh` (pre-installed on all GitHub Actions runners):
    ```bash
    gh release view "$TAG" || gh release create "$TAG" --draft --title "Excel Seek $TAG"
    gh release upload "$TAG" "$ARCHIVE_PATH" --clobber
    ```
  - Pros: Native to GitHub Actions runners, zero third-party action dependencies, direct control over `--clobber` behavior.
  - Cons: Requires handling initial release creation check if `publish-tauri` has not run yet.

- **Option B**: `softprops/action-gh-release@v2`
  - Declarative action widely used in Rust/Go release workflows:
    ```yaml
    - uses: softprops/action-gh-release@v2
      with:
        files: xlseek-mcp-*
        draft: true
        name: 'Excel Seek ${{ github.ref_name }}'
        tag_name: ${{ github.ref_name }}
    ```
  - Pros: Automatically creates the release draft if it does not exist, or idempotently updates and appends assets if it already exists; handles retries and file globs seamlessly across all runners.
  - Cons: Third-party action dependency.

- **Decision**: Use `softprops/action-gh-release@v2`. It handles concurrent uploads across runners, draft releases, and idempotently attaches assets to the release matching `${{ github.ref_name }}` without race conditions.

---

### 2. Multi-Platform Matrix Configuration

**Question**: What matrix runner and target specifications should be used for building `xlseek-mcp`?

- **Runner Specifications**:
  - macOS Apple Silicon: `runs-on: macos-latest`, target `aarch64-apple-darwin`
  - Linux x86_64: `runs-on: ubuntu-24.04`, target `x86_64-unknown-linux-gnu`
  - Windows x86_64: `runs-on: windows-latest`, target `x86_64-pc-windows-msvc`
- **Toolchain Setup**:
  - Use `dtolnay/rust-toolchain@stable` with `targets: ${{ matrix.target }}`.
  - For headless `xlseek-mcp`, Linux does NOT require `libwebkit2gtk-4.1-dev` or GUI libraries because `crates/mcp` has no GUI/Tauri dependencies. Only standard compilation tools are required.
- **Decision**: Configure a 3-entry matrix in `publish-mcp-binaries` with `fail-fast: false`.

---

### 3. Archive Packaging per Platform

**Question**: How should archives be created across Unix and Windows runners?

- **macOS / Linux**:
  - Standard `tar`:
    ```bash
    chmod +x target/${{ matrix.target }}/release/xlseek-mcp
    tar -czvf "xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.tar.gz" -C "target/${{ matrix.target }}/release" xlseek-mcp
    ```
  - Ensures Unix executable permissions (`+x`) are preserved.
- **Windows**:
  - PowerShell `Compress-Archive`:
    ```powershell
    Compress-Archive -Path "target/${{ matrix.target }}/release/xlseek-mcp.exe" -DestinationPath "xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.zip" -Force
    ```
  - Preserves `.exe` executable cleanly for Windows extraction.
- **Decision**: Use `tar` on macOS/Linux and `Compress-Archive` (or platform shell condition) on Windows.

---

### 4. Workflow Permissions and Triggers

**Question**: What permissions and triggers are needed?

- Trigger: Existing `push: tags: ['v*']`.
- Job permissions: `contents: write` (required for release asset uploads).
- Secrets: `GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}`.

## Summary of Decisions

1. Define a dedicated GitHub Actions job `publish-mcp-binaries` in `.github/workflows/publish-to-auto-release.yml`.
2. Target matrix:
   - `macos-latest` / `aarch64-apple-darwin` / `.tar.gz`
   - `ubuntu-24.04` / `x86_64-unknown-linux-gnu` / `.tar.gz`
   - `windows-latest` / `x86_64-pc-windows-msvc` / `.zip`
3. Archive packaging preserves binary name (`xlseek-mcp` or `xlseek-mcp.exe`) inside the archive while the archive file name incorporates the release tag and target triple.
4. Upload via `softprops/action-gh-release@v2` with `draft: true` and `tag_name: ${{ github.ref_name }}`.

# Quickstart & Validation Guide: Search Path Normalization and Subpath Deduplication (`026-dedup-search-paths`)

## Prerequisites

- Rust stable toolchain (`cargo`, `rustfmt`, `clippy`)
- Node.js & npm (`npm run build`)

## Verification Commands

### 1. Run Core Unit Tests for Path Normalization & Subpath Pruning
```powershell
cargo test -p xlseek-core search::path::tests
```
**Expected Outcome**:
- Parent and child paths (e.g., `/data, /data/sub` and `/data/sub, /data`) resolve to `[/data]`.
- Relative segments (`.`, `..`), trailing slashes, and `~` home shorthands normalize to the same canonical path and deduplicate.
- Sibling directories with shared prefixes (e.g., `/data/report` and `/data/report_archive`) are both preserved.
- Invalid subpaths (e.g., `/data, /data/missing`) are reported in `invalid_paths` while `/data` is retained in `valid_roots`.
- Normalized roots do not contain Windows verbatim `\\?\` prefixes.

### 2. Run Search Engine Integration Tests (Single Scan of Nested Paths)
```powershell
cargo test -p xlseek-core search::engine::tests
```
**Expected Outcome**:
- Searching with `"<fixtures>, <fixtures>/subdir"` scans each eligible file exactly once (`scanned_files` and `matches_found` are identical to searching `<fixtures>` alone).

### 3. Run CLI Argument & Search Deduplication Tests
```powershell
cargo test -p xlseek-cli
```
**Expected Outcome**:
- Positional and `--path` arguments specifying overlapping parent/child directories or a directory and a file inside it prune the descendant path before discovery and emit zero duplicate matches.

### 4. Full Quality Gate Suite
```powershell
npm run build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```
**Expected Outcome**: All four quality gates pass with exit code 0 and zero warnings.

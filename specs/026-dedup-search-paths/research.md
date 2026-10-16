# Phase 0 Research: Search Path Normalization and Subpath Deduplication (`026-dedup-search-paths`)

## 1. Path Normalization & Subpath Relationship Detection Strategy

- **Decision**: Use filesystem canonicalization (`std::fs::canonicalize`) combined with clean display normalization (`normalize_export_path`) and component-wise ancestor pruning (`Path::starts_with` on canonicalized `PathBuf` instances), with lexical component normalization (`Component::CurDir` / `Component::ParentDir` resolution) as a fallback when `canonicalize` fails on readable virtual/network paths.
- **Rationale**:
  - `std::fs::canonicalize` resolves relative paths, `.` and `..` segments, redundant path separators, symbolic links/junctions, and OS-specific case differences (on Windows and macOS) to the exact physical path.
  - Rust's `Path::starts_with` compares `Path` components rather than raw strings, guaranteeing that `/path/to/folder_2` is never treated as a subpath of `/path/to/folder` (satisfying FR-004).
  - Stripping Windows verbatim prefixes (`\\?\` and `\\?\UNC\`) after canonicalization ensures that paths emitted in GUI search results, previews, and CLI exports remain clean, standard human-readable absolute paths (satisfying FR-009) while preserving exact component-level subpath matching.
- **Alternatives Considered**:
  - *Raw string prefix matching*: Rejected because `/dir/abc` would falsely match `/dir/abcdef` and fail on mixed slashes or uncanonicalized symlinks.
  - *Inode/file-handle deduplication only during file delivery (`same_file::Handle`)*: Rejected because it still traverses nested directory trees multiple times, wasting I/O in `discover_files_with_ignore` and inflating directory walk work.

## 2. Unifying GUI (`xlseek-core` / `src-tauri`) and CLI (`xlseek-cli`) Path Resolution

- **Decision**: Provide shared normalization and subpath-pruning functions in `crates/core/src/search/path.rs` (`normalize_canonical_path`, `deduplicate_and_prune_subpaths`, and `resolve_and_partition_paths`) and invoke them across:
  1. `crates/core/src/search/engine.rs` (resolving `~` via OS home directory environment variables when `home_dir` is not explicitly passed, matching `src-tauri`).
  2. `crates/cli/src/args.rs` (supporting both comma-separated path tokens and multiple positional arguments, and pruning redundant duplicate/subpath roots across directories and files).
- **Rationale**:
  - Previously, `crates/core/src/search/engine.rs` called `resolve_and_partition_paths(&parsed_paths, None)` whereas `src-tauri/src/commands/search_cmd.rs` passed `app.path().home_dir().ok()`. Providing a unified fallback to the OS home directory (`HOME` / `USERPROFILE`) inside `path.rs` ensures `~/...` paths are properly expanded and normalized during actual search execution in `engine.rs` as well.
  - Previously, `crates/cli/src/args.rs` resolved positional arguments without running `parse_search_paths` or subpath pruning, allowing both parent and child directories (or a directory and a file inside it) to remain in `options.input_paths`. Calling the shared `xlseek_core::search::path::deduplicate_and_prune_subpaths` in `cli/src/args.rs` eliminates redundant directory traversal in CLI searches.
- **Alternatives Considered**:
  - *Keeping separate deduplication logic in CLI and Core*: Rejected because it violates DRY and risks divergence in edge cases (such as symlinked subdirectories or mixed directory/file targets).

## 3. Handling Mixed Directory and Individual File Targets in CLI

- **Decision**: When pruning subpaths from a list of canonicalized valid target paths (`Vec<PathBuf>`), any path `P` (whether a directory or an eligible file) is pruned if another path `A` (`A != P`) in the set is a directory (`A.is_dir()`) and `P.starts_with(A)`.
- **Rationale**:
  - In `crates/cli`, users can specify both directories and individual files. If a user specifies `/data` and `/data/sub/report.xlsx`, `/data` is a directory and `/data/sub/report.xlsx` is already validated to have an eligible extension in `parse_args`. Pruning `/data/sub/report.xlsx` when its ancestor directory `/data` is present prevents scanning it as both a `file_root` and a discovered file under `directory_roots`.
  - If two identical file paths are specified, exact deduplication retains only one. If a file path is compared against another file path, a file cannot be an ancestor directory of another path (`A.is_dir()` check ensures safety).
- **Alternatives Considered**:
  - *Only pruning directory-to-directory and relying on `same_file::Handle` for files*: While `Handle` prevents double parsing in CLI, pruning the redundant subpath up front keeps `input_paths` minimal and consistent with directory subpath pruning.

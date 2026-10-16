# Interface Contracts: Search Path Normalization and Subpath Deduplication (`026-dedup-search-paths`)

## 1. Core Path Resolution API (`crates/core/src/search/path.rs`)

### `normalize_canonical_path(path: &Path) -> PathBuf`
- **Purpose**: Canonicalizes an existing path on the filesystem (falling back to lexical component normalization if `fs::canonicalize` fails) and strips OS-internal Windows verbatim prefixes (`\\?\` and `\\?\UNC\`) so the returned `PathBuf` is both canonical for component comparison and clean for user-facing display.
- **Input**: `path: &Path` (resolved filesystem path).
- **Output**: `PathBuf` (clean canonical path).

### `deduplicate_and_prune_subpaths(paths: Vec<PathBuf>) -> Vec<PathBuf>`
- **Purpose**: Removes exact duplicates and prunes any path that is a descendant subpath (`child.starts_with(ancestor)`) of another directory path in the collection, regardless of input order.
- **Input**: `paths: Vec<PathBuf>` (normalized canonical paths).
- **Output**: `Vec<PathBuf>` (minimal top-level root paths preserving first-seen order of surviving roots).
- **Guarantees**:
  - Uses `Path::starts_with` (component-level comparison), never raw string prefix matching.
  - Only treats a path `A` as an ancestor if `A.is_dir()` (or if all items in the caller context are validated directories).
  - Order-independent pruning: `[child, parent]` and `[parent, child]` both return `[parent]`.

### `resolve_and_partition_paths(paths: &[String], home_dir: Option<&Path>) -> PathResolutionResult`
- **Purpose**: Expands home directory shorthands (falling back to OS environment variables `HOME` / `USERPROFILE` when `home_dir` is `None`), validates each directory for existence and readability, records any invalid paths in `invalid_paths`, normalizes valid paths via `normalize_canonical_path`, and prunes duplicates and subpaths via `deduplicate_and_prune_subpaths`.

---

## 2. CLI Argument Resolution Contract (`crates/cli/src/args.rs`)

- **Input**:
  - `--path` / `-p` with comma-separated (and optionally double-quoted) paths, OR
  - Positional path arguments (`xlseek <query> <path1> [path2 ...]`), which may also contain comma-separated paths or individual directory/file paths.
- **Behavior**:
  1. Each input path is expanded (`~`), validated for existence as a file or directory, and normalized via `xlseek_core::search::path::normalize_canonical_path`.
  2. File extensions of individual file targets are validated against active `extensions`.
  3. The list of normalized `input_paths` is passed through `xlseek_core::search::path::deduplicate_and_prune_subpaths` prior to constructing `CliOptions`.
- **Output**: `CliOptions.input_paths` contains only deduplicated top-level directories and non-descendant individual files, ensuring zero redundant directory walks and clean human-readable output paths.

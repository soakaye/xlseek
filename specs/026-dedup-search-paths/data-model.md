# Data Model: Search Path Normalization and Subpath Deduplication (`026-dedup-search-paths`)

## Entities

### 1. Raw Search Path (`String`)
- **Description**: A single unnormalized path token extracted from user input (via `parse_search_paths` from comma-separated strings or via CLI positional arguments).
- **Attributes**:
  - Raw text string (trimmed of outer whitespace and unquoted).
  - May contain home directory shorthand (`~/`, `~\`, or `~`), relative segments (`.`, `..`), mixed path separators (`/` and `\`), trailing slashes, or symbolic links.

### 2. Normalized Canonical Path (`PathBuf`)
- **Description**: A validated, existing filesystem path resolved to its canonical absolute representation and stripped of OS-internal verbatim prefixes (`\\?\` / `\\?\UNC\`) for human-readable display and component-level comparison.
- **Validation & Normalization Rules**:
  1. **Home Expansion**: Leading `~/`, `~\`, or exact `~` is expanded using the provided `home_dir` or OS environment variables (`HOME` / `USERPROFILE`).
  2. **Existence & Access Check**: Validated for existence and readability (`validate_search_directory` for GUI/Core directories, or file/directory existence for CLI) *before* subpath pruning so that invalid paths are always reported as warnings/errors.
  3. **Canonicalization & Display Normalization**: Resolved via `std::fs::canonicalize` (with lexical `.` / `..` cleanup fallback if needed) and normalized via `normalize_export_path` so Windows verbatim prefixes (`\\?\`) are removed while preserving canonical case and resolved symlinks.

### 3. Deduplicated Root Set (`PathResolutionResult` / `Vec<PathBuf>`)
- **Description**: The minimal, order-independent collection of top-level normalized canonical paths dispatched to the directory/file scanner.
- **Fields (`PathResolutionResult`)**:
  - `valid_roots: Vec<PathBuf>`: Deduplicated and subpath-pruned canonical target roots.
  - `invalid_paths: Vec<(String, PathError)>`: Invalid or unreadable raw path strings paired with their classified `PathError`.
- **Invariants**:
  - **No Exact Duplicates**: For all `i != j`, `valid_roots[i] != valid_roots[j]`.
  - **No Ancestor-Descendant Overlap**: For all `i != j`, if `valid_roots[i]` is a directory, `!valid_roots[j].starts_with(&valid_roots[i])`.
  - **Component Boundary Strictness**: Sibling paths with shared string prefixes (e.g., `/data/report` and `/data/report_archive`) both remain in `valid_roots`.
  - **Order Independence**: Permuting the input order of `[parent, parent/child]` yields the exact same `valid_roots` (`[parent]`).

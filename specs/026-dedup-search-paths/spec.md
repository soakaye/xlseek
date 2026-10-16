# Feature Specification: Search Path Normalization and Subpath Deduplication

**Feature Branch**: `026-dedup-search-paths`

**Created**: 2026-10-16

**Status**: Draft

**Input**: User description: "複数指定された検索パスを正規化し指定されたパスがその他指定パスの部分パスである場合は２重に検索が行われない様にする。"

## Clarifications

### Session 2026-10-16

- Q: 親ディレクトリ（例: `/data`）と、その配下にある存在しない（またはアクセス権がない）サブパス（例: `/data/missing`）が同時に指定された場合、システムはどのように振る舞うべきですか？ → A: 存在・アクセス検証を先に行い、無効なサブパスは警告通知し、有効なパス間でのみ部分パス除外を行う。
- Q: 親ディレクトリ（例: `/data`）と、その配下のサブディレクトリ（例: `/data/archive`）の両方が検索パスに指定され、かつ無視パターン（Ignore Patterns）に `archive/` が設定されている場合、`/data/archive` はどのように扱われるべきですか？ → A: パス正規化時に常にサブパスを親パスへ統合し、走査時は設定された無視パターンを通常通り適用する。
- Q: GUIの検索結果一覧のパス表示やCLIの出力において、部分パス（サブディレクトリ）が親パスへ統合された場合、ヒットしたファイルの表示パスはどのように扱われるべきですか？ → A: 内部比較には正規化パスを用いつつ、検索結果や出力のファイルパスはOS標準の読みやすい絶対パス（Windowsの `\\?\` プレフィックスなし）に整えて表示する。

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Prevent Redundant Scanning of Nested Subdirectories (Priority: P1)

As a user specifying multiple search target directories at once,
I want the application to normalize all specified paths and automatically exclude any path that is a subdirectory (subpath) of another specified directory,
So that overlapping directory trees are never traversed or scanned twice, avoiding duplicate results and wasted search time.

**Why this priority**:
This is the core requirement directly requested by the user. Without subpath pruning across all entry points (GUI and CLI), specifying both a parent folder and one of its subfolders causes redundant directory traversal, inflated scan progress counters, or duplicate file evaluations.

**Independent Test**:
Specify both a parent directory and a nested subdirectory containing target documents (e.g., `/project/docs, /project/docs/2026`) in a single search execution. Verify that the subdirectory is consolidated into the parent directory root before traversal begins, each document in the subdirectory is scanned only once, and no duplicate matches are returned.

**Acceptance Scenarios**:

1. **Given** a parent directory `/data` and a nested child directory `/data/reports` both containing searchable documents,
   **When** the user specifies `/data, /data/reports` (in any order, such as child first `/data/reports, /data`) and runs a search,
   **Then** `/data/reports` is recognized as a subpath of `/data` and excluded from the active root list so that `/data/reports` is traversed only once as part of scanning `/data`.
2. **Given** deeply nested overlapping paths (e.g., `/a`, `/a/b`, `/a/b/c`),
   **When** the user initiates a search with all three paths,
   **Then** only the highest-level ancestor `/a` is retained as a traversal root, and files under `/a/b` and `/a/b/c` are scanned exactly once.
3. **Given** sibling directories with similar path prefixes (e.g., `/data/report` and `/data/report_archive`),
   **When** the paths are normalized and compared for subpath relationships,
   **Then** `/data/report_archive` is NOT treated as a subpath of `/data/report`, and both sibling directories are scanned completely.

---

### User Story 2 - Normalize Equivalent and Non-Canonical Path Representations (Priority: P2)

As a user entering paths through various means (manual typing, copy-paste, home directory shorthand, relative segments, or mixed slashes),
I want the system to normalize every specified path before comparing them,
So that identical directories or nested paths written in different syntactic forms are accurately detected and deduplicated.

**Why this priority**:
Users frequently combine paths from different sources (e.g., `~/docs` and `/Users/alice/docs/sub`, or paths with trailing slashes, `.` / `..` segments, or different letter casing on Windows/macOS). Without thorough normalization prior to subpath detection, equivalent or nested paths fail string or naive prefix comparisons and cause duplicate scans.

**Independent Test**:
Provide two paths pointing to the same directory or a parent/child pair using different notations (e.g., `/path/to/dir/../dir` and `/path/to/dir/sub/`, or `~/work` and `<home>/work/project`). Execute a search and confirm that normalization resolves both paths to their canonical forms and eliminates the redundant duplicate or subpath.

**Acceptance Scenarios**:

1. **Given** paths containing relative navigation segments (`.`, `..`), redundant separators (`//` or `\\`), or trailing slashes (e.g., `/work/project/./sub/../` and `/work/project`),
   **When** the search paths are normalized,
   **Then** they resolve to the same canonical target path and are deduplicated to a single scan root.
2. **Given** a path using home shorthand (e.g., `~/documents`) and an explicit absolute path to a subdirectory (e.g., `/Users/username/documents/finance`),
   **When** the user runs a search in GUI or CLI,
   **Then** the home shorthand is expanded and normalized before subpath checks, and `/Users/username/documents/finance` is pruned as a subpath of `~/documents`.
3. **Given** paths with different character casing or path separators on a case-insensitive operating system such as Windows (e.g., `C:\Data\Project` and `c:/data/project/sub`),
   **When** the paths are normalized,
   **Then** the system normalizes casing and separators according to the host filesystem, accurately identifying `c:/data/project/sub` as a subpath of `C:\Data\Project`.

---

### User Story 3 - Consistent Deduplication Across CLI and Individual File Targets (Priority: P3)

As a user running searches via the CLI or specifying individual file paths alongside directory paths,
I want the same path normalization and subpath deduplication rules to apply when a specified file resides inside a specified directory (or when multiple CLI path arguments overlap),
So that CLI and GUI searches behave identically and never scan or report the same file twice.

**Why this priority**:
In addition to comma-separated paths, the CLI accepts multiple positional path arguments and individual file paths alongside directory paths. Ensuring unified normalization and subpath pruning at the root resolution stage prevents unnecessary filesystem traversal in all execution modes.

**Independent Test**:
Run a search specifying both a directory `/data` and an individual file `/data/sub/sheet.xlsx` (or overlapping positional directories in CLI). Verify that the redundant subpath/file is pruned prior to scanning and the total scanned file count and matches reflect a single pass.

**Acceptance Scenarios**:

1. **Given** a search request specifying both a directory `/data` and a specific file `/data/2026/budget.xlsx` located inside that directory,
   **When** the search executes,
   **Then** the specific file path is recognized as a subpath of `/data` (provided it is eligible for directory discovery) or deduplicated so that `budget.xlsx` is parsed and searched only once.
2. **Given** multiple overlapping directories passed as separate positional arguments or comma-separated arguments in the CLI (e.g., `xlseek keyword ./docs ./docs/specs`),
   **When** the CLI resolves input paths,
   **Then** `./docs/specs` is pruned before directory discovery starts, avoiding double directory traversal.

---

### Edge Cases

- **Similar Directory Names (Prefix Collision)**: `/path/to/folder` and `/path/to/folder_2` share a character prefix, but `/path/to/folder_2` is NOT a subpath of `/path/to/folder`. Subpath checks must evaluate path component boundaries, never raw string prefixes.
- **Order Independence**: Whether the user enters `parent, parent/child` or `parent/child, parent`, the resulting deduplicated root list must always retain `parent` and prune `parent/child`.
- **Symbolic Links / Junctions**: When a symlink or directory junction points to a subdirectory of another specified path (or vice versa), canonicalizing both paths resolves them to their real physical targets, preventing duplicate traversal of the underlying target directory.
- **Mixed Valid and Invalid Paths**: Each specified path is validated for existence and readability first; if a user specifies an invalid/non-existent path alongside valid paths (even if the invalid path string looks like a subpath of a valid directory, e.g., `/valid/parent, /valid/parent/missing`), the invalid path is still reported as a non-fatal warning while the valid paths are normalized and deduplicated.
- **Ignored Subpath Explicit Specification**: If a parent directory `/data` and a child `/data/archive` are both specified while `archive/` is in the active ignore patterns, `/data/archive` is unconditionally pruned as a subpath of `/data` during target path normalization, and normal ignore pattern rules apply during traversal of `/data`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST validate each user-specified search target path for existence and readability first, and then normalize all valid target paths into canonical, absolute filesystem paths (expanding home directory shorthands `~`, resolving `.` and `..` relative segments, normalizing path separators, and resolving symbolic links) prior to executing directory traversal.
- **FR-002**: System MUST deduplicate normalized target paths that resolve to the exact same filesystem location, retaining only one instance.
- **FR-003**: System MUST unconditionally detect when any normalized valid target path is a subpath (descendant directory or file) of another normalized valid target directory in the same search request and exclude the descendant path from the active traversal root list prior to directory traversal and ignore-pattern filtering, while preserving standard warning diagnostics for any invalid or inaccessible paths.
- **FR-004**: Subpath comparison MUST operate strictly on path component boundaries so that sibling paths sharing a text prefix (e.g., `/dir/abc` vs `/dir/abcdef`) are never falsely identified as parent and subpath.
- **FR-005**: Subpath detection and deduplication MUST be order-independent: specifying a subpath before its ancestor path MUST still retain only the ancestor path and prune the subpath.
- **FR-006**: Path normalization and subpath comparison MUST respect host filesystem semantics, including case-insensitivity and separator variations (`/` vs `\`) on Windows and macOS.
- **FR-007**: Both the GUI search workflow and the CLI search workflow (whether paths are supplied via comma-separated strings or multiple CLI arguments) MUST apply the unified path normalization and subpath pruning logic before starting file discovery.
- **FR-008**: When overlapping paths are pruned, the search scan progress (`total_files` and `scanned_files`) and search results MUST reflect scanning each eligible file at most once.
- **FR-009**: While canonicalized paths are used for equality and subpath deduplication, all file paths emitted in search results, previews, and exports MUST be formatted as standard human-readable absolute paths (stripping OS-internal verbatim prefixes such as `\\?\` on Windows).

### Key Entities *(include if feature involves data)*

- **Raw Search Path**: An unnormalized path token provided by the user via GUI input or CLI argument (may be relative, use `~`, contain redundant separators, or differ in casing).
- **Normalized Canonical Path**: The resolved, canonical absolute filesystem path used for equality and component-wise ancestor/subpath comparisons.
- **Deduplicated Root Set**: The minimal set of top-level canonical target paths after removing exact duplicates and pruning all descendant subpaths.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of test cases where a parent directory and one or more of its subdirectories (at any nesting depth and in any input order) are specified together, each subdirectory and its files are traversed and scanned exactly once.
- **SC-002**: Zero duplicate search result rows are emitted when overlapping or differently formatted equivalent paths are provided in GUI or CLI.
- **SC-003**: 100% of sibling directories with shared name prefixes (e.g., `report` and `report_old`) are preserved as distinct search roots without false-positive pruning.
- **SC-004**: Path normalization and subpath deduplication complete in under 50 milliseconds for typical multi-path inputs (up to 50 target paths).

## Assumptions

- Filesystem canonicalization is available for all existing, readable target paths on the host OS; if canonicalization is not supported for a specific virtual/network path, component-level lexical normalization is used as a fallback.
- When a user specifies both an ancestor directory and a descendant file path, the descendant file is pruned from separate root processing if its extension is included in the active search extensions (and not excluded by ignore patterns); otherwise, normal directory traversal under the ancestor covers all eligible files.
- The user's raw input string in the GUI search bar and search history remains as entered by the user; normalization and subpath deduplication occur internally when resolving target roots for search execution.

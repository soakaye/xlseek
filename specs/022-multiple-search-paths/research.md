# Phase 0 Research: Multiple Search Target Paths with Quoted Delimitation

**Feature**: `022-multiple-search-paths`  
**Date**: 2026-10-02

## 1. Comma-Separated Path Parsing & Quoted Tokenization

### Context
Users need to supply multiple directories or files separated by commas (`,`), with support for path segments enclosed in double quotation marks (`"`) when paths contain internal commas, whitespace, or special characters (e.g. `"C:\My Reports, 2026", /var/data/sheets`).

### Decision
Implement a specialized, zero-external-dependency streaming tokenizer in `crates/core/src/search/path.rs` named `parse_search_paths(input: &str) -> Result<Vec<String>, PathParseError>`.
- Tokenizer states: `ScanningUnquoted`, `InsideQuoted`, `Escape`.
- Behavior:
  - Outside quotes: comma `,` serves as a segment delimiter; leading/trailing whitespace around delimiters is trimmed; empty segments are discarded.
  - Inside quotes: commas and whitespaces are preserved literally.
  - Quote escaping: consecutive quotes `""` or backslash `\"` inside quotes resolve to a literal quote `\"`.
  - Unclosed quote detection: if EOF is reached while still in `InsideQuoted`, return `PathParseError::UnclosedQuote`.
  - Outer quotes are stripped from the resulting token.

### Rationale
- Standard CSV or regex parsers often mangle Windows backslashes (`\`) or handle quotes inconsistently. A dedicated lightweight character-based state machine in `crates/core` provides 100% predictable semantics across platforms (macOS, Windows, Linux) without adding dependencies.
- Can be mirrored in TypeScript in `src/utils/pathTokenizer.ts` or exposed via a helper for consistent frontend and backend behavior.

### Alternatives Considered
- *Using a full CSV parser crate (`csv`)*: Rejected because standard CSV has strict line/record structures, CRLF assumptions, and treats backslashes differently from standard filesystem path formatting.
- *Simple regex splitting*: Rejected because regular expressions handling balanced quotes, escaped quotes, and commas are brittle and prone to catastrophic backtracking or edge case failures.

---

## 2. Multi-Path Directory Traversal & Deduplication

### Context
`SearchEngine` and `discover_workbooks` need to scan workbooks across multiple root directories without redundant file reads if directories overlap (e.g., `/data, /data/sub`).

### Decision
- Update `SearchEngine::execute_search_with_issues`:
  - When validating `target_dir`: tokenize using `parse_search_paths`.
  - For each raw path token, resolve home prefix and canonicalize/normalize path.
  - Separate valid vs invalid paths: if at least one valid path exists, emit non-fatal warnings for invalid paths and proceed with all valid paths. Abort only if 0 valid paths exist.
  - Deduplicate paths using canonical absolute paths (`std::fs::canonicalize` or normalized `PathBuf`). If path A is an ancestor of path B, drop redundant descendant path B from roots to avoid duplicate scans.
  - Pass the deduplicated roots `Vec<PathBuf>` to `discover_workbooks(&roots, ...)` (which already accepts `&[PathBuf]`).

### Rationale
- `discover_workbooks` in `crates/core/src/search/discovery.rs` already accepts `roots: &[PathBuf]`! Only `SearchEngine`'s caller currently assumed a single path.
- Resolving and canonicalizing roots upfront prevents duplicate workbook discovery and keeps multi-threaded scanning performant.

### Alternatives Considered
- *Running separate search passes per directory*: Rejected because it would split progress reporting, create multiple thread pools, and complicate result streaming.

---

## 3. Frontend Folder Picker & Drag-and-Drop Append Mechanics

### Context
When the search path input already contains paths, picking a folder via native dialog or dropping folders into the input field must append to the existing list rather than overwrite, quoting paths containing commas or whitespace.

### Decision
- Create a frontend utility `appendSearchPath(currentInput: string, newPath: string): string`:
  - If `newPath` contains `,` or whitespace, wrap in double quotes `"{newPath}"`.
  - If `currentInput.trim()` is empty, return formatted path.
  - If `currentInput.trim()` is non-empty: trim trailing commas and whitespace, then return `${trimmed}, ${formatted}`.
- Apply this helper in `SearchBar.tsx` for:
  - Folder open dialog selection callback.
  - Webview drag-and-drop drop callback (`onDragDropEvent` payload `paths`).
- Update inline autocomplete (`complete_directory_path`):
  - Autocomplete operates on the active token under the cursor (finding the comma boundary before and after cursor), replacing only that segment.

### Rationale
- Directly implements the user-confirmed clarification (Session 2026-10-02, Option A).
- Retains full history storage of the multi-path string while making interactive completions intuitive.

### Alternatives Considered
- *Always overwriting*: Rejected by user decision.
- *Modal popup asking replace vs append*: Adds unnecessary friction to common multi-folder searches.

---

## 4. CLI Argument Compatibility (`xlseek-cli`)

### Context
`xlseek-cli` supports positional arguments (e.g. `xlseek-cli "keyword" /dir1 /dir2`) and `--path` / `-p` option.

### Decision
- Support both:
  - Positional multiple paths: already supported by `crates/cli/src/args.rs`.
  - Delimited string in `--path` / `-p`: if `-p "dir1, dir2"` or `-p "\"dir with spaces\", dir2"` is passed, run `parse_search_paths` to expand into multiple input paths.
  - Deduplicate and validate input paths with warning messages for inaccessible directories when other valid paths are present.

### Rationale
- Ensures full feature parity between GUI and CLI.
- No breaking changes for existing CLI scripts.

# Contract: Path Parsing, Validation & IPC for Multi-Path Search

**Feature**: `022-multiple-search-paths`  
**Date**: 2026-10-02

## 1. Core API Contract (`xlseek-core`)

### 1.1 Tokenizer API (`crates/core/src/search/path.rs`)
```rust
/// Parses a comma-separated path string into individual path tokens,
/// respecting segments wrapped in double quotes.
///
/// ## Arguments
/// - `input`: `&str` - The raw comma-delimited path input.
///
/// ## Returns
/// - `Ok(Vec<String>)`: Vector of trimmed, quote-stripped path strings.
/// - `Err(PathParseError)`: Parse error if quotes are mismatched or input is empty.
pub fn parse_search_paths(input: &str) -> Result<Vec<String>, PathParseError>;

/// Resolves home prefixes, validates against the filesystem, deduplicates,
/// and partitions paths into valid roots and nonfatal invalid paths.
///
/// ## Arguments
/// - `paths`: `&[String]` - List of parsed path strings.
/// - `home_dir`: `Option<&Path>` - User home directory for shorthand resolution.
///
/// ## Returns
/// - `PathResolutionResult`: Containing valid root paths and invalid paths with errors.
pub fn resolve_and_partition_paths(
    paths: &[String],
    home_dir: Option<&Path>,
) -> PathResolutionResult;
```

---

## 2. Tauri IPC & Event Contract (`src-tauri`)

### 2.1 `start_search` Command (`commands/search_cmd.rs`)
- **Query Parameter**:
  ```json
  {
    "keyword": "string",
    "target_dir": "path1, \"path2 with spaces\", path3",
    "match_case": false,
    ...
  }
  ```
- **Validation**:
  - `target_dir` is parsed using `parse_search_paths`.
  - If syntax error (e.g. unclosed quote), returns `Err(CommandError { code: ErrorCode::SearchFailed })` with descriptive error or specific code.
  - If all target paths are invalid, returns `Err(CommandError { code: ErrorCode::NotFound })`.
  - If at least one valid path exists, starts scan on all valid paths.
- **Events Emitted**:
  - `EVENT_SEARCH_ISSUE` (`"search-issue"`): Emitted for any invalid or inaccessible path specified in `target_dir` so the frontend can display a warning notification without interrupting the scan of valid paths.
    ```json
    {
      "path": "/inaccessible/path",
      "stage": "discovery",
      "code": "not_found"
    }
    ```
  - `EVENT_SCAN_PROGRESS` (`"scan-progress"`): Progress reporting covers the aggregated files discovered across all valid target paths.
  - `EVENT_SEARCH_MATCH` (`"search-match"`): Matches stream as usual from all scanned workbooks.

### 2.2 `complete_directory_path` Command (`commands/search_cmd.rs`)
- **Input**:
  - `path_input`: The active path segment under cursor (extracted by frontend).
- **Output**:
  - `Vec<String>`: Directory auto-complete suggestions for that segment.

---

## 3. Frontend Utility Contract (`src/utils/pathTokenizer.ts`)

```typescript
/**
 * Splits a comma-separated path string into discrete path segments,
 * respecting double-quoted substrings.
 */
export function splitSearchPaths(input: string): string[];

/**
 * Appends a newly chosen directory or file path to an existing search path string,
 * automatically applying double quotes if the path contains whitespace or commas.
 */
export function appendSearchPath(currentInput: string, newPath: string): string;

/**
 * Identifies the path segment and character range currently active at the cursor position.
 */
export function getActivePathSegment(input: string, cursorPosition: number): {
  segment: string;
  startIndex: number;
  endIndex: number;
};
```

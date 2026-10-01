# Feature Specification: Multiple Search Target Paths with Quoted Delimitation

**Feature Branch**: `022-multiple-search-paths`

**Created**: 2026-10-02

**Status**: Draft

**Input**: User description: "検索対象パスを,区切りで複数指定可能とする。空白や,を含むパスなどは、\"でクオートされた中身のパスを一区切りのパスとする。"

## Clarifications

### Session 2026-10-02

- Q: How should the folder dialog and drag-and-drop actions behave when the search path input already contains existing paths? → A: Append to existing paths separated by `, `, automatically quoting if spaces or commas are present (or replace if input is empty).
- Q: If one path among several specified paths is inaccessible or does not exist, should the search abort entirely or proceed with the remaining valid paths while reporting a warning? → A: Proceed with valid paths, reporting non-fatal warnings/toast for invalid ones.
- Q: How should target path history and autocomplete suggestions behave with multiple paths in the GUI? → A: History stores the complete multi-path query string; auto-completion applies to the path segment under the cursor.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Multiple Comma-Separated Search Paths (Priority: P1)

As a user searching across multiple project folders or drives, I want to provide a comma-separated list of folder or file paths in the search path input so that the search scans all specified locations in a single operation.

**Why this priority**: Core value of the feature request. Users currently have to run separate searches for different directories or relocate files into a single parent folder.

**Independent Test**: Provide two valid directory paths separated by a comma (e.g., `/path/dirA, /path/dirB`) and run a search. The results must include matching items found across both directories.

**Acceptance Scenarios**:

1. **Given** two distinct folders containing Excel workbooks, **When** the user enters both paths separated by a comma into the search path field and executes the search, **Then** the search scans workbooks from both target folders and aggregates matching results into the result view.
2. **Given** multiple comma-separated paths with surrounding whitespace (e.g., `/path/dirA ,  /path/dirB`), **When** the search executes, **Then** leading and trailing whitespace around each unquoted token is trimmed and both directories are scanned.
3. **Given** a trailing comma or repeated commas (e.g., `/path/dirA, , /path/dirB,`), **When** the search executes, **Then** empty segments are ignored without triggering errors.

---

### User Story 2 - Quoted Paths with Spaces and Commas (Priority: P2)

As a user whose directory or file names contain whitespace, commas, or special characters, I want to enclose path segments in double quotes (`"`) so that the application treats each quoted segment as an individual path unit without splitting on internal commas or spaces.

**Why this priority**: Essential correctness for file systems that allow arbitrary path characters (such as macOS/Windows directories with commas or spaces, e.g., `"C:\Reports, 2026 Q1"` or `"/Users/name/Projects, Archive/data"`).

**Independent Test**: Enter a path containing a comma wrapped in double quotes alongside an unquoted path (e.g., `"/Volumes/Data, 2026", /tmp/docs`) and verify that exactly two distinct paths are scanned, preserving the comma inside the quoted path.

**Acceptance Scenarios**:

1. **Given** a directory path containing an internal comma, **When** the path is enclosed in double quotes (e.g., `"C:\Sales, Records"`), **Then** the tokenizer treats the entire quoted content as a single path unit.
2. **Given** a directory path containing whitespace, **When** the path is enclosed in double quotes (e.g., `"/Users/alice/My Documents"`), **Then** internal whitespace is preserved while outer quotes are stripped before path validation and scanning.
3. **Given** escaped or nested double quotes within a quoted segment (e.g. standard CSV-style double quotes `""` or backslash escaping), **When** parsed, **Then** the escape sequence resolves properly to a literal quote character within the path name.

---

### User Story 3 - Graceful Handling of Mixed Valid and Invalid Paths (Priority: P3)

As a user who specifies several paths at once, I want clear feedback if any path does not exist or cannot be accessed, while still having valid paths processed or receiving informative diagnostics.

**Why this priority**: Prevents silent failures or total aborts when a user makes a typo in one path among several valid target locations.

**Independent Test**: Provide one valid path and one non-existent path separated by comma (e.g., `/valid/path, /invalid/path/does_not_exist`). Verify that the application indicates which path is invalid or proceeds with available valid paths with clear user notification.

**Acceptance Scenarios**:

1. **Given** an invalid or non-existent path alongside at least one valid path, **When** the search starts, **Then** the system proceeds with scanning all valid paths while presenting a non-fatal warning/toast identifying the invalid path(s).
2. **Given** that ALL specified paths are invalid or non-existent, **When** the search is triggered, **Then** the search aborts before scanning and displays an error indicating no valid target paths were found.
3. **Given** duplicate paths specified in the comma-separated input, **When** the search starts, **Then** duplicate paths are deduplicated so that the same files are not scanned multiple times.
4. **Given** an unclosed double quote in the input string (e.g., `"/path/one, /path/two`), **When** the input is submitted, **Then** the system provides a clear syntax error warning indicating an unclosed quote.

---

### Edge Cases

- **Trailing / Leading Commas**: Input strings like `,/path/a,` must cleanly extract `/path/a` without producing empty path entries.
- **Consecutive Commas**: Input strings like `/path/a,,/path/b` must discard the empty token between the commas.
- **Only Whitespace or Commas**: Input strings consisting only of spaces, tabs, or commas must be treated as empty input and trigger the standard "search path is required" validation.
- **Unclosed Quotes**: An input with an odd number of quotes must report an unclosed quote error or parse up to the end of string safely without crashing.
- **Nested / Overlapping Directories**: If a user enters both a parent directory and a subdirectory (e.g., `/data, /data/sub`), file enumeration should avoid emitting duplicate search results for the same file.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept multiple search target paths separated by comma delimiters (`,`) in both the GUI search target input and the backend search engine interface.
- **FR-002**: System MUST treat any path segment enclosed in double quotation marks (`"..."`) as a single discrete path unit, preserving all internal spaces, commas, and special characters.
- **FR-003**: System MUST strip the enclosing double quotation marks from each parsed path unit before verifying filesystem existence and scanning.
- **FR-004**: System MUST trim extraneous whitespace surrounding unquoted path segments and outside quotation delimiters.
- **FR-005**: System MUST discard empty path segments resulting from leading, trailing, or consecutive commas without failing.
- **FR-006**: System MUST deduplicate target paths after normalization so that identical directories or files are not scanned redundantly.
- **FR-007**: System MUST validate each parsed target path against the filesystem; if at least one valid path exists, it MUST proceed scanning the valid paths while reporting non-fatal diagnostics/warnings for unreachable paths, and only abort if no valid paths exist.
- **FR-008**: System MUST report a user-friendly validation error when the input contains mismatched or unclosed quotation marks.
- **FR-009**: The GUI directory selection dialog and drag-and-drop actions MUST append newly selected or dropped folders to the current path input separated by `, ` (automatically wrapping them in double quotes if the path contains commas or spaces), or populate as the sole path if the input is currently empty.
- **FR-010**: CLI commands MUST support both comma-separated quoted path strings in the target path argument and multiple path arguments consistently.
- **FR-011**: The GUI search history MUST preserve the complete comma-separated path query string, while inline folder path completion MUST evaluate and replace only the active path segment under the user's cursor.

### Key Entities *(include if feature involves data)*

- **Search Path Input**: Raw string entered by the user containing one or more paths, optional comma separators, whitespace, and quotation marks.
- **Parsed Path Token**: A normalized individual filesystem path string extracted after splitting, quote unescaping, and whitespace trimming.
- **Target Location List**: A validated, deduplicated collection of accessible filesystem directory or file paths dispatched to the search scanner.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can specify 2 or more distinct target directories in a single search query and receive combined results without executing separate searches.
- **SC-002**: 100% of paths containing commas or whitespace enclosed in double quotes are correctly parsed as single path units without corruption.
- **SC-003**: Redundant/duplicate directory paths specified in the input do not result in duplicate result items in the search results table.
- **SC-004**: Any malformed input (such as unclosed quotation marks or non-existent directories) produces clear feedback within 1 second.
- **SC-005**: Backward compatibility is preserved: entering a single path without quotes or commas continues to function identically to previous versions.

## Assumptions

- Users understand standard CSV-like quoting conventions where double quotes group characters containing delimiter characters.
- In GUI file dialog folder selection and drag-and-drop, newly chosen directories append to existing comma-separated paths (with quotes if needed) or set the path if empty.
- Filesystem path separators (`/` on Unix/macOS, `\` on Windows) within quotes are preserved as-is.
- If both a parent directory and a subdirectory are specified, file deduplication based on canonicalized absolute paths ensures each workbook is scanned only once.

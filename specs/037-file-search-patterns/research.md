# Research & Technical Decisions: File Search Patterns with Extension Complement and Sequential Filtering

**Feature**: `037-file-search-patterns`  
**Date**: 2026-10-27  
**Status**: Completed  

---

## 1. Pattern Tokenization and Delimiter Parsing

### Context
Users can specify multiple inclusion file patterns (e.g. `*budget*, annual_report, "quarterly, q1*"`). We need a parsing strategy that splits comma-separated tokens while respecting quoted patterns containing commas or spaces and trimming unnecessary whitespace.

### Decision
Reuse the quote-aware comma tokenization pattern established in `crates/core/src/search/ignore.rs` (`parse_ignore_patterns`), parameterized for file patterns as `parse_file_patterns`.
- Commas serve as the delimiter between multiple patterns.
- Double quotes (`"..."`) protect commas within a single pattern token.
- Whitespace surrounding tokens is trimmed; empty tokens are discarded.

### Rationale
- Behavioral consistency across ignore patterns and file search patterns reduces cognitive load for users.
- Proven implementation in `crates/core` with existing unit tests covering quoted commas and whitespace edge cases.

### Alternatives Considered
- *Whitespace delimiter*: Rejected because filenames commonly contain spaces (e.g., `Annual Budget 2024.xlsx`).
- *Semicolon delimiter*: Rejected because comma is already standard across the application for multi-value inputs (search paths, ignore patterns).

---

## 2. Extension Complement and Explicit Extension Detection

### Context
User requirement: "入力パターンに拡張子が指定されていない場合はオプションで指定された拡張子で補完" (If no extension is specified in the input pattern, complement with the extensions specified in options). Furthermore, if an extension is explicitly specified, it must not be complemented with duplicate suffixes, and an explicit extension takes precedence over unchecked toggle options for supported formats.

### Decision
Implement `complement_file_patterns(tokens: &[String], enabled_extensions: &[String]) -> Vec<String>`:
1. Define recognized supported extensions set: `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, `.docx`, `.docm`, `.doc`.
2. Inspect each pattern token:
   - Check if the token ends with a recognized supported extension (e.g. ends with `.xlsx`, `.xls`, etc., case-insensitive).
   - If a recognized extension is present: the pattern is marked explicit. Keep the token as-is without appending any extensions.
   - If no recognized extension is present:
     - For each extension in `enabled_extensions` (e.g. `[".xlsx", ".xlsm"]`), produce a new complemented token:
       - If the token ends with `.` (e.g. `budget.`), append the extension without the leading dot (`budget.xlsx`).
       - Otherwise, append the extension directly (`budget` + `.xlsx` -> `budget.xlsx`; `*report*` + `.xlsx` -> `*report*.xlsx`).
3. If `tokens` is empty or all whitespace, return an empty vector (which signals the search engine to use the default extension-only match).

### Rationale
- Accurately differentiates file patterns containing internal version numbers or dots (e.g. `report.v1.0*`) from actual file extensions.
- Accurately avoids duplicate suffixes like `sample.xlsx.xlsx`.
- Accurately expands multiple enabled extensions (e.g. `budget` becomes `budget.xlsx` and `budget.xlsm`).

### Alternatives Considered
- *Pure suffix check by checking any dot*: Rejected because file names frequently contain dots that are not file extensions (e.g., `v1.2_summary` or `corp.division.*`).
- *Implicit wildcard addition (`*pattern*`)*: Rejected per user clarification in Session 2026-10-27 (standard glob semantics require exact base name match unless `*` is explicitly specified).

---

## 3. Glob Matching Engine Selection

### Context
Candidate files must be tested against the complemented file patterns. We need a performant, thread-safe, cross-platform matching engine in Rust.

### Decision
Use `ignore::gitignore::GitignoreBuilder` (from the already existing `ignore = "0.4"` dependency in `crates/core`) to compile the complemented file patterns into a `FilePatternMatcher`.
- Each complemented pattern is added as a rule line into `GitignoreBuilder`.
- For candidate files, `is_match(path)` checks `gitignore.matched(path, false).is_ignore()`.
- Thread-safe (`Send + Sync`) and directly clonable across parallel discovery threads.

### Rationale
- Zero new dependencies added to `Cargo.toml`.
- High-performance Aho-Corasick / regex DFA compilation natively supported by `ignore`.
- Cross-platform case normalization on Windows and macOS is already handled by `GitignoreBuilder`.
- Supports full standard glob features (`*`, `?`, `[...]`, `**`).

### Alternatives Considered
- *Custom Regex compilation*: Slower for large sets of patterns; requires escaping special characters.
- *`globset` crate*: Would introduce an unnecessary redundant dependency when `ignore` already encapsulates it.

---

## 4. Sequential Traversal Pipeline (Include Patterns -> Ignore Patterns)

### Context
User requirement: "入力パターン→無視パターンの順に適用" (Apply input pattern, then ignore pattern).

### Decision
In `crates/core/src/search/discovery.rs` (`discover_files_with_ignore` / `visit_directory_entries`), evaluate each candidate file in strict sequential order:
```text
Candidate Path
      │
      ▼
Is Directory? ───► Yes ───► Ignore Matcher (prune directory?) ───► Push to traversal queue
      │ No
      ▼
Step 1: Include Filter (File Search Patterns)
      ├─ If file_patterns provided: FilePatternMatcher.is_match(&file_path)
      └─ If file_patterns empty: is_eligible_file(&file_path, enabled_extensions)
      │
      ├─ No Match ──► SKIP (do not search, skip ignore evaluation)
      │
      ▼ Match
Step 2: Ignore Filter (Ignore Patterns)
      ├─ IgnoreMatcher.is_ignored(&file_path, false)
      │
      ├─ Ignored ───► SKIP (exclude matching temporary/backup files)
      │
      ▼ Accepted
Deliver to search worker (parse workbook contents)
```

### Rationale
- Guarantees that non-matching files are discarded at the earliest possible stage without invoking ignore rule evaluation.
- Preserves directory pruning for ignore patterns while ensuring file-level filters strictly follow the requested pipeline.

---

## 5. UI Layout, Row Ordering, and History Persistence

### Context
The search bar needs an ergonomic, uncluttered layout incorporating Search Directory, File Search Patterns, Ignore Patterns, and Search Query.
Clarification Session 2026-10-27 confirmed:
- Dedicated row between Search Directory and Ignore Patterns (`Directory` -> `File Search Patterns` -> `Ignore Patterns` -> `Search Query / Options`).
- History storage for File Search Patterns (LRU up to 20 entries).
- Settings default configuration under Saved Defaults tab.

### Decision
1. **Frontend Component**: Add a dedicated row in `src/components/search/SearchBar.tsx` directly below `SearchDirectoryInput` and above `IgnorePatternsInput`.
2. **History Management**: Create `src/utils/filePatternHistory.ts` mimicking `src/utils/ignoreHistory.ts`:
   - Key: `constants::STORAGE_KEY_FILE_PATTERN_HISTORY` (`"xlseek.filePatternHistory"`).
   - Max entries: 20 (`constants::MAX_FILE_PATTERN_HISTORY_ENTRIES`).
   - Deduplication and LRU ordering.
3. **Settings Dialog**: Add `defaultFilePatterns: string` to `SearchOptionsConfig` in `src/settings-core.ts` and `SavedDefaultsTab.tsx`.

---

## 6. CLI and MCP Server Integration

### Context
CLI and MCP tools need to expose the file search pattern functionality with matching semantics.
Clarification Session 2026-10-27 confirmed:
- CLI flag: `-g` and `--file-patterns <PATTERNS>` (avoiding `-p` collision with `--path`).
- MCP tool parameter: `file_patterns` (optional string, comma-separated) in the `search` tool schema.

### Decision
1. **CLI (`crates/cli/src/args.rs`)**:
   - Add constants:
     - `CLI_SHORT_FILE_PATTERNS = "-g"`
     - `CLI_LONG_FILE_PATTERNS = "--file-patterns"`
     - `CLI_FILE_PATTERNS_FIELD = "file_patterns"`
   - Map into `SearchQuery.file_patterns`.
2. **MCP (`crates/mcp/src/tools/search.rs`)**:
   - Add `"file_patterns"` to JSON Schema with description: `"Comma-separated file search patterns (e.g. '*report*, budget*'). Extensionless patterns are complemented with active extensions."`.
   - Extract string and inject into `SearchQuery.file_patterns`.

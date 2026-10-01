# Data Model: Multiple Search Target Paths with Quoted Delimitation

**Feature**: `022-multiple-search-paths`  
**Date**: 2026-10-02

## 1. Entities & Data Structures

### 1.1 `SearchPathInput` (Frontend & CLI string)
The raw user-provided input string from UI input box, CLI argument, or query payload.
- **Type**: `String`
- **Constraints**: Non-empty after trimming, balanced quotes.

### 1.2 `PathParseError` (Core Enum)
Classification of syntax errors during path string tokenization.
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathParseError {
    EmptyInput,
    UnclosedQuote,
}
```

### 1.3 `ParsedPathToken` (Core Struct)
An unescaped, trimmed path candidate before filesystem validation.
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPathToken {
    pub raw: String,
    pub unquoted: String,
}
```

### 1.4 `PathResolutionResult` (Core Struct)
The outcome of resolving, canonicalizing, validating, and deduplicating parsed path tokens.
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathResolutionResult {
    pub valid_roots: Vec<std::path::PathBuf>,
    pub invalid_paths: Vec<(String, PathError)>,
}
```

- **Invariants**:
  - `valid_roots` contains only canonicalized / absolute directories or files that exist and have read permissions.
  - `valid_roots` contains no duplicates or redundant sub-paths if a parent ancestor path is already present.
  - `invalid_paths` contains any specified path tokens that do not exist or lack permission, paired with the specific `PathError`.

---

## 2. State & Data Flow

```text
+-------------------------------------------------------------+
| User Input / CLI Option:                                   |
| '"C:\Sales, 2026", /var/data/sheets, non_existent_dir'     |
+-------------------------------------------------------------+
                              |
                              v
             parse_search_paths(&str)
                              |
                              v
             Vec<ParsedPathToken>:
             1. "C:\Sales, 2026" (quotes removed, comma preserved)
             2. "/var/data/sheets"
             3. "non_existent_dir"
                              |
                              v
             resolve_and_validate_paths()
                              |
         +--------------------+--------------------+
         |                                         |
         v                                         v
   valid_roots:                            invalid_paths:
   - PathBuf("C:\Sales, 2026")             - ("non_existent_dir", NotFound)
   - PathBuf("/var/data/sheets")                   |
         |                                         v
         |                                  emit DiscoveryIssue /
         |                                  warning toast to UI
         v
   discover_workbooks(&valid_roots, ...)
```

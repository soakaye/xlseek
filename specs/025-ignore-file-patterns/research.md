# Research: Ignore File Patterns with Gitignore-Style Matching and History

**Feature**: `025-ignore-file-patterns`
**Date**: 2026-10-15

## Research Overview

This document analyzes technical options and architectural decisions for implementing comma-separated gitignore-style ignore patterns with history management and Settings persistence across the xlseek workspace (`crates/core`, `crates/cli`, `src-tauri`, and `src/`).

---

## 1. Gitignore Pattern Matching in Rust (`crates/core`)

### Context
The user specified: "gitignorパターン指定と同形式でカンマ区切りで無視ファイルリストをサポートする" (Support ignore file list in comma-separated format matching gitignore pattern format).
Patterns include file globs (`~$*`, `*.tmp`), directory-only markers (`node_modules/`, `temp/`), rooted paths (`/build`), and negation (`!keep.xlsx`).

### Options Evaluated
1. **`ignore::gitignore::GitignoreBuilder` (from ripgrep's `ignore` crate)**:
   - **How it works**: Implements full `.gitignore` parsing and evaluation as used by `ripgrep`. `GitignoreBuilder::new(root)` parses patterns using `add_line(None, pattern)`. Then `matched(path, is_dir)` evaluates whether a file or directory is ignored.
   - **Pros**:
     - 100% conforming `.gitignore` syntax specification (wildcards `*`, `?`, `**`, trailing `/` for directories, negation `!`, character classes).
     - Highly optimized DFA compiled via `globset`.
     - Handles directory vs file matching distinction natively.
     - Battle-tested in ripgrep with zero edge-case discrepancies.
   - **Cons**: Adds `ignore` crate dependency (~150KB compiled).
2. **`globset::GlobSet` directly**:
   - **How it works**: Compiles a set of `Glob` items into a single regex/DFA.
   - **Pros**: Minimal crate.
   - **Cons**: Requires manually implementing gitignore rules (such as trailing `/` meaning directory-only, distinguishing basename matches from full path matches, handling `!` negation).
3. **Custom regex-based glob transpiler**:
   - **How it works**: Convert globs to regexes manually using `regex`.
   - **Pros**: No new dependency.
   - **Cons**: High complexity, prone to subtle edge-case bugs (character escaping, unicode, directory separators, path boundaries, negation order).

### Decision
**Adopt `ignore::gitignore::GitignoreBuilder` (or `globset`)**:
`ignore::gitignore` provides exact gitignore semantics with native directory pruning and file filtering.
We encapsulate this into a clean domain struct in `crates/core/src/search/ignore.rs`:
```rust
pub struct IgnoreMatcher {
    gitignore: ignore::gitignore::Gitignore,
}
```
With methods:
- `IgnoreMatcher::from_patterns(root: &Path, patterns: &[String]) -> Result<Self, String>`
- `is_ignored(&self, path: &Path, is_dir: bool) -> bool`

If no patterns are provided (or only whitespace), a no-op / passthrough matcher is used with zero overhead.

---

## 2. Comma-Separated Pattern String Parsing (`crates/core` & `src/utils`)

### Context
Users supply patterns as a comma-separated string, e.g. `~$*, *.tmp, "dir, with, comma/*"`.

### Decision
Follow the proven pattern from Feature 022 (`parse_search_paths` in `crates/core/src/search/path.rs` and `splitSearchPaths` in `src/utils/pathTokenizer.ts`):
- Split on unquoted commas `,`.
- Trim surrounding whitespace around each pattern token.
- Support enclosing double quotes `"` for patterns containing commas or spaces.
- Discard empty tokens resulting from leading, trailing, or consecutive commas.
- Maintain parity between frontend tokenization and Rust core parser.

---

## 3. UI Placement & UX Layout (`src/components/search/SearchBar.tsx`)

### Context
The user specified: "検索ディレクトリ近くにパターン指定を配置。パターン履歴を保存し選択可能とする。"
In the clarify session, it was decided (Option A):
- Place the ignore pattern input on a dedicated row directly beneath the search directory input row and above the file extension toggles.
- Pre-populate `~$*` as the initial default pattern to protect users from Excel temporary lock file errors.

### Component Design
- Input row layout:
  - Filter/EyeOff/FileX icon + localized label ("無視パターン:" / "Ignore patterns:")
  - Text input field with placeholder (e.g. `~$*, *.tmp, backup/*`)
  - History dropdown trigger button (Clock/History icon)
  - Clear button (X) when text is non-empty
- Dropdown menu:
  - Displays up to 20 previously executed ignore pattern combinations.
  - Supports keyboard navigation (Arrow Down, Arrow Up, Enter, Escape) and click selection.

---

## 4. History & Settings Persistence (`src/hooks/useSearchHistory.ts`, `src/settings-core.ts`)

### Context
Clarification confirmed:
- Pattern history saved in localStorage across sessions.
- Default ignore patterns configurable in Settings dialog ("Saved Defaults" tab).

### Storage Keys & Schema
- `xlseek.ignorePatternHistory`: Array of unique strings, max 20 entries, MRU ordered.
- `xlseek.default_search_options`: Extend `DefaultSearchOptions` with `ignore_patterns: string` (default: `"~$*"`).
- Query initialization: When search starts or resets, `query.ignore_patterns` initializes from saved defaults (or `"~$*"` if not set).

---

## 5. Directory Traversal Integration (`crates/core/src/search/discovery.rs`)

### Context
`discover_files` traverses roots serially or with concurrent workers.
- **Directory evaluation**: When `file_type.is_dir()`:
  Check `ignore_matcher.is_ignored(&entry_path, true)`.
  If true, skip the directory immediately! It never pushes `DirectoryFrame` and never adds tasks to the pending queue.
- **File evaluation**: When `file_type.is_file()`:
  Check `ignore_matcher.is_ignored(&entry_path, false)`.
  If true, skip the file without invoking `on_file`.

This ensures early pruning of excluded folder trees (e.g., `node_modules` or `.git`) with maximal throughput gains.

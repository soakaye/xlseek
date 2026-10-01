# Bug Assessment: Strip Verbatim Prefix (`\\?\`) on File and Stdout Output

- **Slug**: cli-windows-verbatim-path
- **Created**: 2026-10-02T01:28:30+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> CLIのWindows版ローカルファイルパスに\\?\が付いている. stdout,ファイルに出力時にパス名から`\\?\`を除く

On Windows, the local file path has a verbatim `\\?\` prefix. The requirement is specifically to remove `\\?\` from file paths when outputting to stdout or to export files (e.g. CSV / XLSX), while keeping it internally during filesystem traversal/processing if needed.

## Symptom

When executing the CLI tool (`xlseek-cli`) on Windows, local file paths written to search result outputs (CSV/XLSX export files and stdout stream) include the Windows verbatim extended-length prefix `\\?\` (e.g., `\\?\C:\Users\...\sample.xlsx` instead of `C:\Users\...\sample.xlsx`).

Expected behavior: In search result output destinations (stdout CSV output, exported CSV files, and exported XLSX files), file paths in the `full_path` column must have the raw `\\?\` or `\\?\UNC\` prefix stripped to display clean standard paths (e.g. `C:\Users\...\sample.xlsx` or `\\server\share\...`). Internal operations during search and file access may continue to use whatever internal path representation is appropriate.

## Reproduction

1. On Windows, execute `xlseek-cli` providing an input file or directory:
   ```cmd
   xlseek-cli -p C:\work\reports -q "keyword"
   ```
2. Inspect the resulting search match output on stdout, or export to CSV/XLSX:
   ```cmd
   xlseek-cli -p C:\work\reports -q "keyword" -o result.csv
   ```
3. Observe that the `full_path` column in the output contains `\\?\C:\work\reports\...`.

## Suspected Code Paths

- `crates/core/src/export/csv_export.rs:84-98` (`write_csv_to_writer`):
  Directly serializes `item.full_path` into the CSV row for stdout and file export.
- `crates/core/src/export/xlsx_export.rs:122-127` (`export_to_xlsx`):
  Directly writes `&item.full_path` into column 2 of the XLSX worksheet.
- `crates/cli/src/lib.rs:77-80` (`run` / `execute` summary):
  Displays `output_path` in the terminal summary (`path.display()`).

## Root Cause Hypothesis

**Confidence: High**

When resolving search input paths on Windows, `std::fs::canonicalize` returns extended-length verbatim paths starting with `\\?\` (for local drives) or `\\?\UNC\` (for network shares). The search parser populates `SearchMatch.full_path` directly with this string.

During export (`write_csv_to_writer` and `export_to_xlsx`), `item.full_path` is serialized verbatim into the output stream or file without stripping the Windows `\\?\` prefix.

## Proposed Remediation

**Preferred**:
1. Implement a shared, cross-platform path normalization helper `normalize_display_path(path: &str) -> Cow<'_, str>` (or `strip_verbatim_prefix_str(path: &str) -> &str` / `PathBuf`) in `xlseek_core::export` (or `xlseek_core::search::path`):
   - Handles `\\?\UNC\` -> converts to `\\` (standard UNC share prefix).
   - Handles `\\?\` -> strips the prefix for local drive paths (e.g., `\\?\C:\...` -> `C:\...`).
   - Handles forward-slash variants (e.g., `//?/`) safely.
   - For all other paths, returns the path unmodified.
2. Define centralized path prefix constants in `crates/core/src/constants.rs` conforming to Constitution Principle II:
   - `VERBATIM_PATH_PREFIX_WINDOWS = r"\\?\"`
   - `VERBATIM_UNC_PATH_PREFIX_WINDOWS = r"\\?\UNC\"`
   - `STANDARD_UNC_PREFIX_WINDOWS = r"\\"`
3. In `crates/core/src/export/csv_export.rs` (`write_csv_to_writer`) and `crates/core/src/export/xlsx_export.rs` (`export_to_xlsx`), format `item.full_path` using this helper before writing the record/cell.
4. Also format the summary output path in `crates/cli/src/lib.rs` when presenting `output_path` to users.
5. Add unit tests in `crates/core/src/export/csv_export.rs`, `crates/core/src/export/xlsx_export.rs`, and/or `crates/cli` verifying that verbatim prefixes (`\\?\C:\foo.xlsx` and `\\?\UNC\server\share\foo.xlsx`) are cleaned in CSV and XLSX outputs.

**Alternatives**:
- *Strip verbatim prefixes in `args.rs` when canonicalizing inputs*:
  - Trade-off: Stripping at the input resolution stage alters internal paths, whereas the user specifically clarified: *"ファイル探索結果の出力に`\\?\`を削除する。内部で使用するのは構わない。"* (Remove `\\?\` from search result output; keeping it for internal usage is fine). Cleaning at output formatting aligns directly with this instruction while ensuring internal canonicalization can still utilize verbatim paths if necessary.

**Files likely to change**:
- `crates/core/src/constants.rs`
- `crates/core/src/export/mod.rs`
- `crates/core/src/export/csv_export.rs`
- `crates/core/src/export/xlsx_export.rs`
- `crates/cli/src/lib.rs`

**Tests to add or update**:
- Unit tests in `crates/core/src/export/csv_export.rs` testing that `write_csv_to_writer` outputs clean paths without `\\?\` or `\\?\UNC\`.
- Unit tests in `crates/core/src/export/xlsx_export.rs` testing that `export_to_xlsx` writes clean paths without `\\?\`.

## Risks & Considerations

- Path normalization should handle both disk drive paths (`\\?\C:\...` -> `C:\...`) and network share UNC paths (`\\?\UNC\server\share\...` -> `\\server\share\...`).
- Must not affect standard UNIX paths or paths without verbatim prefixes.
- Zero extra third-party dependencies required; adheres strictly to the project constitution.

## Open Questions

- None.

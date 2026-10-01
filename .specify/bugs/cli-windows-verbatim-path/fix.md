# Bug Fix: Strip Verbatim Prefix (`\\?\`) on File and Stdout Output

- **Slug**: cli-windows-verbatim-path
- **Fixed**: 2026-10-02T01:46:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Implemented path normalization (`normalize_export_path`) in `xlseek_core::export` to automatically strip the Windows verbatim extended-length prefix (`\\?\` and `\\?\UNC\`) from search match file paths (`full_path`) during search result exports (stdout CSV, exported CSV files, and exported XLSX files) and in the CLI execution summary. This ensures clean, standard paths are output without altering internal path handling during search traversal.

## Changes

| File | Change | Notes |
|---|---|---|
| `crates/core/src/constants.rs` | modified | Added `VERBATIM_PATH_PREFIX_WINDOWS`, `VERBATIM_UNC_PATH_PREFIX_WINDOWS`, and `STANDARD_UNC_PREFIX_WINDOWS` constants (Principle II). |
| `crates/core/src/export/mod.rs` | modified | Added `normalize_export_path` helper function and its unit test. |
| `crates/core/src/export/csv_export.rs` | modified | Applied `normalize_export_path` to `item.full_path` before writing CSV records, and added a unit test. |
| `crates/core/src/export/xlsx_export.rs` | modified | Applied `normalize_export_path` to `item.full_path` before writing XLSX cells, and added a unit test. |
| `crates/cli/src/lib.rs` | modified | Applied `normalize_export_path` to summary `output_path` before console display. |

## Diff Highlights

```diff
--- a/crates/core/src/constants.rs
+++ b/crates/core/src/constants.rs
+pub const VERBATIM_PATH_PREFIX_WINDOWS: &str = r"\\?\";
+pub const VERBATIM_UNC_PATH_PREFIX_WINDOWS: &str = r"\\?\UNC\";
+pub const STANDARD_UNC_PREFIX_WINDOWS: &str = r"\\";

--- a/crates/core/src/export/mod.rs
+++ b/crates/core/src/export/mod.rs
+pub fn normalize_export_path(path: &str) -> Cow<'_, str> {
+    if let Some(suffix) = path.strip_prefix(crate::constants::VERBATIM_UNC_PATH_PREFIX_WINDOWS) {
+        return Cow::Owned(format!(
+            "{}{}",
+            crate::constants::STANDARD_UNC_PREFIX_WINDOWS,
+            suffix
+        ));
+    }
+    if let Some(suffix) = path.strip_prefix(crate::constants::VERBATIM_PATH_PREFIX_WINDOWS) {
+        return Cow::Borrowed(suffix);
+    }
+    Cow::Borrowed(path)
+}

--- a/crates/core/src/export/csv_export.rs
+++ b/crates/core/src/export/csv_export.rs
+        let clean_path = crate::export::normalize_export_path(&item.full_path);
         csv_writer
             .write_record([
                 item.id.to_string(),
                 item.file_name.clone(),
-                item.full_path.clone(),
+                clean_path.into_owned(),
```

## Tests Added or Updated

- `crates/core/src/export/mod.rs::tests::test_normalize_export_path`: Verifies that drive paths with `\\?\` (e.g., `\\?\C:\...`) and UNC paths with `\\?\UNC\` (e.g., `\\?\UNC\server\share\...`) have their prefixes converted/stripped properly, while standard Unix and Windows paths remain untouched.
- `crates/core/src/export/csv_export.rs::localization_tests::write_csv_to_writer_strips_windows_verbatim_paths`: Verifies that `write_csv_to_writer` outputs clean paths without `\\?\` in CSV rows.
- `crates/core/src/export/xlsx_export.rs::localization_tests::export_to_xlsx_strips_windows_verbatim_paths`: Verifies that `export_to_xlsx` writes clean paths without `\\?\` in column 2 of the worksheet.

## Local Verification

- `cargo test --workspace`: 69/69 Rust unit and integration tests passed across all workspace crates.
- `cargo clippy --workspace --all-targets -- -D warnings`: 0 compiler or Clippy warnings.
- `cargo fmt --check`: 0 formatting violations.
- `npm test`: 18/18 test files passed (114/114 tests passed).
- `npm run build`: TypeScript check and Vite production bundle passed.
- `npm run lint`: 0 ESLint warnings or errors.

## Deviations from Assessment

None. The implementation follows the preferred remediation in `assessment.md` as clarified by user requirements.

## Follow-ups

None.

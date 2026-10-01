# Bug Verification: Strip Verbatim Prefix (`\\?\`) on File and Stdout Output

- **Slug**: cli-windows-verbatim-path
- **Tested**: 2026-10-02T01:53:30+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

The fix for stripping the Windows verbatim extended-length prefix (`\\?\` and `\\?\UNC\`) was validated. Path normalization in `xlseek_core::export::normalize_export_path` successfully converts verbatim drive paths (`\\?\C:\...` -> `C:\...`) and verbatim UNC paths (`\\?\UNC\server\share\...` -> `\\server\share\...`) during CSV output (including stdout streaming), XLSX worksheet generation, and CLI completion summary output. Unit and integration tests verify that all exported paths are clean and free of `\\?\` without impacting internal filesystem operations.

## Checks Performed

| Check | Command / Action | Result | Notes |
|---|---|---|---|
| Reproduction (post-fix) | `cargo test -p xlseek-core --lib export::csv_export::localization_tests::write_csv_to_writer_strips_windows_verbatim_paths` | pass | Verified that `full_path` in CSV output strips `\\?\` prefix. |
| New / updated tests (XLSX export) | `cargo test -p xlseek-core --lib export::xlsx_export::localization_tests::export_to_xlsx_strips_windows_verbatim_paths` | pass | Verified that `full_path` in XLSX worksheet cells strips `\\?\` prefix. |
| New / updated tests (Path normalizer) | `cargo test -p xlseek-core --lib export::tests::test_normalize_export_path` | pass | Verified `normalize_export_path` on drive, UNC, and Unix path strings. |
| Regression suite (Rust CLI) | `cargo test -p xlseek-cli` | pass | All CLI tests (argument parsing, summary, stdout streaming, output safety) pass. |
| Regression suite (Rust Workspace) | `cargo test --workspace` | pass | All 69 workspace unit and integration tests passed cleanly. |
| Rust lints & style | `cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check` | pass | Zero Clippy warnings; rustfmt clean. |
| Frontend test & build suite | `npm test && npm run build && npm run lint` | pass | 114/114 vitest tests passed; TypeScript and Vite build passed; ESLint clean. |

## Output Excerpts

### Rust Export & Normalizer Tests
```text
running 10 tests
test constants::tests::test_csv_export_headers ... ok
test export::tests::test_normalize_export_path ... ok
test export::xlsx_export::localization_tests::rejects_unsupported_language ... ok
test models::localization_tests::export_request_reads_language_and_defaults_legacy_requests_to_english ... ok
test export::csv_export::localization_tests::rejects_unsupported_language ... ok
test export::csv_export::localization_tests::write_csv_to_writer_respects_bom_flag ... ok
test export::csv_export::localization_tests::write_csv_to_writer_strips_windows_verbatim_paths ... ok
test export::csv_export::localization_tests::uses_requested_catalog_for_headers ... ok
test export::xlsx_export::localization_tests::export_to_xlsx_strips_windows_verbatim_paths ... ok
test export::xlsx_export::localization_tests::uses_requested_catalog_for_headers ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.03s
```

### Full Workspace Tests & Clippy
```text
cargo test --workspace
...
test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
```

## Residual Risks

- None. Path normalization only alters user-facing presentation strings upon serialization into CSV, stdout, XLSX, and summary logs, while leaving internal path operations and filesystem canonicalization untouched.

## Recommendation

Close the bug — verified end-to-end.

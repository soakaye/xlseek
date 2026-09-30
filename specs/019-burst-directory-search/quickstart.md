<!--
Purpose: Provides repeatable end-to-end validation steps for desktop and CLI Burst directory search.
Inputs: A built implementation, local sample workbook, and development tools; output: observed results for the acceptance criteria.
Errors: Commands may fail when prerequisites are missing or the implementation is incomplete; investigate the command output before claiming a pass.
-->
# Quickstart: Validate Burst Directory Search

Run these steps from the repository root after implementation. This guide uses PowerShell syntax on Windows; use equivalent commands on macOS or Linux. The interface details are in [contracts/](./contracts/) and the fields and states are in [data-model.md](./data-model.md).

## Prerequisites

- Rust and Cargo, Node.js and npm, and the desktop development prerequisites are installed.
- `npm install` has completed, and `tests/fixtures/sample_report.xlsx` exists.
- The active branch contains the implementation described in [plan.md](./plan.md).

## Create an isolated nested fixture

```powershell
$demoRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('xlseek-burst-' + [guid]::NewGuid().ToString('N'))
$inputDir = Join-Path $demoRoot 'input'
$firstDir = Join-Path $inputDir 'first'
$deepDir = Join-Path $firstDir 'deeper'
$secondDir = Join-Path $inputDir 'second'
New-Item -ItemType Directory -Force -Path $deepDir, $secondDir | Out-Null
Copy-Item -LiteralPath 'tests/fixtures/sample_report.xlsx' -Destination (Join-Path $inputDir 'root.xlsx')
Copy-Item -LiteralPath 'tests/fixtures/sample_report.xlsx' -Destination (Join-Path $firstDir 'first.xlsx')
Copy-Item -LiteralPath 'tests/fixtures/sample_report.xlsx' -Destination (Join-Path $deepDir 'deep.xlsx')
Copy-Item -LiteralPath 'tests/fixtures/sample_report.xlsx' -Destination (Join-Path $secondDir 'second.xlsx')
```

Keep `$demoRoot` in the same PowerShell session for the following commands. The input tree contains a root workbook and three workbooks in subfolders at two depths. The fixture is created under a unique temporary path and does not replace existing files.

## Verify the CLI contract

```powershell
$sequentialCsv = Join-Path $demoRoot 'sequential.csv'
$burstCsv = Join-Path $demoRoot 'burst.csv'
$customCsv = Join-Path $demoRoot 'custom.csv'
$defaultCsv = Join-Path $demoRoot 'default.csv'
cargo run -p xlseek-cli -- --query '.' --regex true --path $inputDir --directory-mode sequential --output $sequentialCsv --language en
cargo run -p xlseek-cli -- --query '.' --regex true --path $inputDir --directory-mode=burst --output $burstCsv --language en
cargo run -p xlseek-cli -- --query '.' --regex true --path $inputDir --burst-workers=2 --output $customCsv --language en
cargo run -p xlseek-cli -- --query '.' --regex true --path $inputDir --output $defaultCsv --language en
$sequentialRows = @(Import-Csv -LiteralPath $sequentialCsv | ForEach-Object { $_ | ConvertTo-Json -Compress } | Sort-Object)
$burstRows = @(Import-Csv -LiteralPath $burstCsv | ForEach-Object { $_ | ConvertTo-Json -Compress } | Sort-Object)
$customRows = @(Import-Csv -LiteralPath $customCsv | ForEach-Object { $_ | ConvertTo-Json -Compress } | Sort-Object)
$defaultRows = @(Import-Csv -LiteralPath $defaultCsv | ForEach-Object { $_ | ConvertTo-Json -Compress } | Sort-Object)
Compare-Object -ReferenceObject $sequentialRows -DifferenceObject $burstRows
Compare-Object -ReferenceObject $sequentialRows -DifferenceObject $customRows
Compare-Object -ReferenceObject $sequentialRows -DifferenceObject $defaultRows
```

Expected: all four commands exit successfully, the CSV files have the same number of semantic match rows, and all comparisons print no differences. The custom-count command implies Burst mode and limits concurrent directory visits to two. Confirm that results include paths under `first/deeper` and `second`. Automated integration tests should also check the selected discovery mode, actual visitor limit, and repeated-file handling; CSV equality alone cannot prove concurrent traversal.

Run the following invalid invocations and confirm each returns a localized usage error with exit status 2:

```powershell
cargo run -p xlseek-cli -- --query '.' --path $inputDir --directory-mode invalid
cargo run -p xlseek-cli -- --query '.' --path $inputDir --burst-workers 1
cargo run -p xlseek-cli -- --query '.' --path $inputDir --burst-workers 33
cargo run -p xlseek-cli -- --query '.' --path $inputDir --burst-workers 2.5
cargo run -p xlseek-cli -- --query '.' --path $inputDir --directory-mode sequential --burst-workers 2
```

Check `cargo run -p xlseek-cli -- --help --language en` and `--language ja` for both new options and the 2 through 32 custom range. A single-file input with Burst mode and a valid count should remain valid and produce the same matches as Sequential mode.

## Verify desktop settings and search

1. Run `npm run tauri dev` and open Settings. Confirm the directory search mode offers Sequential and Burst, with Sequential initially selected and the Burst parallel count set to Automatic.
2. Select Burst and Custom, enter `2`, save, close and reopen Settings, and confirm the count remains `2`. Change other search defaults and confirm they remain saved. Search `$inputDir`, then repeat in Sequential mode. Confirm the same semantic matches, including nested files, with no duplicates.
3. Restart the app. Confirm Burst and Custom `2` are still selected if those were the last saved choices. Switch to Sequential and back to Burst; confirm Custom `2` is retained but inactive during Sequential searches. Reset defaults and confirm Sequential and Automatic are selected.
4. Try `1`, `33`, a fractional value, and nonnumeric text in the custom count field. Confirm each is rejected before save without discarding other valid settings. Confirm old saved settings without the new fields load with Sequential and Automatic while retaining their existing choices.
5. Start a search of a larger directory and cancel it. Confirm pending discovery stops, already displayed matches follow existing cancellation behavior, and the final state is Cancelled rather than Completed.
6. Use a controlled unreadable descendant folder. Confirm accessible workbooks are still searched and the warnings UI identifies the failed path. A missing or unreadable root should still fail through the existing command error path.
7. Open `design/mainui/index.html` and confirm the standalone settings prototype offers the same mode/count choices, validation, save/reset behavior, and warning presentation.

## Run implementation quality gates

Include scheduler cases for a full frontier with local continuation, deep trees without recursive function calls, overlapping CLI roots, and worker failure without false completion. Verify outstanding accounting cannot finish while a child remains pending. Cancellation is cooperative between filesystem operations; a blocked OS filesystem call is outside the cancellation flag's control.

```powershell
npm test
npm run lint
npm run build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

Expected: all commands exit 0; the build, test, lint, Clippy, and format outputs contain no errors or warnings. Tests should cover nested and empty directories, exact-once file discovery, Automatic and custom count resolution, the 2 and 32 boundaries, actual visitor bounds, saved-setting migration, CLI option precedence and invalid counts, descendant errors, progress completion, and cancellation while a queue is full. No fixed elapsed-time speedup is required; record timings only as diagnostic observations.

## Observed implementation checks

Run date: 2026-10-01 (Windows x64).

- `npm test`: passed, 16 test files and 84 tests.
- `npm run lint`: passed.
- `npm run build`: passed.
- `cargo test --workspace`: passed, including the core nested discovery and visitor-bound tests, CLI parser/search/output tests, and Tauri invalid-count validation. The MSVC linker emitted its standard import-library creation diagnostic as a Rust `linker_messages` warning while creating the Tauri test library; the command exited successfully.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed after replacing the manual count bound with `clamp`.
- `cargo fmt --check`: passed.
- Focused frontend warning/settings tests: passed, 2 files and 7 tests.
- `cargo check -p xlseek`: passed after staging a temporary target-specific CLI sidecar for Tauri's build script.

The native desktop settings/search walkthrough, restart persistence, controlled unreadable-folder behavior, and interactive prototype review remain manual checks and were not run in this environment. The automated discovery tests verify nested result parity and observed bounded overlap; they do not simulate an operating-system permission denial or a blocked filesystem call.

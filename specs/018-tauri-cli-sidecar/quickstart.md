# Quickstart: Bundle xlseek-cli as a Tauri Sidecar

This guide verifies that each Tauri distribution contains an `xlseek-cli` built for the same target and that users can launch the CLI directly after installation.

## Prerequisites

- Node.js, npm, Rust/Cargo, and the Tauri build prerequisites for each target OS are installed.
- The Rust target triple and required linker/SDK are available for the target being built.
- Existing CLI usage follows [Specification 014](../../014-cli-search-export/spec.md) and [Specification 015](../../015-cli-short-options/spec.md).

## Build a Sidecar for the native target

```powershell
npm run build:cli
```

Expected result: Cargo builds `xlseek-cli` in release mode for the host target and places a target-suffixed file in Tauri's binaries directory. The build helper exits nonzero if it cannot find the artifact.

## Build a GUI installer

```powershell
npm run build:gui
```

To build for a different Rust target, pass the target to the Tauri CLI:

```powershell
npm run build:gui -- --target x86_64-pc-windows-msvc
```

Expected result: Tauri's before-build hook builds the CLI for the same target as the GUI. The Tauri CLI embeds it in the target package under its bundle directory (native builds default to `target/release/bundle/`). A Tauri build must not succeed if the matching Sidecar is missing or mismatched.

## Inspect the distribution

1. Open an installer or app bundle from the target-specific bundle directory and verify that it contains `xlseek-cli`.
2. Check the CLI's location in the installed layout for that OS and ensure the README describes the observed location.
3. Launch the executable directly from a terminal and check help output and exit status with `xlseek-cli --help`. Do not launch it from the GUI.
4. Use a test workbook and existing CLI search arguments to compare the output file, match count, and exit status with a separately built CLI.
5. Repeat for Windows x64, macOS Intel x64, macOS Apple Silicon arm64, and Linux x64 GNU. Follow existing signing and installation requirements for macOS/Windows.

## Existing quality gates

```powershell
npm run build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

Run build-helper unit tests with `npm run test:build-cli`. Even if all commands succeed, inspect actual distributions and verify post-install launch separately on each OS.

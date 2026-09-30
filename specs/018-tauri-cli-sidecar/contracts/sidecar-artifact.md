# Sidecar Distribution Contract

## Purpose

Defines the artifact contract between a Tauri desktop distribution and the `xlseek-cli` built from the same release. It does not define an API for launching the CLI from the Tauri application.

## Build Inputs

- The Tauri target triple used to build the GUI.
- The `xlseek-cli` Cargo package in the same repository checkout.
- The Tauri Sidecar base path `binaries/xlseek-cli`.

Use `TAURI_ENV_TARGET_TRIPLE` when Tauri's build hook provides it. For a standalone CLI build, use the host triple.

## Sidecar File

Place a file matching the Tauri configuration base path in `src-tauri/binaries/` using this naming convention:

- Windows: `xlseek-cli-<target-triple>.exe`
- macOS/Linux: `xlseek-cli-<target-triple>`

The Tauri configuration specifies only the base path, without a platform suffix or file extension.

## Distribution Contract

1. The Tauri bundler embeds the Sidecar that matches the target triple.
2. The installable distribution contains a launchable CLI executable.
3. Users can launch the installed CLI directly from a terminal without going through the Tauri GUI. `PATH` registration is not guaranteed.
4. `xlseek-cli --help` displays the same usage as the existing CLI and exits successfully.
5. Search, output, and existing exit statuses remain compatible with Specifications 014/015.
6. If the target triple or matching CLI file is missing, distribution creation must not report success.
7. The Sidecar is built from the same checkout and release as the GUI.

## Platform Verification

Inspect actual Windows x64, macOS Intel x64, macOS Apple Silicon arm64, and Linux x64 GNU bundles. Confirm that the CLI launches from the appropriate installed location in each bundle. Determine OS-specific locations from build artifacts and document them in the README. Do not add Sidecar spawning by the Tauri app, expand Shell capabilities, or change the OS `PATH`.

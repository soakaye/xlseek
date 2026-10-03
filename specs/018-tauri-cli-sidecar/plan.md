# Implementation Plan: Bundle xlseek-cli as a Tauri Sidecar

**Branch**: `018-tauri-cli-sidecar` | **Date**: 2026-09-30 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/018-tauri-cli-sidecar/spec.md`

## Summary

Bundle an `xlseek-cli` built from the same release into the desktop distribution so users can run it directly after installation. Declare it through Tauri's `bundle.externalBin`, build and stage an executable matching Tauri's target triple, and then create each distribution. Do not add Sidecar launching from the Tauri application or change the GUI.

## Technical Context

**Language/Version**: Rust 2021 edition, Node.js ES modules (same runtime as the existing `scripts/build-cli.js`)

**Primary Dependencies**: Existing Cargo package `xlseek-cli`, Tauri v2 bundler, and `@tauri-apps/cli` v2. No new runtime dependencies.

**Storage**: No persistent data is added. Target-specific CLI executables are generated during the build and temporarily staged in `src-tauri/binaries/`; `.gitignore` excludes the generated files from source control.

**Testing**: Unit tests for build helpers, existing CLI integration tests, inspection of Tauri distributions, and post-install execution of `xlseek-cli --help` and an existing search flow. Quality gates: `npm run build`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check`.

**Target Platform**: Windows x64, macOS Intel x64 / Apple Silicon arm64, and Linux x64 GNU, as documented for the existing project. Build macOS architectures separately; do not add a universal binary.

**Project Type**: Rust CLI and Tauri v2 desktop application workspace.

**Performance Goals**: Do not change runtime search performance. Provide a CLI built for the same target as each desktop distribution.

**Constraints**: Tauri `externalBin` requires an executable suffixed with the target triple appended to its configured base name. Windows files include `.exe`. Launching the CLI from Tauri, registering it on `PATH`, and changing CLI arguments or output are out of scope. Inspect actual packages on each OS to confirm their installed layout and direct launch behavior.

**Scale/Scope**: Build one matching `xlseek-cli` for each Tauri build target and include it in that distribution. Do not change GUI screens, IPC, or the search core.

## Constitution Check

*GATE: Evaluate before Phase 0 research and reevaluate after Phase 1 design.*

- **Principle I — Language**: These generated design artifacts are written in English, the constitution's default when the user has not explicitly requested another document language. Existing localized resources are not translated. Do not change the CLI's existing localization.
- **Principle II — Constants**: Centralize reusable fixed values in build helper code, such as the CLI package/binary names, stage location, and Tauri environment variable, in a dedicated build constants module. Add a constant-reference comment at each use site. Keep values required by Tauri/Cargo manifests in their declarative configuration.
- **Principle III — Comments**: Add comments to new or modified scripts, modules, and functions that explain behavior, arguments and return values, and error conditions. Do not include change history.
- **Principle IV — Responsibilities and standards**: Limit changes to build/distribution settings and their verification. Separate CLI build helpers, Tauri bundle declarations, and tests; preserve Rust and frontend quality gates.
- **Principle V — Errors and tests**: Do not treat an unknown target triple, Cargo failure, missing artifact, or staging failure as success. Add target-specific tests and run existing quality gates.
- **Security**: Do not grant the app permission to launch the Sidecar. Search remains local as in the existing CLI; do not add external transmission.

**Gate result (pre-design)**: PASS. No design conflict with the constitution or exception request is identified.

## Project Structure

### Documentation (this feature)

```text
specs/018-tauri-cli-sidecar/
├── plan.md
├── spec.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
│   └── requirements.md
└── contracts/
    └── sidecar-artifact.md
```

### Source Code (repository root)

```text
scripts/
├── build-cli.js                 # Build for the selected target and stage under the Sidecar name
├── build-cli-utils.js           # Pure target, executable name, and artifact path helpers
├── build-cli.test.js            # Triple, suffix, and artifact failure tests
└── build-constants.js           # Shared build-helper constants

package.json                    # Runs the built-in Node test runner via test:build-cli
src-tauri/
├── tauri.conf.json              # Declares the Sidecar base path through bundle.externalBin
└── binaries/                    # Build-generated files for each target
    └── xlseek-cli-<target-triple>[.exe]
README.md                       # Bundled CLI and direct launch instructions
README_ja.md                    # Japanese localization
.gitignore                      # Excludes generated files in src-tauri/binaries/
```

**Structure Decision**: Reuse the existing `scripts/build-cli.js` from Tauri's build hook and add target-aware build/staging. Put pure target/name/path functions in `scripts/build-cli-utils.js` so they can be tested without running Cargo. Use `src-tauri/binaries/` for generated artifacts and keep executables out of source control. Declare the Sidecar in the existing Tauri configuration. Do not change the frontend, Rust IPC, or capabilities.

## Phase 0: Research

Research findings and decisions are recorded in [research.md](./research.md). Key decisions: use Tauri `externalBin`; prefer `TAURI_ENV_TARGET_TRIPLE` to build a CLI for the same target; stage the filename required by the Tauri configuration in `src-tauri/binaries/`; and build macOS Intel and Apple Silicon targets separately.

## Phase 1: Design & Contracts

- [data-model.md](./data-model.md): Defines build inputs, the Sidecar executable, and desktop distribution artifact properties and lifecycle.
- [contracts/sidecar-artifact.md](./contracts/sidecar-artifact.md): Defines the artifact contract between the build helper and Tauri bundler, and between a distribution and CLI users.
- [quickstart.md](./quickstart.md): Describes local builds, package inspection, post-install launch checks, and CLI regression checks.
- No runtime API contract is added. The specification excludes launching the CLI from Tauri, so no Shell sidecar permission or GUI/IPC change is needed.

**Gate result (post-design)**: PASS. The design preserves existing CLI compatibility, local processing, least privilege, testability, and module responsibilities. It explicitly validates target-matched artifacts and failure when they are missing.

## Complexity Tracking

No constitution conflict or complexity exception is identified.

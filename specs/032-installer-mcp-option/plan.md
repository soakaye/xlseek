# Implementation Plan: Windows Installer Component Selection for xlseek-mcp and xlseek-cli

**Branch**: `032-installer-mcp-option` | **Date**: 2026-10-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/032-installer-mcp-option/spec.md`

## Summary

Enable optional selection of `xlseek-mcp` (MCP Server, default OFF) and `xlseek-cli` (CLI tool, default ON) during Windows desktop installation via custom NSIS (`.exe`) and WiX (`.msi`) templates. Extend Tauri Sidecar staging in `scripts/build-cli.js` to compile and stage both `xlseek-cli` and `xlseek-mcp` across all supported targets, bundle both binaries in desktop distributions (`bundle.externalBin`), and keep version manifests synchronized.

## Technical Context

**Language/Version**: Rust 2021 edition, Node.js (ES modules, v18+), NSIS script (Modern UI 2), WiX XML (v3.14).

**Primary Dependencies**: Tauri v2 (`@tauri-apps/cli`), existing workspace crates `xlseek-core`, `xlseek-cli`, `xlseek-mcp`, `src-tauri`.

**Storage**: Local file extraction in user/system install directories (`$LOCALAPPDATA\xlseek` or `Program Files\xlseek`).

**Testing**: `npm run test:build-cli`, `npm run test:sync-version`, `npm run build`, `npm test`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.

**Target Platform**: Windows 10/11 (x64), macOS (Apple Silicon / Intel), Linux (x86_64).

**Project Type**: Desktop application (Tauri v2 GUI) + CLI and MCP sidecars.

**Performance Goals**: Zero impact on GUI/CLI execution performance; instantaneous installer component toggle; < 1s build script staging.

**Constraints**:
- Constitution Principle I: Accurate English in comments, docs, commit messages.
- Constitution Principle II: Centralized constants with explicit references.
- Constitution Principle III: 3-part header comments on all functions/scripts. Synthetic dummy data in design artifacts.
- Constitution Principle IV & V: Zero clippy/lint warnings, robust error handling, full unit testing.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS — All specification and planning artifacts, source comments, and commit messages are written in accurate English. User communications provided in Japanese.
- **Principle II (No Hardcoded Constants)**: PASS — Binary names, package names, target regexes, and template paths are defined in centralized constants files (`scripts/build-constants.js`, `scripts/sync-version-constants.js`).
- **Principle III (Comprehensive Header Comments & Hygiene)**: PASS — All script functions and modules retain comprehensive 3-part headers. Data models use strictly synthetic dummy data.
- **Principle IV (Modular Design & Code Standards)**: PASS — Script logic is split into pure utils and CLI entry points. Conforms to ESLint, Clippy, and Rust formatting.
- **Principle V (Robust Error Handling & Testing)**: PASS — Script execution verifies artifact existence and exits nonzero on failure; unit tests verify triple resolution and path construction.

## Project Structure

### Documentation (this feature)

```text
specs/032-installer-mcp-option/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Technical research and decisions
├── data-model.md        # Entities and state transitions
├── quickstart.md        # Verification and testing guide
├── contracts/           # Installer and sidecar contracts
│   └── installer-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Tasks created by /speckit-tasks
```

### Source Code (repository root)

```text
scripts/
├── build-constants.js        # Add MCP constants and sidecar targets
├── build-cli-utils.js        # Parameterize executable/artifact derivation
├── build-cli.js              # Stage both xlseek-cli and xlseek-mcp
├── build-cli.test.js         # Tests for both sidecars
├── sync-version-constants.js # Add crates/mcp/Cargo.toml
└── sync-version.test.js      # Verify mcp version synchronization

src-tauri/
├── tauri.conf.json           # Add externalBin and windows template settings
└── windows/
    ├── nsis/
    │   └── installer.nsi     # NSIS template with MUI_PAGE_COMPONENTS
    └── wix/
        └── main.wxs          # WiX template with WixUI_FeatureTree

README.md                     # Document Windows installer options and MCP paths
README_ja.md                  # Japanese localization
```

**Structure Decision**: Place custom Windows installer templates under `src-tauri/windows/{nsis,wix}/`. Extend existing build scripts without adding new npm scripts to preserve CI compatibility with `publish-to-auto-release.yml`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |

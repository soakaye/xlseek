# Implementation Plan: Standalone Multi-Platform xlseek-mcp Binary Release

**Branch**: `036-release-mcp-binaries` | **Date**: 2026-10-25 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/036-release-mcp-binaries/spec.md`

## Summary

Update `.github/workflows/publish-to-auto-release.yml` to build and upload standalone, platform-native `xlseek-mcp` executable archives (`.tar.gz` for macOS/Linux, `.zip` for Windows) directly to GitHub Releases upon version tag push (`v*`). This is accomplished by adding a dedicated, decoupled matrix job (`publish-mcp-binaries`) that compiles `crates/mcp` across macOS Apple Silicon (`aarch64-apple-darwin`), Linux x86_64 (`x86_64-unknown-linux-gnu`), and Windows x86_64 (`x86_64-pc-windows-msvc`) and publishes the archives via `softprops/action-gh-release@v2`.

## Technical Context

**Language/Version**: Rust 2021 edition (Rust stable via `dtolnay/rust-toolchain@stable`)

**Primary Dependencies**: GitHub Actions runner environments (`macos-latest`, `ubuntu-24.04`, `windows-latest`), `softprops/action-gh-release@v2`, `cargo`

**Storage**: N/A (CI pipeline artifact generation and GitHub Release assets storage)

**Testing**: Local Cargo release build verification (`cargo build --release -p xlseek-mcp`), YAML syntax validation, and simulated archive packaging verification

**Target Platform**: GitHub Actions CI (`macos-latest`, `ubuntu-24.04`, `windows-latest`) targeting:
- macOS Apple Silicon: `aarch64-apple-darwin`
- Linux x86_64: `x86_64-unknown-linux-gnu`
- Windows x86_64: `x86_64-pc-windows-msvc`

**Project Type**: CI/CD Workflow configuration (`.github/workflows/publish-to-auto-release.yml`)

**Performance Goals**: Standalone MCP binary compilation and upload completes within standard CI runner timeouts (< 10 minutes per runner without GUI library build overhead)

**Constraints**:
- Zero desktop GUI/Tauri runtime dependencies required for MCP binary execution
- Executable permissions (`+x`) preserved in Unix `.tar.gz` archives
- Seamless coexistence with existing draft release creation from `publish-tauri`

**Scale/Scope**: 1 GitHub Actions workflow file (`.github/workflows/publish-to-auto-release.yml`), 3 matrix target architectures

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Language-Directed Quality**: All workflow comments and documentation are written in natural English; user communication is provided in Japanese as requested.
- **Principle II: No Hardcoded Constants**: Environment variables and matrix configurations cleanly parameterize binary names, targets, and archive extensions.
- **Principle III: Comprehensive Header Comments & Documentation Hygiene**: Workflow headers clearly describe purpose, inputs, outputs, and failure conditions. All generated documents (`data-model.md`, `quickstart.md`, `plan.md`) use strictly synthetic dummy data (e.g. `v1.0.0`, dummy paths).
- **Principle IV: Modular Design & Code Standards**: Dedicated, single-responsibility job `publish-mcp-binaries` decouples headless MCP binary distribution from the desktop GUI builder.
- **Principle V: Robust Error Handling & Testing**: `fail-fast: false` ensures non-interfering builds across runners; `softprops/action-gh-release@v2` ensures idempotent asset uploading.

*Status: PASS (Zero violations)*

## Project Structure

### Documentation (this feature)

```text
specs/036-release-mcp-binaries/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   └── workflow-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
.github/
└── workflows/
    └── publish-to-auto-release.yml  # Updated release workflow with publish-mcp-binaries job
crates/
└── mcp/                             # Existing MCP server crate compiled by the workflow
    ├── Cargo.toml
    └── src/
```

**Structure Decision**: Update `.github/workflows/publish-to-auto-release.yml` with the new `publish-mcp-binaries` matrix job while maintaining existing `publish-tauri` desktop bundler.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*Zero violations. No complexity tracking entries required.*

# Implementation Plan: Minimal Version Bump Files and Consolidated Display Copyright

**Branch**: `033-minimize-version-files` | **Date**: 2026-10-25 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/033-minimize-version-files/spec.md`

## Summary

This feature minimizes the files modified during semantic version bumps (`npm version`) and consolidates user-facing copyright notices.
Currently, a version bump modifies 11+ non-lock files, and `Cargo.lock` has frequently been omitted from tagged release commits.
By introducing Cargo workspace package inheritance (`version.workspace = true`), configuring Tauri v2 to reference `../package.json`, injecting `APP_VERSION` via Vite, generating display copyright dynamically (`Copyright © {year} {author}`), and ensuring `Cargo.lock` is updated and staged during `npm version`, the modified file set is reduced to just 2 primary manifests (`package.json`, root `Cargo.toml`) and their lock files (`package-lock.json`, `Cargo.lock`), with 100% guarantee that `Cargo.lock` is included in tagged release commits.

---

## Technical Context

**Language/Version**: TypeScript 5.5, Node.js 18+ (ES Modules), Rust 2021 edition (Rust 1.75+).

**Primary Dependencies**: Tauri v2 (`@tauri-apps/api`, `tauri-build`), React 18, Vite 6, `calamine`, `rayon`, `rust_xlsxwriter`.

**Storage**: Local configuration files (`package.json`, `Cargo.toml`, `Cargo.lock`, `src-tauri/tauri.conf.json`).

**Testing**: `node --test` for version synchronization scripts, `cargo test --workspace` for Rust workspace crates, `vitest` / React Testing Library for frontend components.

**Target Platform**: Desktop (macOS, Windows, Linux).

**Project Type**: Multi-crate Cargo workspace + Tauri v2 React desktop application.

**Performance Goals**: `npm version` lifecycle hook execution completes in under 3 seconds including `cargo check --workspace` and Git staging checks.

**Constraints**:
- Existing source code file header comments (`.rs`, `.ts`, `.tsx`, etc.) MUST NOT be modified or removed (user constraint & Constitution Principle III).
- All modified files must pass `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, and `npm run build` (`tsc --noEmit`).
- No hardcoded magic strings outside centralized constant modules (Constitution Principle II).

**Scale/Scope**: 4 Rust crates (`xlseek-core`, `xlseek-cli`, `xlseek-mcp`, `xlseek`), 1 Tauri configuration, 4 localization catalogs, and 1 frontend UI component.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Verification Criteria | Status | Notes |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English documentation, comments, commit messages. Multilingual labels preserved. | **PASS** | Design docs in English. Japanese/English labels preserved in catalogs. |
| **II. No Hardcoded Constants** | All constants extracted to `src/constants/index.ts` or `src-tauri/src/constants.rs`. | **PASS** | `APP_AUTHOR` centralized in `src/constants/index.ts`. No hardcoded strings in code. |
| **III. Header Comments & Hygiene** | 3-part header comments for all functions. Zero real user data in docs. | **PASS** | Synthetic data only in docs. Header comments preserved across existing files. |
| **IV. Modular Design & Standards** | Comply with `clippy`, `rustfmt`, and `tsc --noEmit`. SRP applied. | **PASS** | Synchronization logic cleanly separated into validator, updater, and runner. |
| **V. Error Handling & Testing** | Non-zero exit on failure. Automated test coverage in `sync-version.test.js`. | **PASS** | Explicit error aborts prevent partial or untracked version commits. |

---

## Project Structure

### Documentation (this feature)

```text
specs/033-minimize-version-files/
├── spec.md                  # Feature specification
├── plan.md                  # This file (/speckit-plan command output)
├── research.md              # Phase 0 architecture research and decisions
├── data-model.md            # Phase 1 data entities and sync transaction model
├── quickstart.md            # Phase 1 verification and quickstart guide
├── contracts/               # Phase 1 interface contracts
│   └── version-sync-contract.md
├── checklists/
│   └── requirements.md      # Specification quality checklist
└── tasks.md                 # Phase 2 task decomposition (/speckit-tasks output)
```

### Source Code (repository root)

```text
# Root configuration & scripts
Cargo.toml                           # Add [workspace.package] with canonical version
Cargo.lock                           # Maintained atomically in version commits
package.json                         # Primary entry point for npm version
scripts/
├── sync-version.js                  # Lean npm version lifecycle runner
├── sync-version-constants.js        # Updated constants: minimal target paths & messages
├── sync-version-utils.js            # Lean sync logic: Cargo.toml update + lock verification & git add
└── sync-version.test.js             # Unit tests verifying minimal sync and staging checks

# Rust member crates (inherit version from root workspace)
crates/
├── core/
│   ├── Cargo.toml                   # version.workspace = true
│   └── locales/
│       ├── en.yml                   # Removed about.APP_VERSION & about.COPYRIGHT
│       └── ja.yml                   # Removed about.APP_VERSION & about.COPYRIGHT
├── cli/
│   └── Cargo.toml                   # version.workspace = true
└── mcp/
    └── Cargo.toml                   # version.workspace = true

# Tauri desktop application
src-tauri/
├── Cargo.toml                       # version.workspace = true
├── tauri.conf.json                  # "version": "../package.json"
└── locales/
    ├── en.yml                       # Removed about.APP_VERSION & about.COPYRIGHT
    └── ja.yml                       # Removed about.APP_VERSION & about.COPYRIGHT

# Frontend (React & Vite)
vite.config.ts                       # define __APP_VERSION__ from package.json
src/
├── constants/
│   └── index.ts                     # APP_CONSTANTS.APP_VERSION & APP_AUTHOR
└── components/
    └── about/
        └── AboutDialog.tsx          # Dynamic copyright rendering & APP_CONSTANTS version
```

**Structure Decision**:
The existing monorepo structure is maintained without creating new directories. Changes are strictly confined to:
1. Root `Cargo.toml` and crate manifests (workspace inheritance).
2. `scripts/sync-version*` (slimming down targets, enforcing Cargo.lock staging).
3. Localization files (deleting redundant static version and copyright strings).
4. `vite.config.ts`, `src/constants/index.ts`, and `AboutDialog.tsx` (dynamic version and copyright injection).

---

## Complexity Tracking

*No constitutional violations identified. No complex abstractions or additional dependencies required.*

| Aspect | Selected Approach | Simpler Alternative Rejected Because |
| :--- | :--- | :--- |
| Version Inheritance | Cargo workspace package inheritance | Manual updates across 4 Cargo.toml files lead to version drift and 4x file diffs. |
| Desktop Version | `tauri.conf.json` `"version": "../package.json"` | Custom build scripts or regex rewrites add unnecessary failure points. |
| Frontend Version | Vite `define` + constant reference | Hardcoding version in TypeScript constants requires file churn on every version bump. |
| Display Copyright | Dynamic `Copyright © {year} {author}` | Static YAML strings require yearly commits across 4 files for purely cosmetic updates. |
| Lockfile Staging | Verify & `git add Cargo.lock` in lifecycle hook | Post-version tagging scripts create out-of-sync tags missing the updated lockfile. |

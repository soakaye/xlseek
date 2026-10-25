# Tasks: Windows Installer Component Selection for xlseek-mcp and xlseek-cli

**Input**: Design documents from `/specs/032-installer-mcp-option/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Unit tests for build scripts and version synchronization are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Centralize build constants and version target manifests

- [X] T001 [P] Define `MCP_CARGO_PACKAGE_NAME`, `MCP_BINARY_NAME`, `MCP_SIDECAR_BASE_PATH`, and `SIDECAR_TARGETS` in `scripts/build-constants.js`
- [X] T002 [P] Add `MCP_CARGO_TOML_PATH = "crates/mcp/Cargo.toml"` to `ALL_VERSION_TARGET_PATHS` in `scripts/sync-version-constants.js`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core build helper and configuration infrastructure required across all user stories

**⚠️ CRITICAL**: Must complete before user story installation templates can be finalized

- [X] T003 [P] Parameterize `getExecutableName`, `getCargoArtifactPath`, and `getSidecarPath` in `scripts/build-cli-utils.js` to accept `binaryName`
- [X] T004 Update `scripts/build-cli.js` to compile and stage both `xlseek-cli` and `xlseek-mcp` for the resolved target triple into `src-tauri/binaries/`
- [X] T005 [P] Update unit tests in `scripts/build-cli.test.js` to verify path generation and missing-artifact diagnostics for both `xlseek-cli` and `xlseek-mcp`
- [X] T006 [P] Update unit tests in `scripts/sync-version.test.js` to verify `crates/mcp/Cargo.toml` is included in version synchronization
- [X] T007 Update `src-tauri/tauri.conf.json` to include `"binaries/xlseek-mcp"` under `bundle.externalBin` and register custom Windows templates under `bundle.windows`

**Checkpoint**: Foundation ready — `npm run build:cli` builds and stages both sidecars.

---

## Phase 3: User Story 1 - Optional MCP Server Selection in Windows NSIS Installer (Priority: P1) 🎯 MVP

**Goal**: Deliver Windows NSIS setup wizard with component selection where `xlseek-cli` is enabled by default and `xlseek-mcp` is disabled by default

**Independent Test**: Build and run NSIS installer; verify component page shows locked Core, default-checked CLI, and default-unchecked MCP; verify only selected files are copied to `$INSTDIR`.

- [X] T008 [US1] Create baseline NSIS template at `src-tauri/windows/nsis/installer.nsi` based on Tauri v2 template and insert `!insertmacro MUI_PAGE_COMPONENTS` after `MUI_PAGE_DIRECTORY`
- [X] T009 [US1] Configure mandatory core application section `Section "Excel Seek (GUI)" SecCore` (`SectionIn RO`) and prefix internal sections with `-` (`-EarlyChecks`, `-WebView2`) in `src-tauri/windows/nsis/installer.nsi`
- [X] T010 [US1] Add `Section "xlseek-cli (CLI)" SecCli` (enabled by default) to copy `xlseek-cli.exe` into `$INSTDIR` in `src-tauri/windows/nsis/installer.nsi`
- [X] T011 [US1] Add `Section /o "xlseek-mcp (MCP Server)" SecMcp` (`/o`, disabled by default) to copy `xlseek-mcp.exe` into `$INSTDIR` in `src-tauri/windows/nsis/installer.nsi`
- [X] T012 [US1] Add bilingual component descriptions (`LangString` for `LANG_JAPANESE` and `LANG_ENGLISH`) via `MUI_FUNCTION_DESCRIPTION_BEGIN` in `src-tauri/windows/nsis/installer.nsi`
- [X] T013 [US1] Verify `Section Uninstall` in `src-tauri/windows/nsis/installer.nsi` cleans up both `xlseek-cli.exe` and `xlseek-mcp.exe` if present

**Checkpoint**: User Story 1 complete — NSIS installer offers interactive component selection with desired defaults.

---

## Phase 4: User Story 2 - Optional Feature Selection in Windows WiX (MSI) Installer (Priority: P2)

**Goal**: Deliver Windows WiX MSI installer with Feature Tree customization dialog supporting interactive and unattended selection

**Independent Test**: Run MSI installer in interactive mode to verify feature tree and directory selection in `CustomizeDlg`; verify `msiexec /i` with and without `McpFeature` installs expected files.

- [X] T014 [US2] Create baseline WiX template at `src-tauri/windows/wix/main.wxs` with `<UIRef Id="WixUI_FeatureTree" />` and route `WelcomeDlg` to `CustomizeDlg`
- [X] T015 [US2] Configure core feature `MainProgram` as `Level="1"`, `Absent="disallow"`, and `ConfigurableDirectory="INSTALLDIR"` in `src-tauri/windows/wix/main.wxs`
- [X] T016 [US2] Add `<Feature Id="CliFeature">` referencing `<ComponentRef Id="xlseek_cli.exe" />` with `Level="1"` (enabled by default) in `src-tauri/windows/wix/main.wxs`
- [X] T017 [US2] Add `<Feature Id="McpFeature">` referencing `<ComponentRef Id="xlseek_mcp.exe" />` with `Level="2"` (disabled by default) in `src-tauri/windows/wix/main.wxs`

**Checkpoint**: User Story 2 complete — WiX MSI installer matches NSIS feature defaults.

---

## Phase 5: User Story 3 - Multi-Platform Sidecar Bundling for xlseek-mcp (Priority: P3)

**Goal**: Ensure `xlseek-mcp` is bundled across all platforms and documented clearly for users

**Independent Test**: Inspect output of `npm run build:cli` on host system; verify README instructions describe Windows installer options and installed paths for all platforms.

- [X] T018 [US3] Verify `npm run build:cli` generates both target sidecars in `src-tauri/binaries/` and gracefully reports errors if a target build fails
- [X] T019 [US3] Update `README.md` and `README_ja.md` documenting Windows installer component options and installed locations for `xlseek-mcp`

**Checkpoint**: User Story 3 complete — multi-platform packaging and documentation aligned.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates, static analysis, and regression testing

- [X] T020 [P] Execute Node test suite `npm run test:build-cli` and `npm run test:sync-version`
- [X] T021 [P] Execute frontend build and test suite `npm run build` and `npm test`
- [X] T022 Execute Rust workspace checks `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`
- [X] T023 Run validation scenarios in `specs/032-installer-mcp-option/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies
- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) — blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2).
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2); can run in parallel with US1.
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2).
- **Polish (Phase 6)**: Runs after all user stories are implemented.

### Parallel Opportunities
- T001 and T002 in Phase 1 can run in parallel.
- T003, T005, and T006 in Phase 2 can run in parallel.
- T020 and T021 in Polish can run in parallel.

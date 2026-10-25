# Feature Specification: Windows Installer Component Selection for xlseek-mcp and xlseek-cli

**Feature Branch**: `032-installer-mcp-option`

**Created**: 2026-10-25

**Status**: Draft

**Input**: User description: "windows 版のインストール時のオプションにxlseek-mcpを選択を加えたい"

## Clarifications

### Session 2026-10-25

- Q: インストーラーで「xlseek-mcp」を選択した際の動作として、どれを想定されていますか？ → A: Option A (xlseek-mcp.exe のインストール有無を切り替える: 選択時のみインストール先に配置)
- Q: インストーラー画面での「xlseek-mcp」オプションの初期状態（デフォルト）はどうしますか？ → A: Option B (デフォルトでOFF: 未チェック・必要なユーザーのみ選択)
- Q: 既存の CLI（xlseek-cli）の扱いはどうしますか？ → A: Option B (xlseek-cli と xlseek-mcp の両方を個別に選択可能にする: xlseek-cli はデフォルトON、xlseek-mcp はデフォルトOFF)
- Q: 対象とするインストーラー形式の範囲を教えてください → A: Option A (NSIS (.exe インストーラー) と WiX (.msi) の両方に対応する。他OSにも同梱)

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Optional MCP Server Selection in Windows NSIS Installer (Priority: P1) 🎯 MVP

As a Windows user installing Excel Seek via the setup wizard (`.exe`),
I want to see a component selection screen where I can choose whether to install `xlseek-mcp` (MCP Server) and `xlseek-cli` (CLI), with `xlseek-mcp` unchecked by default and `xlseek-cli` checked by default,
So that I can opt into installing the MCP server only when I need AI assistant integration without cluttering my system with unused server binaries.

**Why this priority**:
This is the core requirement requested by the user. Providing an explicit opt-in choice for `xlseek-mcp` during Windows installation ensures lightweight desktop installations for regular users while giving developers and AI power users a single-click setup.

**Independent Test**:
Run the Windows NSIS installer (`xlseek_*_x64-setup.exe`). Advance to the component selection screen. Verify that "Excel Seek (GUI)" is mandatory and cannot be unselected, "xlseek-cli (CLI)" is checked by default, and "xlseek-mcp (MCP Server)" is unchecked by default. Complete installation without checking `xlseek-mcp` and verify that `xlseek-mcp.exe` is absent from the installation folder. Re-run installer with `xlseek-mcp` checked and verify `xlseek-mcp.exe` is installed and launchable.

**Acceptance Scenarios**:

1. **Given** a standard interactive Windows NSIS installation, **When** the user reaches the component selection screen, **Then** the main application is locked as required, `xlseek-cli` is enabled by default, and `xlseek-mcp` is disabled by default.
2. **Given** the component selection screen, **When** the user leaves `xlseek-mcp` unchecked and completes installation, **Then** `xlseek.exe` and `xlseek-cli.exe` are placed into `$INSTDIR`, and `xlseek-mcp.exe` is NOT copied.
3. **Given** the component selection screen, **When** the user checks `xlseek-mcp` and completes installation, **Then** `xlseek-mcp.exe` is placed into `$INSTDIR` alongside `xlseek.exe` and can be launched directly.
4. **Given** the component selection screen, **When** the user hovers over each component, **Then** a clear description is displayed explaining what each component is for (in Japanese or English depending on selected language).
5. **Given** an installed application where `xlseek-mcp.exe` was installed, **When** the user runs `uninstall.exe`, **Then** `xlseek-mcp.exe` is cleanly removed along with all other installed files.

---

### User Story 2 - Optional Feature Selection in Windows WiX (MSI) Installer (Priority: P2)

As a Windows enterprise administrator or user installing Excel Seek via Windows Installer (`.msi`),
I want to customize features in the setup wizard or via `msiexec` command-line properties,
So that I can control whether `xlseek-cli` and `xlseek-mcp` are installed in managed or unattended enterprise deployments.

**Why this priority**:
Corporate environments and managed fleets frequently rely on MSI packages. Aligning MSI feature trees with NSIS component options provides a consistent installation experience across both Windows distribution formats.

**Independent Test**:
Run the Windows MSI installer (`xlseek_*_x64_en-US.msi`) in interactive mode. Verify that the feature customization screen allows toggling `xlseek-cli` (default: installed) and `xlseek-mcp` (default: not installed). Run `msiexec /i xlseek.msi ADDLOCAL=MainProgram,McpFeature /qn` in silent mode and verify that `xlseek-mcp.exe` is deployed.

**Acceptance Scenarios**:

1. **Given** an interactive MSI installation, **When** the customization dialog appears, **Then** the user can choose the installation destination folder and toggle individual features (`MainProgram`, `CliFeature`, `McpFeature`).
2. **Given** default MSI installation options, **When** installation completes, **Then** `xlseek-cli.exe` is installed and `xlseek-mcp.exe` is omitted.
3. **Given** the user selects the `McpFeature` in the feature tree, **When** installation completes, **Then** `xlseek-mcp.exe` is installed to the application folder.
4. **Given** an unattended deployment via `msiexec`, **When** `ADDLOCAL` parameters include or exclude `McpFeature`, **Then** the Windows Installer service accurately respects the feature selection.

---

### User Story 3 - Multi-Platform Sidecar Bundling for xlseek-mcp (Priority: P3)

As a macOS or Linux desktop user,
I want `xlseek-mcp` bundled alongside `xlseek-cli` inside the desktop application distribution packages (`.dmg`, `.app`, `.deb`, `.AppImage`),
So that MCP server capabilities are available out-of-the-box across all supported operating systems.

**Why this priority**:
Excel Seek is cross-platform. While Windows provides interactive component checkboxes during installation, macOS and Linux distributions should bundle the binary sidecar so users on all platforms can configure their local MCP clients (Claude Desktop, Antigravity, Cursor) pointing to the bundled executable.

**Independent Test**:
Inspect built macOS `.app` bundle and Linux `.deb` / `.AppImage` packages. Confirm that `xlseek-mcp` is included alongside `xlseek-cli` within the application binaries folder.

**Acceptance Scenarios**:

1. **Given** a macOS build, **When** the app bundle is assembled, **Then** `xlseek-mcp` is included in `Contents/MacOS/xlseek-mcp` alongside `xlseek-cli`.
2. **Given** a Linux build, **When** Debian or AppImage packages are created, **Then** `xlseek-mcp` is included and accessible.
3. **Given** `npm run build:cli`, **When** executed for any valid target triple, **Then** both `xlseek-cli` and `xlseek-mcp` executables are built and staged into `src-tauri/binaries/`.

---

### Edge Cases

- **Existing installation upgrade**: When updating an existing installation where `xlseek-mcp.exe` was previously omitted, re-running the installer allows selecting `xlseek-mcp` to add it, and vice versa.
- **Uninstallation of partial components**: If `xlseek-mcp.exe` was never installed, uninstallation must not fail or report errors when attempting to delete non-existent files.
- **Silent/Passive installation on Windows**: When running silent installers (`/S` for NSIS or `/qn` for MSI), default component selections (`xlseek-cli` ON, `xlseek-mcp` OFF) MUST be applied unless explicitly overridden by command-line parameters.
- **Missing build artifact before packaging**: If either `xlseek-cli` or `xlseek-mcp` fails to build during `npm run build:cli`, packaging MUST immediately halt with a clear error and not generate an incomplete installer.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide component/feature selection options in both Windows NSIS (`.exe`) and WiX (`.msi`) installers.
- **FR-002**: The main desktop application (Excel Seek GUI) MUST be mandatory and cannot be deselected during installation.
- **FR-003**: The command-line tool `xlseek-cli` MUST be presented as an optional component with its default state set to enabled (checked / ON).
- **FR-004**: The Model Context Protocol server `xlseek-mcp` MUST be presented as an optional component with its default state set to disabled (unchecked / OFF).
- **FR-005**: If the user enables `xlseek-mcp`, the installer MUST deploy `xlseek-mcp.exe` into the application installation directory (`$INSTDIR` alongside `xlseek.exe`). If left disabled, `xlseek-mcp.exe` MUST NOT be copied.
- **FR-006**: Both NSIS and WiX installer components MUST provide clear bilingual (Japanese and English) descriptions explaining the purpose of each component on hover or selection.
- **FR-007**: Uninstaller scripts MUST cleanly remove `xlseek-mcp.exe` if present without raising errors if it was never installed.
- **FR-008**: Build helper scripts (`scripts/build-cli.js`) and Tauri configuration (`tauri.conf.json`) MUST build, stage, and bundle both `xlseek-cli` and `xlseek-mcp` for all supported platforms and targets.
- **FR-009**: Version synchronization scripts (`scripts/sync-version.js`) MUST include `crates/mcp/Cargo.toml` in version propagation to ensure version parity across GUI, CLI, and MCP binaries.

### Key Entities

- **Desktop Installer Package**: The distribution archive or setup executable (`.exe`, `.msi`, `.dmg`, `.deb`, `.AppImage`) containing the desktop GUI and bundled sidecars.
- **Install Component / Feature**: A selectable unit within the installer wizard (`CoreApplication`, `CliTool`, `McpServer`) determining whether target executables are extracted to disk.
- **Installed Layout**: The local filesystem structure within the user or machine installation path containing `xlseek.exe`, optionally `xlseek-cli.exe`, and optionally `xlseek-mcp.exe`.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of standard default installations on Windows via NSIS or MSI install `xlseek.exe` and `xlseek-cli.exe` while omitting `xlseek-mcp.exe`.
- **SC-002**: 100% of installations where `xlseek-mcp` is explicitly selected install a working, launchable `xlseek-mcp.exe` in the application directory.
- **SC-003**: Component selection dialog in NSIS and Feature Tree dialog in MSI accurately reflect component sizes and descriptions in both Japanese and English.
- **SC-004**: Build scripts (`npm run build:cli`) stage both `xlseek-cli` and `xlseek-mcp` with zero manual intervention across all supported platform targets.

---

## Assumptions

- Selecting `xlseek-mcp` in the installer only controls whether the binary is copied to the application directory; automatic registration in system `PATH` or modification of external AI client configuration files (`claude_desktop_config.json`) is out of scope for the installer wizard.
- On macOS and Linux, full component selection wizards do not exist in standard `.dmg` / `.deb` workflows; `xlseek-mcp` is bundled alongside `xlseek-cli` by default.
- Users who need `xlseek-mcp` on Windows can launch it directly from the installation path or configure their MCP client to point to `%LOCALAPPDATA%\xlseek\xlseek-mcp.exe` (or `%ProgramFiles%\xlseek\xlseek-mcp.exe`).

---

## Out of Scope

- Modifying the system `PATH` environment variable during installation.
- Automatically searching for or modifying third-party AI assistant configuration files (e.g. Claude Desktop, Cursor config).
- Adding background service / daemon registration for `xlseek-mcp` (the server runs on demand via stdio when invoked by an MCP client).

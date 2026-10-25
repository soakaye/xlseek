# Research Findings: Windows Installer Component Selection for xlseek-mcp and xlseek-cli

## Decision 1: Use Tauri `bundle.externalBin` for Multi-Platform Sidecar Bundling

- **Decision**: Declare both `binaries/xlseek-cli` and `binaries/xlseek-mcp` in `src-tauri/tauri.conf.json` under `bundle.externalBin`.
- **Rationale**: Declaring both binaries as external binaries enables Tauri's bundler to automatically bundle `xlseek-cli` and `xlseek-mcp` into macOS application bundles (`Contents/MacOS/`) and Linux packages (`/usr/bin/` or AppImage root). It also resolves source files for Windows installer compilation into the temporary packaging directory.
- **Alternatives considered**:
  - Packaging `xlseek-mcp` only on Windows via raw NSIS/WiX files without `externalBin`: Rejected because `xlseek-mcp` is useful on macOS and Linux as well, and using `externalBin` standardizes binary naming and staging conventions across the workspace.
- **Evidence**: [Tauri v2 Sidecar Documentation](https://v2.tauri.app/develop/sidecar/).

---

## Decision 2: Custom NSIS Template with `MUI_PAGE_COMPONENTS`

- **Decision**: Provide a custom NSIS template at `src-tauri/windows/nsis/installer.nsi` configured via `bundle.windows.nsis.template` in `tauri.conf.json`. Insert `!insertmacro MUI_PAGE_COMPONENTS` after `MUI_PAGE_DIRECTORY`. Split the installation logic into three sections:
  1. `Section "Excel Seek (GUI)" SecCore` (`SectionIn RO`): Mandatory core application, shortcuts, uninstaller.
  2. `Section "xlseek-cli (CLI)" SecCli`: Enabled by default; copies `xlseek-cli.exe`.
  3. `Section /o "xlseek-mcp (MCP Server)" SecMcp`: Disabled by default (`/o`); copies `xlseek-mcp.exe`.
  Prefix internal sections (`Section "-EarlyChecks"`, `Section "-WebView2"`) with `-` so they remain hidden from the components dialog.
- **Rationale**: Tauri's default `installer.nsi` template unconditionally copies all `{{#each binaries}}` inside `Section Install` and does not include `MUI_PAGE_COMPONENTS`. NSIS `installerHooks` cannot reorder pages or split the unconditional binaries loop without overriding the template. By deriving `src-tauri/windows/nsis/installer.nsi` directly from Tauri v2's upstream template, we achieve exact page ordering, accurate disk space calculations, and bilingual descriptions (`LangString`).
- **Alternatives considered**:
  - Modifying files post-install via `installerHooks`: If `xlseek-mcp` is bundled unconditionally in `Section Install` and then deleted in `NSIS_HOOK_POSTINSTALL`, the user experience is sluggish and disk usage is incorrectly calculated.
  - Custom NSIS nsDialogs page: Reinvents standard NSIS component selection while lacking standard accessibility and translation strings.
- **Evidence**: NSIS Modern UI 2 Component Page Specification (`MUI_PAGE_COMPONENTS`, `SectionIn RO`, `Section /o`).

---

## Decision 3: Custom WiX Template with `WixUI_FeatureTree`

- **Decision**: Provide a custom WiX template at `src-tauri/windows/wix/main.wxs` configured via `bundle.windows.wix.template` in `tauri.conf.json`. Replace `<UIRef Id="WixUI_InstallDir" />` with `<UIRef Id="WixUI_FeatureTree" />`, link `WelcomeDlg` to `CustomizeDlg`, and configure features:
  1. `MainProgram` (`Title="Excel Seek (GUI)"`, `Level="1"`, `Absent="disallow"`, `ConfigurableDirectory="INSTALLDIR"`): Core GUI application.
  2. `CliFeature` (`Title="xlseek-cli (CLI)"`, `Level="1"`, `Absent="allow"`): References `<ComponentRef Id="xlseek_cli.exe" />` (installed by default).
  3. `McpFeature` (`Title="xlseek-mcp (MCP Server)"`, `Level="2"`, `Absent="allow"`): References `<ComponentRef Id="xlseek_mcp.exe" />` (omitted by default since `Level="2"` exceeds `INSTALLLEVEL="1"`).
- **Rationale**: Standard WiX installers use `WixUI_FeatureTree` to present the interactive component customization screen (`CustomizeDlg`) where users can select/deselect features and configure the installation folder. `Level="1"` installs by default, while `Level="2"` is opt-in, fully adhering to user requirements.
- **Alternatives considered**:
  - WiX Fragments alone without template override: Tauri v2's default `main.wxs` wraps all external binaries into the main feature and uses `WixUI_InstallDir`, which never presents `CustomizeDlg`. Overriding `main.wxs` is necessary to alter the UI sequence and feature hierarchy.
- **Evidence**: WiX Toolset standard UI dialog sets (`WixUI_FeatureTree`, `CustomizeDlg`).

---

## Decision 4: Extend Build and Version Synchronization Scripts

- **Decision**: Update `scripts/build-constants.js`, `scripts/build-cli-utils.js`, `scripts/build-cli.js`, and `scripts/build-cli.test.js` to handle both `xlseek-cli` and `xlseek-mcp`. Update `scripts/sync-version-constants.js` to include `crates/mcp/Cargo.toml`.
- **Rationale**: `beforeBuildCommand` in `tauri.conf.json` invokes `npm run build:cli`. Compiling and staging both binaries in `src-tauri/binaries/` ensures that `tauri build` and GitHub Actions CI (`publish-to-auto-release.yml`) execute smoothly without modifying workflow files.
- **Alternatives considered**:
  - Separate `npm run build:mcp` command: Would require updating `package.json`, `tauri.conf.json`, and CI steps, increasing coupling and maintenance overhead.
- **Evidence**: Existing `scripts/build-cli.js` modular helper design.

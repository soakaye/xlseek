# Installer and Sidecar Distribution Contract

## 1. Sidecar Staging Contract

### Build Hook Protocol
- Command: `npm run build:cli`
- Environment Variables:
  - `TAURI_ENV_TARGET_TRIPLE` (optional): Target triple passed by Tauri bundler. Defaults to host triple if unset or empty.
- Input:
  - Cargo packages: `crates/cli` (`xlseek-cli`) and `crates/mcp` (`xlseek-mcp`).
- Staged Artifact Outputs:
  - `src-tauri/binaries/xlseek-cli-<target-triple>[.exe]`
  - `src-tauri/binaries/xlseek-mcp-<target-triple>[.exe]`
- Failure Modes:
  - If either compilation fails or an expected artifact is not found, process exits with code `1`.

---

## 2. Windows NSIS Installer Contract

### Wizard Sequence
1. `Welcome Page`
2. `Reinstall Page` (if previous installation detected)
3. `Directory Page` (`$INSTDIR` selection)
4. `Components Page` (`MUI_PAGE_COMPONENTS`):
   - `Section "Excel Seek (GUI)" SecCore`: `SectionIn RO` (Mandatory, installs `xlseek.exe`, uninstaller, shortcuts).
   - `Section "xlseek-cli (CLI)" SecCli`: Standard section (Default ON, installs `xlseek-cli.exe`).
   - `Section /o "xlseek-mcp (MCP Server)" SecMcp`: Optional section (`/o`, Default OFF, installs `xlseek-mcp.exe`).
5. `Start Menu Page`
6. `Install Files Page`
7. `Finish Page`

### Uninstaller Contract
- `uninstall.exe` removes `xlseek.exe`, `xlseek-cli.exe` (if present), and `xlseek-mcp.exe` (if present) without throwing errors on missing optional files.

---

## 3. Windows WiX (MSI) Installer Contract

### Feature Hierarchy
- `<Feature Id="MainProgram" Title="Excel Seek (GUI)" Level="1" Absent="disallow" ConfigurableDirectory="INSTALLDIR">`:
  - Contains core app binaries, shortcuts, and registry entries.
- `<Feature Id="CliFeature" Title="xlseek-cli (CLI)" Level="1" Absent="allow" AllowAdvertise="no">`:
  - Contains `<ComponentRef Id="xlseek_cli.exe" />`.
  - Installed by default (`Level="1"` <= default `INSTALLLEVEL="1"`).
- `<Feature Id="McpFeature" Title="xlseek-mcp (MCP Server)" Level="2" Absent="allow" AllowAdvertise="no">`:
  - Contains `<ComponentRef Id="xlseek_mcp.exe" />`.
  - Not installed by default (`Level="2"` > default `INSTALLLEVEL="1"`).

### Silent Execution Contract
- Default: `msiexec /i xlseek.msi /qn` (installs `MainProgram` and `CliFeature`).
- Full install: `msiexec /i xlseek.msi ADDLOCAL=MainProgram,CliFeature,McpFeature /qn` (installs all components including `McpFeature`).

# Quickstart: Windows Installer Component Selection for xlseek-mcp and xlseek-cli

## 1. Build and Stage Sidecars

Run the build script locally:
```bash
npm run build:cli
```

Expected result:
- Cargo compiles both `xlseek-cli` and `xlseek-mcp` in release mode.
- Staged executables exist in `src-tauri/binaries/`:
  - `xlseek-cli-<target-triple>`
  - `xlseek-mcp-<target-triple>`

Run the build-helper tests:
```bash
npm run test:build-cli
npm run test:sync-version
```

---

## 2. Windows Installer Package Verification

### NSIS Installer (`.exe`)
1. Run the NSIS installer: `target/release/bundle/nsis/xlseek_*_x64-setup.exe`.
2. Progress to the **Components** selection page.
3. Verify:
   - "Excel Seek (GUI)" is grayed out / checked (mandatory).
   - "xlseek-cli (CLI)" is checked by default.
   - "xlseek-mcp (MCP Server)" is unchecked by default.
4. Complete installation with default settings:
   - Verify `$LOCALAPPDATA\xlseek\xlseek-cli.exe` exists.
   - Verify `$LOCALAPPDATA\xlseek\xlseek-mcp.exe` is absent.
5. Re-run installer with "xlseek-mcp" checked:
   - Verify `$LOCALAPPDATA\xlseek\xlseek-mcp.exe` is installed and executes `xlseek-mcp --help` successfully.
6. Run `uninstall.exe`:
   - Verify all executables are cleanly deleted.

### WiX Installer (`.msi`)
1. Run `msiexec /i xlseek_*_x64_en-US.msi`.
2. On the **Customize** dialog (`CustomizeDlg`), verify:
   - "xlseek-mcp (MCP Server)" has a red "X" (will not be installed) by default.
   - "xlseek-cli (CLI)" is marked for local installation by default.
3. Complete installation and verify binary presence on disk.

---

## 3. CI / Full Workspace Verification

```bash
npm run build
npm test
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

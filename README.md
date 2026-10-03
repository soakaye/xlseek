# Excel Seek (`xlseek`)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**English** | [日本語](README_ja.md)

**Excel Seek (`xlseek`)** is a fast, lightweight, and secure desktop search application and CLI tool for recursive searching across Excel workbooks (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`). It enables users to search cell values, formulas, notes, and shape text using plain text queries or regular expressions.

---

## Key Features

- **Multi-Format Support**: Recursively searches `.xlsx`, `.xlsm`, `.xlsb`, and `.xls` files within designated directories.
- **Flexible Search Options**:
  - Case-sensitive / Case-insensitive matching.
  - Regular expression (regex) search.
  - Search across cell values, formulas, comments/notes, and shape text.
  - Include or exclude hidden worksheets.
- **Fast & Parallelized**: Powered by [calamine](https://github.com/tafia/calamine) for high-performance spreadsheet parsing and [rayon](https://github.com/rayon-rs/rayon) for multi-threaded parallel execution.
- **Rich Preview & Direct Opening**: Inspect matched cell values, formulas, and surrounding cells directly in the preview grid, or open the original file with your system's default application.
- **Search History & Autocomplete**: Remembers recent search queries and target directory paths with autocompletion.
- **Export Capabilities**: Export search results directly to CSV or formatted Excel (`.xlsx`) files.
- **Headless CLI Included**: Run batch searches and export results without launching the GUI via `xlseek-cli`.
- **Bilingual Interface**: Seamless switching between Japanese and English UI, configurable default search options, and history limits.
- **Cross-Platform Desktop & CLI**: Full native support for Windows, macOS, and Linux, including OS-specific file manager integration (Explorer, Finder, DBus ShowItems / xdg-open) and external spreadsheet application launching.
- **Transparent Open Source**: View application details, version information, and third-party OSS licenses in the About dialog.

> **Default Search Behavior**: Searching cell formulas is enabled by default. Shape text and hidden sheets are excluded by default. All 4 file extensions (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`) are enabled by default.

---

## Tech Stack

| Area | Technologies |
| :--- | :--- |
| **Desktop Framework** | [Tauri v2](https://tauri.app/) |
| **Frontend** | React 18, TypeScript, Vite, Tailwind CSS, Lucide React |
| **Backend & Engine** | Rust 2021 edition |
| **Excel Parser** | `calamine` |
| **Concurrency** | `rayon` |
| **Regular Expressions**| `regex` |
| **Export Formats** | `csv`, `rust_xlsxwriter` |

---

## Supported Platforms

Excel Seek provides first-class support for Windows, macOS, and Linux with native OS integration.

| Platform | Supported Versions / Architectures | Package Formats | File Manager Integration | Spreadsheet App Detection |
| :--- | :--- | :--- | :--- | :--- |
| **Windows** | Windows 10, 11 (x64) | NSIS Installer (`.exe`), WiX (`.msi`) | File Explorer (`explorer.exe /select,<path>`) | Microsoft Excel, LibreOffice Calc, WPS Office (via Registry) |
| **macOS** | macOS 11.0+ (Apple Silicon) | Apple Disk Image (`.dmg`), App bundle (`.app`) | Finder (`open -R <path>`) | Microsoft Excel, Apple Numbers, LibreOffice Calc (via `/Applications`) |
| **Linux** | Ubuntu, Debian, Fedora, Arch Linux (x86_64) | Debian package (`.deb`), AppImage (`.AppImage`) | DBus `ShowItems` (Nautilus, Dolphin, Nemo) / `gio` / `xdg-open` / WSL `explorer.exe` | System default spreadsheet handler |

### Platform Highlights

- **Windows**:
  - Automatically queries the Windows Registry (`OpenWithProgids` / `UserChoice`) to discover installed spreadsheet applications (Excel, Calc, etc.).
  - "Open in Folder" launches File Explorer highlighting the selected file (`explorer.exe /select,`).
  - Native "Open With" dialog invoked via `rundll32.exe shell32.dll,OpenAs_RunDLL`.
  - Requires Microsoft Edge WebView2 (preinstalled on Windows 10 version 1803+ and Windows 11).
- **macOS**:
  - Native Cocoa/WebKit rendering optimized for Apple Silicon (`aarch64-apple-darwin`).
  - Scans `/Applications` to detect Microsoft Excel, Apple Numbers, and LibreOffice Calc.
  - "Open in Folder" reveals and selects the target file in Finder via `open -R`.
  - "Open With" invokes a native application picker pointing to `/Applications`.
- **Linux**:
  - Built with WebKitGTK and GTK 3 for broad Linux desktop environment compatibility (GNOME, KDE Plasma, XFCE, Cinnamon, etc.).
  - "Open in Folder" uses FreeDesktop DBus `org.freedesktop.FileManager1.ShowItems` to highlight files in GNOME Files (Nautilus), KDE Dolphin, Nemo, and PCManFM-Qt, with fallbacks to `gio open` and `xdg-open`.
  - **WSL (Windows Subsystem for Linux)**: Automatically detects WSL sessions, converts Linux paths with `wslpath`, and opens Windows File Explorer seamlessly.

---

## Prerequisites & Development

### Requirements

- **Node.js & npm** (v18+ LTS recommended)
- **Rust stable & Cargo** (2021 edition)
- **Platform-specific build dependencies**:
  - **Windows**:
    - [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with the "Desktop development with C++" workload
    - [Microsoft Edge WebView2 runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (preinstalled on Windows 10/11)
  - **macOS**:
    - Xcode Command Line Tools (`xcode-select --install`)
  - **Linux (Debian / Ubuntu)**:
    - System packages:
      ```bash
      sudo apt-get update && sudo apt-get install -y \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libwebkit2gtk-4.1-dev
      ```
      *(For distributions utilizing WebKitGTK 4.0, install `libwebkit2gtk-4.0-dev` instead)*

### Getting Started

```bash
# Clone the repository
git clone https://github.com/soakaye/xlseek.git
cd xlseek

# Install frontend dependencies
npm install

# Start development server with Tauri desktop window
npm run tauri dev
```

---

## Command Line Interface (CLI)

Tauri installers include `xlseek-cli` as a Sidecar. You can run the CLI directly from a terminal without launching the GUI or installing the CLI separately. For development, build a standalone CLI executable with Cargo:

```bash
# Build the standalone CLI binary for the host platform
cargo build --release -p xlseek-cli --bin xlseek-cli

# Display help and available options
./target/release/xlseek-cli --help

# Example: Search and export to XLSX
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format xlsx --output ./results.xlsx

# Example: Search and export to CSV with English headings
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format csv --output ./results.csv --language en
```

On Windows, the standalone executable built by this Cargo command is `target\release\xlseek-cli.exe`. Packaging builds the target-specific CLI and stages it for Tauri automatically.

### Run the CLI from an installed package

The CLI executable `xlseek-cli` is bundled with desktop installer packages as a Tauri Sidecar. Because the Sidecar is not registered on system `PATH` by default (except for standard Linux system package directories), invoke it using its full path or create a shell alias or symlink.

#### Windows

The Windows x64 NSIS and MSI packages install `xlseek-cli.exe` alongside `xlseek.exe`:

| Installer | Default CLI location |
| :--- | :--- |
| NSIS (current user) | `%LOCALAPPDATA%\xlseek\xlseek-cli.exe` |
| NSIS (all users) | `%ProgramFiles%\xlseek\xlseek-cli.exe` |
| MSI | `%ProgramFiles%\xlseek\xlseek-cli.exe` |

Run it from PowerShell using its full path:

```powershell
# Display help and options
& "$env:LOCALAPPDATA\xlseek\xlseek-cli.exe" --help

# Search and export to CSV
& "$env:LOCALAPPDATA\xlseek\xlseek-cli.exe" --path .\reports --query 'Revenue' --format csv --output .\results.csv
```

#### macOS

On macOS, `xlseek-cli` is located inside the application bundle:

| Package Format | Default CLI location |
| :--- | :--- |
| DMG / Installed App | `/Applications/xlseek.app/Contents/MacOS/xlseek-cli` |

Run it from Terminal:

```bash
# Display help and options
"/Applications/xlseek.app/Contents/MacOS/xlseek-cli" --help

# Search and export to XLSX
"/Applications/xlseek.app/Contents/MacOS/xlseek-cli" --path ./reports --query 'Revenue' \
  --format xlsx --output ./results.xlsx
```

*Tip*: You can create a symlink in `/usr/local/bin` to run it directly:
```bash
sudo ln -sf "/Applications/xlseek.app/Contents/MacOS/xlseek-cli" /usr/local/bin/xlseek-cli
```

#### Linux

On Linux, the CLI location depends on the package format:

| Package Format | CLI Location / Usage |
| :--- | :--- |
| Debian package (`.deb`) | `/usr/bin/xlseek-cli` (installed to system `PATH`) |
| AppImage (`.AppImage`) | Extracted via `./xlseek_*.AppImage --appimage-extract` (`squashfs-root/usr/bin/xlseek-cli`) |

Run it from Terminal:

```bash
# When installed via Debian package (.deb)
xlseek-cli --help
xlseek-cli --path ./reports --query 'Revenue' --format csv --output ./results.csv

# When running from extracted AppImage
./squashfs-root/usr/bin/xlseek-cli --path ./reports --query 'Revenue' --format csv --output ./results.csv
```

### Directory Search Modes

Desktop Settings lets you choose Sequential or Burst directory discovery. Sequential is the default. Burst uses an Automatic bounded worker count by default, or a custom count from 2 through 32. The selected defaults are saved for later searches. A search that cannot read a descendant folder continues with accessible folders and reports the failed path; a root directory error remains fatal. See the [desktop search mode contract](specs/019-burst-directory-search/contracts/desktop-search-mode.md).

### CLI Notes & Options

- **Mandatory Arguments**: `--path`, `--query`, `--format <csv|xlsx>`, and `--output <path>`.
- **Directory Discovery**: Recursive search defaults to Sequential mode. Use `--directory-mode burst` to visit subdirectories concurrently, or `--burst-workers <2..32>` to select a custom Burst limit (the worker-count option implies Burst). Burst Automatic chooses a bounded count based on available CPU parallelism. These options affect directory traversal only; single-file searches remain valid. See [directory search option contract](specs/019-burst-directory-search/contracts/cli-directory-mode.md) and [settings contract](specs/019-burst-directory-search/contracts/settings.md).
- **Search Scope**: By default, searches cell values, formulas, comments/notes, and shapes, while excluding hidden sheets.
- **Comment Support**: Extracts classic comments/notes from `.xlsx` and `.xlsm` files (threaded comments and `.xls` / `.xlsb` notes are currently unsupported).
- **Safety**: Existing output files will not be overwritten unless explicitly allowed with `--overwrite` (returns exit code `2`). Attempting to output to an input file path or hard link is strictly rejected.
- **Exit Codes**:
  - `0`: Success (including 0 matches found).
  - `1`: Partial failure (e.g., specific files could not be read or parsed).
  - `2`: Request error, fatal processing error, or file export failure.

---

## Packaging & Building

```bash
# Full build of frontend and CLI binary
npm run build:all

# Package the desktop application and installers with the CLI Sidecar
npm run build:gui
```

For native builds, application bundles and installers are generated under `target/release/bundle/`:
- **Windows**: `target/release/bundle/nsis/` (`.exe`) and `target/release/bundle/msi/` (`.msi`)
- **macOS**: `target/release/bundle/dmg/` (`.dmg`) and `target/release/bundle/macos/` (`.app`)
- **Linux**: `target/release/bundle/deb/` (`.deb`) and `target/release/bundle/appimage/` (`.AppImage`)

The build script automatically executes `npm run build:cli` to compile and stage the matching target CLI binary for Tauri Sidecar bundling. Cross-compilation or specific target builds can be initiated with the `--target` flag:

```bash
# Windows x64
npm run build:gui -- --target x86_64-pc-windows-msvc

# macOS Apple Silicon
npm run build:gui -- --target aarch64-apple-darwin

# Linux x86_64
npm run build:gui -- --target x86_64-unknown-linux-gnu
```

---

## Code Quality & Verification

To run tests and code linters across the frontend and backend:

```bash
# Frontend typecheck, tests, and linting
npm run build
npm test
npm run lint

# Rust backend tests, clippy, and code formatting
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

---

## Project Structure

```text
xlseek/
├── crates/
│   ├── core/           # xlseek-core: shared engine, parsing, i18n, models, and export logic
│   └── cli/            # xlseek-cli: standalone command-line executable
├── src/                # React / TypeScript frontend
│   ├── components/     # UI components (search, results, preview, about, dialogs)
│   ├── hooks/          # Custom hooks for search, history, and internationalization
│   ├── constants/      # Unified frontend constants and OSS license catalogs
│   └── types/          # TypeScript domain type definitions
├── src-tauri/          # Tauri v2 desktop application wrapper (xlseek)
│   ├── capabilities/   # Tauri v2 security capabilities configuration
│   └── src/            # IPC commands, Tauri application lifecycle, and menu handlers
├── specs/              # Specification documentation, architecture, and task plans
└── design/             # UI mockups and standalone HTML prototypes
```

---

## License

This project is licensed under the [MIT License](LICENSE).

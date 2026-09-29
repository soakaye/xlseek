# Excel Grep (`xlseek`)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Excel Grep (`xlseek`)** is a fast, lightweight, and secure desktop search application and CLI tool for recursive searching across Excel workbooks (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`). It enables users to search cell values, formulas, notes, and shape text using plain text queries or regular expressions.

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

## Prerequisites & Development

### Requirements

- **Node.js & npm** (LTS recommended)
- **Rust stable & Cargo**
- Platform-specific Tauri build tools and libraries (see [Tauri Prerequisites](https://tauri.app/start/prerequisites/))

### Getting Started

```bash
# Clone the repository
git clone https://github.com/soakaye/exgrep.git
cd exgrep

# Install frontend dependencies
npm install

# Start development server with Tauri desktop window
npm run tauri dev
```

---

## Command Line Interface (CLI)

The CLI tool (`xlseek-cli`) is distributed as an independent standalone binary alongside the GUI application. You can build and run it directly:

```bash
# Build the CLI binary
npm run build:cli

# Display help and available options
./target/release/xlseek-cli --help

# Example: Search and export to XLSX
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format xlsx --output ./results.xlsx

# Example: Search and export to CSV with English headings
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format csv --output ./results.csv --language en
```

### CLI Notes & Options

- **Mandatory Arguments**: `--path`, `--query`, `--format <csv|xlsx>`, and `--output <path>`.
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

# Package the desktop application & installers (bundles both GUI and CLI automatically)
npm run tauri build
```

The resulting application bundles and platform installers (e.g., Windows `.msi` / `.exe`, macOS `.dmg` / `.app`, Linux `.deb` / `.AppImage`) will be generated under `target/release/bundle/`. Both the desktop application (`xlseek`) and CLI tool (`xlseek-cli`) are packaged together.

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
exgrep/
├── crates/
│   ├── core/           # exlgrep-core: shared engine, parsing, i18n, models, and export logic
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

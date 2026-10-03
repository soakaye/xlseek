# Interface Contract: xlseek Crate and Package Identity

**Feature**: `021-rename-to-xlseek` | **Date**: 2026-10-02  
**Spec Reference**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md)

## 1. Rust Workspace Crates Contract

### 1.1 Crate Definition Contract (`crates/core/Cargo.toml`)
```toml
[package]
name = "xlseek-core"
version = "0.1.0"
edition = "2021"

[lib]
name = "xlseek_core"
path = "src/lib.rs"
```

### 1.2 Downstream Crate Dependencies
```toml
# In crates/cli/Cargo.toml
[dependencies]
xlseek-core = { path = "../core" }

# In src-tauri/Cargo.toml
[dependencies]
xlseek-core = { path = "../crates/core" }
```

### 1.3 Rust Import Contract
Downstream packages and test suites MUST import items using `xlseek_core`:
```rust
use xlseek_core::models::{DirectorySearchMode, ExportFormat, SearchQuery};
use xlseek_core::search::discovery::discover_files;
use xlseek_core::constants::*;
```

---

## 2. Storage & Configuration Key Contract

All persistence keys exposed to web storage or settings loaders MUST follow the canonical pattern `xlseek.<domain>`:

| Key | Format | Validation / Type |
|---|---|---|
| `xlseek.language` | string | `"default" \| "ja" \| "en"` |
| `xlseek.searchHistory` | JSON array string | `string[]` |
| `xlseek.default_search_options` | JSON object string | `SearchOptions` |

### Backward-Compatibility Fallback Contract:
When accessing a key:
1. `localStorage.getItem("xlseek.<key>")`
2. If `null`: `localStorage.getItem("exlgrep.<key>")` (or mapped legacy equivalent).
3. If non-null legacy value found: set `localStorage.setItem("xlseek.<key>", value)`.

# Quickstart: Validating the xlseek Rename

**Feature**: `021-rename-to-xlseek` | **Date**: 2026-10-02  
**Spec Reference**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md)

## 1. Prerequisites
- Node.js & npm installed
- Rust stable toolchain & Cargo installed

---

## 2. Validation Steps

### Step 1: Verify Zero Legacy Name Occurrences
Run grep across active source directories to verify no `exlgrep` or `exgrep` remain in active code or docs (excluding historical `specs/001-020`):

```bash
git grep -i -E "exlgrep|exgrep" -- :!specs
```

**Expected Outcome**: 0 lines matched.

---

### Step 2: Verify Rust Workspace Compilation and Tests
Verify that all crates compile and pass tests under the new crate name `xlseek-core`:

```bash
# Test the renamed core library
cargo test -p xlseek-core

# Test the CLI tool
cargo test -p xlseek-cli

# Test the GUI desktop crate
cargo test -p xlseek

# Run full workspace test suite
cargo test --workspace
```

**Expected Outcome**: All tests compile and pass with zero failures.

---

### Step 3: Verify Formatting and Clippy
Verify strict adherence to Constitution Principle IV:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

**Expected Outcome**: Zero formatting issues and zero Clippy warnings.

---

### Step 4: Verify Frontend Build and Tests
Check that TypeScript types and Vite bundling succeed:

```bash
npm run build
npm test
```

**Expected Outcome**: Clean build with zero TypeScript or bundle errors.

---

### Step 5: Verify License Generator Script
Run the license generator script to confirm it processes the renamed packages properly:

```bash
python3 scripts/generate-licenses.py
```

**Expected Outcome**: Generates license catalog containing `xlseek` and `xlseek-cli` without error.

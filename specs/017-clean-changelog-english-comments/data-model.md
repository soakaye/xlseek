# Data Model: Clean Changelog Headers and English Source Comments

**Feature**: `017-clean-changelog-english-comments`
**Date**: 2026-09-30

## Entities & Comment Structures

This feature focuses on code comments and docstrings across the codebase. Below are the canonical conceptual entities and structural schemas for documentation comments.

### 1. Header Documentation Structure (File / Module / Struct / Function)

Each documented code element adheres to a 3-element schema (Constitution v3.0.0 Principle III):

```text
┌─────────────────────────────────────────────────────────────┐
│ 1. Purpose / Description                                   │
│    - Detailed explanation of what the module/function does  │
├─────────────────────────────────────────────────────────────┤
│ 2. Parameters & Returns                                     │
│    - Parameter types, roles, and return values              │
├─────────────────────────────────────────────────────────────┤
│ 3. Errors / Panics / Exceptions                             │
│    - Failure modes, Result/Option semantics, panic triggers │
└─────────────────────────────────────────────────────────────┘
  (Revision History / Changelog MUST NOT be present)
```

#### Rust Doc Comment Schema (`///` or `//!`)

```rust
//! # Module / Item Name
//!
//! Detailed description of the module or item's functionality and behavior.
//!
//! ## Arguments / Return
//! - `arg_name`: Description of parameter
//! - Returns `Result<T, E>` where `T` is ...
//!
//! ## Errors
//! - Returns `Err(AppError::NotFound)` if the target file cannot be opened.
```

#### TypeScript JSDoc Schema (`/** ... */`)

```typescript
/**
 * Detailed description of the component, hook, or function.
 *
 * @param paramName - Explanation of parameter
 * @returns Explanation of return value
 * @throws Error condition if applicable
 */
```

### 2. Constant Reference Annotation

Per Constitution Principle II:

```text
Rust:       // Constant reference: constants::DEFAULT_SEARCH_PATH
TypeScript: // Constant reference: DEFAULT_PAGE_SIZE
```

### 3. Excluded Assets & Resources

- `crates/core/locales/ja.yml`: Key-value translation catalog for user interface. Values remain in Japanese.
- `src/constants/licenses.json`: Third-party open-source license attribution registry. Legal texts remain in original language.

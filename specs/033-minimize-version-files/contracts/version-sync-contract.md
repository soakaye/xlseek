# Contract: Version Synchronization and Display Copyright

**Feature Branch**: `033-minimize-version-files`
**Date**: 2026-10-25

## 1. CLI Synchronization Contract (`scripts/sync-version.js`)

### 1.1 Invocations
- **Via npm lifecycle**: Automatically triggered during `npm version <newversion>`.
- **Direct CLI execution**: `node scripts/sync-version.js [target-version]`

### 1.2 Input Arguments & Environment
| Source | Key / Parameter | Description |
| :--- | :--- | :--- |
| CLI Argument | `process.argv[2]` | Optional candidate semver version string. |
| Environment | `process.env.npm_package_version` | Provided by `npm version` during lifecycle execution. |
| Filesystem Fallback | `package.json` | Read `version` field if neither argument nor env variable is provided. |

### 1.3 Preconditions
- The repository must be a Git worktree.
- `package.json` must be present and contain a valid semver `version`.
- `Cargo.toml` must contain a `[workspace.package]` table.

### 1.4 Postconditions (Success)
- `Cargo.toml` root manifest `[workspace.package].version` equals `targetVersion`.
- `Cargo.lock` reflects `targetVersion` for all workspace packages (`xlseek`, `xlseek-core`, `xlseek-cli`, `xlseek-mcp`).
- `git status` indicates both `Cargo.toml` and `Cargo.lock` are staged in the Git index (`Changes to be committed`).
- `git diff Cargo.lock` reports zero unstaged modifications.
- Exit code is `0`.
- Diagnostic output on STDOUT:
  ```text
  [sync-version] Successfully synchronized project version to vX.Y.Z:
    ✓ Cargo.toml
    ✓ Cargo.lock
  ```

### 1.5 Postconditions (Failure)
- If validation, Cargo execution, or Git staging encounters an error:
  - Process exits with code `1`.
  - Detailed diagnostic error is written to STDERR:
    ```text
    [sync-version] Failed: <Reason>
    ```
  - `npm version` immediately aborts without creating a Git commit or release tag.

---

## 2. Cargo Workspace Manifest Contract

### 2.1 Root Manifest (`Cargo.toml`)
Must define workspace package metadata:
```toml
[workspace.package]
version = "X.Y.Z"
authors = ["soakaye"]
edition = "2021"
license = "MIT"
```

### 2.2 Member Manifests (`crates/{core,cli,mcp}/Cargo.toml`, `src-tauri/Cargo.toml`)
Must inherit package metadata:
```toml
[package]
name = "..."
version.workspace = true
authors.workspace = true
edition.workspace = true
```
Member manifests MUST NOT declare explicit `version = "..."` strings.

---

## 3. Desktop Application Contract (`src-tauri/tauri.conf.json`)

The desktop configuration must delegate version extraction to the root package manifest:
```json
{
  "$schema": "https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-cli/schema.json",
  "productName": "xlseek",
  "version": "../package.json",
  ...
}
```

---

## 4. Frontend Constants and Display Copyright Contract

### 4.1 Vite Build Configuration (`vite.config.ts`)
```typescript
import packageJson from "./package.json";

export default defineConfig({
  define: {
    __APP_VERSION__: JSON.stringify(packageJson.version),
  },
  // ...
});
```

### 4.2 Centralized Constants (`src/constants/index.ts`)
```typescript
declare const __APP_VERSION__: string | undefined;

// Constant reference: __APP_VERSION__ injected by Vite build configuration
export const APP_CONSTANTS = {
  ROOT_ELEMENT_ID: "root",
  APP_VERSION: typeof __APP_VERSION__ !== "undefined" ? __APP_VERSION__ : "1.0.0",
  APP_AUTHOR: "soakaye",
} as const;
```

### 4.3 Localization Catalogs (`crates/core/locales/` & `src-tauri/locales/`)
- `about.APP_VERSION`: **Prohibited** (must not exist).
- `about.COPYRIGHT`: **Prohibited** (must not exist).
- `about.COPYRIGHT_LABEL`: **Mandatory** (`"Copyright:"` in `en.yml`, `"著作権:"` in `ja.yml`).

### 4.4 About Dialog Render Contract (`src/components/about/AboutDialog.tsx`)
```tsx
<div className="flex justify-between items-center py-1 border-b border-zinc-800/80">
  <span className="text-zinc-500">{t("about.COPYRIGHT_LABEL")}</span>
  <span className="font-mono text-zinc-300">
    Copyright © {new Date().getFullYear()} {APP_CONSTANTS.APP_AUTHOR}
  </span>
</div>
```

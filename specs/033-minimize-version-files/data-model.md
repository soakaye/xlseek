# Data Model: Minimal Version Bump Files and Consolidated Display Copyright

**Feature Branch**: `033-minimize-version-files`
**Date**: 2026-10-25

## Entity Definitions

### 1. PackageVersion
Represents the semantic release version of the Excel Seek application suite.

- **Fields**:
  - `major` (Integer): Major version number according to SemVer 2.0.0.
  - `minor` (Integer): Minor version number according to SemVer 2.0.0.
  - `patch` (Integer): Patch version number according to SemVer 2.0.0.
  - `prerelease` (Optional String): Optional prerelease tag (e.g., `beta.1`, `rc.2`).
  - `buildMetadata` (Optional String): Optional build identifier.
  - `raw` (String): Normalized string representation (e.g., `"1.0.3"`, `"1.1.0-beta.1"`).
- **Validation Rules**:
  - Must conform to SemVer 2.0.0 specification regex:
    `/^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/`
  - Leading `v` prefix is permitted in CLI input but must be stripped during normalization.
- **Authoritative Flow**:
  1. `npm version <newversion>` updates `package.json` and `package-lock.json`.
  2. Lifecycle script `scripts/sync-version.js` updates `Cargo.toml` (`[workspace.package].version`).
  3. `crates/core`, `crates/cli`, `crates/mcp`, and `src-tauri` inherit via `version.workspace = true`.
  4. `src-tauri/tauri.conf.json` inherits via `"version": "../package.json"`.
  5. Frontend UI inherits via build-time `__APP_VERSION__` define.

---

### 2. DisplayCopyright
Represents the runtime copyright notice displayed in the About dialog and application metadata.

- **Fields**:
  - `author` (String): Author name or copyright holder (e.g., `"soakaye"`), sourced from `APP_CONSTANTS.APP_AUTHOR`.
  - `year` (Integer): Active calendar year dynamically evaluated via `new Date().getFullYear()`.
  - `renderedText` (String): Formatted display notice:
    `Copyright © {year} {author}` (e.g., `"Copyright © 2026 soakaye"`).
- **Validation Rules**:
  - `author` must not be empty.
  - `year` must be a valid 4-digit positive integer.
  - Notice formatting must adhere to universal attribution standards and not depend on language locale.

---

### 3. LocaleCopyrightLabel
Represents the localized label displayed alongside the copyright notice in UI dialogs.

- **Fields**:
  - `localeKey` (String): Translation key `about.COPYRIGHT_LABEL`.
  - `translations`:
    - English (`en`): `"Copyright:"`
    - Japanese (`ja`): `"著作権:"`
- **Validation Rules**:
  - Key must exist in both `en.yml` and `ja.yml`.
  - Must only represent the field label, not the attribution string itself.

---

### 4. VersionSyncTransaction
Represents the atomic execution state of `scripts/sync-version.js` during the `npm version` lifecycle hook.

- **Fields**:
  - `targetVersion` (String): Normalized SemVer version being applied.
  - `rootManifestUpdated` (Boolean): Indicates whether root `Cargo.toml` was successfully rewritten.
  - `cargoLockUpdated` (Boolean): Indicates whether `Cargo.lock` reflects `targetVersion`.
  - `stagedFiles` (Array<String>): List of paths added to Git index (`["Cargo.toml", "Cargo.lock"]`).
  - `status` (Enum): `Pending` | `Completed` | `Aborted`.
  - `errorMessage` (Optional String): Diagnostic failure description if transaction aborted.
- **State Transitions**:
  ```text
  [Start]
     │
     ▼
  [Validate SemVer] ──(invalid)──► [Abort with exit code 1]
     │
     ▼
  [Update root Cargo.toml] ──(I/O error)──► [Abort with exit code 1]
     │
     ▼
  [Run cargo check --workspace] ──(cargo failed)──► [Abort with exit code 1]
     │
     ▼
  [Verify Cargo.lock contains version] ──(not updated)──► [Abort with exit code 1]
     │
     ▼
  [Stage Cargo.toml & Cargo.lock (git add)] ──(git error)──► [Abort with exit code 1]
     │
     ▼
  [Verify git staged status] ──(not staged or unstaged diff)──► [Abort with exit code 1]
     │
     ▼
  [Complete (exit code 0)] ──► npm creates version commit and tag
  ```

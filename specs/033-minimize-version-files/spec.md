# Feature Specification: Minimal Version Bump Files and Consolidated Display Copyright

**Feature Branch**: `033-minimize-version-files`

**Created**: 2026-10-25

**Status**: Draft

**Input**: User description: "バージョン番号更新ファイルの最小化を行いたい。npm versionでバージョンを更新すると多数のファイルが更新される。バージョン番号の更新忘れなどの問題も生じる可能性も高く変更は最小化したい。また、表示対象となっているCopyright 文字列も最小化したい。各ファイルヘッダ中のCopyrightはそれぞれ保持する。"

## Clarifications

### Session 2026-10-25

- Q: `npm version` 実行時のバージョン同期について、どの方式で最小化・自動化することを想定されていますか？ → A: Option A (最小限の `npm version` フックスクリプトで自動更新する: `scripts/sync-version.js` を大幅にスリム化し、ルート `Cargo.toml` と `Cargo.lock` のみ最小限更新。`npm version patch` 一発で作業完了)
- Q: Aboutダイアログ等で表示される Copyright 文字列をどのように集約・管理しますか？ → A: Option A (動的生成: `Copyright © {year} {author}`。`package.json` の `author` や現在年等から動的に組み立て。4つのロケールYAMLから静的な著作権表記を全削除し年次更新を不要化。各ファイルヘッダ中のCopyrightは保持)
- Q: `npm version` でバージョン変更した際に `Cargo.lock` がコミットされる前にタグが付けられバージョンから漏れる問題はどう対応しますか？ → A: `npm version` のコミットおよびタグ作成前に、`Cargo.lock` の更新とステージング (`git add Cargo.lock`) をスクリプト内で確実に完了させ、同一のバージョンコミット・タグに `Cargo.lock` が必ず含まれるように保証する。更新またはステージングに失敗した場合は即座に異常終了してコミット/タグ作成を中止する。

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Streamlined Version Update via npm version (Priority: P1) 🎯 MVP

As a project maintainer or release manager,
I want to bump the application version using standard tooling (`npm version`) with modifications strictly limited to the primary manifests and lock files,
So that release diffs are clean, version propagation is automated and reliable, and `Cargo.lock` is guaranteed to be included in the tagged release commit rather than omitted.

**Why this priority**:
Currently, updating the application version alters more than 10 files across frontend configurations, Rust manifests, localization catalogs, and mockups. Furthermore, `Cargo.lock` has historically been left behind as uncommitted/unstaged changes after `npm version` creates the commit and tag. Resolving both file proliferation and lockfile omission in the tagged commit is the critical MVP goal.

**Independent Test**:
Execute a version bump command (e.g., `npm version patch`). Verify that only `package.json`, `package-lock.json`, root `Cargo.toml`, and `Cargo.lock` are modified. Inspect the git commit created by `npm version` and the corresponding git tag: confirm that `Cargo.lock` is included in the commit, `git status` reports a completely clean working tree immediately afterwards, and all components report the new version.

**Acceptance Scenarios**:

1. **Given** a clean repository state at version `1.0.0`, **When** the maintainer bumps the version to `1.0.1` using `npm version`, **Then** file changes are strictly restricted to primary version manifests (`package.json`, `package-lock.json`, root `Cargo.toml`, and `Cargo.lock`).
2. **Given** a newly bumped version, **When** building and running the GUI application, **Then** the About dialog displays the updated version number without requiring version changes in localization catalogs or mockups.
3. **Given** a newly bumped version, **When** building and executing CLI and MCP binaries with `--version`, **Then** each binary reports the updated version matching the primary version manifest.
4. **Given** any automated validation or build check, **When** validating version consistency across all workspace crates and packages, **Then** all workspace members report identical version numbers derived from the root authoritative source.
5. **Given** an `npm version` bump execution, **When** the version git commit and release tag are created, **Then** `Cargo.lock` is already updated and staged, ensuring `Cargo.lock` is included in that exact version commit and tag with zero dirty changes remaining in `git status`.

---

### User Story 2 - Consolidated Display Copyright Information (Priority: P2)

As an application user viewing the About dialog or checking application metadata,
I want to see consistent, accurate copyright attribution without duplicate, divergent copyright strings scattered across localization files,
So that copyright notices remain accurate over time without redundant maintenance across language catalogs.

**Why this priority**:
Currently, display copyright strings (e.g., "Copyright © 2026 soakaye") are duplicated across multiple language catalogs (`ja.yml`, `en.yml`) in multiple directories (`crates/core` and `src-tauri`). Because copyright notices are language-neutral attribution notices, managing them redundantly across multiple localization files creates unnecessary maintenance overhead. Note that per repository rules, header comments in individual source files remain untouched.

**Independent Test**:
Inspect the About dialog in both Japanese and English locales. Verify that the copyright notice displays correctly with appropriate labels (e.g., "著作権:" in Japanese, "Copyright:" in English) and that the attribution string itself is dynamically formatted as `Copyright © {year} {author}` based on application metadata rather than redundantly duplicated across language catalogs.

**Acceptance Scenarios**:

1. **Given** the application running in English, **When** opening the About dialog, **Then** the copyright attribution is displayed with the English label and dynamically generated attribution text (`Copyright © {year} {author}`).
2. **Given** the application running in Japanese, **When** opening the About dialog, **Then** the copyright attribution is displayed with the Japanese label and the identical dynamically generated attribution text (`Copyright © {year} {author}`).
3. **Given** the localization catalogs (`ja.yml` and `en.yml`), **When** inspecting translation keys, **Then** static copyright attribution strings (`about.COPYRIGHT`) are completely eliminated from all language catalogs.

---

### User Story 3 - Preservation of Source File Header Copyrights (Priority: P3)

As a developer or contributor reviewing source files,
I want existing copyright and license headers in individual source files (`.rs`, `.ts`, `.tsx`, `.js`, etc.) to remain strictly intact and unaltered during version and copyright minimization work,
So that legal attributions, file authorship records, and compliance with the project constitution are fully preserved.

**Why this priority**:
The user explicitly specified: "各ファイルヘッダ中のCopyrightはそれぞれ保持する。" (Preserve the copyright in each file header). This constraint ensures that minimization efforts focus solely on version bump operational files and user-facing display copyright strings, preventing unintended bulk modifications to source code headers.

**Independent Test**:
Perform an automated check comparing file headers before and after the change. Verify that existing `Copyright (c) 2026 soakaye` header comments across all Rust, TypeScript, and script source files are completely preserved.

**Acceptance Scenarios**:

1. **Given** existing source code files with header comments, **When** version minimization changes are applied, **Then** all existing file header comments remain intact.
2. **Given** version bump execution, **When** git diff is inspected, **Then** no source file headers are modified by version bump processes.

---

### Edge Cases

- What happens when a pre-release or build metadata version (e.g., `1.1.0-beta.1`) is passed to `npm version`? The minimal propagation mechanism must handle semantic versioning suffixes uniformly across root package and workspace manifests.
- How does the system handle version synchronization when a member crate or package is built independently without running the full npm lifecycle? Member crates must inherit their version from workspace definitions so individual builds always reflect the authoritative version.
- What happens if the display copyright year changes across calendar years? The dynamic generation will automatically reflect the active year without requiring manual commits to localization files.
- What happens if `Cargo.lock` fails to synchronize or stage during `npm version`? The synchronization hook must immediately exit with a non-zero exit code, aborting `npm version` before any git commit or tag is created, preventing partial or untracked versions.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST establish a minimal set of authoritative version files (single source of truth) so that executing `npm version` modifies only the necessary primary package and workspace manifests and their associated lock files (`package.json`, `package-lock.json`, root `Cargo.toml`, and `Cargo.lock`).
- **FR-002**: Rust workspace members (including `xlseek-core`, `xlseek-cli`, `xlseek-mcp`, and desktop application backend) MUST inherit their package version from the root workspace manifest via workspace package inheritance (`version.workspace = true`) rather than maintaining independent hardcoded version strings in individual `Cargo.toml` files.
- **FR-003**: The desktop application configuration (`tauri.conf.json`) MUST dynamically reference or synchronize from the primary package manifest without requiring separate manual version maintenance.
- **FR-004**: Auxiliary localization catalogs (`en.yml`, `ja.yml`) MUST NOT contain hardcoded application version strings (`about.APP_VERSION`); runtime components MUST source the version from application metadata or build-time environment constants.
- **FR-005**: Static design mockups and HTML prototypes MUST NOT be included in mandatory version bump files; prototype mockups MUST remain decoupled from automated version bumps.
- **FR-006**: Display copyright strings MUST be consolidated into a single authoritative dynamic format (`Copyright © {year} {author}`), eliminating duplicated static copyright attribution strings (`about.COPYRIGHT`) across all localization catalogs.
- **FR-007**: Localization catalogs MUST retain locale-specific UI labels (such as "Copyright:" in English and "著作権:" in Japanese) while sourcing the attribution content dynamically from application metadata.
- **FR-008**: Source code header comments across all existing files MUST NOT be modified or removed during version or copyright minimization.
- **FR-009**: Automated consistency verification tests MUST be provided to guarantee that version numbers reported by GUI, CLI, and MCP components strictly match the authoritative package version.
- **FR-010**: Version propagation during `npm version` MUST be automated via a minimal `npm version` lifecycle hook script (`scripts/sync-version.js`) that updates only the root workspace manifest (`Cargo.toml`) and synchronizes `Cargo.lock`, with all workspace member crates inheriting the version automatically.
- **FR-011**: Display copyright construction MUST be dynamically formatted as `Copyright © {year} {author}` using application metadata (author from package configuration and current calendar year), removing static copyright attribution strings from all localization catalogs (`ja.yml`, `en.yml`) while retaining locale-specific labels and preserving all source file header comments.
- **FR-012**: The version synchronization lifecycle hook MUST ensure `Cargo.lock` is updated and explicitly staged in git prior to `npm version` creating the release commit and tag, and MUST abort with an error if `Cargo.lock` update or staging fails, guaranteeing that `Cargo.lock` is never omitted from the version commit and release tag.

---

### Key Entities *(include if feature involves data)*

- **Package Version**: Semantic version string (e.g., `MAJOR.MINOR.PATCH`) representing the release version of the Excel Seek suite. Authoritative source defined in root `package.json` and synchronized to root `Cargo.toml`, then inherited across workspace packages.
- **Display Copyright Attribution**: The user-facing copyright string presented in application information dialogs, dynamically generated as `Copyright © {year} {author}` from application metadata.
- **Copyright Label**: Locale-dependent UI label displayed alongside the attribution (e.g., "Copyright:" in English, "著作権:" in Japanese).

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Running a version bump command (e.g., `npm version patch`) reduces the number of modified version tracking files from 11+ non-lock files to 2 primary manifests (`package.json` and root `Cargo.toml`) plus their standard lock files.
- **SC-002**: 100% of subcrates (`crates/core`, `crates/cli`, `crates/mcp`, `src-tauri`) and desktop configuration inherit versioning from the authoritative manifests without individual version drift.
- **SC-003**: Duplicated static version strings in localization catalogs (`crates/core/locales/*.yml` and `src-tauri/locales/*.yml`) are reduced from 4 instances to 0 instances.
- **SC-004**: Duplicated static copyright attribution strings across localization catalogs are reduced from 4 duplicate entries to 0 static entries, fully replaced by dynamic generation.
- **SC-005**: 100% of existing source file header comments retain their original copyright notices with zero header regressions.
- **SC-006**: Automated test execution confirms zero discrepancy between version reported by CLI (`xlseek-cli --version`), MCP server (`xlseek-mcp --version`), and GUI About dialog.
- **SC-007**: 100% of git commits and tags created by `npm version` contain the synchronized `Cargo.lock`, leaving the working tree completely clean with zero unstaged changes immediately following the version bump.

---

## Assumptions

- `package.json` remains the primary entry point for triggering version bumps via `npm version`.
- Cargo workspace inheritance (`[workspace.package]` with `version.workspace = true`) is supported across all workspace crates (`crates/core`, `crates/cli`, `crates/mcp`, `src-tauri`).
- Tauri v2 configuration (`tauri.conf.json`) supports reading version directly from `package.json` (`"version": "../package.json"`).
- Frontend React components can dynamically access application version via build-time environment constants (or Tauri runtime metadata) and compute the display copyright year dynamically.
- Existing and future file header comments adhering to Project Constitution Principle III are strictly preserved and unaffected by versioning and display copyright changes.
- In Git repositories, `npm version` automatically commits all files staged in the index during the `version` script before applying the git tag.

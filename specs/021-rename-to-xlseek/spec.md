# Feature Specification: Unify Project and Package Naming to xlseek

**Feature Branch**: `021-rename-to-xlseek`

**Created**: 2026-10-02

**Status**: Ready for Planning

**Input**: User description: "exgrep, exlgrep などとある名称は、xlseekに変更してください"

## Clarifications

### Session 2026-10-02

- Q: Should the repository Git clone URL and GitHub links in the documentation (`README.md` and `README_ja.md`) be updated to `xlseek.git`, or should they remain pointing to `exgrep.git` until a remote repository rename occurs? (FR-006) → A: Option A (Update documentation clone URLs to `https://github.com/soakaye/xlseek.git` and `cd xlseek` for full branding consistency).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consistent Branding and Binary Identification (Priority: P1)

As a user running the desktop application, command-line interface, or inspecting installed files and system preferences, I want all product identifiers, binary names, package names, configuration keys, and documentation to uniformly use the canonical name `xlseek`, so that there is no confusion caused by legacy names like `exgrep` or `exlgrep`.

**Why this priority**: Eliminating legacy naming inconsistencies across the codebase, packages, binaries, and persistent storage keys is essential for branding coherence, user clarity, and platform maintainability.

**Independent Test**: Can be fully tested by building and inspecting workspace package names, binary names, storage keys, and documentation references, ensuring `exgrep` and `exlgrep` are replaced by `xlseek` without broken dependencies.

**Acceptance Scenarios**:

1. **Given** the workspace packages and backend crates, **When** building and running tests across all crates, **Then** the core crate is named `xlseek-core` (or `xlseek_core`), the CLI crate is named `xlseek-cli` (or `xlseek_cli`), and the desktop application references `xlseek`.
2. **Given** application storage keys and configuration filenames (e.g., local storage or preferences for search history and settings), **When** settings or histories are saved or retrieved, **Then** persistent keys use `xlseek` prefixes with backward-compatible migration from legacy `exlgrep` keys.
3. **Given** test fixtures, temporary output filenames, and utility scripts, **When** tests or scripts are executed, **Then** temporary prefixes and test artifacts use the `xlseek` prefix.

---

### User Story 2 - Smooth Migration for Existing User Configurations (Priority: P2)

As an existing user upgrading from previous versions, I want my existing search history and application configurations that were stored under legacy `exlgrep` keys to be smoothly migrated to `xlseek` storage keys without losing saved settings or history.

**Why this priority**: Users upgrading to the new version must not experience configuration loss or reset history simply because internal configuration keys were renamed.

**Independent Test**: Can be independently verified by populating mock storage with legacy keys and checking that the application reads and migrates the values into `xlseek` keys.

**Acceptance Scenarios**:

1. **Given** existing local preferences or history saved under legacy keys (e.g., `exlgrep.search-history.*`), **When** the application starts or loads settings, **Then** it seamlessly reads the legacy value, writes it to the new `xlseek.*` key, and operates with the migrated data.
2. **Given** a new environment without legacy keys, **When** the application starts, **Then** it initializes and writes directly to `xlseek.*` keys without error.

---

### User Story 3 - Coherent Documentation and Developer References (Priority: P3)

As a developer or contributor reviewing the project documentation (such as README files, architecture diagrams, guidelines, and licenses generator), I want all references to project structure, crate names, clone URLs, and run commands to accurately reflect `xlseek`, so that instructions are clear and reproducible.

**Why this priority**: Accurate documentation prevents onboarding confusion and ensures automated license generators and scripts remain in sync with package names.

**Independent Test**: Can be tested by running static documentation checks, license generation scripts, and validating that setup/run commands reference `xlseek`.

**Acceptance Scenarios**:

1. **Given** project documentation and build scripts, **When** reviewing READMEs, AGENTS.md, or license scripts, **Then** crate names, binary targets, and directory descriptions are listed as `xlseek` / `xlseek-core` / `xlseek-cli`.
2. **Given** installation instructions and clone commands in `README.md` and `README_ja.md`, **When** a user reads the setup section, **Then** the clone command uses `https://github.com/soakaye/xlseek.git` and `cd xlseek`.

---

### Edge Cases

- **Existing Saved Settings**: If both legacy `exlgrep` keys and new `xlseek` keys exist in storage, `xlseek` keys MUST take precedence to preserve new edits.
- **Git Workspace Directory / Remote Repository**: The filesystem directory path or remote Git URL (e.g., GitHub repo name or local folder name) may retain `exgrep` externally; internal code, manifests, and documentation paths should use `xlseek` while tolerating existing repository URLs if specified.
- **Historical Specs and Logs**: Past feature specification directories under `specs/` (e.g., `specs/001` through `specs/020`) and historical bug reports document historical state at that time and should not be modified to preserve revision integrity; all current documentation, scripts, and code must use `xlseek`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST rename shared core crate package and library names from `exlgrep-core` / `exlgrep_core` to `xlseek-core` / `xlseek_core`.
- **FR-002**: System MUST update all crate dependencies and internal imports in CLI (`crates/cli`), desktop (`src-tauri`), and tests to reference `xlseek-core` / `xlseek_core`.
- **FR-003**: System MUST update CLI and desktop binary manifests, comments, and identifiers to ensure `xlseek` is used consistently across targets.
- **FR-004**: System MUST update internal prefix constants (e.g., temporary directory prefixes, test output file names) from `exlgrep` to `xlseek`.
- **FR-005**: System MUST update storage keys (e.g. search history keys, preference settings) to use `xlseek` namespace while supporting backward-compatible fallback for existing `exlgrep` keys.
- **FR-006**: System MUST update build scripts, license generation tools (`scripts/generate-licenses.py`), and documentation (`README.md`, `README_ja.md`, `AGENTS.md`) to reflect `xlseek` naming, including updating Git clone URLs in documentation to `https://github.com/soakaye/xlseek.git`.
- **FR-007**: System MUST preserve historical specification artifacts under `specs/001-` through `specs/020-` as immutable historical records.

### Key Entities *(include if feature involves data)*

- **Package Manifest**: Configuration entity (`Cargo.toml`, `package.json`) defining the crate or application identity, dependencies, and binary targets under the `xlseek` namespace.
- **Configuration & Storage Keys**: Key-value namespace identifier used in frontend and desktop layers to persist search history and user preferences, standardized under `xlseek.*`.
- **Runtime Temporary Artifacts**: Temporary files and folders generated during export and search operations, prefixed with `.xlseek-`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero occurrences of `exlgrep` or `exgrep` remain in active source code, Cargo manifests, test suites, and documentation (excluding historical specs under `specs/001` through `specs/020` and Git commit logs).
- **SC-002**: 100% of workspace tests across all crates (`cargo test --workspace`) pass without any compilation error or broken import path after crate renaming.
- **SC-003**: 100% of frontend build checks (`npm run build`) and lint validations complete with zero errors.
- **SC-004**: Users with legacy stored configuration retain their settings and history seamlessly upon first launch after the upgrade.

## Assumptions

- Preserving historical specification directories (`specs/001-` to `specs/020-`) avoids mutating past project records, focusing changes on active source files, documentation, and tooling.
- The project constitution and AGENTS.md rule requiring comprehensive header comments and externalized constants will be strictly upheld during refactoring.
- The license generation script `scripts/generate-licenses.py` will maintain matching logic for `xlseek` packages.

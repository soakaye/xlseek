# Tasks: Standalone Multi-Platform xlseek-mcp Binary Release

**Input**: Design documents from `/specs/036-release-mcp-binaries/`
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/workflow-contract.md](contracts/workflow-contract.md), [quickstart.md](quickstart.md)

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to ([US1], [US2], [US3])
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify repository build prerequisites and current release workflow configuration

- [X] T001 Inspect current workflow structure in `.github/workflows/publish-to-auto-release.yml` and MCP crate configuration in `crates/mcp/Cargo.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Verify standalone `xlseek-mcp` builds and runs cleanly without GUI dependencies

- [X] T002 Verify local headless build and execution of `xlseek-mcp` via `cargo build --release -p xlseek-mcp` and test `./target/release/xlseek-mcp --help`

**Checkpoint**: Core MCP crate verified — workflow implementation can now proceed

---

## Phase 3: User Story 1 - Standalone xlseek-mcp Binary Release Job Foundation (Priority: P1) 🎯 MVP

**Goal**: Establish the dedicated `publish-mcp-binaries` job in `.github/workflows/publish-to-auto-release.yml` to compile `crates/mcp` as a standalone binary decoupled from desktop GUI bundlers.

**Independent Test**: Verify that the job definition in `.github/workflows/publish-to-auto-release.yml` configures `contents: write`, checks out the repository, and invokes `cargo build --release -p xlseek-mcp`.

### Implementation for User Story 1

- [X] T003 [US1] Define `publish-mcp-binaries` job with `contents: write` permissions and checkout/toolchain setup in `.github/workflows/publish-to-auto-release.yml`
- [X] T004 [US1] Add the standalone release build step `cargo build --release -p xlseek-mcp --target ${{ matrix.target }}` in `.github/workflows/publish-to-auto-release.yml`

**Checkpoint**: At this point, `publish-mcp-binaries` is defined as a standalone compilation job.

---

## Phase 4: User Story 2 - Multi-Platform Distribution for macOS, Linux, and Windows (Priority: P2)

**Goal**: Configure multi-platform matrix runners and archive packaging (`.tar.gz` for Unix, `.zip` for Windows) preserving executable permissions.

**Independent Test**: Inspect the matrix configuration and archive commands in `.github/workflows/publish-to-auto-release.yml` against the contracts in `specs/036-release-mcp-binaries/contracts/workflow-contract.md`.

### Implementation for User Story 2

- [X] T005 [US2] Configure runner matrix with `fail-fast: false` across `macos-latest` (`aarch64-apple-darwin`), `ubuntu-24.04` (`x86_64-unknown-linux-gnu`), and `windows-latest` (`x86_64-pc-windows-msvc`) in `.github/workflows/publish-to-auto-release.yml`
- [X] T006 [US2] Implement platform archive packaging step using `tar` (with `chmod +x`) on Unix and `Compress-Archive` on Windows in `.github/workflows/publish-to-auto-release.yml`

**Checkpoint**: Archives are generated per platform with standard names `xlseek-mcp-${{ github.ref_name }}-${{ matrix.target }}.${{ matrix.archive_ext }}`.

---

## Phase 5: User Story 3 - Robust Automated Release Pipeline (Priority: P3)

**Goal**: Upload generated release archives to GitHub Releases via `softprops/action-gh-release@v2` targeting `${{ github.ref_name }}` with `draft: true` and idempotent replacement.

**Independent Test**: Validate that the release upload step in `.github/workflows/publish-to-auto-release.yml` attaches the archives using `GITHUB_TOKEN` and runs concurrently with `publish-tauri`.

### Implementation for User Story 3

- [X] T007 [US3] Add release upload step using `softprops/action-gh-release@v2` with `tag_name: ${{ github.ref_name }}` and `draft: true` in `.github/workflows/publish-to-auto-release.yml`

**Checkpoint**: Complete release pipeline configuration attaches per-platform MCP archives to GitHub Releases.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validation, syntax verification, and repository quality gates

- [X] T008 [P] Validate workflow YAML syntax in `.github/workflows/publish-to-auto-release.yml`
- [X] T009 [P] Run Rust workspace quality gates via `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check`
- [X] T010 Execute local quickstart verification steps per `specs/036-release-mcp-binaries/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS workflow edits
- **User Stories (Phases 3-5)**:
  - User Story 1 (P1): Job definition and build command
  - User Story 2 (P2): Matrix and archive packaging (extends US1)
  - User Story 3 (P3): Asset upload action step (extends US2)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Establishes base job and build steps
- **User Story 2 (P2)**: Depends on US1 job structure to add matrix and archive packaging
- **User Story 3 (P3)**: Depends on US2 archive generation to upload the files

### Parallel Opportunities

- T008 (YAML validation) and T009 (Rust quality gates) in Phase 6 can run in parallel

---

## Parallel Example: Polish Phase

```bash
# Validate workflow YAML and workspace tests concurrently:
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/publish-to-auto-release.yml'))"
cargo test --workspace
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (verify headless MCP build)
3. Complete Phase 3: User Story 1 (add job definition)
4. Validate workflow structure

### Incremental Delivery

1. Setup + Foundational → Verify local crate build
2. Add US1 → Standalone job foundation in CI workflow
3. Add US2 → Multi-platform matrix runners and archive packaging
4. Add US3 → Release asset upload step
5. Phase 6 → Comprehensive verification (YAML syntax, cargo test, clippy, fmt)

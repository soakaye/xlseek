# Tasks: Clean Changelog Headers and English Source Comments

**Feature**: Clean Changelog Headers and English Source Comments
**Branch**: `017-clean-changelog-english-comments`
**Date**: 2026-09-30
**Spec**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/hamlet/specs/017-clean-changelog-english-comments/spec.md)
**Plan**: [plan.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/hamlet/specs/017-clean-changelog-english-comments/plan.md)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verification tooling and baseline inspection

- [X] T001 Verify baseline repository status and ensure clean git status
- [X] T002 Inspect and catalogue in-file changelog headers and Japanese comments across crates/, src-tauri/, and src/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Update overarching developer guidelines and rulebooks per Constitution v3.0.0

- [X] T003 Update AGENTS.md to align with Constitution v3.0.0 (3-element header comments, English default) in AGENTS.md
- [X] T004 [P] Verify documentation contract and scanning regex rules in specs/017-clean-changelog-english-comments/contracts/documentation-contract.md

**Checkpoint**: Foundation ready - guideline and rule alignment complete. User story tasks can now proceed.

---

## Phase 3: User Story 1 - Remove In-File Changelogs from Source and Prototype Headers (Priority: P1) 🎯 MVP

**Goal**: Eliminate all in-file changelog / revision history sections from header docstrings and comments across all code modules and design prototypes, complying with Constitution Principle III.

**Independent Test**: Run `git grep -n "変更履歴" -- 'crates/' 'src-tauri/' 'src/' 'design/mainui/index.html'` and `git grep -n -i "revision history" -- 'crates/' 'src-tauri/' 'src/' 'design/mainui/index.html'` to verify 0 matches.

### Implementation for User Story 1

- [X] T005 [P] [US1] Remove in-file changelog sections from crates/cli/src/ in crates/cli/src/args.rs, crates/cli/src/lib.rs, crates/cli/src/output.rs, crates/cli/src/main.rs, crates/cli/src/constants.rs
- [X] T006 [P] [US1] Remove in-file changelog sections from crates/core/src/ in crates/core/src/lib.rs, crates/core/src/constants.rs, crates/core/src/i18n.rs, crates/core/src/models/*.rs, crates/core/src/search/*.rs, crates/core/src/export/*.rs
- [X] T007 [P] [US1] Remove in-file changelog sections from src-tauri/src/ in src-tauri/src/lib.rs, src-tauri/src/main.rs, src-tauri/src/constants.rs, src-tauri/src/commands/*.rs
- [X] T008 [P] [US1] Remove in-file changelog sections from src/ in src/App.tsx, src/main.tsx, src/constants/index.ts, src/types/*.ts, src/hooks/*.ts, src/components/**/*.tsx
- [X] T009 [P] [US1] Remove in-file changelog sections from design/mainui/index.html in design/mainui/index.html

**Checkpoint**: User Story 1 complete. All in-file changelog sections are removed across all crates, Tauri backend, React frontend, and design prototype.

---

## Phase 4: User Story 2 - English Translation of In-Code Comments (Priority: P2)

**Goal**: Convert all docstrings, inline comments, block comments, and constant annotations to concise, natural English across the codebase, while keeping `crates/core/locales/ja.yml` intact.

**Independent Test**: Run `git grep -P '//.*[\p{Hiragana}\p{Katakana}\p{Han}]' -- 'crates/' 'src-tauri/' 'src/'` and verify zero Japanese comment matches. Run `cargo test --workspace` and `npm run build` to confirm zero regression.

### Implementation for User Story 2

- [X] T010 [P] [US2] Translate comments, docstrings, and constant annotations to English in crates/core/src/ (preserving crates/core/locales/ja.yml)
- [X] T011 [P] [US2] Translate comments, docstrings, and constant annotations to English in crates/cli/src/
- [X] T012 [P] [US2] Translate comments, docstrings, and constant annotations to English in src-tauri/src/
- [X] T013 [P] [US2] Translate comments, docstrings, and constant annotations to English in src/ (App.tsx, components, hooks, types, constants)
- [X] T014 [P] [US2] Translate comments and annotations to English in tests/ across crates/ and src-tauri/
- [X] T015 [P] [US2] Translate code comments and HTML comments to English in design/mainui/index.html (preserving mock data)

**Checkpoint**: User Story 2 complete. All code comments and docstrings are in English. Japanese localization messages in `ja.yml` remain unaltered.

---

## Phase 5: Polish & Quality Gates

**Purpose**: Comprehensive automated validation across all crates and frontend build pipelines

- [X] T016 Run automated search scans per quickstart.md to verify 0 changelogs and 0 non-English comments in code
- [X] T017 Run Rust formatting verification with cargo fmt --check
- [X] T018 Run Rust Clippy checks with cargo clippy --workspace --all-targets -- -D warnings
- [X] T019 Run Rust tests with cargo test --workspace
- [X] T020 Run Frontend typecheck and build with npm run build

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Independent, runs first.
- **Foundational (Phase 2)**: Depends on Setup; blocks user story phases.
- **User Story 1 (Phase 3)**: Depends on Foundational; can proceed immediately.
- **User Story 2 (Phase 4)**: Can proceed alongside or immediately following US1 refactoring per crate.
- **Polish (Phase 5)**: Depends on completion of User Stories 1 & 2.

### Parallel Opportunities

- T005, T006, T007, T008, T009 can be executed independently across distinct file trees.
- T010, T011, T012, T013, T014, T015 can be executed independently across separate crates/directories.

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 & Phase 2.
2. Complete Phase 3 (User Story 1) - removing all in-file changelogs.
3. Validate with static grep checks.

### Incremental Delivery (User Story 2 & Polish)

1. Complete Phase 4 (User Story 2) - translating comments to English crate-by-crate.
2. Complete Phase 5 (Quality Gates) - validating all 4 gates pass cleanly.

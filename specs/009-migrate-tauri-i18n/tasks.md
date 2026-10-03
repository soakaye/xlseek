# Tasks: i18n 基盤を Tauri プラグインへ移行する

**Input**: Design documents from `specs/009-migrate-tauri-i18n/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/localization-contract.md, quickstart.md

**Tests**: プロジェクト憲章と AGENTS.md が変更箇所に対応する単体テストと全品質ゲートを要求するため、ストーリーごとにテストタスクを含める。

**Organization**: タスクはユーザーストーリー単位にまとめる。共有のワークスペース・プラグイン導入後、優先度順に独立確認する。

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: upstream プラグインの locale bundler が翻訳資源を見つけるための Cargo 配置と依存関係を用意する。

- [X] T001 Add a root Cargo workspace with `members = ["src-tauri"]` and resolver 2 in `Cargo.toml`; move the existing lockfile to root `Cargo.lock` and keep `src-tauri/Cargo.toml` as the application package.
- [X] T002 [P] Add Rust `tauri-plugin-i18n = 2.0.2` in `src-tauri/Cargo.toml`, the matching ESM package `@razein97/tauri-plugin-i18n = 2.0.1` in `package.json`, and direct `yaml` 2.x dependency for frontend fallback parsing.
- [X] T003 [P] Add `src-tauri/locales/en.yml` and `src-tauri/locales/ja.yml` in the plugin-supported version 1 YAML layout, with a four-element header comment and stable dotted keys for existing app-owned UI, About, menu, error, and export strings. Include the required English key `common.translationUnavailable`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Register the same bundled catalog and permission boundary for all user stories.

**Checkpoint**: Plugin initialization can load both locale catalogs from the built Tauri app.

- [X] T004 Register `tauri_plugin_i18n::init(None)` in the Tauri builder in `src-tauri/src/lib.rs`.
- [X] T005 [P] Add `i18n:default` permission to `src-tauri/capabilities/default.json` for translation loading and locale reading/changing.
- [X] T006 Add a startup test in `tests/app-locale.test.tsx` that verifies plugin initialization completes before localized application content is rendered.

---

## Phase 3: User Story 1 - 言語設定と表示を維持する (Priority: P1) 🎯 MVP

**Goal**: Keep saved language settings, runtime switching, screen text, native menus, and exported headings working through the plugin.

**Independent Test**: Start from each existing saved value (`default|ja|en`), verify the expected language, switch Japanese/English without restarting, and confirm search state plus menu/export text remain consistent.

### Tests for User Story 1

- [X] T007 [US1] Extend `tests/app-locale.test.tsx` to verify the existing `exlgrep.language` values load unchanged, switching language rerenders the screen without losing search state, and an open toast retains its interpolation values.
- [X] T008 [US1] Add translation-key lookup and locale-change behavior tests in `tests/locale-ui.test.tsx` for search, results, settings, About, and accessibility labels.
- [X] T009 [US1] Add tests in `src-tauri/src/export/csv_export.rs` and `src-tauri/src/export/xlsx_export.rs` that assert output text follows the requested language and rejects invalid language values.

### Implementation for User Story 1

- [X] T010 [US1] Add a thin plugin-backed translator in `src/i18n.ts` that loads the plugin catalog and exposes typed key lookup while keeping React responsible for rendering.
- [X] T011 [US1] Update `src/locale-core.ts` and `src/hooks/useLocale.tsx` to preserve storage key `exlgrep.language`, values `default|ja|en`, Japanese-tag matching, English on locale lookup failure, and current-session choice when saving fails; await locale initialization before showing the app.
- [X] T012 [US1] Connect `src/constants/index.ts`, `src/hooks/useSearch.ts`, `src/components/search/SearchBar.tsx`, `src/components/results/ResultTable.tsx`, `src/components/preview/PreviewHeader.tsx`, `src/components/preview/SpreadsheetGrid.tsx`, `src/components/preview/FormulaBar.tsx`, `src/components/preview/MetaInfoCard.tsx`, `src/components/preview/SheetTabs.tsx`, `src/components/common/StatusBar.tsx`, `src/components/common/Toast.tsx`, `src/components/about/AboutDialog.tsx`, `src/components/about/PackageDetail.tsx`, `src/components/about/PackageList.tsx`, `src/components/layout/WindowFrame.tsx`, `src/components/settings/SettingsDialog.tsx`, and `src/App.tsx` to plugin translation keys; remove duplicated Japanese/English dictionaries and the global Proxy.
- [X] T013 [US1] Store notification message keys and interpolation data in `src/App.tsx` and resolve open notifications using the currently selected locale.
- [X] T014 [US1] Replace bilingual menu constants in `src-tauri/src/lib.rs` and `src-tauri/src/constants.rs` with plugin-backed menu labels that change on startup and locale selection.
- [X] T015 [US1] Replace bilingual export constants in `src-tauri/src/export/csv_export.rs`, `src-tauri/src/export/xlsx_export.rs`, and `src-tauri/src/constants.rs` with catalog lookup using the immutable `ExportRequest.language` value; keep rejection of invalid languages.

**Checkpoint**: Existing users retain their saved setting and can switch all app-owned UI, menu, and export text without losing search state.

---

## Phase 4: User Story 2 - 翻訳の欠落時も操作できる (Priority: P1)

**Goal**: Show English for an individual missing translation or when plugin catalog loading fails.

**Independent Test**: Remove one Japanese key and separately simulate plugin-load failure; the missing string or startup UI uses English and the app remains usable.

### Tests for User Story 2

- [X] T016 [US2] Add tests in `tests/locale.test.ts` and `tests/app-locale.test.tsx` for missing Japanese keys, missing English keys, and plugin catalog load failure without exposing raw keys.
- [X] T017 [US2] Add Rust unit tests in `src-tauri/src/i18n.rs` for a missing requested-locale key falling back to the English catalog without mutating the active locale; register the module with `pub mod i18n;` in `src-tauri/src/lib.rs`.

### Implementation for User Story 2

- [X] T018 [US2] Extend `src/i18n.ts` to resolve target-language key, then the same English key, then `common.translationUnavailable`; parse the English catalog bundled from `src-tauri/locales/en.yml` with `yaml` if plugin loading fails.
- [X] T019 [US2] Add a Rust translation helper in `src-tauri/src/i18n.rs` that reads the requested locale and falls back per key to English for menus and exports without changing plugin global locale.
- [X] T020 [US2] Handle plugin startup and locale-change errors in `src/hooks/useLocale.tsx` and `src/App.tsx` by continuing with bundled English strings and recording an English diagnostic message; verify the diagnostic is English under both UI locales.

**Checkpoint**: Missing translations and plugin load failures do not block startup or leave translation keys visible.

---

## Phase 5: User Story 3 - 一貫した翻訳資源で言語を追加できる (Priority: P2)

**Goal**: Keep catalog maintenance consistent so future locales use the same translation mechanism.

**Independent Test**: Run catalog checks to detect missing English keys and mismatched interpolation placeholders; verify all current app-owned surfaces read from the shared catalog.

### Tests for User Story 3

- [X] T021 [US3] Add `tests/catalog.test.ts` checks that every Japanese key has an English fallback, `common.translationUnavailable` exists in English, and corresponding translations use the same interpolation placeholders.

### Implementation for User Story 3

- [X] T022 [US3] Document stable key naming, interpolation placeholders, and how to add a locale without changing UI consumers in `src-tauri/locales/README.md`, including the required four-element header comment and YAML layout.

**Checkpoint**: A future locale follows the same catalog structure and lookup path; no screen-specific translation mechanism is needed.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify packaged resources and all regression paths.

- [ ] T023 [P] Update `specs/009-migrate-tauri-i18n/quickstart.md` with any command or manual-step changes discovered during implementation.
- [X] T024 Run the automated quality gates from repository root and confirm bundled locale discovery: `npm test`, `npm run lint`, `npm run build`, `cargo test --manifest-path src-tauri/Cargo.toml`, `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`, and `cargo fmt --manifest-path src-tauri/Cargo.toml --check`.
- [ ] T025 Run the desktop scenarios in `specs/009-migrate-tauri-i18n/quickstart.md` on a packaged Tauri build, covering saved settings, system default, runtime switching, bundled locale discovery, menu labels, and CSV/XLSX headers. Simulated plugin/catalog failure is covered by automated tests.

  実施記録: パッケージ版の起動、日本語のシステム既定、英語への切替、再起動後の英語設定復元を確認し、最後に設定を `default` に戻した。macOS ネイティブメニューとファイル選択を伴う CSV/XLSX 出力は手動操作していない。出力言語は Rust 自動テストで確認済み。

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 is first because the plugin build script must discover a workspace root. T002 and T003 can then proceed in parallel.
- **Foundational (Phase 2)**: Depends on T001–T003; registration, capability, and initialization must be complete before story work.
- **User Stories (Phase 3+)**: Depend on Phase 2. US2 depends on US1’s plugin-backed translator. US3 depends on the shared catalogs from US1.
- **Polish (Phase 6)**: Depends on all three user stories.

### User Story Dependencies

- **US1 (P1)**: Starts after Foundation; provides migrated app behavior and is the MVP.
- **US2 (P1)**: Starts after US1’s translator exists; adds key-by-key English fallback and failed-load recovery.
- **US3 (P2)**: Starts after US1 catalogs exist; documents and verifies future catalog maintenance.

### Within Each User Story

- Write the listed tests first and confirm they fail for the missing behavior.
- Complete implementation tasks before the story checkpoint.
- Keep the export language fixed per request; do not depend on a mutable global locale during file generation.

### Parallel Opportunities

- T002 dependency declarations and T003 catalog creation can proceed in parallel after T001.
- T005 capability configuration can proceed independently of T004 after dependencies and catalogs exist.
- In US1, frontend integration (T010–T013) and Rust menu work (T014) can proceed in parallel after T009’s contracts and key naming are agreed; T015 follows T014 because both remove strings from `src-tauri/src/constants.rs`.
- In US3, the catalog checks (T021) and authoring guide (T022) touch separate files and can proceed in parallel.

---

## Parallel Example: User Story 1

```text
After T009 confirms the shared keys:
Frontend: T010–T013 in src/
Rust:     T014 in src-tauri/src/lib.rs
Then:     T015 in src-tauri/src/export/
```

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundation; verify the bundled catalogs are present.
2. Complete US1 tests and implementation.
3. Verify stored preference compatibility, live switching, search-state preservation, menu labels, and export text.
4. Add US2 recovery before shipping so missing or failed catalogs remain usable.

### Incremental Delivery

1. Setup + Foundation establishes the plugin and shared catalogs.
2. US1 migrates current screens, menus, exports, and saved preference behavior.
3. US2 adds per-key fallback and plugin failure recovery.
4. US3 protects catalog consistency and documents future language additions.
5. Polish runs the complete quality gates and packaged-app scenarios.

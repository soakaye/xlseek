# Tasks: プロジェクト憲章準拠のプログラム改修 (Align Program with Constitution)

**Branch**: `004-align-with-constitution`  
**Input Documents**: [spec.md](spec.md), [plan.md](plan.md), [data-model.md](data-model.md), [quickstart.md](quickstart.md), [contracts/](contracts/)  

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 定数一元化とモジュール配置の初期セットアップ

- [X] T001 Create backend constant definition module file in `src-tauri/src/constants.rs` and register module in `src-tauri/src/lib.rs`
- [X] T002 [P] Create frontend constant directory and module file in `src/constants/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全ユーザーストーリーの前提となる定数群および基盤の定義

**⚠️ CRITICAL**: 本フェーズの定数定義が完了するまで各ユーザーストーリーの実装は開始できない

- [X] T003 Define core backend constants (extensions, intervals, radii, events, CSV export headers, error templates) in `src-tauri/src/constants.rs`
- [X] T004 [P] Define core frontend constants (IPC command names, event names, UI timings, default table metrics) in `src/constants/index.ts`
- [X] T005 Create constant validation unit tests in `src-tauri/src/constants.rs`

**Checkpoint**: 基盤定数モジュールが準備完了し、全ユーザーストーリーの実装へ進むことが可能

---

## Phase 3: User Story 1 - 定数値の一元管理とハードコード排除 (Priority: P1) 🎯 MVP

**Goal**: コードベース内の0と空文字以外の全リテラル（マジックナンバー、固定文字列等）を定数定義から参照し、利用箇所に参照コメントを明記する（憲章原則II）。

**Independent Test**: 定数モジュールを配置し、各コードの該当リテラルが定数参照に置換され、`// 定数参照:` コメントが存在することを検証。

### Implementation for User Story 1

- [X] T006 [P] [US1] Extract constants and add reference comments in `src-tauri/src/models/mod.rs` (default extensions)
- [X] T007 [P] [US1] Extract constants and add reference comments in `src-tauri/src/search/parser.rs` (batch check interval `0x7F`, snippet context length `30`, ellipsis `...`)
- [X] T008 [P] [US1] Extract constants and add reference comments in `src-tauri/src/search/preview.rs` (preview row/col radius, error templates)
- [X] T009 [P] [US1] Extract constants and add reference comments in `src-tauri/src/search/engine.rs` (progress notify interval `50`, event name)
- [X] T010 [P] [US1] Extract constants and add reference comments in `src-tauri/src/export/csv_export.rs` (CSV headers, error messages)
- [X] T011 [P] [US1] Extract constants and add reference comments in `src-tauri/src/export/xlsx_export.rs` (Excel sheet name, column headers, format styles)
- [X] T012 [P] [US1] Extract constants and add reference comments in `src-tauri/src/commands/search_cmd.rs` and `src-tauri/src/commands/preview_cmd.rs`
- [X] T013 [P] [US1] Extract constants and add reference comments in `src-tauri/src/commands/system_cmd.rs` and `src-tauri/src/commands/export_cmd.rs`
- [X] T014 [P] [US1] Extract constants and add reference comments in `src/hooks/useSearch.ts` (event name, command names)
- [X] T015 [P] [US1] Extract constants and add reference comments in `src/components/common/Toast.tsx` and `src/components/common/StatusBar.tsx`
- [X] T016 [P] [US1] Extract constants and add reference comments in `src/components/results/ResultTable.tsx` (row heights, column headers)
- [X] T017 [P] [US1] Extract constants and add reference comments in `src/components/search/SearchBar.tsx` and `src/components/preview/SpreadsheetGrid.tsx`

**Checkpoint**: すべてのハードコード値が外部抽出され、利用箇所に参照コメントが付与された状態となる（MVP達成）

---

## Phase 4: User Story 2 - 網羅的なヘッダコメントとドキュメンテーション (Priority: P1)

**Goal**: すべてのファイル、モジュール、構造体、関数、メソッドに4必須要素（処理詳細、引数/戻り値、エラー/例外条件、変更履歴）を含むヘッダコメントを付与する（憲章原則III）。

**Independent Test**: 全対象ファイルのヘッダおよび関数コメントを走査し、4セクションがすべて記載されていることを確認。

### Implementation for User Story 2

- [X] T018 [P] [US2] Add 4-element header comments to all files, modules, and structs in `src-tauri/src/models/mod.rs` and `src-tauri/src/constants.rs`
- [X] T019 [P] [US2] Add 4-element header comments to all modules and functions in `src-tauri/src/search/parser.rs` and `src-tauri/src/search/preview.rs`
- [X] T020 [P] [US2] Add 4-element header comments to all structs, methods, and functions in `src-tauri/src/search/engine.rs` and `src-tauri/src/search/mod.rs`
- [X] T021 [P] [US2] Add 4-element header comments to all export functions in `src-tauri/src/export/csv_export.rs`, `src-tauri/src/export/xlsx_export.rs`, and `src-tauri/src/export/mod.rs`
- [X] T022 [P] [US2] Add 4-element header comments to all command handlers in `src-tauri/src/commands/search_cmd.rs`, `src-tauri/src/commands/preview_cmd.rs`, `src-tauri/src/commands/system_cmd.rs`, `src-tauri/src/commands/export_cmd.rs`, and `src-tauri/src/commands/mod.rs`
- [X] T023 [P] [US2] Add 4-element header comments to `src-tauri/src/lib.rs` and `src-tauri/src/main.rs`
- [X] T024 [P] [US2] Add 4-element JSDoc header comments to `src/types/search.ts` and `src/constants/index.ts`
- [X] T025 [P] [US2] Add 4-element JSDoc header comments to custom hooks in `src/hooks/useSearch.ts`
- [X] T026 [P] [US2] Add 4-element JSDoc header comments to UI components in `src/components/common/WindowFrame.tsx` (via `src/components/layout/WindowFrame.tsx`), `src/components/common/StatusBar.tsx`, and `src/components/common/Toast.tsx`
- [X] T027 [P] [US2] Add 4-element JSDoc header comments to UI components in `src/components/search/SearchBar.tsx`, `src/components/results/ResultTable.tsx`, and `src/components/preview/` components
- [X] T028 [P] [US2] Add 4-element JSDoc header comments to `src/App.tsx` and `src/main.tsx`

**Checkpoint**: 全コード要素が完全なヘッダドキュメントを備え、仕様・入出力・エラー挙動・履歴が自己文書化された状態となる

---

## Phase 5: User Story 3 - 自然かつ正確な日本語品質の保証 (Priority: P1)

**Goal**: UI表示文言、エラーメッセージ、エクスポート項目、トースト通知の日本語を自然かつ正確に整え、文字化けや `<PAD>` 等の不要トークンを完全排除する（憲章原則I）。

**Independent Test**: ソースコードおよびリソース内から `<pad>` が検出されず、UI表示文言が自然な日本語であることを確認。

### Implementation for User Story 3

- [X] T029 [P] [US3] Verify and refine Japanese error messages and templates in `src-tauri/src/constants.rs` and backend command handlers
- [X] T030 [P] [US3] Verify and refine Japanese UI labels, placeholders, tooltips, and toast notifications in `src/constants/index.ts` and `src/components/`
- [X] T031 [US3] Perform automated token audit across `src/` and `src-tauri/src/` to verify zero `<PAD>` or unwanted tokens exist

**Checkpoint**: 全ユーザー向けテキストが高品質な日本語で統一され、不要トークンがゼロであることが保証される

---

## Phase 6: User Story 4 - 責務に応じたモジュール構成と標準スタイル準拠 (Priority: P2)

**Goal**: Clippy指摘事項7件の解消、`rustfmt` の適用、TypeScript型整合性の確認による公式スタイル完全準拠（憲章原則IV）。

**Independent Test**: `cargo clippy --all-targets -- -D warnings` および `cargo fmt --check`、`tsc --noEmit` が警告ゼロでパス。

### Implementation for User Story 4

- [X] T032 [P] [US4] Fix Clippy `unnecessary_sort_by` in `src-tauri/src/commands/system_cmd.rs` using `sort_by_key`
- [X] T033 [P] [US4] Fix Clippy `needless_borrows_for_generic_args` in `src-tauri/src/export/csv_export.rs`
- [X] T034 [P] [US4] Implement `Default` for `SearchEngine` in `src-tauri/src/search/engine.rs` to resolve `new_without_default`
- [X] T035 [P] [US4] Replace `cancel_flag.map_or(false, ...)` with `cancel_flag.is_some_and(...)` in `src-tauri/src/search/parser.rs`
- [X] T036 [US4] Format all Rust source files with `cargo fmt` and verify zero diff with `cargo fmt --check` in `src-tauri/`
- [X] T037 [P] [US4] Run TypeScript type-check and ensure zero compiler errors or warnings in `src/`

**Checkpoint**: 言語標準の静的解析およびスタイルガイドに完全準拠し、警告ゼロを達成

---

## Phase 7: User Story 5 - 堅牢なエラーハンドリングとテストによる品質保証 (Priority: P2)

**Goal**: 不用意な `unwrap`/`expect` パニックの排除、`Result` 型による安全なエラー伝搬、単体テストスイートの全パス（憲章原則V）。

**Independent Test**: `cargo test` が全テストパス（成功率100%）、異常系入力に対する安全なエラー返却の確認。

### Implementation for User Story 5

- [X] T038 [P] [US5] Audit and eliminate potential panic calls in `src-tauri/src/search/parser.rs` and `src-tauri/src/search/preview.rs`, replacing them with `Result` error propagation
- [X] T039 [P] [US5] Add unit test cases for constant integrity and default configurations in `src-tauri/src/constants.rs`
- [X] T040 [P] [US5] Add unit test cases for error propagation when opening invalid workbook files in `src-tauri/src/search/preview.rs`
- [X] T041 [US5] Execute full test suite `cargo test` in `src-tauri/` and ensure 100% pass rate

**Checkpoint**: 予期せぬパニックが排除され、独立した単体テストにより堅牢性が保証される

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: 憲章準拠の包括的検証と最終ビルドチェック

- [X] T042 Run quickstart validation scenarios defined in `specs/004-align-with-constitution/quickstart.md`
- [X] T043 Execute repository-wide search to confirm zero unexempt hardcoded literals and verify all constant references have explicit comments
- [X] T044 Execute repository-wide search to confirm 100% header comment coverage across all files, functions, and modules
- [X] T045 Final build verification with `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `npm run build`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 依存関係なし。直ちに開始可能。
- **Foundational (Phase 2)**: Phase 1 に依存。完了するまでユーザーストーリーに着手不可。
- **User Story 1 (Phase 3)**: Phase 2 に依存。定数抽出と参照コメントのベースラインを確立。
- **User Story 2 (Phase 4)**: Phase 3 と並行または順次実施可能。
- **User Story 3 (Phase 5)**: Phase 3 および Phase 4 に連動して文言品質を検証。
- **User Story 4 (Phase 6)**: Phase 2〜5 のコード変更に伴うスタイル・Clippy警告の解消。
- **User Story 5 (Phase 7)**: パニック排除と単体テストカバレッジの強化。
- **Polish (Phase 8)**: 全ユーザーストーリー完了後に総合検証。

### Parallel Opportunities

- Phase 1: T001 と T002 は並行実施可能。
- Phase 2: T003 と T004 は並行実施可能。
- Phase 3 (US1): バックエンド各モジュールの定数抽出（T006〜T013）およびフロントエンド各コンポーネントの定数抽出（T014〜T017）は互いに競合せず並行実施可能。
- Phase 4 (US2): 各ファイルへのヘッダコメント付与（T018〜T028）は完全に並行実施可能。
- Phase 6 (US4): Clippy個別エラー修正（T032〜T035）は並行実施可能。

---

## Implementation Strategy

### MVP First (User Story 1)
1. Phase 1 (Setup) および Phase 2 (Foundational) を完了。
2. Phase 3 (User Story 1) で定数値の一元化と参照コメントの明記を完了。
3. コードベース内のハードコードがゼロ化されたことを確認し、MVP達成。

### Incremental Delivery
1. Phase 1 + Phase 2 → 定数基盤の確立
2. Phase 3 (US1) → 定数外部化（MVP）
3. Phase 4 (US2) → 4要素ヘッダコメントの網羅
4. Phase 5 (US3) → 日本語テキスト品質と `<PAD>` 排除の確認
5. Phase 6 (US4) → Clippy・rustfmt・型検査の警告ゼロ化
6. Phase 7 (US5) → エラーハンドリング堅牢化とテスト全パス
7. Phase 8 (Polish) → 総合バリデーション

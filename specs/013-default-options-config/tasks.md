---
description: "デフォルト動作設定およびCSV/Excel出力の実装タスク"
---

<!-- 処理内容: 仕様と設計を実行順のタスクへ分解。入力: spec.md、plan.md、設計資料。出力: 検証可能な作業項目。エラー: 依存や受け入れ条件の欠落は実装前に修正。変更履歴: v1.0.0 2026-09-28 AI Agent 初版策定。 -->
# Tasks: デフォルト動作設定 (Default Options Configuration)

**Input**: `specs/013-default-options-config/` の [仕様](spec.md)、[計画](plan.md)、[データモデル](data-model.md)、[契約](contracts/default-options-contract.md)、[調査](research.md)、[クイックスタート](quickstart.md)

**Tests**: プロジェクト憲章原則 V に従い、追加・変更するデータ管理・UIロジックの単体テストを先に作成する（TDDアプローチ）。

**Organization**:
- Phase 1: Setup (定数・翻訳リソースの定義)
- Phase 2: Foundational (コアデータモデル・保存・復元ロジック)
- Phase 3: User Story 1 (デフォルト検索オプション設定・画面同期・次回起動時反映・リセット機能) 🎯 MVP
- Phase 4: User Story 2 (検索結果のCSV出力およびExcel出力の保存ダイアログフロー検証・確定)
- Phase 5: Polish & Cross-Cutting Concerns (品質ゲート・ドキュメント・総合検証)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 検索オプション初期値、ストレージキー、およびUI文言の定数・多言語カタログを整備する。

- [ ] T001 [P] `src/constants/index.ts` にデフォルト検索オプションの初期値（`DEFAULT_SEARCH_OPTIONS`: match_case=false, use_regex=false, include_formula=true, include_shape=false, include_comment=true, include_hidden=false, extensions=[".xlsx", ".xlsm", ".xlsb", ".xls"]）および永続化キー `DEFAULT_OPTIONS_STORAGE_KEY`（`"exlgrep.default_search_options"`）を追加する。
- [ ] T002 [P] `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` に設定ダイアログ内のデフォルト検索オプション項目用文言（セクション見出し、各トグル説明、拡張子選択、デフォルト復元ボタン、保存ボタン、最低1拡張子選択バリデーション警告メッセージ等）の対訳キーを追加する。

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: デフォルト設定の検証・保存・復元ロジックを独立したコアモジュールとして整備し、壊れた保存値のフォールバック動作を保証する。

- [ ] T003 [P] `tests/default-options.test.ts` に、規定の初期値ロード、正常値の保存・読込、不正値・型不一致時のフォールバック復元、空拡張子配列の防止バリデーション、および「デフォルトに戻す」リセット関数の失敗する単体テストを作成する。
- [ ] T004 `src/default-options-core.ts` に `DefaultSearchOptions` 型の型ガード `isDefaultSearchOptions`、`loadDefaultSearchOptions`、`saveDefaultSearchOptions`、`resetDefaultSearchOptions` を実装し、T003 のテストを全てパスさせる。

**Checkpoint**: デフォルト検索オプションの永続化およびバリデーションロジックが単体テストで 100% 検証され、Foundation が完了する。

---

## Phase 3: User Story 1 - デフォルト検索オプションのカスタマイズと次回起動時への反映 (Priority: P1) 🎯 MVP

**Goal**: 設定画面からデフォルト検索オプションを変更・保存でき、メイン画面の検索バーへ即座に同期され、次回起動時にも保存内容が初期値として反映される。また、出荷時既定値へのリセットボタンを提供する。

**Independent Test**: 設定画面を開きオプション（例: 数式=OFF、Shape=ON、.xlsb除外）を変更して保存した際、メイン画面の検索バーが即座に同期され、再起動後もその値で初期化されること。また「デフォルトに戻す」で規定値へ復元できることを確認する。

### Tests for User Story 1

- [ ] T005 [P] [US1] `tests/settings-default-options.test.tsx` に、設定ダイアログ内でのデフォルト検索オプション表示、チェック・トグル変更、拡張子選択変更、空拡張子保存防止、デフォルト復元ボタンクリック時の規定値復帰、保存時のコールバック呼出を検証する失敗する画面テストを作成する。
- [ ] T006 [P] [US1] `tests/app-default-options.test.tsx` に、`App.tsx` / `useSearch.ts` 起動時に保存済みデフォルト設定が検索クエリ初期値へ適用されること、設定保存時に現在表示中の検索バーへ即座に反映されることを検証する失敗する画面テストを作成する。

### Implementation for User Story 1

- [ ] T007 [US1] `src/components/settings/SettingsDialog.tsx` に「デフォルト検索オプション」セクションを追加し、大文字小文字・正規表現・数式・Shape・コメント・非表示シート・対象拡張子の設定UIと「デフォルトに戻す」リセットボタン、保存ハンドラを実装する（T005 をパスさせる）。
- [ ] T008 [US1] `src/hooks/useSearch.ts` のクエリ初期化処理において `loadDefaultSearchOptions` を参照し、デフォルトオプションでクエリ初期状態を設定する。また、外部からのデフォルトオプション更新をクエリへ即時反映するメソッド `applyDefaultOptions(options)` を実装する。
- [ ] T009 [US1] `src/App.tsx` において、`default-options-core` の読込・保存状態を管理し、`SettingsDialog` の `onSaveDefaultOptions` ハンドラから `useSearch` のクエリ状態を即時更新するように連携する（T006 をパスさせる）。

**Checkpoint**: User Story 1 単独で、デフォルト検索オプションの設定・保存・画面即時同期・次回起動時反映・初期値リセットが完全に動作する（MVP達成）。

---

## Phase 4: User Story 2 - 検索結果のCSV出力およびExcel出力（保存先ダイアログによる明示的保存） (Priority: P2)

**Goal**: 検索結果をCSVまたはExcel (.xlsx) としてエクスポートする際、OS標準の保存ダイアログで保存先フォルダとファイル名を指定して確実に保存できる。また、0件時の抑止やキャンセル時の安全性を保証する。

**Independent Test**: 検索結果が1件以上ある状態で「CSV出力」「Excel出力」をクリックし、保存ダイアログから指定したパスにファイルが正しく生成されること。0件時の警告通知、およびキャンセル時にエラーが発生しないことを確認する。

### Tests for User Story 2

- [ ] T010 [P] [US2] `tests/export-dialog.test.tsx` に、検索結果0件時のエクスポートボタン押下でダイアログが開かず警告トーストが表示されること、1件以上の結果で保存ダイアログ呼び出しとエクスポートコマンドが正しくトリガーされること、ダイアログキャンセル時に例外が発生しないことを検証する単体テストを作成する。

### Implementation for User Story 2

- [ ] T011 [US2] `src/components/common/StatusBar.tsx` のエクスポート処理（`handleExport`）において、仕様書（FR-005, FR-006, FR-007, FR-008）および受入シナリオ・エッジケースに適合していることを検証・整備し、T010 のテストをパスさせる。

**Checkpoint**: User Story 1 と User Story 2 が共に独立して動作し、全受入基準を満たす。

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: 品質ゲート（TypeScriptビルド、Clippy、rustfmt、テスト網羅、UIモック同期）の検証と最終調整。

- [ ] T012 [P] `design/mainui/index.html` の設定ダイアログプロトタイプおよびステータスバーエクスポートUIを更新し、最新の実装UIと完全に同期させる（`syncing-mainui-mock` スキルに準拠）。
- [ ] T013 `specs/013-default-options-config/quickstart.md` に記載された全シナリオを手動・自動で実行し、期待される動作を検証する。
- [ ] T014 全品質ゲート（`npm run build`, `npm test`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`）を実行し、エラーおよび警告が 0 件であることを確認する。

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: 依存関係なし。即時開始可能。
- **Phase 2 (Foundational)**: Phase 1 完了に依存。User Story 1 の着手をブロック。
- **Phase 3 (User Story 1 - MVP)**: Phase 2 完了に依存。
- **Phase 4 (User Story 2)**: Phase 1 完了に依存（US1 と並行して進めることも可能）。
- **Phase 5 (Polish)**: Phase 3 および Phase 4 完了に依存。

### Parallel Opportunities

- **Phase 1**: T001 (`constants/index.ts`) と T002 (`locales/*.yml`) は並列実行可能 `[P]`。
- **Phase 2**: T003 (テスト) と T004 (実装) は TDD 順序で進行。
- **Phase 3**: T005 (設定ダイアログテスト) と T006 (App連携テスト) は並列作成可能 `[P]`。
- **Phase 4**: T010 (エクスポートテスト) は US1 と並行して作成可能 `[P]`。
- **Phase 5**: T012 (UIモック同期) は独立して実行可能 `[P]`。

---

## Parallel Example: User Story 1

```bash
# Launch test creation for User Story 1 in parallel:
Task: T005 [P] [US1] Create tests in tests/settings-default-options.test.tsx
Task: T006 [P] [US1] Create tests in tests/app-default-options.test.tsx
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: 定数・対訳キーの追加 (T001, T002)
2. Phase 2: コアロジックとテスト (T003, T004)
3. Phase 3: デフォルト設定UIと画面同期の実装 (T005〜T009)
4. **検証**: User Story 1 を独立して動作確認し、MVP を確認。

### Incremental Delivery

1. MVP 完成後、Phase 4 (User Story 2: CSV/Excel出力の保存ダイアログフローテストと整合性検証) を実施。
2. Phase 5 で UI モック同期（`design/mainui/index.html`）および総合品質ゲートをパスして完了。

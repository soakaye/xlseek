<!-- 処理内容: 履歴一覧のキー操作を実装・検証する作業を依存順に定義する。引数・戻り値: spec.md、plan.md、設計資料を入力とし、実行可能なタスク一覧を出力する。エラー: 前提タスクや品質ゲートが未完了なら完了扱いにしない。変更履歴: v1.0.0 (2026-09-27, Codex): 初版作成。v1.0.1 (2026-09-27, Codex): 2キー以内と20件全項目の検証を明記。 -->
# Tasks: 履歴一覧のキーボード操作

**Input**: `specs/011-history-keyboard-selection/` の仕様・計画・設計資料

**Prerequisites**: [plan.md](plan.md)、[spec.md](spec.md)、[research.md](research.md)、[data-model.md](data-model.md)、[UI contract](contracts/history-keyboard-ui-contract.md)

**Tests**: 憲章原則 V に従い、既存の画面操作テストに回帰ケースを追加する。

**Organization**: P1 と P2 を別々に確認できる順序で実装する。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 依存タスクの完了後、別ファイルで並行して進められる。
- **[Story]**: 仕様の User Story 1 または 2 に対応する。
- 記載パスはリポジトリ直下からの相対パス。

## Phase 1: Setup

**Purpose**: 既存の履歴画面とテストを再利用する前提を確認する。

- [X] T001 `src/components/search/SearchBar.tsx`、`tests/search-history-ui.test.tsx`、`design/mainui/index.html` の履歴ボタン・項目・フォーカス離脱・補完の現在動作を確認し、`specs/011-history-keyboard-selection/contracts/history-keyboard-ui-contract.md` の対象操作を対応付ける。

---

## Phase 2: Foundational

**Purpose**: 両ストーリーで使うキーと固定値を憲章に沿って準備する。

- [X] T002 `src/constants/index.ts` に新たに参照するキー名・固定値を定義し、`src/components/search/SearchBar.tsx` の追加箇所で定数参照コメントを使えるようにする。不要な定数や依存パッケージは追加しない。

**Checkpoint**: 共通の固定値を参照でき、P1・P2 の作業を開始できる。

---

## Phase 3: User Story 1 - 履歴をキーだけで選ぶ (Priority: P1) 🎯 MVP

**Goal**: 両履歴のボタンから項目へ移り、上下矢印・Tab で選んで Enter で対応する入力欄へ反映する。

**Independent Test**: 両履歴を 2 件以上用意し、履歴ボタンへ Tab で移動して Enter で開き、Tab または下矢印で先頭へ移る。上下矢印は両端で止まり、Tab は末尾から先頭へ戻る。Enter は対応欄だけを更新し、検索を開始しない。

### Tests for User Story 1

- [X] T003 [US1] `tests/search-history-ui.test.tsx` に両履歴のボタンからの Tab／下矢印、上下矢印の両端、Tab の循環、0 件での通常の Tab、1 件での Tab、項目減少時の無効選択防止、Enter 確定、フォーカス・視覚状態と `aria-selected` の一致、検索非開始を検証する失敗テストを追加する。履歴表示後の先頭項目選択が 2 キー以内であることと、20 件の各項目へキーだけで到達して選択または取消しできることも検証する。

### Implementation for User Story 1

- [X] T004 [P] [US1] `src/components/search/SearchBar.tsx` で既存の項目ボタンと `selectOption` を使い、両履歴のボタンから先頭項目への移動、上下矢印の範囲内移動、Tab の末尾から先頭への循環、Enter 確定、フォーカス表示を実装する。`data-model.md` の「0～50 件。値の重複は既存の履歴管理が防ぐ」「表示項目に含まれる場合だけ有効。空一覧では存在しない」「確定時だけ対応する入力欄を更新する」を守り、表示中に履歴が減っても無効な項目を選ばない。
- [X] T005 [P] [US1] `design/mainui/index.html` の検索テキスト・検索ディレクトリ履歴に T004 と同じボタンからの遷移、項目移動、0 件・1 件の Tab、Tab 循環、Enter 確定、フォーカス表示を反映する。

**Checkpoint**: P1 の自動テストと画面操作で、両履歴のキー選択を単独で確認できる。

---

## Phase 4: User Story 2 - 選択せずに履歴を閉じる (Priority: P2)

**Goal**: Escape で入力値を保って一覧を閉じ、対応する入力欄へ戻る。IME 変換中や一覧外クリックでも誤操作を起こさない。

**Independent Test**: マウスで履歴項目へフォーカスを移した状態からでも Escape で一覧を閉じ、対応欄へ戻り、値を維持できる。IME 変換中は履歴操作を起こさず、一覧外クリックでは値を変えずに閉じる。

### Tests for User Story 2

- [X] T006 [US2] `tests/search-history-ui.test.tsx` に両履歴の Escape と入力欄への復帰、一覧外クリック、IME 変換中の無反応、既存のディレクトリ補完とマウス選択の回帰を検証する失敗テストを追加する。

### Implementation for User Story 2

- [X] T007 [P] [US2] `src/components/search/SearchBar.tsx` で項目上の Escape による `closeList` と対応入力欄への復帰、IME ガードを実装し、既存の一覧外クリック、ディレクトリ補完、マウス選択を維持する。
- [X] T008 [P] [US2] `design/mainui/index.html` に Escape での入力欄復帰と IME 変換中の挙動を反映し、既存の一覧外クリックで閉じる動作を維持する。

**Checkpoint**: P2 の自動テストと画面操作で、取消しと境界条件を P1 と独立に確認できる。

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: 全体の品質ゲートと実画面の受け入れ条件を確認する。

- [X] T009 `specs/011-history-keyboard-selection/quickstart.md` の自動検証に従い、`tests/search-history-ui.test.tsx` を含む `npm test`、`npm run lint`、`npm run build`、`src-tauri/` で `cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check` を実行し、警告・失敗を解消する。
- [X] T010 `specs/011-history-keyboard-selection/quickstart.md` の画面手順で両履歴のキー操作とディレクトリ補完を確認し、`design/mainui/index.html` の構文と表示の同期、`src/components/search/SearchBar.tsx` と `src/constants/index.ts` の定数参照・4 要素ヘッダ、生成文書の不要トークンを点検する。

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 の T001 → Phase 2 の T002 → P1 または P2。
- 同じ `tests/search-history-ui.test.tsx` と `src/components/search/SearchBar.tsx` を編集するため、統合順は P1 → P2 とする。P2 の受け入れ条件は、既存のマウスによる履歴表示と項目フォーカスだけでも独立に検証できる。
- 最終 Phase は P1 と P2 の完了後に行う。

### User Story Dependencies

- **US1 (P1)**: T001・T002 の後に開始。T003 → T004／T005。
- **US2 (P2)**: T001・T002 の後に開始可能。共有ファイルの変更衝突を避ける実行順は US1 の後。T006 → T007／T008。

### Parallel Opportunities

- US1: T003 の失敗を確認後、別ファイルの T004 と T005 を並行できる。
- US2: T006 の失敗を確認後、別ファイルの T007 と T008 を並行できる。
- T009 は全体の結果に依存するため、並行対象にしない。

## Parallel Example: User Story 1

```text
T004: src/components/search/SearchBar.tsx の履歴キー操作
T005: design/mainui/index.html の同操作
```

## Parallel Example: User Story 2

```text
T007: src/components/search/SearchBar.tsx の取消し・境界条件
T008: design/mainui/index.html の同操作
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001～T002 を完了する。
2. T003 を追加して失敗を確認する。
3. T004～T005 を実装し、P1 の両履歴を単独で確認する。

### Incremental Delivery

1. P1 の選択操作を完成させる。
2. T006～T008 で P2 の取消しと境界条件を完成させる。
3. T009～T010 で全品質ゲートと実画面を確認する。

## Notes

- [P] は、先行テスト後に別ファイルで進められる作業だけに付ける。
- キー操作の結果は [UI contract](contracts/history-keyboard-ui-contract.md) に従う。
- 機能追加に伴うテストと全品質ゲートは `.specify/memory/constitution.md` と `AGENTS.md` の必須条件に従う。

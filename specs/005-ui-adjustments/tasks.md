---
description: "Task list for UI adjustments feature implementation"
---

# Tasks: UI項目調整 (UI Adjustments)

**Input**: Design documents from `/specs/005-ui-adjustments/`

**Prerequisites**: [plan.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/plan.md) (required), [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/spec.md) (required for user stories), [research.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/research.md), [data-model.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/data-model.md), [contracts/](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/contracts/)

**Tests**: E2Eおよび手動検証シナリオは [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/quickstart.md) に定義。自動ビルド・検証コマンド（`npm run build`, `cargo test`, `python3 html.parser`）を含む。

**Organization**: ユーザーストーリー単位（US1, US2, US3）でタスクをグループ化し、独立した実装・検証・MVPリリースを可能とする。

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並行実行可能（異なるファイル、依存関係なし）
- **[Story]**: 紐づくユーザーストーリー（US1, US2, US3）
- 各タスク記述に対象ファイルパスを明記

## Path Conventions

- フロントエンド（React）: `src/`
- デザインプロトタイプ（HTMLモック）: `design/mainui/index.html`
- バックエンド（Tauri Rust）: `src-tauri/`
- 仕様・検証ドキュメント: `specs/005-ui-adjustments/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 共通定数、型定義、および検証環境の基盤整備

- [ ] T001 定数定義ファイル `src/constants/index.ts` にUI文言（探索中表示、フォルダスキャン）、拡張子定義、およびレイアウト定数を定義・整理
- [ ] T002 [P] TypeScript型定義ファイル `src/types/index.ts` に検索進行フェーズ（`SearchPhase`）、探索状態、およびプレビュー用モデルの型を定義
- [ ] T003 [P] 検証環境の健全性確認（`npm run build`, `cargo test --manifest-path src-tauri/Cargo.toml`）

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: ユーザーストーリーの実装開始前に完了必須となる共通ステートおよびプロトタイプ基盤の確立

**⚠️ CRITICAL**: 共通ステートフックおよびモック基礎定義が完了するまで、ユーザーストーリーの個別実装には着手しないこと

- [ ] T004 `src/hooks/useSearch.ts` の状態管理にフォルダ走査フェーズ（`scanning`）および探索中フォルダパスの保持ロジックを追加
- [ ] T005 [P] `design/mainui/index.html` に共通状態変数（`currentPhase`, `targetDir`, `isComposing`, `compositionEndTime`）の管理機構を定義

**Checkpoint**: 共通ステート・定数基盤が整い、各ユーザーストーリーの実装へ並行して着手可能

---

## Phase 3: User Story 1 - 検索バー・フォルダ入力欄のキー入力制御と拡張子選択 (Priority: P1) 🎯 MVP

**Goal**: 日本語IME確定時のEnter誤爆検索を完全抑止し、フォルダパス入力欄でのEnterキー無効化（フォーカス維持）および対象拡張子のトグルボタンスタイリング（最低1つ維持）を実現する。

**Independent Test**:
1. キーワード入力欄で日本語入力・IME変換確定のEnterを押下し、検索が開始されないことを確認。確定後に再度Enterを押下して検索が開始されることを確認。
2. フォルダ入力欄でEnterを押下し、検索が開始されずフォーカスが維持されることを確認。
3. 拡張子ボタンをクリックしてトグルし、残り1つの状態で解除が抑止されることを確認。

### Implementation for User Story 1

- [ ] T006 [P] [US1] `src/components/search/SearchBar.tsx` に日本語IME変換確定時のEnter誤爆防止ガード（`isComposing`, `keyCode === 229`, `compositionEndTime` < 50ms）を実装
- [ ] T007 [P] [US1] `src/components/search/SearchBar.tsx` のフォルダ入力欄（`#folderInput`）においてEnterキー押下時の検索実行を抑止し、フォーカスを維持するイベントハンドラを実装
- [ ] T008 [US1] `src/components/search/SearchBar.tsx` に対象拡張子トグルボタン（`.xlsx`, `.xlsm`, `.xlsb`, `.xls`）の選択・解除ロジックおよび全解除防止ガード（最低1つ選択維持）を実装
- [ ] T009 [US1] `design/mainui/index.html` において検索キーワード欄のIME確定保護、フォルダ欄のEnter抑止、および拡張子トグル制御を1:1で同期実装（[contracts/search-bar-events.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/contracts/search-bar-events.md) 準拠）
- [ ] T010 [US1] [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/quickstart.md) のシナリオ1、2、3に基づき、ブラウザおよびTauri環境でキー入力および拡張子トグル動作を手動検証

**Checkpoint**: 検索バーおよびフォルダ入力欄のキー入力制御・拡張子選択が独立して動作し、誤爆率0%を達成（MVP完成）

---

## Phase 4: User Story 2 - ステータスバーの進捗表示改善とフォルダスキャン可視化 (Priority: P2)

**Goal**: ステータスバーにおいてプログレスバーをメッセージの手前に固定幅（幅44相当）で配置し、ファイル解析開始前のフォルダ探索中フェーズのアニメーションとパス表示を提供し、不要な内部エンジン表記を削除する。

**Independent Test**:
検索実行直後、ステータスバーにおいてプログレスバーが左側に固定幅で配置され、フォルダ探索中パルスアニメーションと対象パスが表示された後、ファイル解析・セル走査の進捗率表示へ切り替わり、完了時に結果概要が表示されることを確認。

### Implementation for User Story 2

- [ ] T011 [P] [US2] `src/components/common/StatusBar.tsx` においてプログレスバーをメッセージの手前に固定幅（`w-44`）で配置し、フォルダ探索中（`scanning`）フェーズのアニメーション表示とパス表示を実装
- [ ] T012 [P] [US2] `src/components/common/StatusBar.tsx` から内部エンジン名称（Calamine Engine）の固定テキストを削除し、ステータス領域を整理
- [ ] T013 [US2] `design/mainui/index.html` のステータスバー表示ロジックを更新し、プログレスバーの固定幅配置（`w-44`）、フォルダスキャン中フェーズのアニメーション、および内部エンジン表記の削除を1:1同期
- [ ] T014 [US2] [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/quickstart.md) のシナリオ4に基づき、検索ライフサイクルを通じたステータスバー表示の変化を手動検証

**Checkpoint**: ステータスバーの進捗表示とフォルダ探索中フェーズが独立して機能し、視覚的フィードバックが向上

---

## Phase 5: User Story 3 - 周辺セルプレビューの縦横両方向スクロールと固定見出し (Priority: P3)

**Goal**: 周辺セルプレビューグリッドにおいて、縦横両方向のスクロールをサポートし、列見出しおよび行番号見出しを固定（Freeze Panes）表示し、十分なサンプル行（30行以上）によりスクロール操作を快適に検証可能とする。

**Independent Test**:
プレビュー領域を縦方向および横方向にスクロールした際、列見出し（A, B, C...）が上端に、行番号（8, 9, 10...）が左端に、角の `#` が左上端に常に固定され、セルクリックで番地・数式が連動更新されることを確認。

### Implementation for User Story 3

- [ ] T015 [P] [US3] `src/components/preview/SpreadsheetGrid.tsx` においてフリーズペイン（Freeze Panes）スタイル（`sticky top-0 z-20`, `sticky left-0 z-10`, `sticky left-0 top-0 z-30`）を適用し、縦横両スクロールコンテナを整備（[contracts/spreadsheet-grid-layout.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/contracts/spreadsheet-grid-layout.md) 準拠）
- [ ] T016 [P] [US3] `src/components/preview/SpreadsheetGrid.tsx` および `src/components/preview/FormulaBar.tsx` においてスクロール中のセルクリック・選択状態・番地/数式連動表示を維持
- [ ] T017 [US3] `design/mainui/index.html` においてプレビューグリッドのサンプル行を30行以上（行8〜40）に拡充し、フリーズペイン固定見出しおよびスクロール動作を1:1同期
- [ ] T018 [US3] [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/quickstart.md) のシナリオ5に基づき、30行以上の縦横スクロールと固定見出しの挙動を手動検証

**Checkpoint**: 縦横双方向スクロールと固定見出しが完全動作し、大規模データでも見出しを見失わずにプレビュー可能

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: コード品質、憲章準拠、モック同期（Iron Law）、および自動テストの総合検証

- [ ] T019 [P] `design/mainui/index.html` の構文およびDOM整合性を `python3 -c "import html.parser; ..."` で自動検証
- [ ] T020 [P] TypeScript型チェックおよびViteビルド（`npm run build`）、Rustバックエンドテスト（`cargo test --manifest-path src-tauri/Cargo.toml`）を実行してエラーゼロを確認
- [ ] T021 [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/fixui/specs/005-ui-adjustments/quickstart.md) に記載の全5シナリオのエンドツーエンド総合検証を実施
- [ ] T022 変更対象ファイル（`SearchBar.tsx`, `StatusBar.tsx`, `SpreadsheetGrid.tsx`, `index.html`）のヘッダコメント（目的・構成・例外・変更履歴）が憲章原則IIIに準拠していることを監査

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 依存関係なし - 即時着手可能
- **Foundational (Phase 2)**: Phase 1 完了に依存 - 全ユーザーストーリーの実装をブロック
- **User Stories (Phase 3+)**: Phase 2 完了に依存
  - 優先度順（P1: US1 → P2: US2 → P3: US3）または並行して実装可能
- **Polish (Phase 6)**: 全ユーザーストーリーの実装完了に依存

### User Story Dependencies

- **User Story 1 (P1)**: Phase 2 完了後に着手可能。他ストーリーへの依存なし（MVP対象）。
- **User Story 2 (P2)**: Phase 2 完了後に着手可能。ステータス管理フック（`useSearch.ts`）を利用。
- **User Story 3 (P3)**: Phase 2 完了後に着手可能。独立したプレビューコンポーネントおよびモックデータ。

### Within Each User Story

- 共通型・定数の参照確認
- Reactコンポーネントの実装
- HTMLモック（`design/mainui/index.html`）への同期反映（Iron Law）
- 各ストーリーの独立検証シナリオ実行

### Parallel Opportunities

- Phase 1 の T002, T003 は並行実行可能
- Phase 2 の T005 は T004 と並行実行可能
- Phase 3 の T006, T007 は並行実行可能
- Phase 4 の T011, T012 は並行実行可能
- Phase 5 の T015, T016 は並行実行可能
- Phase 6 の T019, T020 は並行実行可能

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup 完了
2. Phase 2: Foundational 完了
3. Phase 3: User Story 1 実装（IME確定Enter誤爆防止、フォルダEnter無効化、拡張子選択）
4. **検証と停止**: シナリオ1〜3で手動検証し、MVPとして動作確認

### Incremental Delivery

1. Setup + Foundational → 基盤完成
2. User Story 1 → IME/フォルダキー制御・拡張子選択（MVP!）
3. User Story 2 → ステータスバー進捗バー配置＆フォルダ探索表示
4. User Story 3 → 周辺セルプレビューFreeze Panes＆30行スクロール検証
5. Polish → 全自動検証＆憲章ヘッダ監査

---

## Notes

- `[P]` タスク = 異なるファイル、依存関係なし
- `[Story]` タグは各タスクをユーザーストーリー（US1, US2, US3）に紐づけ、追跡性を保証
- 憲章原則IIに基づき、定数は `src/constants/index.ts` から参照すること
- Iron Law に基づき、`src/` の変更は `design/mainui/index.html` に必ず1:1で同期すること

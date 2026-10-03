---
description: "検索履歴とディレクトリ補完の実装タスク"
---

<!-- 処理内容: 仕様と設計を実行順のタスクへ分解。入力: spec.md、plan.md、設計資料。出力: 検証可能な作業項目。エラー: 依存や受け入れ条件の欠落は実装前に修正。変更履歴: v1.0.0 2026-09-27 Codex 初版。v1.1.0 2026-09-27 Codex 保存上限と Windows パス試験を更新。v1.2.0 2026-09-27 Codex 事前検証と補完境界のタスクを明確化。v1.3.0 2026-09-27 Codex 履歴データ制約と正規表現エラー試験を明記。v1.4.0 2026-09-27 Codex 実装結果と確認範囲を反映。 -->
# Tasks: 検索履歴とディレクトリ補完

**Input**: `specs/010-search-history-path-completion/` の [仕様](spec.md)、[計画](plan.md)、[データモデル](data-model.md)、[契約](contracts/search-history-path-contract.md)、[調査](research.md)

**Tests**: プロジェクト憲章原則 V に従い、変更する機能の単体テストを先に作成する。

**Organization**: US1 は履歴、US2 は保存件数、US3 はパス補完。`[P]` は同じフェーズ内で別ファイルを変更し、先行タスクに依存しない作業だけに付ける。

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 既存プロジェクトの定数と翻訳資源を拡張する。新しい依存やアプリ骨格は作らない。

- [X] T001 [P] `src/constants/index.ts` に履歴保存キー、既定 20 件・範囲 0～50 件、補完の待機時間、`complete_directory_path` コマンド名を定数として追加する。
- [X] T002 [P] `src-tauri/src/constants.rs` に `~/` 判定文字列、補完候補の最大返却数、必要な固定エラー文言を定数として追加する。
- [X] T003 [P] `src-tauri/locales/ja.yml` と `src-tauri/locales/en.yml` に履歴ボタン・空状態・補完候補・保存件数・保存失敗の対訳キーを追加し、既存の設定画面タイトルと説明を言語設定専用から設定全体に合う表現へ直す。

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 検索開始成功を履歴追加の根拠にできるよう、パス処理と受付前検証を整える。

- [X] T004 `src-tauri/src/search/path.rs` に絶対パス・`~/`・Windows の `~\` の解決、空欄・存在しない・読めないディレクトリの拒否、空白・日本語を含むパスの失敗する単体テストを先に記述し、`src-tauri/src/search/mod.rs` からモジュールを公開する。
- [X] T005 T004 のテストを通すパス解決・検索前の読取検証を `src-tauri/src/search/path.rs` に実装し、元の入力表記を変えず検索用の実パスを返す。
- [X] T006 `src-tauri/src/commands/search_cmd.rs` の `start_search` で、検索処理起動前の必須入力・T005 のパス・正規表現の妥当性と、不正な正規表現に `InvalidRegex` を返す単体テストを先に記述する。読取確認はブロッキング用スレッドで完了させ、失敗時は既存の構造化エラーを返して起動せず、成功時は解決済みパスを検索エンジンへ渡す。`src-tauri/src/search/engine.rs` の実行時検証は残す。

**Checkpoint**: 存在しない・読めないディレクトリと不正な正規表現では検索開始コマンドが成功を返さず、`~/` の実パスでは検索を開始できる。

---

## Phase 3: User Story 1 - 過去の検索条件を再利用する (Priority: P1) 🎯 MVP

**Goal**: 両入力欄の履歴を保存・再表示し、専用ボタンから選んで再利用できる。

**Independent Test**: 有効な検索を複数回開始し、両履歴の新しい順・重複排除・再起動後の保持・選択時に検索しないことを確認する。無効パスと不正な正規表現は保存せず、中断・結果ゼロ件は保存する。

### Tests for User Story 1

- [X] T007 [P] [US1] `tests/search-history.test.ts` に、`maxEntries` の初期値 20、`keywords` と `directories` が文字列配列・重複なし・新しい順・各 `maxEntries` 件以下となること、空白のみの値を追加しないこと、完全一致の再利用、壊れた保存値、保存失敗、再読込の失敗する単体テストを作る。
- [X] T008 [P] [US1] `tests/search-history-ui.test.tsx` に、両欄の履歴ボタン、選択時の片欄だけの更新、検索の非起動、検索受付成功後だけの履歴追加、不正な正規表現での履歴非追加、キーボード操作と IME 変換中の非確定を確認する失敗する画面テストを作る。

### Implementation for User Story 1

- [X] T009 [US1] `src/hooks/useSearchHistory.ts` に単一の端末内保存値から `maxEntries`・`keywords`・`directories` を検証して読み、検索受付時に空白のみの値を追加せず、両配列を完全一致で重複排除して先頭へ移し、各上限で切って保存する処理を実装する。元の入力表記を保存し、検索オプションと結果は保存しない。読書き失敗は検索を止めず保存失敗を通知する。
- [X] T010 [US1] `src/hooks/useSearch.ts` の `start_search` 成功応答後だけ元の検索テキスト・入力パスを通知し、`src/App.tsx` から T009 の履歴と通知を検索フック・検索バーへ渡す。失敗・中断時の履歴条件を T008 で確認する。
- [X] T011 [US1] `src/components/search/SearchBar.tsx` に両入力欄の履歴専用ボタンと新しい順のドロップダウンを加え、マウス・キーボード選択は対応する入力欄だけに反映し、IME 変換中の Enter と検索開始を阻害しない。

**Checkpoint**: US1 だけで履歴の保存・再起動後の再利用が動き、既存検索・フォルダ選択・ドラッグ＆ドロップを維持する。

---

## Phase 4: User Story 2 - 保存件数を調整する (Priority: P2)

**Goal**: 設定画面から両履歴共通の保存件数を変更できる。

**Independent Test**: 保存済み履歴を用意し、0・2・50 件と無効値を設定して、即時切り詰め・0 件での消去・再起動後の設定保持を確認する。

### Tests for User Story 2

- [X] T012 [P] [US2] `tests/search-history.test.ts` に、`maxEntries` は 0～50 の整数、既定 20、2 への減少で両配列を各 2 件以内に切ること、0 で両配列を消去し以後追加しないこと、保存と再読込の失敗する単体テストを作る。
- [X] T013 [P] [US2] `tests/settings-history.test.tsx` に、設定画面で現在値を表示し、範囲外・整数でない値を拒否して範囲を示し、有効値だけを確定する失敗する画面テストを作る。

### Implementation for User Story 2

- [X] T014 [US2] `src/hooks/useSearchHistory.ts` に `maxEntries` を 0～50 の整数として更新し、両履歴を直ちに上限で切って単一保存値へ保存する操作を追加する。0 は両配列を空にし、保存失敗は通知する。
- [X] T015 [US2] `src/components/settings/SettingsDialog.tsx` に履歴保存件数の入力・現在値・許容範囲の案内を加え、無効値の確定を防ぎ、既存の言語設定操作を維持する。
- [X] T016 [US2] `src/App.tsx` で T014 の件数と変更操作を設定画面へ接続し、設定変更直後の検索バーと再起動後の履歴表示が上限に従うようにする。

**Checkpoint**: 設定画面の値と両履歴の件数が一致し、0 件では保存済み履歴が再起動後も復活しない。

---

## Phase 5: User Story 3 - ディレクトリのパスを補完する (Priority: P2)

**Goal**: OS に応じた絶対パスとホーム省略表記の入力中に、親直下のディレクトリだけを補完できる。

**Independent Test**: 共通の接頭辞、通常ファイル、空白・日本語、`~/`、相対パス、読めない親を含む試験用フォルダで候補と選択結果を確認する。Windows ではドライブ付き絶対パス、UNC 共有パス、`~\`、ドライブ相対・ルート相対パスも確認する。マウスなしの操作と IME 変換中の非確定も確認する。

### Tests for User Story 3

- [X] T017 [P] [US3] `src-tauri/src/search/path.rs` に親直下の前方一致・安定順・返却件数上限、末尾の区切り文字で空の接頭辞、通常ファイル除外、絶対パス・ホーム省略表記維持、相対パス・読めない親の空配列、空白・日本語の失敗する単体テストを追加する。Windows 専用テストではドライブ付き絶対パス、UNC 共有パス、`~\`、ドライブ相対・ルート相対・デバイス・拡張長パスの除外を検証する。
- [X] T018 [P] [US3] `tests/search-history-ui.test.tsx` に、入力中の候補、履歴ボタンでの候補切替、古い非同期応答の破棄、矢印・Enter・Escape、IME 変換中の非確定、候補選択時の検索非起動を確認する失敗する画面テストを作る。

### Implementation for User Story 3

- [X] T019 [US3] `src-tauri/src/search/path.rs` に親ディレクトリ直下の非再帰的な候補列挙、前方一致、末尾の区切り文字で空の接頭辞、ファイル除外、整列後の件数制限、ホーム省略表記維持を実装し、T017 のテストを通す。Windows のドライブ付き・UNC 絶対パスを扱い、接頭辞の種類を確認して現在位置に依存するパスとデバイス・拡張長パスは補完しない。
- [X] T020 [US3] `src-tauri/src/commands/search_cmd.rs` に列挙をブロッキング用スレッドで実行して `string[]` を返す `complete_directory_path` IPC を追加し、`src-tauri/src/lib.rs` に登録する。読めない親・対象外の入力は空配列とし、入力欄の値を変えない。
- [X] T021 [US3] `src/components/search/SearchBar.tsx` で入力中の補完 IPC を待機時間付きで呼び、古い応答を破棄して候補を表示する。履歴ボタンで候補を閉じ、マウス・キーボード選択、IME、フォルダ選択・ドラッグ＆ドロップの既存動作を T018 で確認する。

**Checkpoint**: 候補選択後の絶対パスとホーム省略表記が意図したディレクトリを指し、検索欄は自動実行しない。

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 実画面・試作・品質ゲートを揃える。

- [X] T022 [P] `design/mainui/index.html` に両履歴ボタン、ディレクトリ候補、保存件数の操作と表示を反映し、`src/components/search/SearchBar.tsx`・`src/components/settings/SettingsDialog.tsx` の試作と整合させる。
- [X] T023 `src/constants/index.ts`、`src-tauri/src/constants.rs`、変更した `src/`・`src-tauri/src/` の各ファイルで固定値の定数参照コメントと処理・入出力・エラー・変更履歴の4要素ヘッダを確認し、`src-tauri/locales/{ja,en}.yml` の文言対応を検査する。
- [X] T024 `specs/010-search-history-path-completion/quickstart.md` に従い `npm test`、`npm run lint`、`npm run build`、`src-tauri` で `cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check` を実行し、デスクトップで履歴・設定・補完・日英表示・IME・保存失敗を確認する。

---

**環境確認メモ**: macOS で自動品質ゲートをすべて実行し、起動中のアプリで設定画面と空履歴表示を目視確認した。Windows 専用の実行時確認と保存失敗の手動注入はこの環境では行っていない。Windows 条件付きテストはコードに追加済み。

## Dependencies & Execution Order

- **Phase 1 → Phase 2**: T001～T003 の後、T004→T005→T006。T001～T003 は互いに独立。
- **US1**: Phase 2 の後、T007・T008 の失敗を確認して T009→T010→T011。これが MVP。
- **US2**: US1 の履歴管理に依存する。T012・T013 の失敗を確認して T014→T015→T016。US1 の画面動作と分けて件数変更を検証できる。
- **US3**: Phase 2 のパス処理に依存する。T017・T018 の失敗を確認して T019→T020→T021。T021 は US1 の `SearchBar.tsx` 完了後に行う。Rust 側 T017・T019・T020 は US1 の画面作業と並行できる。
- **Polish**: T022 は UI が確定した後、T023→T024 は全ストーリーの実装後に行う。

## Parallel Execution Examples

- **US1**: T007 (`tests/search-history.test.ts`) と T008 (`tests/search-history-ui.test.tsx`) は別ファイルの失敗するテストとして並行して作成できる。
- **US2**: T012 (`tests/history-limit.test.ts`) と T013 (`tests/settings-history.test.tsx`) は別ファイルの失敗するテストとして並行して作成できる。
- **US3**: T017 (`src-tauri/src/search/path.rs`) と T018 (`tests/path-completion.test.tsx`) は別ファイルの失敗するテストとして並行して作成できる。T019 以降は T017 の完了を待つ。

## Implementation Strategy

1. T001～T011 を完了し、US1 の履歴保存・再利用を単独で検証する。
2. T012～T016 で件数設定を加えて US2 を検証する。
3. T017～T021 で補完を加えて US3 を検証する。
4. T022～T024 で試作と実画面を揃え、[quickstart.md](quickstart.md) の全検証を完了する。

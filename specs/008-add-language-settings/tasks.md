---

description: "表示言語の自動選択と設定画面の実装タスク"
---

# Tasks: 表示言語の自動選択と設定画面

**Input**: `specs/008-add-language-settings/` の `spec.md`、`plan.md`、`research.md`、`data-model.md`、`contracts/localization-contract.md`、`quickstart.md`

**Tests**: 憲章が変更・追加するモジュールの単体テストを要求するため、各ストーリーに実装前のテストタスクを置く。すべてのコード変更で定数の一元管理と4要素ヘッダコメントを適用する。

**Organization**: US1 は未設定時の自動表示、US2 は設定画面と保存、US3 は英語ログ。各フェーズの独立確認後に次へ進む。

## Format: `[ID] [P?] [Story] Description`

- `[P]` は別ファイルで、先行タスクの完了後にほかの `[P]` タスクと並行可能。
- `[US1]`、`[US2]`、`[US3]` は `spec.md` のユーザーストーリーに対応する。
- パスはすべてリポジトリのルートからの相対パス。

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 端末言語取得の依存関係と権限を用意する。

- [X] T001 Tauri OS 情報プラグインを `package.json` と `src-tauri/Cargo.toml` に追加し、`package-lock.json` と `src-tauri/Cargo.lock` を同期して、`src-tauri/capabilities/default.json` に `os:allow-locale` を許可する
- [X] T002 `src-tauri/src/lib.rs` で OS 情報プラグインを登録し、既存の起動・メニュー初期化を維持する
- [X] T003 `package.json` と `package-lock.json` に `vitest`・`jsdom`・`eslint`・`typescript-eslint` の開発依存と `test`・`lint` スクリプトを追加し、`vitest.config.ts` で jsdom を、`eslint.config.js` で TypeScript/React とテストファイルの警告0件検査を設定する

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: すべてのストーリーが使う言語値と文言の契約を定義する。

- [X] T004 [P] `src/constants/index.ts` に `default`・`ja`・`en` の設定値、`ja`・`en` の表示言語、`exlgrep.language` 保存キー、英語を基準とする日英の文言辞書を定義し、既存の固定文言を移す
- [X] T005 [P] `src-tauri/src/constants.rs` に同じ表示言語コード、英語ログ文、日英のメニュー・CSV・Excel 見出しを定義し、既存の固定文言を移す
- [X] T006 `src/types/search.ts` と `src-tauri/src/models/mod.rs` に表示言語と `ErrorCode`、`ScanProgress.phase`・`error_code`、`ExportRequest.language: "ja" | "en"` の契約を追加する。表示言語コードは「今回の有効な表示言語。未対応値は受け付けない」、`current_file` は実際のフォルダ名またはファイル名だけとし、ない場合は空文字列にする

**Checkpoint**: 言語値、文言、IPC の型が揃い、ストーリー実装を開始できる。

---

## Phase 3: User Story 1 - 環境に合った表示言語で起動する (Priority: P1) 🎯 MVP

**Goal**: 設定が未保存なら、端末言語が日本語のとき日本語、それ以外と取得失敗時は英語でアプリ全体の文言を示す。

**Independent Test**: 保存値なしで `ja-JP`、`en-US`、第三言語、言語取得失敗を試し、検索画面・About・進捗・エラー・出力・macOS メニューの言語を確認する。

### Tests for User Story 1

- [X] T007 [P] [US1] `tests/locale.test.ts` に `ja`・`ja-*` 判定、その他と取得失敗時の `en`、不正・未設定の保存値、対象キー欠落時にその文言だけ英語へ戻るテストを先に書く
- [X] T008 [P] [US1] `src-tauri/src/models/mod.rs` と `src-tauri/src/search/engine.rs` に進捗の固定文言排除、既知・未知エラーコード、ファイル名と進捗段階の分離を検証する Rust 単体テストを先に書く
- [X] T009 [US1] `src-tauri/src/models/mod.rs`、`src-tauri/src/export/csv_export.rs`、`src-tauri/src/export/xlsx_export.rs` に `ExportRequest.language` の日英・不正値と日英の見出し・一致種別・Excel シート名を検証する Rust 単体テストを先に書く
- [ ] T010 [P] [US1] `tests/locale-ui.test.tsx` に `SearchBar`、`StatusBar`、`Toast`、結果・プレビュー・About・画面枠の変更対象コンポーネントごとの描画テストを先に書き、英語表示、アクセシビリティ用名称、進捗、文言欠落時の英語フォールバック、利用者データの原文保持を確認する
- [X] T011 [P] [US1] `tests/app-locale.test.tsx` に `src/App.tsx`、`src/hooks/useLocale.tsx`、`src/hooks/useSearch.ts` の起動時言語判定と IPC エラー表示をモックした単体テストを先に書く
- [ ] T012 [P] [US1] `src-tauri/src/commands/search_cmd.rs`、`src-tauri/src/commands/preview_cmd.rs`、`src-tauri/src/commands/export_cmd.rs`、`src-tauri/src/commands/system_cmd.rs` に既知・未知の失敗が `{ "code": ErrorCode }` となり、利用者向け JSON に外部エラーの生文言を含めない Rust 単体テストを先に書く
- [ ] T013 [P] [US1] `src-tauri/src/lib.rs` に `set_menu_locale` の `ja`・`en` 検証とメニュー更新失敗時の挙動を検証する Rust 単体テストを先に書く

### Implementation for User Story 1

- [X] T014 [US1] `src/locale-core.ts` に保存値の読み取りと検証、OS 言語タグからの判定、文言キー単位の英語フォールバックを実装する。保存済み設定は「保存に成功した選択だけを保持。未設定は `default` と同じ動作」、今回の表示言語は「設定が `default` なら端末言語、`ja`・`en` なら設定値から決定」とする
- [X] T015 [US1] `src/hooks/useLocale.tsx` と `src/App.tsx` に起動時の言語判定と共有状態を接続し、言語取得・保存値読み取り失敗でも起動を継続させる
- [X] T016 [US1] `src-tauri/src/models/mod.rs`、`src-tauri/src/commands/search_cmd.rs`、`src-tauri/src/commands/preview_cmd.rs`、`src-tauri/src/commands/export_cmd.rs`、`src-tauri/src/commands/system_cmd.rs` で利用者向け失敗を `{ "code": ErrorCode }` に揃え、外部エラーの生文言を返さない
- [X] T017 [US1] `src-tauri/src/search/engine.rs` と `src-tauri/src/models/mod.rs` で検索進捗を `phase`・件数・実ファイル名・`error_code` に分離し、固定の日本語を `current_file` へ入れない
- [ ] T018 [US1] `src/hooks/useSearch.ts`、`src/App.tsx`、`src/components/common/Toast.tsx` で通知を `message_key` と差し込み値として保持し、検索・プレビュー・エクスポートのエラーコードから現在の表示言語で説明を作る
- [X] T019 [US1] `src/components/search/SearchBar.tsx` と `src/components/common/StatusBar.tsx` のラベル、入力案内、操作名、進捗、通知を文言辞書から描画し、検索語・ファイル名は原文のままにする
- [X] T020 [P] [US1] `src/components/results/ResultTable.tsx`、`src/components/preview/SheetTabs.tsx`、`src/components/preview/PreviewHeader.tsx`、`src/components/preview/SpreadsheetGrid.tsx`、`src/components/preview/FormulaBar.tsx`、`src/components/preview/MetaInfoCard.tsx` のアプリ作成文言を辞書から描画し、セル値と数式を変えない
- [X] T021 [P] [US1] `src/components/about/AboutDialog.tsx`、`src/components/about/PackageList.tsx`、`src/components/about/PackageDetail.tsx`、`src/components/layout/WindowFrame.tsx` のアプリ作成文言とアクセシビリティ用名称を辞書から描画し、第三者ライセンス本文を変えない
- [X] T022 [US1] `src-tauri/src/models/mod.rs`、`src-tauri/src/export/csv_export.rs`、`src-tauri/src/export/xlsx_export.rs`、`src-tauri/src/commands/export_cmd.rs` で `ExportRequest.language: "ja" | "en"` を検証し、不正値は `internal_error` として拒否して、列見出し・一致種別・Excel シート名を表示言語で出力する
- [X] T023 [US1] `src-tauri/src/lib.rs` と `src-tauri/src/constants.rs` に `set_menu_locale(language: "ja" | "en")` を追加し、起動後に確定した表示言語から macOS アプリメニューの About と定義済み項目を日英で表示する
- [X] T024 [US1] `src/App.tsx`、`src/components/common/StatusBar.tsx`、`src/types/search.ts` で起動時の言語をエクスポート要求とメニュー更新へ渡し、主要操作が未設定時も選択言語で完結するよう接続する

**Checkpoint**: 設定画面がなくても、初回起動時の日本語・英語表示を独立に確認できる。

---

## Phase 4: User Story 2 - 設定画面で表示言語を切り替える (Priority: P1)

**Goal**: デフォルト・日本語・英語を即時に切り替え、保存に成功すれば次回も同じ設定を使い、失敗時は今回の起動中だけ選択を維持する。

**Independent Test**: 3選択肢を往復し、検索条件・結果・選択セルの維持、通知とメニューの再描画、保存成功・失敗、デフォルトでの端末言語変更後の再起動を確認する。

### Tests for User Story 2

- [X] T025 [P] [US2] `tests/locale.test.ts` に `default`・`ja`・`en` の保存と再読込、無効値からデフォルトへの復帰、書き込み失敗時のセッション内設定維持と次回起動時の保存済み設定復元を先に書く
- [ ] T026 [P] [US2] `tests/settings.test.tsx` に `SettingsDialog`、設定ボタン、`App`・`Toast`・`StatusBar` の切り替え時の描画・操作テストを先に書き、日本語時の「デフォルト」・英語時の「Default」、キーボード、検索状態維持、保存失敗通知、切り替え時の `set_menu_locale` 呼び出しを確認する

### Implementation for User Story 2

- [X] T027 [US2] `src/locale-core.ts` と `src/hooks/useLocale.tsx` で選択した `default`・`ja`・`en` を保存し、`default` 選択時は端末言語を再取得する。今回の設定は「設定画面で選択中の値。保存失敗後も今回の起動中は選択値を保つ」、保存失敗時は「今回の設定と表示言語だけを更新し、保存済み設定は変えない」とする
- [X] T028 [US2] `src/components/settings/SettingsDialog.tsx` に現在の設定と表示言語、日本語時は「デフォルト」・英語時は「Default」と表示する設定値 `default`、固定の「日本語」・「English」の3選択肢、キーボード操作、Esc と閉じるボタンを実装する
- [X] T029 [US2] `src/components/common/StatusBar.tsx` と `src/App.tsx` に設定画面の入口と開閉を接続し、言語変更で検索条件・結果・選択状態を再初期化しない
- [X] T030 [US2] `src/App.tsx` と `src/components/common/Toast.tsx` で言語切り替え時に開いている通知・ダイアログを再描画し、保存失敗と次回起動時の設定・表示言語を現在の表示言語で通知する
- [X] T031 [US2] `src/App.tsx` と `src-tauri/src/lib.rs` で設定変更時に既存の `set_menu_locale` を呼び、macOS メニューを更新する。他 OS は成功として扱い、失敗時は英語ログと現在の表示言語の案内を出す
- [X] T032 [US2] `src/components/common/StatusBar.tsx` と `src/types/search.ts` で言語切り替え後の CSV・Excel 出力要求に現在の表示言語を渡し、画面とファイルの言語を揃える
- [X] T033 [US2] `design/mainui/index.html` に設定画面の3選択肢、日本語時の「デフォルト」・英語時の「Default」、現在の設定・表示言語を反映し、実画面と構造・スタイル・操作案内を揃える

**Checkpoint**: 保存された明示言語は端末言語に優先し、保存されたデフォルトは次回起動または再選択時に端末言語に従う。

---

## Phase 5: User Story 3 - 言語に左右されない診断ログを得る (Priority: P2)

**Goal**: 表示が日本語でも英語でも、アプリ自身の診断ログ本文とアプリ起因のエラー理由を英語にする。

**Independent Test**: 両表示言語で同じ検索・ファイル操作の成功と失敗を起こし、アプリ作成ログを比較する。画面は表示言語を維持し、検索語・セル内容はログへ出ない。

### Tests for User Story 3

- [ ] T034 [P] [US3] `src-tauri/src/commands/search_cmd.rs`、`src-tauri/src/commands/preview_cmd.rs`、`src-tauri/src/commands/export_cmd.rs`、`src-tauri/src/commands/system_cmd.rs` の Rust 単体テストに、アプリ起因の診断ログと失敗理由が表示言語に関係なく英語である確認を先に加える
- [ ] T035 [P] [US3] `tests/locale.test.mjs` に表示言語の切り替えが英語ログ用定数とエラーコードを変更しない確認を先に加える
- [ ] T036 [P] [US3] `tests/logging.test.tsx` に `App`、`useSearch`、検索・プレビュー・About・ステータスのアプリ作成ログが英語で、検索語とセル内容を含まない単体テストを先に書く

### Implementation for User Story 3

- [X] T037 [US3] `src-tauri/src/commands/search_cmd.rs`、`src-tauri/src/commands/preview_cmd.rs`、`src-tauri/src/commands/export_cmd.rs`、`src-tauri/src/commands/system_cmd.rs`、`src-tauri/src/search/engine.rs`、`src-tauri/src/lib.rs` のアプリ作成ログと内部エラー理由を英語に統一する
- [X] T038 [US3] `src/App.tsx`、`src/hooks/useSearch.ts`、`src/components/search/SearchBar.tsx`、`src/components/common/StatusBar.tsx`、`src/components/about/AboutDialog.tsx`、`src/components/preview/PreviewHeader.tsx` の `console` ログ説明を英語に統一し、`src/hooks/useSearch.ts` で説明文と `query` 全体を `console.log` へ渡す既存行を除去して検索語とセル内容をログへ出さない

**Checkpoint**: 画面言語を変えてもアプリ作成ログの言語は英語のままである。

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 主要操作、品質ゲート、実機での表示を最終確認する。

- [X] T039 `src/constants/index.ts` と `src-tauri/src/constants.rs` について英語文言の全キー網羅、対象言語のキー欠落時の英語フォールバック、固定文字列の外部抽出を確認し、変更した全コードの4要素ヘッダコメントと不要トークン・文字化けの有無を点検する
- [ ] T040 `specs/008-add-language-settings/quickstart.md` の手順で日本語・英語・デフォルト、再起動、保存失敗、エラー、出力、macOS メニュー、英語ログを実機確認し、設定画面を開いてから選択まで30秒以内・選択後の反映1秒以内を別々に計測する
- [X] T041 `package.json` の `npm test`、`npm run lint`、`npm run build` を実行し、変更したフロントエンドモジュールの単体テストと型チェックが通り、ESLint 警告が0件であることを確認する
- [X] T042 `src-tauri/Cargo.toml` で `cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check` を実行し、失敗と警告がないことを確認する
- [X] T043 `design/mainui/index.html` と `src/components/settings/SettingsDialog.tsx` の設定画面を照合し、差分があれば試作を同期する

---

## Dependencies & Execution Order

- Phase 1 → Phase 2 → US1 → US2 → US3 → Phase 6。
- US1 は共通辞書・言語判定・IPC 契約を完成させる。US2 はその共有状態に設定画面と保存を追加する。US3 は同じ処理経路の診断ログを英語へ統一する。
- 各ストーリー内はテスト作成 → 期待する失敗の確認 → 型・モデル → 処理 → 画面・統合 → 独立確認の順とする。
- `[P]` は先行フェーズの完了後、記載したファイルが重ならない別作業と並行可能。US1 の T012 と T013、T020 と T021、US2 の T025 と T026、US3 の T034 と T035 が例となる。

## Parallel Execution Examples

- **US1**: `src-tauri/src/commands/` のエラー契約テスト T012 と `src-tauri/src/lib.rs` のメニューテスト T013 を並行する。T008 と T009 は `src-tauri/src/models/mod.rs` を共有するため順に進める。T020 の結果・プレビュー文言と T021 の About・枠文言も別ファイルで並行できる。
- **US2**: `tests/locale.test.mjs` の保存テスト T025 と `tests/settings.test.tsx` の設定画面テスト T026 を、テスト対象と編集ファイルの重複を避けて進める。その後は設定画面、共有状態、メニューを順に接続する。
- **US3**: Rust コマンドのログテスト T034 とフロントエンドの言語状態テスト T035 を並行し、実装後に同じ操作を両言語で比較する。

## Implementation Strategy

1. Phase 1・2 のあと US1 を完成し、未設定時に日英で使える状態を MVP とする。
2. US2 を追加してデフォルト・明示言語の切り替えと保存を確認する。
3. US3 を追加して画面言語と英語ログの独立性を確認する。
4. Phase 6 の単体テスト・ビルド・Rust 品質ゲートと `quickstart.md` の実機確認を終える。

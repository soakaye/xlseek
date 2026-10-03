# 実装タスク一覧: マルチプラットフォーム対応 (macOS対応・システム標準タイトルバー・OS固有機能の同等実装) (Tasks)

**機能ブランチ**: `003-macos-support` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md) | **実装計画書**: [plan.md](./plan.md)

---

## Phase 1: Setup (Shared Infrastructure)

**目的**: 全プラットフォーム共通のウィンドウ装飾設定および型定義の更新

- [X] T001 [P] `src-tauri/tauri.conf.json` で `"decorations": true` を設定し、OS ネイティブのウィンドウタイトルバー・コントロールを全プラットフォームで有効化
- [X] T002 [P] `src/types/search.ts` の `SupportedApp` インターフェースの `icon_hint` に `"numbers"` を追加

---

## Phase 2: Foundational (Blocking Prerequisites)

**目的**: プラットフォーム別の条件コンパイル基盤の整備と macOS ネイティブメニューバーの導入

**⚠️ CRITICAL**: 本フェーズ完了まで各ユーザーストーリーの実装・UI連携はブロックされます

- [X] T003 `src-tauri/src/commands/system_cmd.rs` の条件コンパイル構造を整備し、Windows 既存実装を `#[cfg(target_os = "windows")]` に分離しつつ、macOS 用（`#[cfg(target_os = "macos")]`）およびその他環境用の関数スタブを定義
- [X] T004 `src-tauri/src/lib.rs` にて macOS 向けに Tauri v2 のネイティブメニューバー（`tauri::menu::Menu::default(&app.handle())`）を初期化・登録

**Checkpoint**: 条件コンパイルの骨格とメニューバー基盤が完成し、プラットフォーム固有の機能実装が可能になります

---

## Phase 3: User Story 1 - 全プラットフォームでのOS標準ウィンドウフレームへの統一 (Priority: P1) 🎯 MVP

**ゴール**: フロントエンドの独自タイトルバー・偽ボタンを廃止し、OS ネイティブのウィンドウ枠（macOS: トラフィックライト、Windows: 標準ボタン）に一本化。補助情報はステータスバーへ移設する。

**独立テスト方法**: アプリを起動し、OS ネイティブのタイトルバーとコントロールボタンが表示され、ドラッグ移動・ダブルクリックズーム/最大化が機能することを確認。ステータスバーに「Calamine Engine」とバージョン情報が表示されることを確認。

### Implementation for User Story 1

- [X] T005 [P] [US1] `src/components/layout/WindowFrame.tsx` から独自タイトルバー（ドラッグ領域、Excelアイコン、最小化/最大化/閉じるボタン、関連イベントハンドラ）を完全撤去し、OS ネイティブ装飾に適合した外枠レイアウトにリファクタリング
- [X] T006 [P] [US1] `src/components/common/StatusBar.tsx` に旧タイトルバーから移設した「Calamine Engine」バッジおよび「v0.1.0」バージョン表記の表示コンポーネントを追加

**Checkpoint**: User Story 1 (MVP) 単体で OS 標準ウィンドウ枠への移行が完了し、テスト可能です

---

## Phase 4: User Story 2 - macOS におけるファイルマネージャー連携（Finder 選択表示） (Priority: P2)

**ゴール**: プレビューヘッダーの「フォルダを開く」ボタン押下時に、macOS 上で Finder を起動して該当ファイルをハイライト選択した状態で開く（Windows の `explorer /select` と同等の機能を提供）。

**独立テスト方法**: 検索結果セルを選択してプレビューヘッダーの「フォルダを開く」ボタンをクリックし、Finder が前面に開き、対象ファイルがハイライト選択された状態でフォルダが開くことを確認。

### Implementation for User Story 2

- [X] T007 [US2] `src-tauri/src/commands/system_cmd.rs` の `open_in_folder` において、macOS 向け（`#[cfg(target_os = "macos")]`）に `open -R <file_path>` を呼び出して Finder 上でファイルをハイライト選択表示する処理を実装
- [X] T008 [US2] `src-tauri/src/commands/system_cmd.rs` の `open_in_folder` において、対象ファイルが存在しない場合の安全なエラー返却と、親フォルダのみ存在する場合のフォールバック（`open::that`）処理を実装

**Checkpoint**: User Story 2 のファイルマネージャー連携が完了し、テスト可能です

---

## Phase 5: User Story 3 - macOS における関連付けアプリの検出と起動 (Priority: P3)

**ゴール**: macOS 環境において `.xlsx` / `.xls` を開くことができるインストール済み対応アプリ（Microsoft Excel, Numbers, LibreOffice 等）の検出、既定アプリの特定、ワンクリック起動、および「別のプログラムを選択...」ダイアログを提供する。

**独立テスト方法**: 起動ボタンのツールチップに macOS 既定アプリ名が表示され、メインボタンで既定アプリが起動すること、▼メニューから Numbers や Excel を選択して起動できること、および「別のプログラムを選択...」で `/Applications` ダイアログが開くことを確認。

### Implementation for User Story 3

- [X] T009 [P] [US3] `src-tauri/src/commands/system_cmd.rs` にて macOS 向け（`#[cfg(target_os = "macos")]`）に `/Applications`, `/System/Applications`, `~/Applications` から主要な表計算アプリ（Microsoft Excel, Apple Numbers, LibreOffice Calc, WPS Office 等）を検出し `SupportedApp` 一覧を生成・ソートする `get_supported_apps` を実装
- [X] T010 [P] [US3] `src-tauri/src/commands/system_cmd.rs` にて macOS 向けに指定された `.app` バンドルまたは既定ハンドラでファイルを開く `launch_associated_app`（`open -a` または `open`）を実装
- [X] T011 [US3] `src-tauri/src/commands/system_cmd.rs` にて macOS 向けに `/Applications` を起点とするアプリケーション選択ダイアログを表示して選択されたアプリでファイルを開く `show_open_with_dialog` を実装
- [X] T012 [P] [US3] `src/components/preview/PreviewHeader.tsx` にて `icon_hint === "numbers"` のアイコン表示サポートおよび macOS 向けアプリ表示名のスタイルを調整

**Checkpoint**: User Story 3 の外部アプリ検出・起動・選択ダイアログ連携が完了し、テスト可能です

---

## Phase 6: User Story 4 - マルチプラットフォーム共通のパス操作・ドラッグ＆ドロップおよびキーボード操作 (Priority: P4)

**ゴール**: POSIX パス形式のフォルダ DnD 登録、パスのクリップボードコピー、および macOS システムメニューバーと連動した標準キーボードショートカット（`⌘C`, `⌘V`, `⌘A`, `⌘Z`, `⌘W`, `⌘Q`）を保証する。

**独立テスト方法**: Finder からフォルダをドラッグ＆ドロップして正しい POSIX パスが設定されること、パスコピーで POSIX パスが取得できること、入力欄で `⌘A` / `⌘C` / `⌘V` が正常に動作すること、`⌘W` / `⌘Q` でウィンドウやアプリを閉じられることを確認。

### Implementation for User Story 4

- [X] T013 [US4] `src-tauri/src/commands/system_cmd.rs` の `resolve_dropped_path` において、macOS の POSIX パス（`/Users/...`）が渡された際のディレクトリ解決動作を検証・保証
- [X] T014 [US4] `src/components/search/SearchBar.tsx` および `src/components/preview/PreviewHeader.tsx` において、Finder からのドロップパス反映および「パスをコピー」時の POSIX パス取得動作を検証・調整
- [X] T015 [US4] `src/App.tsx` および各入力コンポーネントにおいて、macOS システムメニューバーと連携したキーボード操作（`⌘C`, `⌘V`, `⌘A`, `⌘Z`, `⌘W`, `⌘Q`, `Enter`, `Escape`）の動作を検証・保証

**Checkpoint**: 全ユーザーストーリーの実装が完了し、独立および統合テスト可能です

---

## Phase 7: Polish & Cross-Cutting Concerns

**目的**: 複数ストーリーにまたがる品質改善・警告解消・E2E 動作検証

- [X] T016 [P] `cargo check` および `npm run build` を実行し、macOS 環境における Rust 未使用変数等の警告解消およびフロントエンドビルドの通過を確認
- [X] T017 `specs/003-macos-support/quickstart.md` の全 6 検証シナリオ（タイトルバー、Finder 連携、アプリ検出・起動、アプリ選択、ショートカット、POSIX DnD）に沿った E2E 動作検証を実施

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 依存関係なし - 即座に開始可能
- **Foundational (Phase 2)**: Setup 完了に依存 - 全ユーザーストーリーの実装をブロック
- **User Stories (Phase 3〜6)**: Foundational フェーズ完了に依存
  - 優先度順（P1 → P2 → P3 → P4）に順次実装・検証、または並行実装が可能
- **Polish (Phase 7)**: 全ユーザーストーリー完了後に実施

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完了後に即座に開始可能（MVP）。他ストーリーへの依存なし。
- **User Story 2 (P2)**: Foundational 完了後に開始可能。独立してテスト可能。
- **User Story 3 (P3)**: Foundational 完了後に開始可能。独立してテスト可能。
- **User Story 4 (P4)**: Foundational 完了後に開始可能。全コンポーネントでのパスおよびショートカット挙動を保証。

---

## Parallel Opportunities

- **Phase 1**: T001（Tauri設定）と T002（フロント型定義）は並行実行可能 `[P]`
- **Phase 3 (US1)**: T005（WindowFrame タイトルバー撤去）と T006（StatusBar バッジ追加）は並行実行可能 `[P]`
- **Phase 5 (US3)**: T009（アプリ検出）と T010（アプリ起動）と T012（PreviewHeader UI）は並行実行可能 `[P]`
- **Phase 7**: T016（ビルド検証）は他検証と並行実行可能 `[P]`

---

## Parallel Example: User Story 1

```bash
# User Story 1 のコンポーネント変更を並行実行:
Task: "src/components/layout/WindowFrame.tsx から独自タイトルバーを完全撤去"
Task: "src/components/common/StatusBar.tsx に Calamine Engine バッジおよびバージョン表記を追加"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup 完了
2. Phase 2: Foundational 完了
3. Phase 3: User Story 1 完了
4. **検証**: OS ネイティブウィンドウタイトルバーの表示と操作感を検証（MVP 達成！）

### Incremental Delivery

1. Setup + Foundational 完了 → 基盤確立
2. User Story 1 完了 → OS ネイティブタイトルバー対応（MVP）
3. User Story 2 完了 → macOS Finder ファイル選択表示連携
4. User Story 3 完了 → macOS 対応表計算アプリ（Excel, Numbers, LibreOffice）起動連携
5. User Story 4 完了 → macOS ショートカットおよび POSIX パス DnD 保証
6. Phase 7 完了 → 全体 E2E 検証とビルド確認

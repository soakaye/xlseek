# 実装タスク一覧: xlsx/xls 登録アプリおよびサポートアプリ起動・フォルダDnD対応 (Tasks)

**機能ブランチ**: `002-launch-associated-app` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md) | **実装計画書**: [plan.md](./plan.md)

---

## Phase 1: Setup (Shared Infrastructure)

**目的**: プロジェクト設定・依存クレートおよび型定義の共通基盤整備

- [ ] T001 [P] `src-tauri/Cargo.toml` に Windows レジストリ走査用の依存クレート `winreg = "0.55"` を追加
- [ ] T002 [P] `src/types/search.ts` に `SupportedApp` インターフェース（id, name, executable_path, is_default, icon_hint）を追加定義
- [ ] T003 [P] `src-tauri/src/models/mod.rs` に `SupportedApp` 構造体（Serialize/Deserialize, Clone, Debug）を追加定義

---

## Phase 2: Foundational (Blocking Prerequisites)

**目的**: 全ユーザーストーリーの前提となるバックエンド IPC コマンド群の実装

**⚠️ CRITICAL**: 本フェーズ完了までユーザーストーリーの実装・UI連携はブロックされます

- [ ] T004 `src-tauri/src/commands/system_cmd.rs` に Windows レジストリ（`FileExts\.xlsx\OpenWithProgids` / `OpenWithList`）を走査してインストール済みサポートアプリ一覧を列挙する `get_supported_apps` コマンドを実装
- [ ] T005 `src-tauri/src/commands/system_cmd.rs` に指定アプリまたは既定アプリで起動する `launch_associated_app` および OS の「プログラムから開く」ダイアログを呼び出す `show_open_with_dialog` コマンドを実装
- [ ] T006 `src-tauri/src/lib.rs` の `invoke_handler` に `get_supported_apps`, `launch_associated_app`, `show_open_with_dialog` を登録

**Checkpoint**: バックエンド API 基盤が完成し、フロントエンドからの各機能連携が可能になります

---

## Phase 3: User Story 1 - 既定登録アプリによるワンクリック起動 (Priority: P1) 🎯 MVP

**ゴール**: プレビューヘッダーの「Excel で開く」ボタンを「アプリで開く」スプリットボタンのメイン部分に変更し、ワンクリックで OS 既定の表計算アプリを開けるようにする

**独立テスト方法**: プレビューヘッダーの「アプリで開く」メインボタンをクリックし、現在 OS に登録されている既定アプリ（Excel、LibreOffice 等）で対象ファイルが開くことを確認する。マウスホバー時にツールチップで「[既定アプリ名] で開く」が表示されることを確認する。

### Implementation for User Story 1

- [ ] T007 [US1] `src/components/preview/PreviewHeader.tsx` の「Excel で開く」ボタンを固定ラベル「アプリで開く」と表計算アイコンを持つスプリットボタン形式に刷新し、ツールチップで現在の OS 既定アプリ名（例: "Microsoft Excel で開く"）を表示する UI を実装
- [ ] T008 [US1] `src/components/preview/PreviewHeader.tsx` でメインボタン押下時に `invoke("launch_associated_app", { filePath, appPath: null })` を呼び出し、OS 既定アプリでのワンクリック起動を連携

**Checkpoint**: User Story 1 (MVP) 単体で登録アプリの直接起動が完了し、テスト可能です

---

## Phase 4: User Story 2 - ポップアップメニューからのサポートアプリ選択起動 (Priority: P2)

**ゴール**: スプリットボタン右側の▼からポップアップメニューを展開し、インストール済みサポートアプリ一覧および「別のプログラムを選択...」から任意アプリを選択して起動できるようにする

**独立テスト方法**: スプリットボタンの▼ボタンをクリックしてメニューを表示し、一覧の中のアプリ（または「別のプログラムを選択...」）をクリックして選択したツールで対象ファイルが開くことを確認する。外側クリックや Esc キーでメニューが閉じることを確認する。

### Implementation for User Story 2

- [ ] T009 [US2] `src/components/preview/PreviewHeader.tsx` にスプリットボタンのドロップダウントリガー（▼ボタン）と開閉状態管理（`isMenuOpen`）、および外側クリック / Esc キー検知のイベントリスナーを実装
- [ ] T010 [US2] `src/components/preview/PreviewHeader.tsx` にポップアップメニューコンポーネント（サポートアプリ一覧、各アプリの表示名、既定バッジ「(既定)」、区切り線、末尾の「別のプログラムを選択...」）を実装
- [ ] T011 [US2] `src/components/preview/PreviewHeader.tsx` でメニュー内のアプリ項目クリック時に `invoke("launch_associated_app", { filePath, appPath })`、および「別のプログラムを選択...」クリック時に `invoke("show_open_with_dialog", { filePath })` を呼び出す起動処理を連携

**Checkpoint**: User Story 1 と User Story 2 が共に独立して動作し、テスト可能です

---

## Phase 5: User Story 3 - フォルダ選択領域へのフォルダドラッグ＆ドロップ指定 (Priority: P2)

**ゴール**: 検索バーのフォルダ選択入力枠へエクスプローラーからフォルダ（またはファイル）をドラッグ＆ドロップして検索パスを素早く設定できるようにする（ドロップ時はパス設定のみ行い、自動検索は開始しない）

**独立テスト方法**: エクスプローラーからフォルダまたはファイルを検索バーのフォルダ入力枠にドラッグ＆ドロップし、正しいフォルダパス（ファイルの場合は親フォルダパス）が入力欄に即時反映されることを確認する。ドロップ後に検索が勝手に始まらず待機することを確認する。

### Implementation for User Story 3

- [ ] T012 [US3] `src-tauri/src/commands/system_cmd.rs` にドロップパス解決コマンド `resolve_dropped_path` を実装・検証（ディレクトリならそのまま、ファイルなら親ディレクトリを返す）
- [ ] T013 [US3] `src/components/search/SearchBar.tsx` に Tauri `onDragDropEvent` および HTML5 DnD ハンドラを接続し、フォルダ選択領域へのドラッグオーバー時に破線枠とハイライト演出（「フォルダをここにドロップ」）を表示する UI を実装
- [ ] T014 [US3] `src/components/search/SearchBar.tsx` でドロップ時に `resolve_dropped_path` 経由でパスを解決して `target_dir` に反映し、自動検索は実行せずユーザーの明示的な検索操作（SEARCHボタン/Enter）を待機する処理を確立

**Checkpoint**: フォルダ選択領域への直感的な DnD 操作が完了し、テスト可能です

---

## Phase 6: User Story 4 - アプリ未登録時および起動エラーの安全な通知 (Priority: P3)

**ゴール**: サポートアプリが未登録の場合や起動に失敗した場合でも、クラッシュすることなく分かりやすいトースト通知を表示する

**独立テスト方法**: 存在しないアプリパスの起動要求や破損した関連付けをシミュレートし、エラーメッセージがトースト通知され、アプリが正常に動作を継続できることを確認する。

### Implementation for User Story 4

- [ ] T015 [US4] `src/components/preview/PreviewHeader.tsx` の起動処理呼び出しに try-catch を適用し、起動失敗時にエラー内容をトースト通知（`onShowToast("アプリケーションを起動できませんでした: ...")`）するハンドリングを実装
- [ ] T016 [US4] `src/components/preview/PreviewHeader.tsx` でサポートアプリ一覧が 0 件の場合に「利用可能なアプリが見つかりません」の空状態表示と「別のプログラムを選択...」への誘導項目を表示するよう調整

**Checkpoint**: 全ユーザーストーリーの正常系・異常系がすべて完了し、テスト可能です

---

## Phase 7: Polish & Cross-Cutting Concerns

**目的**: 複数ストーリーにまたがる品質改善・レイアウト調整・最終動作検証

- [ ] T017 [P] `src/components/preview/PreviewHeader.tsx` のポップアップメニューについて、ウィンドウ端や右端で見切れない配置調整（`right-0` や z-index 制御）を実施
- [ ] T018 `specs/002-launch-associated-app/quickstart.md` に記載の全 E2E 検証シナリオ（DnD、既定起動、ポップアップ選択、Open With ダイアログ、エラー通知）を実施し動作確認
- [ ] T019 [P] `npm run build` によるフロントエンドビルドおよび `cargo check` による Rust バックエンドの最終検証

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: 依存関係なし、即時開始可能
- **Phase 2: Foundational**: Setup 完了に依存。全ユーザーストーリーの実装をブロック
- **Phase 3〜6: User Stories**: Foundational 完了後に着手可能
  - US1 (P1, MVP) → US2 (P2) / US3 (P2) → US4 (P3) の順で順次または並行進行
- **Phase 7: Polish**: 全ユーザーストーリー完了後に着手

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完了後に開始可能（他のストーリーへの依存なし）
- **User Story 2 (P2)**: Foundational 完了後に開始可能（US1のスプリットボタンUIを拡張）
- **User Story 3 (P2)**: Foundational 完了後に開始可能（独立して実装・テスト可能）
- **User Story 4 (P3)**: US1 および US2 の起動処理と連動して実装

---

## Parallel Example: User Story 1 & User Story 3

```bash
# Setup の型定義・モデル作成を並行実行:
Task: "T002 [P] src/types/search.ts に SupportedApp インターフェースを追加定義"
Task: "T003 [P] src-tauri/src/models/mod.rs に SupportedApp 構造体を追加定義"

# Foundational 完了後、US1 (アプリ起動) と US3 (フォルダDnD) を並行進行可能:
Task: "T007 [US1] PreviewHeader.tsx のスプリットボタン化と既定アプリ起動"
Task: "T013 [US3] SearchBar.tsx のフォルダDnDドラッグオーバー演出とパス反映"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 (Setup) および Phase 2 (Foundational) を完了。
2. Phase 3 (User Story 1) を実装。
3. **検証**: プレビューヘッダーの「アプリで開く」ボタンで OS 既定アプリが開くことを確認（MVP 達成）。

### Incremental Delivery

1. MVP 達成後、Phase 4 (User Story 2: ポップアップメニュー選択起動) を追加。
2. Phase 5 (User Story 3: フォルダ DnD) を統合・検証。
3. Phase 6 (User Story 4: エラーハンドリング) および Phase 7 (Polish) で完成。

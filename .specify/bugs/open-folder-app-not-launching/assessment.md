# Bug Assessment: ファイルの場所を開くでファイル管理アプリが起動しない問題

- **Slug**: open-folder-app-not-launching
- **Created**: 2026-10-02T18:07:30+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> ファイルの場所を開くでファイル管理アプリが起動しない

## Symptom

Linux / Ubuntu 環境において、検索結果プレビュー画面の「ファイルの保存場所を開く」ボタン（フォルダーアイコン、`ui.OPEN_LOCATION_TOOLTIP`）をクリックしても、ファイル管理アプリ（Nautilus 等）が起動せずフォルダーが表示されない。
また、エラー通知（トースト）も表示されず、無反応のまま処理が終了する（サイレント障害）。

## Reproduction

1. Linux (Ubuntu) 環境で xlseek を起動する。
2. 検索を実行し、検索結果リストから対象ファイルを1件選択してプレビューを表示する。
3. プレビューヘッダー上にある「ファイルの保存場所を開く」ボタン（フォルダーアイコン）をクリックする。
4. ファイル管理アプリ（Nautilus / Files 等）のウィンドウが起動せず、画面上にも何のエラーメッセージも表示されない（無反応となる）ことを確認する。

## Suspected Code Paths

- `src-tauri/src/commands/system_cmd.rs:460-532` (`open_in_folder`):
  Linux 向け処理として `open_file_in_folder_linux(path)` を呼び出している。この関数が `Ok(())` を返すとフォールバックを実行せずに正常終了とみなす。
- `src-tauri/src/commands/system_cmd.rs:581-659` (`open_file_in_folder_linux`):
  D-Bus 経由で `org.freedesktop.FileManager1.ShowItems` を呼び出すために `dbus-send` を実行している。しかし、`dbus-send` の引数に `--print-reply`（応答待ちオプション）が含まれておらず、メッセージを送信キューに入れた時点で終了コード `0` を返してしまう。
- `src-tauri/src/constants.rs:255-276` (D-Bus コマンド関連定数):
  `CMD_DBUS_SEND`, `DBUS_ARG_SESSION`, `DBUS_DEST_FILEMANAGER` 等が定義されているが、`--print-reply` やタイムアウトに関する定数が未定義。
- `src/components/preview/PreviewHeader.tsx:160-168` (`handleOpenFolder`):
  `invoke(COMMANDS.OPEN_IN_FOLDER, ...)` を実行。バックエンドが偽陽性の `Ok(())` を返すため、`catch` ブロック（トースト通知）に入らない。

## Root Cause Hypothesis

**確信度: High（高）**

根本原因は `src-tauri/src/commands/system_cmd.rs` の `open_file_in_folder_linux` において、`dbus-send` コマンド呼び出し時に `--print-reply` オプションが付与されていないことです。

FreeDesktop / D-Bus の仕様上、`dbus-send` は `--print-reply` を指定しない場合、非同期（fire-and-forget）で動作し、宛先サービス（`org.freedesktop.FileManager1`）が存在するかどうかやメソッド実行の成否にかかわらず、ローカルの D-Bus ソケットにメッセージをエンキューできた時点で **常に終了ステータス `0`（成功）** を返します。

そのため：
1. Nautilus などのファイルマネージャがデーモンとしてセッションバス上で `org.freedesktop.FileManager1` をリッスンしていない環境（一般的なデスクトップ環境や Nautilus が事前起動していない状態、非 GNOME / XFCE / 最小構成環境等）では、D-Bus 呼び出し自体は失敗（`ServiceUnknown` 等）しているにもかかわらず、`dbus-send` プロセスが終了ステータス `0` で終了します。
2. `dbus_status` のチェック（`s.success()`）が `true` と評価され、`open_file_in_folder_linux` は「対象ファイルの強調表示に成功した」と誤認して即座に `Ok(())` を返します。
3. これにより、後続のフォールバック処理（`gio open <parent>`、`xdg-open <parent>`、`open::that(parent)`）が **一切実行されません**。
4. フロントエンド側にも `Ok` が返却されるためエラーハンドリング（トースト表示）もトリガーされず、ユーザーから見ると「ボタンを押してもファイルマネージャが起動せず、何のエラーも出ない」という完全なサイレント不具合となります。

## Proposed Remediation

**Preferred（推奨）**:
1. `src-tauri/src/commands/system_cmd.rs` の `open_file_in_folder_linux` を改修:
   - `dbus-send` 実行時に `--print-reply` および適切なタイムアウト（例: `--reply-timeout=2000`）を付与する。これにより、`org.freedesktop.FileManager1` が存在しない場合や応答がない場合に `dbus-send` が非ゼロ（失敗ステータス）を返し、正常に応答があった場合のみ成功とみなすようにする。
   - `dbus-send` が失敗した際、ダイレクトなファイルマネージャ起動フォールバック（ファイル選択機能付きの `nautilus --select <path>` や `dolphin --select <path>`）および既存の `gio open` / `xdg-open` / `open::that` フォールバックへ確実に処理が遷移するようにする。
   - すべてのオープン手段が失敗した場合には `Err(ERR_FOLDER_OPEN)` を返し、フロントエンド側で適切にエラー通知（`ui.OPEN_FOLDER_FAILED`）が表示されるようにする。
2. `src-tauri/src/constants.rs` に必要な定数を追加:
   - `pub const DBUS_ARG_PRINT_REPLY: &str = "--print-reply";`
   - `pub const DBUS_ARG_REPLY_TIMEOUT: &str = "--reply-timeout=2000";`
   - ファイルマネージャ実行コマンド（`nautilus`, `--select` 等）の定数管理。

**Alternatives（代替案）**:
- *代替案1: D-Bus ShowItems を使わず、直接 `xdg-open <parent>` や `nautilus --select` のみを行う*:
  - シンプルだが、Spec 023 で合意された「Linux においても可能な限り対象ファイルをハイライト（選択）して開く」要件を満たせなくなる。

**Files likely to change**:
- `src-tauri/src/commands/system_cmd.rs`
- `src-tauri/src/constants.rs`

**Tests to add or update**:
- `dbus-send` のコマンド引数構成に `--print-reply` が含まれていることの単体テスト。
- D-Bus が失敗した際のフォールバックロジックが正しく機能することの検証テスト。

## Risks & Considerations

- `--print-reply` を付与することで D-Bus 応答待ち（IPC）が発生するため、D-Bus デーモンやサービスがハングした場合にブロックしないよう、必ずタイムアウト引数（`--reply-timeout`）を設定し、非同期スレッド（`spawn_blocking`）内で実行することが不可欠です。

## Open Questions

- 特になし（不具合の原因および修正方針は明確）。

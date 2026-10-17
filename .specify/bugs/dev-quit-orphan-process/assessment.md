# Bug Assessment: アプリ終了後も Dock に「xlseek バックグランドで実行中」アイコンが残存する

- **Slug**: dev-quit-orphan-process
- **Created**: 2026-10-17
- **Source**: pasted text, attached screenshot `/var/folders/pl/7w4kq9mj72zg76cjpl14dcg00000gn/T/orca-paste-1792199785448-8d620926-3d1e-4b82-8b40-992496e9a2f4.png`
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> `npm run tauri dev` でアプリ起動->検索実行->放置->最小化->復旧->アプリ終了[システムメニュー]の操作を行うと`xlseek バックグランドで実行中`のターミナルアイコンがタスクに表示され残る。
> 画像添付: macOS の Dock にターミナル実行アイコン（黒背景に緑の `exec` 文字）が表示され、吹き出しで `xlseek` `バックグラウンドで実行中` と表示されている。

## Symptom

`npm run tauri dev` 経由でアプリを起動し、検索実行・放置・最小化・復旧を経てシステムメニューからアプリ終了（Quit）を行った際、GUI ウィンドウ自体は閉じるものの、macOS の Dock に「`xlseek` バックグラウンドで実行中」という名前のターミナル実行バイナリアイコン（黒地に緑色の `exec` バッジ）が消えずに残り続ける。期待される動作は、システムメニューから「アプリ終了」を選択した際に、アプリプロセスおよび開発ランナーの関連プロセスが完全に終了し、Dock からアイコンが消去されることである。

## Reproduction

1. macOS ターミナルで `npm run tauri dev` を実行してアプリを起動する。
2. 任意の検索を実行する（バックグラウンド検索タスクやスレッドプールの稼働）。
3. しばらく放置後、ウィンドウを最小化し、再度 Dock 等から復旧（前面表示）する。
4. macOS のメニューバー（システムメニュー）の「xlseek」->「Quit xlseek」（または `Cmd+Q`）を実行してアプリを終了する。
5. GUI ウィンドウは閉じるが、Dock にターミナル実行アイコン（黒地に `exec`）が残り、「xlseek バックグラウンドで実行中」とツールチップ／吹き出しに表示されたまま残存する。

[NEEDS CLARIFICATION: アプリ終了後、起動元のターミナル側で `npm run tauri dev` のコマンドプロンプトに戻っていたか、それともターミナルプロセス自体がまだブロック（実行中）のままだったか。]

## Suspected Code Paths

- `src-tauri/src/lib.rs:212-223` — `on_window_event`: `WindowEvent::CloseRequested` に対する `api.prevent_close()` と `window.hide()`。macOS においてウィンドウクローズ（`Cmd+W` / 赤ボタン）だけでなく、メニュー Quit やシステム終了要求イベントと競合・連動している可能性がある。
- `src-tauri/src/lib.rs:243-250` — `app.run(|app_handle, event| ...)`: `RunEvent::ExitRequested` や `RunEvent::Exit` をハンドリングしていないため、Tauri 内部または OS の終了要求時にプロセスやバックグラウンドワーカーが残存してイベントループが抜け出せない可能性がある。
- `src-tauri/src/commands/search_cmd.rs:74-165` — `tauri::async_runtime::spawn_blocking` による検索エンジン実行およびスレッドプール。検索完了後も Rayon のグローバルスレッドプールやチャネルリスナー、またはバックグラウンド監視タスクが存続している場合、プロセス正常終了を妨げる要因になり得る。
- `src-tauri/tauri.conf.json:7-10` — `"beforeDevCommand": "npm run build:cli && npm run dev"`: `tauri dev` 実行時、Tauri CLI が Vite 開発サーバープロセス（`npm run dev`）と Rust バイナリプロセス（`target/debug/xlseek`）を子プロセスとして管理する。Rust バイナリが終了した際に親の `tauri dev` や Vite 開発サーバーが子プロセス終了を検知して正しくクリーンアップ終了できずに残存している可能性がある。

## Root Cause Hypothesis

**確信度: high。** 以下の 2 つの要因が主因として推測される。

1. **`on_window_event` とメニュー Quit (`PredefinedMenuItem::quit`) の終了シーケンス不整合**:
   前回の修正で追加した `src-tauri/src/lib.rs` の `on_window_event` 内で、すべての `WindowEvent::CloseRequested` に対して無条件に `api.prevent_close()` と `window.hide()` を行っている。macOS では「Quit（終了）」時にも開いている各ウィンドウに対して `CloseRequested` が発火する場合があり、これが `prevent_close()` されるとウィンドウは隠れるものの、アプリケーションイベントループが「開いているウィンドウ／管理対象ウィンドウがある」と判定して終了フェーズへ移行できず、ヘッドレス（非表示）状態のままバックグラウンドプロセスとして生き残り続ける。
2. **`tauri dev` 実行時のバイナリ・バンドル構造とプロセス残存**:
   添付画像にあるアイコンは macOS の `.app` バンドルアイコンではなく、Mach-O 実行可能バイナリ（`target/debug/xlseek`）が直接起動された際に macOS が割り当てる「ターミナル実行可能ファイル（黒地に `exec`）」のデフォルトアイコンである。`tauri dev` で起動されたデバッグバイナリが上記 1 の理由で完全終了せず非表示で残留した場合、macOS は「GUI ウィンドウを持たないバックグラウンド実行中プロセス」として Dock にこのアイコンを表示し続ける。また、終了イベント（`RunEvent::ExitRequested`）で明示的に `app_handle.exit(0)` を行うか、Quit 時のクローズ抑止を解除する必要がある。

## Proposed Remediation

**Preferred**:
1. **アプリ終了要求時のフラグ管理と `CloseRequested` での条件付き制御 (`src-tauri/src/lib.rs`)**:
   - アプリケーションが「終了要求中（Quitting）」であるかを示すアトミックフラグ（またはメニュー Quit / `RunEvent::ExitRequested` 連携）を導入する。
   - `on_menu_event` で明示的にアプリ終了（Quit）を捕捉するか、または `RunEvent::ExitRequested` をリッスンする。
   - `WindowEvent::CloseRequested` において、通常のウィンドウクローズ（赤ボタン / `Cmd+W`）のときのみ `api.prevent_close()` と `window.hide()` を実行し、アプリ全体の終了（Quit / `ExitRequested`）時には `prevent_close()` を呼ばずにウィンドウ破棄およびプロセス終了を正常に進行させる。
   - 必要に応じて `RunEvent::ExitRequested { api, .. }` で `api.prevent_exit()` を呼ばず、速やかに全ウィンドウとランタイムを終了させる。
2. **メニューの Quit 動作確認と exit 呼び出しの確実化**:
   - macOS の App メニューにおける `PredefinedMenuItem::quit` が正しく `app_handle.exit(0)` をトリガーすることを確認・補強する。

**Alternatives** (optional):
- ウィンドウを `hide()` 方式ではなく、閉じる時は完全に破棄し、Dock 再クリック時に新規作成する方式。ただしメモリ効率やステート保持の観点からは、終了フラグによって `hide()` と `exit` を切り分ける Preferred 案が安全かつ標準的。

**Files likely to change**:
- `src-tauri/src/lib.rs`
- `src-tauri/src/constants.rs` (必要に応じて終了イベント定数等の追加)

**Tests to add or update**:
- `src-tauri/src/lib.rs` または `src-tauri/src/constants.rs`: アプリ終了イベントのハンドリングおよび定数の整合性を検証するユニットテスト。

## Risks & Considerations

- `Cmd+Q` やシステムメニューの Quit による正規の終了と、ウィンドウの赤ボタン（×）による閉じる操作が明確に区別されて動作することを確認する必要がある。
- Windows / Linux では `CloseRequested` でウィンドウを閉じるとそのままアプリ終了となる標準挙動を維持し、macOS 専用の処理を `#[cfg(target_os = "macos")]` で適切に分離する。

## Open Questions

- [NEEDS CLARIFICATION: アプリ終了時、メニューバーの「xlseek -> Quit xlseek」をクリックしたか、ショートカットキー `Cmd+Q` を押したか。]
- [NEEDS CLARIFICATION: その後、ターミナルで `Control+C` を押すと該当の Dock アイコンは消えたか。]

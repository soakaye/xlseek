# Bug Assessment: 放置後のバックグラウンド移行および再表示時の空白画面表示

- **Slug**: background-blank-window
- **Created**: 2026-10-17
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> 起動したまま放置しているとバックグランド実行となる。タスクのアイコンをクリックすると次のターミナルメッセージが表示され空白のアプリ画面が表示される。
> ターミナルメッセージ:
> ```text
> Last login: Sat Oct 17 03:25:47 on tty??
> /Users/soakaye/Develop/Projects/workspaces/exgrep/main/target/debug/xlseek ; exit;
> soakaye@Akis-MacBook-Pro ~ % /Users/soakaye/Develop/Projects/workspaces/exgrep/main/target/debug/xlseek ; exit;
> 2026-10-17 03:27:16.551 xlseek[42079:25759388] error messaging the mach port for IMKCFRunLoopWakeUpReliable
> ```

## Symptom

アプリ（特にデバッグビルド `target/debug/xlseek`）を起動したまま放置してバックグラウンドに移行した後、macOS の Dock（タスク）アイコンをクリックしてアプリを再度前面化・再表示した際、ターミナルに `IMKCFRunLoopWakeUpReliable` の診断ログが出力され、アプリのウィンドウが空白（UI が描画されない状態）になる。期待される動作は、バックグラウンド放置後に Dock アイコンをクリックした際にも既存のウィンドウまたは再表示されたウィンドウに通常の検索画面 UI が正常に表示されることである。

## Reproduction

1. `target/debug/xlseek` を直接起動する（Finder から実行ファイルを開く、またはターミナル経由で起動する）、あるいはアプリを起動した状態でウィンドウを閉じる／最小化／バックグラウンドに放置する。
2. しばらく放置してバックグラウンド状態にする（または Vite 開発サーバー `http://localhost:1420` が停止・スリープした状態、あるいは macOS により WebView プロセスがサスペンド／終了した状態になる）。
3. macOS の Dock（タスク）アイコンをクリックして `xlseek` をアクティブ化する。
4. ターミナルに `error messaging the mach port for IMKCFRunLoopWakeUpReliable` が出力され、空白のウィンドウが表示される。

[NEEDS CLARIFICATION: 放置中に Vite 開発サーバー（`npm run dev` / `localhost:1420`）が起動していたか、または最初から Finder 等で `target/debug/xlseek` 単体をダブルクリック起動していたか。また、放置前にウィンドウの閉じるボタン（赤ボタン / `Cmd+W`）を押していたか、開いたまま別アプリに切り替えて放置していたか。]

## Suspected Code Paths

- `src-tauri/tauri.conf.json:6-24` — `"devUrl": "http://localhost:1420"` と `"frontendDist": "../dist"`、および `"main"` ウィンドウ定義。`cargo build` で生成されたデバッグバイナリ `target/debug/xlseek` は `tauri::generate_context!()` により `devUrl` (`http://localhost:1420`) を参照する設定になっており、Vite 開発サーバーが起動していない（または停止した）状態で起動・再読み込みされると WebView が接続できず空白画面になる。
- `src-tauri/src/lib.rs:163-202` — `tauri::Builder::default() ... .run(tauri::generate_context!())`。現在 `.run(...)` に直接渡しており、`.build(...)?.run(|app_handle, event| ...)` による `tauri::RunEvent::Reopen`（macOS で Dock アイコンがクリックされた際のイベント）や `tauri::WindowEvent::CloseRequested` のハンドリングが行われていない。また、macOS の AppMenu (`src-tauri/src/lib.rs:87, 126`) には `PredefinedMenuItem::close_window` が登録されている。
- `src/App.tsx:75, 160` / `src/hooks/useLocale.tsx:30-51` — `if (!ready) return null;` により、`useLocale` 内の非同期処理（`@tauri-apps/plugin-os` の `locale()` および `initializeI18n()`）が完了するまで React ルートが `null`（空白、`index.html` の `#121214` 背景のみ）を描画する。

## Root Cause Hypothesis

**確信度: medium-high。** 主因として以下の 2 つのメカニズム（またはその複合）が考えられる。

1. **デバッグバイナリ (`target/debug/xlseek`) の直接起動と `devUrl` 依存、および放置後の WebView 再描画/再接続失敗**:
   ログの `/Users/soakaye/.../target/debug/xlseek ; exit;` は、macOS の Finder 等からデバッグ用実行ファイル `target/debug/xlseek` が Terminal.app 経由で直接起動された際の典型的なコマンドライン出力である。Tauri の仕様上、`cargo build`（`custom-protocol` フィーチャなし）でビルドされた `target/debug/xlseek` はフロントエンドアセットをバイナリに埋め込まず、`src-tauri/tauri.conf.json` の `devUrl` (`http://localhost:1420`) から読み込む。Vite サーバーが稼働していない、あるいは放置中に停止・切断された状態で WebView がロードや復帰を行うと、フロントエンドを取得できず空白画面となる。
2. **macOS における Dock アイコンクリック時 (`RunEvent::Reopen`) と WebView 復帰時のライフサイクル未制御**:
   macOS ではアプリをバックグラウンドに長時間放置した際、OS の省電力機能（App Nap や WebKit WebContent プロセスのサスペンド／メモリ回収）が働くほか、ウィンドウが閉じられた状態でもプロセスは Dock に残り続ける。現在 `src-tauri/src/lib.rs` では `Builder::run` をそのまま呼び出しており、`RunEvent::Reopen`（Dock アイコンクリック時のイベント）やウィンドウの `CloseRequested`（macOS で赤ボタンを押した際にウィンドウを破棄せず `hide()` する、または Dock クリック時に `show()` / `unminimize()` / `set_focus()` する処理）を実装していない。
   なお、同時に出力された `error messaging the mach port for IMKCFRunLoopWakeUpReliable` は、バックグラウンドから Dock クリックによってアプリ（および WebView 内の入力コンテキスト）がフォーカスを取り戻した際に macOS の InputMethodKit が発行する既知の OS 診断ログ（`.specify/bugs/ime-mach-port-error/assessment.md` 参照）であり、空白画面の直接原因ではなく「ウィンドウのフォーカス復帰タイミング」を示す付随ログである。

## Proposed Remediation

**Preferred**:
1. **macOS のウィンドウライフサイクルと Dock 再アクティベーション (`RunEvent::Reopen`) の適切なハンドリング (`src-tauri/src/lib.rs`)**:
   - `tauri::Builder` の `.run(...)` を `.build(tauri::generate_context!())?.run(|app_handle, event| { ... })` に変更する。
   - macOS においてメインウィンドウの `WindowEvent::CloseRequested` を補足し、ウィンドウインスタンス（および WebView 状態）が破棄されて不整合になるのを防ぐため、`api.prevent_close()` のうえで `window.hide()` を呼ぶ（アプリの完全終了は `Cmd+Q` またはメニューの Quit で行う標準的な macOS 挙動に合わせる）。
   - `tauri::RunEvent::Reopen { has_visible_windows, .. }` を処理し、Dock アイコンがクリックされた際にメインウィンドウ（`"main"`）を取得して `unminimize()`、`show()`、`set_focus()` を確実に呼び出す。
2. **フロントエンド初期描画時の空白防止 (`src/App.tsx` / `src/hooks/useLocale.tsx`)**:
   - 万一 WebView が OS によって再ロードされた場合でも IPC 待機中に画面が完全な空白（`return null`）で固まらないよう、`useLocale` の初期化に安全タイムアウトまたはデフォルト言語での即時フォールバックを持たせる。
3. **スタンドアロン起動方法の明確化**:
   - `npm run tauri dev` を使わずに単体バイナリとして実行する場合は、フロントエンドアセットをバンドルしたビルド（`npm run build` 後の `cargo build --features tauri/custom-protocol` または `npm run build:gui`）を使用する必要がある点を開発・検証手順で確認する。

**Alternatives** (optional):
- `CloseRequested` でウィンドウを破棄させたまま、`RunEvent::Reopen` 時に `WebviewWindowBuilder` でメインウィンドウを再生成する方式。ただし、検索結果やプレビュー状態（React ステート）がウィンドウを閉じるたびにリセットされるため、macOS としては `hide()` / `show()` で保持する方が UX と安定性の両面で優れる。

**Files likely to change**:
- `src-tauri/src/lib.rs`
- `src-tauri/src/constants.rs`
- `src/hooks/useLocale.tsx` (必要に応じて初期化フォールバックの強化)

**Tests to add or update**:
- `src-tauri/src/constants.rs` または `src-tauri/src/lib.rs`: メインウィンドウラベル定数やメニュー・イベント処理に関するユニットテストの追加・更新。
- フロントエンド (`src/`): `useLocale` が IPC 遅延・失敗時にも UI 描画をブロックしないことを確認するテスト。

## Risks & Considerations

- macOS で `WindowEvent::CloseRequested` を `hide()` に変更する場合、`Cmd+Q` やメニューからの「終了 (Quit)」時にはプロセスが正常終了し、ウィンドウの閉じるボタン（赤ボタン / `Cmd+W`）のみが非表示（バックグラウンド待機）になることを確認する必要がある（Tauri v2 の `PredefinedMenuItem::quit` や `Cmd+Q` は `CloseRequested` ではなくアプリケーション終了として処理されるが、実機・テストでの検証が必要）。
- Windows / Linux ではメインウィンドウを閉じた際にアプリが終了するのが標準挙動であるため、`CloseRequested` の `hide()` 制御および `RunEvent::Reopen` は `#[cfg(target_os = "macos")]` に限定し、他 OS の挙動に影響を与えないようにする。

## Open Questions

- [NEEDS CLARIFICATION: 実行時は Finder から `target/debug/xlseek` をダブルクリックして起動していたか（その場合 Vite サーバー `localhost:1420` は動いていたか）。]
- [NEEDS CLARIFICATION: 放置中にウィンドウは開いたままだったか、最小化または閉じるボタン（×）で閉じていたか。]

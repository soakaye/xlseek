# Bug Assessment: macOS のメニュー関連および終了ライフサイクル処理のコードレビューと根本原因評価

- **Slug**: macos-menu-quit-review
- **Created**: 2026-10-17
- **Source**: pasted text + `code-review` (`5a48147...HEAD`: commits `854eff2`, `311f3dd`)
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> macOSのメニュー関連、終了関連処理の問題が続いているのでcode-review を行い問題の解消をしてください。

### Code Review Results (`5a48147...HEAD`)

#### 1. Standards Axis Review
- **`src-tauri/src/lib.rs` (L246-247)**:
  - **Hard Violation — No Hardcoded Constants (`AGENTS.md` §2 Principle II)**: `.expect("error while building tauri application")` に文字列リテラルがハードコードされている（`src-tauri/src/constants.rs` へ抽出し `// Constant reference:` コメントを付与する必要がある）。
- **`src-tauri/src/constants.rs` (L158-162, L426-430, L549-560)**:
  - **Hard Violation — Missing Constant Reference Comments (`AGENTS.md` §2 Principle II)**: テストコード内で定数参照時の `// Constant reference:` コメントが欠落しており、`test_tauri_config_main_window_label` 内に文字列リテラルがハードコードされている。
  - **Judgement Call — Speculative Generality (Fowler Ch. 3 / YAGNI)**: `MENU_ITEM_QUIT_ID` (`"quit"`) および `MENU_KEY_QUIT` (`"menu.quit"`) が `constants.rs` に定義されているが、`src-tauri/src/lib.rs` および `crates/core/locales/{ja,en}.yml` から一切参照されていないデッドコードになっている。
  - **Judgement Call — Duplicated Code (Fowler Ch. 3)**: `test_tauri_config_main_window_label` が `test_tauri_config_product_name` と全く同じ `tauri.conf.json` 読み込み処理を繰り返している。
- **`src/hooks/useLocale.tsx` (L71-73) & `src/hooks/__tests__/useLocale.test.tsx` (L32, L46, L83, L93-94, L115)**:
  - **Hard Violation — Missing Constant Reference Comments (`AGENTS.md` §2 Principle II)**: `DISPLAY_LANGUAGES.EN`、`DISPLAY_LANGUAGES.JA`、`LANGUAGE_PREFERENCES.DEFAULT` の使用箇所に `// Constant reference:` コメントが欠落している。
  - **Judgement Call — Middle Man (Fowler Ch. 3)**: `useLocale.test.tsx` の `mockSetI18nLocaleWrapper` は単なる委譲ラッパーであり不要。

#### 2. Spec & macOS Lifecycle Axis Review
- **Incorrect Implementation — Tauri v2 (`tauri-runtime-wry` / `tao` / `muda`) における macOS イベント順序の誤認**:
  1. `tauri-runtime-wry` (`src/lib.rs:4253-4271`) において、`RunEvent::ExitRequested { code: None, .. }` は「最後のウィンドウが実際にクローズ・破棄（`TaoWindowEvent::Destroyed`）されて `windows.is_empty()` になった**後**」、または `app_handle.exit(code)` が明示的に呼ばれたときにのみ発火する。
  2. そのため、`on_window_event` で `WindowEvent::CloseRequested` を捕捉して `!is_quitting` のときに `api.prevent_close()` + `window.hide()` を呼ぶ実装では、ウィンドウの `Destroyed` が永遠に発生せず、`RunEvent::ExitRequested` に到達しないデッドロック的な順序逆転が生じている。
  3. さらに、ユーザーが「ウィンドウの閉じるボタン（赤ボタン `×`）」「ファイルメニュー -> ウィンドウを閉じる (`Cmd+W`)」を実行した際にもウィンドウが破棄されずに `window.hide()` で非表示化されるため、GUI ウィンドウが消えた後も `target/debug/xlseek` プロセスが終了せず、Dock に「`xlseek` バックグラウンドで実行中」（黒地に緑の `exec` アイコン）として必ず残り続ける原因となっていた。
  4. また、`muda` の `PredefinedMenuItem::quit` は Cocoa の `sel!(terminate:)` を直接呼び出し、`on_menu_event` も `RunEvent::ExitRequested` も経由せずに `applicationWillTerminate:` (`RunEvent::Exit`) へ進むが、`PredefinedMenuItem` 全般に `None` を渡しているため、`set_menu_locale` で日本語を選択してもメニュー項目（Quit / Hide / Close Window / Edit 項目等）が英語のまま固定される問題、および `PredefinedMenuItem::show_all` が欠落している問題も確認された。

## Symptom

1. **終了・ウィンドウクローズ時のバックグラウンド残留**:
   `src-tauri/src/lib.rs` の `on_window_event` が `WindowEvent::CloseRequested` で `api.prevent_close()` と `window.hide()` を呼んでいるため、ウィンドウを閉じた際（`×` ボタン、`Cmd+W`、メニューの「ウィンドウを閉じる」）にプロセスが終了せず、ウィンドウを失ったバックグラウンドプロセスとして Dock に「`xlseek` バックグラウンドで実行中」アイコンが残り続ける。また、`is_quitting` フラグをセットする `RunEvent::ExitRequested` はウィンドウ破棄後（または `app_handle.exit()` 呼び出し時）にしか発火しないため、`CloseRequested` の時点では常に `is_quitting == false` となり機能していない。
2. **macOS システムメニューの終了処理・ローカライズ不整合**:
   - アプリ終了（Quit）項目が Tauri のイベントハンドラ（`on_menu_event` -> `app_handle.exit(0)`）を通らない `PredefinedMenuItem::quit` 任せになっている一方、`constants.rs` には未使用の `MENU_ITEM_QUIT_ID` / `MENU_KEY_QUIT` が放置されている。
   - `create_app_menu` 内の定義済みメニュー項目（Quit, Hide, Hide Others, Show All, Services, Close Window, Undo, Redo, Cut, Copy, Paste, Select All, Fullscreen, Minimize, Maximize）に翻訳テキストが渡されておらず（`None`）、かつ `Show All` が欠落している。
3. **初期問題（放置後の Dock クリック時の空白画面）との両立**:
   - 最初のバグ（`background-blank-window`）の本質は、ウィンドウを最小化または放置した後に Dock アイコンをクリックした際（`RunEvent::Reopen`）の復帰処理と、`useLocale` の IPC タイムアウトフォールバックであった。`CloseRequested` でウィンドウクローズを `prevent_close()` + `hide()` で妨害する必要はなく、逆にそれがプロセス残留バグ（`dev-quit-orphan-process`）を引き起こしていた。

## Reproduction

1. `npm run tauri dev` でアプリを起動する。
2. 検索を実行後、ウィンドウを最小化・復旧する。
3. ウィンドウの閉じるボタン（`×` / `Cmd+W` / メニューの Close Window）またはメニューから終了操作を行う。
4. `CloseRequested` が `api.prevent_close()` + `window.hide()` によってインターセプトされるケースでは、ウィンドウが非表示になるだけでプロセスが終了せず、Dock に「`xlseek` バックグラウンドで実行中」アイコンが残存する。
5. また、UI 言語を日本語（`ja`）に切り替えても、macOS システムメニューのサブメニュー内の各項目（Quit, Close Window, Copy, Paste 等）が英語のまま表示される。

## Suspected Code Paths

- `src-tauri/src/lib.rs:55-145` (`create_app_menu`) — App サブメニューおよび各サブメニューの構築。`PredefinedMenuItem::show_all` の欠落、各項目へのローカライズ文字列の未指定、および Quit メニュー項目の構成。
- `src-tauri/src/lib.rs:186-256` (`run`) — `on_window_event` における `WindowEvent::CloseRequested` の `api.prevent_close()` + `window.hide()` がウィンドウ破棄と正常なアプリ終了を阻害している。また `on_menu_event` で `MENU_ITEM_QUIT_ID` を処理して `app_handle.exit(0)` を呼ぶ処理が欠落している。
- `src-tauri/src/constants.rs:150-165, 420-435, 534-556` — 未使用だった `MENU_ITEM_QUIT_ID` / `MENU_KEY_QUIT`、およびメニュー項目翻訳キー定数・テストコード内の定数参照コメント不足。
- `crates/core/locales/ja.yml` & `crates/core/locales/en.yml` (および `src-tauri/locales/{ja,en}.yml`) — `menu.quit` を含む macOS メニュー項目の翻訳カタログ定義。
- `src/hooks/useLocale.tsx` & `src/hooks/__tests__/useLocale.test.tsx` — Constitution Principle II に基づく `// Constant reference:` コメントの不足および不要なテストラッパー関数。

## Root Cause Hypothesis

**確信度: high。**
1. Tauri v2 (`tauri-runtime-wry`) のイベントモデルでは、`RunEvent::ExitRequested` は「すべてのウィンドウが `Destroyed` になった後」または「`app_handle.exit()` が呼ばれたとき」に初めて発火する。しかしコミット `854eff2` と `311f3dd` で `WindowEvent::CloseRequested` に対して `api.prevent_close()` + `window.hide()` を入れたため、ウィンドウが決して `Destroyed` にならず、ウィンドウを閉じた際に GUI のみが消えてバックグラウンドプロセス（`xlseek バックグラウンドで実行中`）が残留するバグを作り込んでいた。
2. 単一ウィンドウの本アプリケーション（`xlseek`）においては、メインウィンドウのクローズ（`CloseRequested`）を抑止してバックグラウンドに隠すのではなく、通常どおりクローズ・終了させ、システムメニューの Quit（`Cmd+Q`）からも確実に `app_handle.exit(0)`（または標準終了）が呼ばれてプロセス全体がクリーンに終了する設計にすべきである。一方、ウィンドウが開いたまま最小化・背面放置された状態から Dock アイコンをクリックした際の復帰処理（`RunEvent::Reopen` での `restore_main_window`）および `useLocale` の IPC タイムアウトフォールバックはそのまま維持することで、最初の「放置後の空白画面」と「終了後のバックグラウンドプロセス残留」の両方を副作用なく解決できる。

## Proposed Remediation

**Preferred**:
1. **`WindowEvent::CloseRequested` での `prevent_close()` + `hide()` の撤去とクリーンな終了処理 (`src-tauri/src/lib.rs`)**:
   - `on_window_event` でメインウィンドウのクローズを妨害していた `api.prevent_close()` / `window.hide()` および機能していなかった `is_quitting` フラグを撤去し、ウィンドウを閉じた際や Quit した際にプロセスがバックグラウンドに残留せず確実に終了するようにする。
   - `create_app_menu` の Quit 項目に `MenuItem::with_id(app_handle, MENU_ITEM_QUIT_ID, menu_text(MENU_KEY_QUIT)?, true, Some( ACCELERATOR_QUIT ))`（または `PredefinedMenuItem::quit(app_handle, Some(&menu_text(MENU_KEY_QUIT)?))` と `on_menu_event` での `MENU_ITEM_QUIT_ID` -> `app_handle.exit(0)`）を設定し、システムメニューからの終了時に確実にプロセスを終了させる。
   - `RunEvent::Reopen` 時の `restore_main_window`（最小化解除・表示・フォーカス）は維持し、放置・最小化後の Dock クリック時に確実にウィンドウを前面復帰させる。
2. **macOS システムメニューの標準項目整備とローカライズ対応 (`src-tauri/src/lib.rs`, `crates/core/locales/{ja,en}.yml`, `src-tauri/locales/{ja,en}.yml`)**:
   - `menu.quit` などのメニュー項目翻訳キーを `ja.yml` / `en.yml` に追加し、`PredefinedMenuItem`（Services, Hide, Hide Others, Show All, Quit, Close Window, Undo, Redo, Cut, Copy, Paste, Select All, Fullscreen, Minimize, Maximize）にローカライズされたテキストを渡すとともに、App サブメニューに `PredefinedMenuItem::show_all` を追加する。
   - メニュー構築ロジックの単体テスト（全必須メニュー翻訳キーが `en.yml` / `ja.yml` に存在し解決できることのテスト）を追加する。
3. **Standards 指摘事項の完全解消 (`src-tauri/src/lib.rs`, `src-tauri/src/constants.rs`, `src/hooks/useLocale.tsx`, `src/hooks/__tests__/useLocale.test.tsx`)**:
   - `src-tauri/src/lib.rs` の `.expect(...)` メッセージ等を `src-tauri/src/constants.rs` に定数化する。
   - `src-tauri/src/constants.rs` のテストコードの重複（`tauri.conf.json` 読み込み）をヘルパー関数に共通化し、ハードコード文字列を定数化する。
   - `src/hooks/useLocale.tsx` および `src/hooks/__tests__/useLocale.test.tsx` に `// Constant reference:` コメントを追加し、不要な `mockSetI18nLocaleWrapper` を削除する。

**Files likely to change**:
- `src-tauri/src/lib.rs`
- `src-tauri/src/constants.rs`
- `crates/core/locales/en.yml`
- `crates/core/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `src-tauri/locales/ja.yml`
- `src/hooks/useLocale.tsx`
- `src/hooks/__tests__/useLocale.test.tsx`

**Tests to add or update**:
- `src-tauri/src/constants.rs`: メニュー翻訳キー定数、エラーメッセージ定数、`tauri.conf.json` 検証ヘルパーのユニットテスト。
- `src-tauri/src/lib.rs`: macOS メニューで使用する全翻訳キーが日英カタログ（`ja` / `en`）で欠落なく解決できることを検証するユニットテスト。
- `src/hooks/__tests__/useLocale.test.tsx`: 不要ラッパー削除後のロケール初期化・タイムアウトテスト。

## Risks & Considerations

- `crates/core/locales/{ja,en}.yml` と `src-tauri/locales/{ja,en}.yml` のカタログキーが完全に一致していることを確認する（`tests/catalog.test.ts` および `crates/core` の i18n テストで検証される）。
- `CloseRequested` の `prevent_close()` を撤去することで、ウィンドウの赤ボタン（`×`）、`Cmd+W`、システムメニューの Quit（`Cmd+Q`）のいずれでもバックグラウンドに孤児プロセスが残らずクリーンに終了するようになる。

## Open Questions

- なし（`code-review` の Standards / Spec 両軸の分析により原因と修正箇所がすべて特定済み）。

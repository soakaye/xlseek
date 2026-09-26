# Bug Assessment: システムメニューとアプリ内Aboutダイアログの不一致

- **Slug**: about-dialog-system-menu
- **Created**: 2026-09-26T12:47:30+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> aboutdialog がシステムメニューのaboutdaialogと異なる

## Symptom

macOSのシステムメニューバー（アプリケーションメニュー:「Excel Grep」>「About Excel Grep」）から開くAboutダイアログと、アプリケーション画面右下のステータスバー（Infoアイコン）から開くカスタムAboutダイアログの内容およびUIが一致していない。
期待される動作としては、システムメニューから「About Excel Grep」を選択した場合でも、アプリ内で一元化されたリッチなカスタムAboutダイアログ（アプリ情報およびオープンソースライセンスタブ、各パッケージ情報）が表示されること。

## Reproduction

1. macOS環境で `npm run tauri dev` によりアプリケーションを起動する。
2. macOSの画面上部メニューバーから「Excel Grep」>「About Excel Grep」をクリックする。
   - OSネイティブの標準Aboutパネル（Cocoa標準の小型ダイアログ）が表示される。
3. アプリケーションウィンドウ右下のステータスバーにある情報アイコン（`ABOUT_DIALOG_CONSTANTS.BUTTON_ABOUT_TOOLTIP`）をクリックする。
   - ダークテーマのカスタムモーダルAboutダイアログ（アプリ情報、著作権、および全12サードパーティパッケージのライセンス一覧・詳細タブ）が表示される。
4. 両者の表示形式・提供情報・UIが乖離していることを確認する。

## Suspected Code Paths

- `src-tauri/src/lib.rs:43-48` — `Menu::default(app.handle())` を使用してmacOSのシステムメニューを構築しており、標準の `PredefinedMenuItem::about` が設定されているため、クリック時にOS標準の `orderFrontStandardAboutPanel` が直接呼び出される。
- `src/App.tsx:54, 150-155` — `AboutDialog` の表示制御（`isAboutOpen`）がフロントエンド内のステータスバー操作（`onOpenAbout`）のみに接続されており、Tauriメニューイベントを購読するリスナーが存在しない。
- `src-tauri/src/constants.rs:62-70` / `src/constants/index.ts:20-24` — システムメニューとフロントエンド間でAboutダイアログ起動イベントを通知するためのイベント名定数およびメニュー識別子定数が定義されていない。

## Root Cause Hypothesis

**確信度: High（高）**

Tauriの初期化処理（`src-tauri/src/lib.rs`）において `tauri::menu::Menu::default(app.handle())` を呼び出してシステムメニューを設定しています。Tauriのデフォルトメニュー実装では、macOSのアプリケーションサブメニューの第1項目に `PredefinedMenuItem::about` が自動配置されます。この定義済みメニュー項目は、OSネイティブの `orderFrontStandardAboutPanel:`（Cocoaの標準バージョン情報パネル）をOSレベルで直接実行するため、TauriのWebviewやReactフロントエンドに一切のイベントが通知されません。
そのため、React側で新規実装されたリッチな `AboutDialog.tsx`（ライセンス一覧・詳細表示機能付き）と連携されず、システムメニュー側ではOS標準の簡素なパネルが表示されたままとなり、UIと機能の不整合が生じています。

## Proposed Remediation

**Preferred（推奨）**:
1. **定数の定義**:
   - `src-tauri/src/constants.rs`: メニュー項目ID `MENU_ITEM_ABOUT_ID`（`"open_about"`）、メニュー表示名 `MENU_ITEM_ABOUT_TEXT`（`"Excel Grep について"`）、およびTauriイベント名 `EVENT_OPEN_ABOUT_DIALOG`（`"open-about-dialog"`）を定義する。
   - `src/constants/index.ts`: フロントエンド側の `EVENT_NAMES` に `OPEN_ABOUT_DIALOG: "open-about-dialog"` を追加する。
2. **バックエンドでのカスタムメニュー構築とイベント中継**:
   - `src-tauri/src/lib.rs` において、macOSメニュー構築時に `PredefinedMenuItem::about` の代わりにカスタムメニュー項目 `MenuItem::with_id(app.handle(), MENU_ITEM_ABOUT_ID, MENU_ITEM_ABOUT_TEXT, true, None::<&str>)` を配置する。
   - `app.on_menu_event` ハンドラを登録し、メニューIDが `MENU_ITEM_ABOUT_ID` の場合に `app.emit(EVENT_OPEN_ABOUT_DIALOG, ())` を発行してWebviewへ通知する。
3. **フロントエンドでのメニューイベント購読**:
   - `src/App.tsx` において、`@tauri-apps/api/event` の `listen` を用いて `EVENT_NAMES.OPEN_ABOUT_DIALOG` を購読し、イベント受信時に `setIsAboutOpen(true)` を呼び出してカスタムAboutダイアログを開く（アンマウント時のクリーンアップも含む）。

**Alternatives（代替案）**:
- *代替案1: OS標準パネルのメタ情報強化*: `PredefinedMenuItem::about` のメタ情報引数（`AboutMetadata`）を充実させる。
  - *トレードオフ*: OS標準パネルではライセンスタブや2ペイン検索ビューを表示できず、アプリ内Aboutダイアログとの根本的な不整合が解消されない。
- *代替案2: 別ウィンドウとしてのAbout表示*: AboutダイアログをReactモーダルではなくTauriの独立サブウィンドウとして開く。
  - *トレードオフ*: ウィンドウ管理やルーティングの複雑性が増大し、既存の完成されたモーダルUIコンポーネントを活かせない。

**Files likely to change**:
- `src-tauri/src/constants.rs`
- `src-tauri/src/lib.rs`
- `src/constants/index.ts`
- `src/App.tsx`

**Tests to add or update**:
- `src-tauri/src/constants.rs` 内の新規定数に対する単体テスト（`cargo test`）。
- `cargo check` および `npm run build` によるビルド・型チェックの通過検証。
- 手動検証:
  1. macOSメニューバー「Excel Grep」>「Excel Grep について」をクリックした際、アプリ内のカスタムAboutダイアログが表示されること。
  2. ステータスバー右端のInfoアイコンをクリックした際も、同様にカスタムAboutダイアログが表示されること。
  3. ESCキーまたは閉じるボタン押下で正常にダイアログが閉じること。

## Risks & Considerations

- macOSのアプリケーションメニューにおいて `PredefinedMenuItem::about` 以外の標準機能（Services、Hide、Hide Others、Show All、Quit等）が欠落しないよう、サブメニューの構成要素を慎重に保持して構築する必要がある。
- Tauri v2のパーミッション設定（`src-tauri/capabilities/default.json`）において `core:event:default` が既に許可されているため、フロントエンドとバックエンド間のイベント送受信に追加のパーミッション変更は不要。

## Open Questions

- なし（原因・改修方針ともに完全に特定済み）。

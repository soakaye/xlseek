# 技術調査・設計決定書: マルチプラットフォーム対応 (macOS対応) (Technical Research)

**ブランチ**: `003-macos-support` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md)

---

## 1. ウィンドウ装飾・タイトルバーのネイティブ化

### 決定事項 (Decision)
- `src-tauri/tauri.conf.json` のメインウィンドウ設定において `"decorations": true` を有効化する。
- フロントエンドの `src/components/layout/WindowFrame.tsx` から、HTML 描画による独自タイトルバー（ドラッグ領域、Excel アイコン、アプリ名、最小化・最大化・閉じるボタン）を完全撤去する。
- 独自タイトルバーに表示されていた補助情報（「Calamine Engine」バッジおよびバージョン「v0.1.0」）は、フッターの `src/components/common/StatusBar.tsx` に再配置する。

### 決定理由 (Rationale)
- macOS では左側に赤黄緑のトラフィックライトボタン、Windows では右側に標準のキャプションボタン（最小化・最大化・閉じる）が OS ネイティブに描画され、プラットフォーム標準の操作感（ダブルクリックでズーム/最大化、ドラッグでの移動、スナップ操作など）が自動的に得られる。
- フロントエンドで各 OS のボタン配置やホバー色、ドラッグイベントを模倣・エミュレートする必要がなくなり、Webview 側のコードが劇的にシンプル化される。
- 仕様書 Clarifications（Question 1）で合意された Option A（両プラットフォームで OS ネイティブ装飾に統一）に完全合致する。

### 検討した代替案 (Alternatives Considered)
- **代替案 A**: `decorations: false` のまま、React 側で OS 判定を行い macOS 風トラフィックライトと Windows 風ボタンを描画し分ける。
  - *却下理由*: macOS のトラフィックライトの微細なアニメーション、フルスクリーン移行、ホバー時の記号表示などを完璧にエミュレートするのは困難であり、不自然さが残るため。
- **代替案 B**: `titleBarStyle: "Overlay"` を使用してコンテンツをタイトルバー裏まで透過させる。
  - *却下理由*: 画面上部の検索バー等とトラフィックライトボタンが重ならないようプラットフォーム別のパディング調整が複雑化するため。標準のネイティブタイトルバーが最も堅牢で OS 標準の作法に適する。

---

## 2. macOS におけるファイルマネージャー連携 (`open_in_folder`)

### 決定事項 (Decision)
- macOS 上で `open_in_folder` が呼び出された際、`open -R <file_path>` を実行して Finder を起動し、該当ファイルを選択・ハイライト表示する。
- 対象ファイルが存在しない場合は早期にエラーを返し、実行時例外を防ぐ。親フォルダのみが存在する場合はフォールバックとして `open::that(parent)` を呼び出す。

### 決定理由 (Rationale)
- Windows における `explorer /select,<file_path>` と完全に同等のユーザー体験（親フォルダを開いた上で対象ファイルをハイライト選択）を macOS 上で実現できる。
- `open -R` は macOS 標準の POSIX コマンドラインツールであり、追加の FFI クレートや AppleScript 不要で高速（数ミリ秒）に動作する。
- ユーザーに macOS のアクセス許可ダイアログ（Automation 権限など）を要求することなく動作する。

### 検討した代替案 (Alternatives Considered)
- **代替案 A**: AppleScript (`osascript -e 'tell application "Finder" to reveal POSIX file ...'`) を使用する。
  - *却下理由*: `osascript` の起動オーバーヘッド（50〜100ms）があり、macOS のセキュリティ設定によっては自動化権限の許可ダイアログが表示されるリスクがあるため。
- **代替案 B**: 単に親ディレクトリを `open::that(parent)` で開くのみとする。
  - *却下理由*: フォルダ内に大量のファイルが存在する場合、目的のファイルを探す手間が発生し、Windows 版（選択表示）との機能同等性を満たせないため。

---

## 3. macOS における対応表計算アプリの検出 (`get_supported_apps`)

### 決定事項 (Decision)
- macOS 環境向けに、代表的な表計算対応アプリケーションの検出ロジックを実装する。
  1. **標準アプリの走査**: `/Applications`, `/System/Applications`, `~/Applications` に存在する主要な表計算アプリバンドルを確認：
     - Microsoft Excel (`/Applications/Microsoft Excel.app`, bundle id: `com.microsoft.Excel`)
     - Apple Numbers (`/Applications/Numbers.app` または `/System/Applications/Numbers.app`, bundle id: `com.apple.iWork.Numbers`)
     - LibreOffice Calc (`/Applications/LibreOffice.app`, bundle id: `org.libreoffice.script`)
     - WPS Office (`/Applications/wpsoffice.app`, bundle id: `cn.wps.moffice`)
  2. **既定アプリの判定**: macOS LaunchServices または既定ハンドラ照会コマンド（または Microsoft Excel / Numbers の存在優先度）に基づき、現在 `.xlsx` の既定となっているアプリを判定して `is_default: true` を付与する。
  3. **戻り値**: 検出されたアプリを `SupportedApp` 構造体（`id`, `name`, `executable_path`, `is_default`, `icon_hint`）に格納し、既定アプリを先頭にソートして返却する。

### 決定理由 (Rationale)
- Windows ではレジストリ（`winreg`）を走査しているが、macOS ではアプリケーションが `.app` バンドル構造で標準ディレクトリに配置されるため、標準パスの走査と Info.plist または bundle id の照会が最も確実かつ高速（数ミリ秒）である。
- 外部クレートへの過度な依存（Objective-C 実行時クレートなど）を排除し、標準ライブラリのファイルシステム操作と `std::process::Command` のみで完結できるため、ビルドの信頼性が高い。

### 検討した代替案 (Alternatives Considered)
- **代替案 A**: `core-foundation` や `objc` クレートを導入し、LaunchServices の C API (`LSCopyApplicationURLsForURL`) を呼び出す。
  - *却下理由*: プラットフォーム固有のビルド設定や unsafe な FFI 呼び出しが増加し、保守性と安全性が低下するため。標準アプリケーション走査で実用上 100% の主要ユースケースをカバーできる。
- **代替案 B**: アプリ検出を行わず、常に既定アプリ起動のみとする。
  - *却下理由*: 仕様書（FR-005）および機能 002 との同等性を損なうため。

---

## 4. macOS における「別のプログラムを選択...」ダイアログ (`show_open_with_dialog`)

### 決定事項 (Decision)
- macOS 環境において `show_open_with_dialog` が呼び出された場合、Tauri のダイアログプラグイン（またはシステムファイル選択）を用いて `/Applications` を既定ディレクトリとするアプリケーション選択ダイアログを表示する。
- ユーザーが選択した `.app` パスを取得し、`open -a <app_path> <file_path>` を実行して対象ファイルを開く。ユーザーがダイアログをキャンセルした場合は安全に終了する。

### 決定理由 (Rationale)
- Windows における `rundll32 shell32.dll,OpenAs_RunDLL` に相当する OS ネイティブダイアログを macOS で自然に提供できる。
- 仕様書 Clarifications（Question 2）で合意された Option A に完全準拠する。

### 検討した代替案 (Alternatives Considered)
- **代替案 A**: 単に Finder を開いて「情報を見る」からアプリを選ばせる。
  - *却下理由*: ユーザーに多くの手動操作を強いるため。
- **代替案 B**: macOS ではメニュー項目自体を非表示にする。
  - *却下理由*: 一覧にないエディタや独自ツールで開きたいユーザーの要求を満たせないため。

---

## 5. macOS アプリケーションメニューと標準ショートカットの有効化

### 決定事項 (Decision)
- `src-tauri/src/lib.rs` において、macOS 向けに Tauri v2 の標準メニュー `tauri::menu::Menu::default(&app.handle())?` を登録する。
- これにより、macOS のシステムメニューバーに「Excel Grep（アプリ情報、終了 `⌘Q`）」「編集（取り消し `⌘Z`、切り取り `⌘X`、コピー `⌘C`、貼り付け `⌘V`、全選択 `⌘A`）」「ウィンドウ（最小化 `⌘M`、ズーム、閉じる `⌘W`）」がネイティブに組み込まれる。

### 決定理由 (Rationale)
- macOS の Webview（WKWebView）では、システムメニューバーに「編集」項目が存在しないと、テキスト入力欄での `⌘C` や `⌘V` などの標準クリップボードショートカットが動作しないという既知のプラットフォーム制約がある。
- 仕様書 Clarifications（Question 3）で合意された Option A に完全準拠し、Mac ユーザーにとって標準的で快適な操作性を実現する。

---

## 6. 条件コンパイルとプラットフォーム分離

### 決定事項 (Decision)
- Rust バックエンドの機能分離は、関数レベルおよびモジュールレベルで `#[cfg(target_os = "windows")]` と `#[cfg(target_os = "macos")]` を用いて完全に分離する。
- Windows 専用クレート `winreg` は `Cargo.toml` の `[target.'cfg(windows)'.dependencies]` に維持し、macOS ビルド時にはコンパイル対象外とする。
- フロントエンド（TypeScript）に対する IPC コマンド（`open_in_folder`, `get_supported_apps`, `launch_associated_app`, `show_open_with_dialog`）のインターフェース契約は一切変更せず、プラットフォーム透過性を保つ。

### 決定理由 (Rationale)
- 既存の Windows 向け実装（機能 001/002）に一切のリグレッションを与えず、安全に macOS サポートを追加できる。
- フロントエンドは OS の違いを意識することなく、同じ API を呼び出すだけでそれぞれのプラットフォームに適した動作が得られる。

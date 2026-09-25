# 技術調査・設計決定書: xlsx/xls 登録アプリおよびサポートアプリ起動・フォルダDnD対応 (Research & Decisions)

**機能ブランチ**: `002-launch-associated-app`  
**日付**: 2026-09-25  
**対象仕様書**: [spec.md](./spec.md)

---

## 1. 調査課題と設計決定

### 調査課題 1: Windows における .xlsx/.xls サポートアプリケーション一覧の検出手法

- **背景**:
  ユーザーがプレビューヘッダーのポップアップメニューを開いた際、システムにインストールされている `.xlsx` および `.xls` を開くことができるアプリケーションの一覧を高速かつ正確に取得する必要がある。
- **検討した選択肢**:
  - **選択肢 A: Windows レジストリ（`winreg` クレート）による走査**
    - `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.xlsx\OpenWithList`
    - `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.xlsx\OpenWithProgids`
    - `HKEY_CLASSES_ROOT\.xlsx\OpenWithProgids`
    - 各 ProgID / アプリケーションキーから `shell\open\command` および `FriendlyAppName` を読み出し、実行ファイルパスと表示名を抽出する。
    - 既定アプリは `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.xlsx\UserChoice` の `ProgId` または既定関連付けから判定。
    - メリット: ネイティブAPIで極めて高速（数ミリ秒）、外部プロセス起動なし、メモリ消費ゼロ。
  - **選択肢 B: PowerShell コマンド経由での WMI / レジストリ問い合わせ**
    - メリット: 外部クレート不要。
    - デメリット: PowerShell プロセスの起動コスト（数百ミリ秒〜1秒）がかかり、メニュー表示の要件（300ms以内）を満たせない恐れがある。
  - **選択肢 C: Windows Shell API (`SHAssocEnumHandlers`) の FFI 呼び出し**
    - メリット: OSの「プログラムから開く」と完全に同一のリストを取得可能。
    - デメリット: unsafe FFI の複雑性が高い。
- **決定 (Decision)**:
  **選択肢 A (`winreg` クレート利用のレジストリ直接走査)** を採用する。
  高速かつ安全に ProgID と OpenWithList を取得し、一般的な表計算アプリ（Microsoft Excel, LibreOffice Calc, WPS Spreadsheets 等）や登録エディタを瞬時に列挙できる。

---

### 調査課題 2: OS 標準「プログラムから開く」ダイアログの呼び出し手法

- **背景**:
  ポップアップメニュー末尾の「別のプログラムを選択...」をクリックした際、Windows 標準のアプリ選択画面を表示したい。
- **検討した選択肢**:
  - **選択肢 A: `rundll32.exe shell32.dll,OpenAs_RunDLL [file_path]`**
    - Windows 7〜11 で長年実績のある標準コマンドライン呼び出し。
    - メリット: 追加の外部依存なしで確実に「プログラムから開く」UIが表示される。
  - **選択肢 B: `openwith.exe [file_path]`**
    - Windows 10/11 のモダン OpenWith アプリケーション呼び出し。
    - メリット: モダンUIが表示される。
    - デメリット: 一部の環境や古いビルドでパスが異なる場合がある。
- **決定 (Decision)**:
  **選択肢 A (`rundll32.exe shell32.dll,OpenAs_RunDLL`)** を採用する。
  Windows 全バージョンで最も安定して動作し、対象ファイルパスを引数として渡すだけでユーザーにアプリ選択ダイアログを提示できる。

---

### 調査課題 3: 指定アプリケーションでのファイル起動

- **背景**:
  ユーザーがポップアップメニューから特定のサポートアプリを選択した際、対象ファイルを開く。
- **決定 (Decision)**:
  `std::process::Command::new(&app_executable_path).arg(&file_path).spawn()` を採用する。
  引数を安全に渡すため、パスに含まれる空白やマルチバイト文字（日本語）によるパース崩れを防ぎ、確実に起動できる。

---

### 調査課題 4: フォルダ選択領域へのドラッグ＆ドロップ（DnD）

- **背景**:
  エクスプローラーから検索対象フォルダ（またはファイル）をドラッグ＆ドロップして即座に `target_dir` に設定する。
- **決定 (Decision)**:
  - Tauri v2 ネイティブの `@tauri-apps/api/webview` の `onDragDropEvent` を利用して、ドロップ位置（論理座標）とフォルダ選択領域の矩形との交差判定を行う。
  - Rust 側に `resolve_dropped_path` を実装し、フォルダならそのまま返し、ファイルなら親フォルダを自動解決して返す。
  - ドロップ時はパスの設定のみ行い、ユーザーの明示的な「SEARCH」ボタン押下または Enter キーを待つ（安全・誤操作防止設計）。

---

## 2. 代替案の比較まとめ

| 検討領域 | 採用案 | 却下した代替案 | 却下理由 |
|:---|:---|:---|:---|
| アプリ一覧取得 | `winreg` によるレジストリ走査 | PowerShell 実行 | プロセス起動オーバーヘッド（約500ms〜1s）によるUI遅延の回避 |
| アプリ選択ダイアログ | `rundll32 shell32.dll,OpenAs_RunDLL` | サードパーティライブラリ | OS標準機能のみで安全に実現可能なため不要な依存を避ける |
| メインボタン動作 | 常にOS既定アプリを開く | 直前選択アプリの記憶追従 | OSのシステム設定と常に同期した予測可能な動作を保証するため |
| フォルダDnD | パス設定のみ（検索自動開始なし） | 即時検索自動開始 | 誤って巨大フォルダを投入した際の意図しない重い検索暴走を防ぐため |

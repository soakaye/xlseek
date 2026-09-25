# Excel Grep

Rust と Tauri で構築された、超高速かつ軽量な **MS-Excel 専用 Grep（テキスト・値・数式検索）デスクトップアプリケーション** です。

大量の Excel ファイル（`.xlsx`, `.xlsm`, `.xls`, `.xlsb`）から、指定したキーワードや正規表現に一致するセル、シート、コメントなどを瞬時に横断検索します。

---

## 🌟 主な特徴 (Features)

- ⚡ **Rust による圧倒的な処理速度**
  - マルチスレッド並列処理（`rayon` 等）により、大量・大容量の Excel ブックも高速にパース＆スキャン。
  - 軽量な Tauri アーキテクチャにより、メモリ消費量を抑え快適に動作。
- 🔍 **柔軟な検索オプション**
  - 通常のテキスト部分一致、完全一致、大文字/小文字の区別。
  - 正規表現（Regex）による高度なパターンマッチング。
  - セル値だけでなく、**数式（Formula）**、**コメント/メモ**、**シート名** の検索にも対応。
  - 非表示シート・非表示行/列の検索対象設定。
- 📁 **フォルダ一括・再帰検索**
  - 指定ディレクトリ配下のサブフォルダも再帰的に走査。
  - 拡張子フィルタ（`.xlsx`, `.xlsm`, `.xlsb`, `.xls`）やファイル名除外フィルタ機能。
- 🖥️ **直感的な UI / プレビュー機能**
  - ヒットしたファイル名、シート名、セル番地（例: `B12`）、一致したテキスト・数式を一覧表示。
  - 検索結果から直接対象ファイルを Excel で開く、または該当セルのプレビューを確認。
- 📤 **結果のエクスポート**
  - 検索結果を CSV または Excel 形式で保存し、レポート作成や証跡記録に活用可能。

---

## 🛠️ 技術スタック (Tech Stack)

| レイヤー | 技術 / ライブラリ | 用途・説明 |
| :--- | :--- | :--- |
| **GUI Framework** | [Tauri](https://tauri.app/) (v2) | 軽量・セキュアなクロスプラットフォーム GUI |
| **Backend** | [Rust](https://www.rust-lang.org/) | コアロジック、Excel パース、並列検索処理 |
| **Excel Parser** | `calamine` | 高速 Excel 読み込みライブラリ（xlsx, xls, xlsb 等） |
| **Concurrency** | `rayon` | 複数ファイルの並列読み込み・マルチスレッド走査 |
| **Search Engine**| `regex` | 正規表現検索エンジン |
| **Frontend** | TypeScript, React (or Svelte / Vue), Tailwind CSS | 直感的でレスポンシブなユーザーインターフェース |

---

## 📂 プロジェクト構成 (Directory Structure)

```text
exgrep/
├── src/                # フロントエンドソースコード (UI / 状態管理)
│   ├── assets/         # アイコン・静的ファイル
│   ├── components/     # UI コンポーネント (検索フォーム, 結果テーブル, プレビュー等)
│   └── App.tsx         # メインアプリケーション画面
├── src-tauri/          # Rust バックエンド
│   ├── src/
│   │   ├── search/     # Excel ファイルパース & Grep コア検索ロジック
│   │   ├── commands/   # Tauri IPC コマンド群
│   │   └── main.rs     # エントリポイント
│   ├── Cargo.toml      # Rust 依存クレート定義
│   └── tauri.conf.json # Tauri アプリケーション設定
├── package.json        # フロントエンド依存関係 & ビルドスクリプト
└── Readme.md
```

---

## 🚀 クイックスタート (Getting Started)

### 前提条件 (Prerequisites)

- [Node.js](https://nodejs.org/) (v18 以上推奨)
- [Rust / Cargo](https://www.rust-lang.org/tools/install) (最新の stable)
- **Windows 環境**: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) ランタイム、C++ Build Tools

### セットアップ & 開発実行

1. **リポジトリのクローン**
   ```bash
   git clone <リポジトリURL>
   cd exgrep
   ```

2. **フロントエンド依存パッケージのインストール**
   ```bash
   npm install
   # または pnpm install / yarn
   ```

3. **開発モードで起動**
   ```bash
   npm run tauri dev
   ```

### プロダクションビルド

インストーラーまたは実行可能バイナリを生成します。

```bash
npm run tauri build
```
ビルド完了後、`src-tauri/target/release/bundle/` 配下にインストーラー（`.msi` / `.exe`）が生成されます。

---

## 📖 使い方 (Usage)

1. **検索対象の指定**
   - 「フォルダ選択」ボタンまたはドラッグ＆ドロップで検索対象のディレクトリを指定します。
2. **条件入力**
   - 検索ワードを入力します（正規表現を使用する場合は「正規表現」トグルを ON）。
   - 検索対象（セル値、数式、コメント、シート名）を選択します。
3. **検索の実行**
   - 「検索」ボタンをクリック（または `Enter` キー）。進行状況プログレスバーが表示されます。
4. **結果の確認・操作**
   - 一覧からセルをクリックすると詳細プレビューが表示されます。
   - 「Excel で開く」ボタンで該当ファイルを即座に起動できます。
   - 「エクスポート」から検索結果一覧を CSV / Excel に出力できます。

---

## 🗺️ 今後のロードマップ (Roadmap)

- [ ] パスワード付き Excel ファイルの復号・スキップ制御
- [ ] 検索結果の置換機能（一括置換・バックアップ自動生成）
- [ ] シェイプ（図形・テキストボックス）内のテキスト検索
- [ ] 検索履歴・お気に入り検索条件の保存機能
- [ ] macOS / Linux へのクロスプラットフォーム対応

---

## 📄 ライセンス (License)

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。
# Excel Grep

Excel Grep は、Excel ブックをフォルダ単位で検索するデスクトップアプリです。セルの値・数式・図形（Shape）内テキストを、通常の文字列または正規表現で検索できます。

## 主な機能

- `.xlsx`、`.xlsm`、`.xlsb`、`.xls` を対象にフォルダを再帰検索
- 大文字小文字の区別、正規表現、数式、図形内テキスト、非表示シートを検索オプションで指定
- 検索中の進捗表示とキャンセル
- 結果一覧から一致内容と周辺セルをプレビューし、元ファイルを関連付けアプリで開く
- 検索キーワードとフォルダの履歴、フォルダパスの入力補完
- 検索結果を CSV または XLSX にエクスポート
- 日本語・英語の表示と、既定の検索オプション・履歴件数の設定
- About 画面でアプリ情報と依存パッケージのライセンスを表示

検索オプションの既定値は、数式を含める設定です。図形内テキストと非表示シートは既定で検索対象外です。対象拡張子は4形式すべてが既定で選択されています。

## 技術構成

| 領域 | 技術 |
| --- | --- |
| デスクトップ基盤 | Tauri 2 |
| フロントエンド | React 18、TypeScript、Vite、Tailwind CSS |
| バックエンド | Rust 2021 |
| Excel 読み込み | calamine |
| 並列検索 | rayon |
| 正規表現 | regex |
| エクスポート | csv、rust_xlsxwriter |

## 開発環境

- Node.js と npm
- Rust stable と Cargo
- Tauri の開発に必要な OS ごとの依存関係（[Tauri 前提条件](https://tauri.app/start/prerequisites/)を参照）

```bash
npm install
npm run tauri dev
```

## ビルド

```bash
npm run tauri build
```

Tauri が現在の OS 向けアプリケーションバンドルを `src-tauri/target/release/bundle/` に生成します。

## 検証

```bash
npm run build
npm test
npm run lint
(cd src-tauri && cargo test)
(cd src-tauri && cargo clippy --all-targets -- -D warnings)
(cd src-tauri && cargo fmt --check)
```

## 主なディレクトリ

```text
src/
  components/     React UI
  hooks/          検索・履歴・ロケール管理
  constants/      フロントエンド定数と依存ライセンス一覧
src-tauri/src/
  commands/       Tauri IPC コマンド
  search/         Excel 検索・プレビュー・図形テキスト抽出
  export/         CSV / XLSX 出力
  models/         IPC・検索データ型
specs/            機能仕様・計画・タスク
design/           UI プロトタイプ
```

## ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。

# Excel Grep

Excel Grep は、Excel ブックをフォルダ単位で検索するデスクトップアプリです。セルの値・数式・図形（Shape）内テキストを、通常の文字列または正規表現で検索できます。

## 主な機能

- `.xlsx`、`.xlsm`、`.xlsb`、`.xls` を対象にフォルダを再帰検索
- 大文字小文字の区別、正規表現、数式、図形内テキスト、非表示シートを検索オプションで指定
- 検索中の進捗表示とキャンセル
- 結果一覧から一致内容と周辺セルをプレビューし、元ファイルを関連付けアプリで開く
- 検索キーワードとフォルダの履歴、フォルダパスの入力補完
- 検索結果を CSV または XLSX にエクスポート
- GUIを起動しない `xlseek-cli` コマンドで検索し、CSVまたはXLSXへ保存
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

## コマンドライン検索

CLI（`xlseek-cli`）は GUI とは別の実行バイナリです。リポジトリルートから以下のスクリプトでビルドして実行できます。

```bash
# CLI のみをビルド
npm run build:cli

# 実行
./target/release/xlseek-cli --help
./target/release/xlseek-cli --path ./reports --query '売上' \
  --format xlsx --output ./results.xlsx
./target/release/xlseek-cli --path ./reports --query '売上' \
  --format csv --output ./results.csv --language en
```

必須指定は `--path`、`--query`、`--format csv|xlsx`、`--output` です。値検索・数式・コメント・Shapeを含め、非表示シートを除外する設定が既定です。コメント抽出は `.xlsx` / `.xlsm` の従来メモに対応します。スレッドコメントと `.xls` / `.xlsb` のメモは未対応です。`--help` で全オプションを表示します。出力ファイルが既にある場合は終了コード2で拒否し、`--overwrite` を指定した場合のみ置換します。入力ファイルとの同一パスまたはハードリンクは常に拒否します。終了コードは0が成功（0件を含む）、1が部分失敗、2が要求・全体処理・保存失敗です。

明示上書きでは、公開直前に出力先を再検査します。検査とrenameの間に別プロセスが保存先を変更しないことが前提です。実行中は入力ファイルと出力先の親ディレクトリも変更しないでください。

## ビルド

```bash
# フロントエンドおよび CLI の一括ビルド
npm run build:all

# デスクトップアプリ & インストーラー生成 (GUI本体とCLIが自動同梱されます)
npm run tauri build
```

Tauri が現在の OS 向けアプリケーションバンドルおよびインストーラーを `target/release/bundle/` に生成します。生成されたインストーラー（Windows: MSI / NSIS）には `xlseek`（GUI）と `xlseek-cli`（CLI）の双方が同梱されます。

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

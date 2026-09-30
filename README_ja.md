# Excel Seek (`xlseek`)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) | **日本語**

**Excel Seek (`xlseek`)** は、Excel ワークブック（`.xlsx`、`.xlsm`、`.xlsb`、`.xls`）をフォルダ単位で高速・軽量かつセキュアに再帰検索できるデスクトップ検索アプリケーションおよび CLI ツールです。セルの値、数式、コメント/メモ、図形（Shape）内テキストを、通常の文字列または正規表現を用いて横断検索できます。

---

## 主な機能

- **マルチフォーマット対応**: 指定したディレクトリ配下の `.xlsx`、`.xlsm`、`.xlsb`、`.xls` ファイルを再帰的に検索。
- **柔軟な検索オプション**:
  - 大文字・小文字の区別 / 非区別。
  - 正規表現（Regex）検索。
  - セル値、数式、コメント/メモ、図形（Shape）内テキストの対象切り替え。
  - 非表示シートの検索対象への追加 / 除外。
- **高速かつ並列化された走査**: 高パフォーマンスなスプレッドシート解析ライブラリ [calamine](https://github.com/tafia/calamine) と、マルチスレッド並列処理ライブラリ [rayon](https://github.com/rayon-rs/rayon) を採用。
- **リッチプレビューと直接起動**: 一致したセルの値、数式、および周辺セルをプレビューグリッドで直接確認でき、システムの既定アプリケーションで元ファイルをワンクリックで開くことが可能。
- **検索履歴と入力補完**: 直近の検索キーワードおよび対象ディレクトリパスを履歴として保持し、パスの入力補完をサポート。
- **エクスポート機能**: 検索結果を CSV または整形された Excel（`.xlsx`）ファイルとして直接保存。
- **ヘッドレス CLI 同梱**: GUI を起動せずにバッチ検索と結果出力が可能な `xlseek-cli` を提供。
- **バイリンガル対応**: 日本語と英語の UI 切り替え、デフォルト検索オプションおよび履歴件数の設定が可能。
- **オープンソース情報の透明性**: About ダイアログからアプリケーション情報、バージョン、およびサードパーティ OSS ライセンス条項を確認可能。

> **検索オプションの既定値**: 数式の検索は既定で有効です。図形内テキストと非表示シートは既定で除外されています。対象拡張子は 4 形式（`.xlsx`、`.xlsm`、`.xlsb`、`.xls`）すべてが既定で選択されています。

---

## 技術スタック

| 領域 | 採用技術 |
| :--- | :--- |
| **デスクトップ基盤** | [Tauri v2](https://tauri.app/) |
| **フロントエンド** | React 18, TypeScript, Vite, Tailwind CSS, Lucide React |
| **バックエンド & コアエンジン** | Rust 2021 edition |
| **Excel パーサー** | `calamine` |
| **並列処理** | `rayon` |
| **正規表現** | `regex` |
| **エクスポート** | `csv`, `rust_xlsxwriter` |

---

## 前提条件と開発手順

### 必要要件

- **Node.js & npm**（LTS 推奨）
- **Rust stable & Cargo**
- プラットフォームごとの Tauri 開発用ツールおよびライブラリ（[Tauri 前提条件](https://tauri.app/start/prerequisites/) を参照）

### はじめ方

```bash
# リポジトリのクローン
git clone https://github.com/soakaye/exgrep.git
cd exgrep

# フロントエンド依存関係のインストール
npm install

# Tauri デスクトップウィンドウを起動して開発サーバーを開始
npm run tauri dev
```

---

## コマンドラインインターフェース (CLI)

CLI ツール（`xlseek-cli`）は、GUI アプリケーションとは独立した単体バイナリとして提供されています。直接ビルドして実行できます：

```bash
# CLI バイナリのビルド
npm run build:cli

# ヘルプと利用可能なオプションを表示
./target/release/xlseek-cli --help

# 実行例: 検索結果を XLSX へエクスポート
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format xlsx --output ./results.xlsx

# 実行例: 英語の見出しで CSV へエクスポート
./target/release/xlseek-cli --path ./reports --query 'Revenue' \
  --format csv --output ./results.csv --language en
```

### CLI の仕様とオプション

- **必須引数**: `--path`、`--query`、`--format <csv|xlsx>`、および `--output <path>`。
- **検索範囲**: 既定ではセルの値、数式、コメント/メモ、図形（Shape）を検索対象とし、非表示シートは除外されます。
- **コメント抽出対応**: `.xlsx` および `.xlsm` ファイルの従来のメモ/コメントに対応しています（スレッドコメントおよび `.xls` / `.xlsb` のメモは現在未対応です）。
- **安全性**: 既存の出力ファイルは、`--overwrite` を明示的に指定しない限り上書きされません（終了コード `2` が返されます）。また、入力ファイルと同一のパスやハードリンクへの出力は厳格に拒否されます。
- **終了ステータスコード**:
  - `0`: 成功（一致件数が 0 件の場合を含む）。
  - `1`: 部分的な失敗（一部のファイルが読み取れない、破損している等）。
  - `2`: リクエストエラー、致命的な処理エラー、またはファイル出力の失敗。

---

## パッケージングとビルド

```bash
# フロントエンドおよび CLI バイナリの一括ビルド
npm run build:all

# デスクトップアプリケーションとインストーラーの生成 (GUI と CLI が自動的に同梱されます)
npm run tauri build
```

生成されたアプリケーションバンドルおよび OS 向けインストーラー（Windows: `.msi` / `.exe`、macOS: `.dmg` / `.app`、Linux: `.deb` / `.AppImage`）は `target/release/bundle/` 配下に出力されます。デスクトップアプリケーション本体（`xlseek`）と CLI ツール（`xlseek-cli`）の双方がパッケージ内に同梱されます。

---

## コード品質と検証

フロントエンドおよびバックエンドのテストとリンターを実行するには：

```bash
# フロントエンドの型チェック、テスト、および静的解析
npm run build
npm test
npm run lint

# Rust バックエンドのテスト、clippy、およびフォーマット検証
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

---

## ディレクトリ構成

```text
exgrep/
├── crates/
│   ├── core/           # exlgrep-core: 共有検索エンジン、パーサー、i18n、データモデル、エクスポート処理
│   └── cli/            # xlseek-cli: 独立 CLI コマンドライン実行バイナリ
├── src/                # React / TypeScript フロントエンド
│   ├── components/     # UI コンポーネント群 (検索、結果、プレビュー、About、ダイアログ等)
│   ├── hooks/          # 検索、履歴、多言語化に関するカスタムフック
│   ├── constants/      # フロントエンド一元管理定数および OSS ライセンスカタログ
│   └── types/          # TypeScript ドメイン型定義
├── src-tauri/          # Tauri v2 デスクトップアプリケーションラッパー (xlseek)
│   ├── capabilities/   # Tauri v2 セキュリティケーパビリティ設定
│   └── src/            # IPC コマンド、Tauri ライフサイクル、メニューハンドラ
├── specs/              # 仕様書、アーキテクチャ設計、タスク計画
└── design/             # UI モックアップおよびスタンドアローン HTML プロトタイプ
```

---

## ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。

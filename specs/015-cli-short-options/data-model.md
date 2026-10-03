<!-- 処理内容: CLI短縮オプション、位置引数、複数パス、フォーマット自動推論、非同期パイプライン、標準出力CSVストリーミングのエンティティとデータ構造を定義する。入力・出力: 機能仕様と調査結果を入力とし、データモデル定義（Markdown）を出力する。エラー: 引数モデルと検証制約を明記する。変更履歴: v1.0.0 2026-09-29 AI Agent 初版作成。v1.1.0 2026-09-29 AI Agent 出力先（Option<PathBuf>）のデータモデル更新。 -->
# Phase 1: Data Model (015-cli-short-options)

## 概要

本ドキュメントでは、短縮オプション、位置引数、複数パス走査、および出力先省略時の標準出力（stdout）を扱うための内部データ構造とバリデーションルールを定義する。

---

## 1. 内部データ構造の変更

### `CliOutputTarget` (src-tauri/src/cli/args.rs)

出力先がファイルか標準出力かを表すモデル。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliOutputTarget {
    /// ファイルへ保存（-o / --output で指定されたパス）
    File(PathBuf),
    /// 標準出力へCSVストリーミング出力（-o / --output 省略時）
    Stdout,
}
```

### `CliOptions` (src-tauri/src/cli/args.rs)

```rust
pub struct CliOptions {
    /// 検索条件（キーワード、大小文字区別、正規表現、対象種別、対象拡張子等）
    pub query: SearchQuery,
    /// 1つ以上の検索対象パス（ファイルまたはディレクトリ）
    pub input_paths: Vec<PathBuf>,
    /// 保存先（ファイルパス、または標準出力Stdout）
    pub output_target: CliOutputTarget,
    /// エクスポート形式（Csv または Xlsx）
    pub format: ExportFormat,
    /// 実行・エラー出力言語（"ja" または "en"）
    pub language: String,
    /// 出力ファイルが既に存在する場合の上書き許可フラグ
    pub overwrite: bool,
}
```

---

## 2. バリデーションルール (Validation & Error Constraints)

| チェック項目 | 条件 | エラー文面 / 終了コード |
|---|---|---|
| **クエリ重複** | Position 0 が存在 かつ (`-q` または `--query`) が指定された | `Duplicate CLI option` / 2 |
| **パス重複** | Position 1以降が存在 かつ (`-p` または `--path`) が指定された | `Duplicate CLI option` / 2 |
| **クエリ欠落** | Position 0 も `-q`/`--query` も存在しない | `Missing required CLI option: --query` / 2 |
| **パス欠落** | Position 1以降も `-p`/`--path` も存在しない | `Missing required CLI option: --path` / 2 |
| **出力先省略とxlsx指定** | `-o` 省略 かつ `-f xlsx` / `--format xlsx` が指定された | `Invalid CLI option value: --format requires output file` / 2 |
| **未知の短縮オプション** | 未定義の `-x` が渡された | `Unknown CLI option: -x` / 2 |
| **フォーマット推論失敗** | `-o` あり、`-f` 省略、拡張子が `.csv` でも `.xlsx` でもない | `Output format and extension must match` / 2 |
| **入力と出力の衝突** | ファイル出力先が走査対象のいずれかのファイルと同一（またはハードリンク） | `Output must not alias a search input` / 2 |

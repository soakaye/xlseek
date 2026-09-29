<!-- 処理内容: 共有コアライブラリ (exlgrep-core) が提供する公開APIおよび型定義の契約を規定する。 引数・戻り値: 各関数のシグネチャ、引数型、Result型の戻り値および公開モジュールを定義する。 エラー: 共通エラー文字列やResult型によるエラー伝播仕様を定義する。 変更履歴: v1.0.0 2026-09-29 Antigravity 初版作成。 -->
# Shared Core API Contract: `exlgrep-core`

**Feature Branch**: `016-extract-cli-crate`  
**Date**: 2026-09-29  
**Spec**: [spec.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/spec.md)

## 1. 公開モジュール構成 (Public Modules)

`exlgrep_core` クレートのトップレベル (`src/lib.rs`) から以下のモジュールを公開する：

```rust
pub mod constants;
pub mod export;
pub mod i18n;
pub mod models;
pub mod search;
```

---

## 2. 検索エンジン API (`exlgrep_core::search`)

### ファイル単体の解析と検索 (`search::parser`)
```rust
pub fn parse_and_search_file(
    path: &std::path::Path,
    query: &models::SearchQuery,
    regex: Option<&regex::Regex>,
    progress_sender: Option<&std::sync::mpsc::Sender<models::SearchMatch>>,
) -> Result<Vec<models::SearchMatch>, String>;
```
- **処理**: 単一の Excel ブック（`.xlsx`, `.xlsm`, `.xls`, `.xlsb`）を開き、シートセル、コメント、図形内テキストを検索する。
- **戻り値**: 成功時は一致した `SearchMatch` のベクター、失敗時はエラー理由文字列。

### 検索エンジン構造体 (`search::engine::SearchEngine`)
```rust
pub struct SearchEngine { /* ... */ }

impl SearchEngine {
    pub fn new() -> Self;
    pub fn start_search(
        &self,
        folder_path: &str,
        query: models::SearchQuery,
        progress_callback: impl Fn(models::SearchProgress) + Send + Sync + 'static,
    ) -> Result<models::SearchReport, String>;
    pub fn cancel_search(&self);
    // ...
}
```

---

## 3. エクスポート API (`exlgrep_core::export`)

### CSV エクスポート
```rust
// ファイルへの書き込み
pub fn export_to_csv(
    path: &str,
    items: &[models::SearchMatch],
    language: &str,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
) -> Result<(), String>;

// 汎用ストリーム (std::io::Write) への書き込み
pub fn write_csv_to_writer<W: std::io::Write>(
    writer: &mut W,
    items: &[models::SearchMatch],
    language: &str,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    include_bom: bool,
) -> Result<(), String>;
```

### Excel (.xlsx) エクスポート
```rust
pub fn export_to_xlsx(
    path: &str,
    items: &[models::SearchMatch],
    language: &str,
    catalogs: &std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
) -> Result<(), String>;
```

---

## 4. 多言語カタログ API (`exlgrep_core::i18n`)

```rust
// 埋め込みYAMLカタログ（ja, en）のパースと取得
pub fn load_embedded_catalogs(
) -> Result<std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>, String>;

// カタログからの指定言語キー引き（英語フォールバック付き）
pub fn resolve_catalog_text<'a>(
    catalogs: &'a std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    language: &str,
    key: &str,
) -> Option<&'a str>;
```

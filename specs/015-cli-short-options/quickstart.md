<!-- 処理内容: CLI短縮オプション、位置引数、複数パス、フォーマット自動推論、非同期パイプライン、標準出力CSV出力のエンドツーエンド動作検証手順を定義する。入力・出力: 実装成果物を入力とし、検証手順と期待結果（Markdown）を出力する。エラー: 失敗時の切り分け手順を明記する。変更履歴: v1.0.0 2026-09-29 AI Agent 初版作成。v1.1.0 2026-09-29 AI Agent 標準出力（stdout）検証シナリオを追加。 -->
# Quickstart & Verification Guide (015-cli-short-options)

本ガイドは、追加された短縮オプション、位置引数、複数パス検索、フォーマット自動推論、非同期パイプライン、および出力先省略時の標準出力（stdout）CSVストリーミングの動作を迅速に検証するための手順書である。

---

## 1. ビルド手順

```powershell
# Tauri バックエンドディレクトリへ移動せずに cargo build を実行
cargo build --manifest-path src-tauri/Cargo.toml --bin exlgrep-cli
```

実行バイナリのパス:
- `target/debug/exlgrep-cli.exe` (Windows)

---

## 2. エンドツーエンド検証シナリオ

### シナリオ 1: 標準出力（stdout）へのCSVストリーミング（-o省略）

```powershell
target/debug/exlgrep-cli "test" src-tauri/tests/fixtures
```
- **期待結果**:
  - 終了コード: `0`
  - 標準出力に直接純粋なCSVデータ（ヘッダー行＋データ行）が出力される。
  - サマリー文言（「検索完了:...」）はstdoutに含まれない。

### シナリオ 2: 位置引数 ＋ 複数パス ＋ フォーマット自動推論（ファイル出力）

```powershell
target/debug/exlgrep-cli "test" src-tauri/tests/fixtures/valid.xlsx src-tauri/tests/fixtures -o target/results.csv -w
```
- **期待結果**:
  - 終了コード: `0`
  - `target/results.csv` が正常に出力される。
  - `--format` を指定しなくても `.csv` 形式として自動推論される。
  - 標準出力に完了サマリーが表示される。

### シナリオ 3: 短縮オプション（ショート形式）の指定

```powershell
target/debug/exlgrep-cli -q "test" -p src-tauri/tests/fixtures -o target/results.xlsx -r false -c true -w
```
- **期待結果**:
  - 終了コード: `0`
  - `target/results.xlsx` が正常に出力される。
  - 大文字小文字区別等のフラグが短縮名で正しく伝達される。

### シナリオ 4: オプション重複・競合エラーの確認

```powershell
# 位置引数と -q の重複
target/debug/exlgrep-cli "query1" src-tauri/tests/fixtures -q "query2" -o target/err.csv
```
- **期待結果**:
  - 終了コード: `2`
  - `Duplicate CLI option` エラーが表示され、検索は実行されない。

### シナリオ 5: ヘルプ表示の確認

```powershell
target/debug/exlgrep-cli -h
```
- **期待結果**:
  - 終了コード: `0`
  - 位置引数の構文（`<QUERY> <PATH>...`）、短縮オプション（`-p`, `-q`, `-o` 等）、および `-o` 省略時のstdout CSV出力の案内が含まれたヘルプが出力される。

---

## 3. 自動テスト実行

```powershell
cargo test --manifest-path src-tauri/Cargo.toml cli
```
- すべての単体・統合テストがパスすることを確認する。

<!-- 処理内容: CLIクレート分離後のビルド、テスト、およびエンドツーエンド検証手順を記述する。 引数・戻り値: コマンドライン実行手順を入力とし、実行結果の期待値を出力する。 エラー: ビルド失敗、テスト失敗、リグレッションの早期検出手順を定義する。 変更履歴: v1.0.0 2026-09-29 Antigravity 初版作成。 -->
# Quickstart & Verification Guide: CLIクレート分離の検証手順

**Feature Branch**: `016-extract-cli-crate`  
**Date**: 2026-09-29  
**Spec**: [spec.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/spec.md)  
**Contracts**: [cli-interface.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/contracts/cli-interface.md), [core-api.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/contracts/core-api.md)

## 1. クレート個別ビルド検証 (Independent Crate Builds)

### 1.1 `crates/core` (共有ライブラリ) のビルドとテスト
GUIやTauriランタイムを一切介さずにビルド・テストが実行できることを検証します。
```powershell
# core クレートのビルド
cargo build -p exlgrep-core

# core クレートの単体・統合テスト
cargo test -p exlgrep-core
```
**期待される結果**:
- コンパイルエラーおよび警告 0 件でビルドが完了すること。
- 図形検索・エクスポートテスト等の全テストが PASS すること。

### 1.2 `crates/cli` (独立CLIバイナリ) のビルドとテスト
Tauriに依存しないスタンドアローンCLI実行ファイルが生成されることを検証します。
```powershell
# CLI クレートのビルド
cargo build -p exlgrep-cli

# CLI クレートの単体・統合テスト
cargo test -p exlgrep-cli
```
**期待される結果**:
- Tauriのビルドスクリプトを実行することなく即座にコンパイルが完了すること。
- CLI引数解析・検索・CSV/Excel出力テストが全件 PASS すること。

### 1.3 `src-tauri` (デスクトップGUI) のビルドとテスト
CLIコードを撤去した `src-tauri` が正常にビルド・テストできることを検証します。
```powershell
# src-tauri のビルド
cargo build -p exlgrep

# src-tauri のテスト
cargo test -p exlgrep
```
**期待される結果**:
- CLI専用のソースコードが存在しない状態でコンパイルが成功すること。
- 全テストが PASS すること。

---

## 2. ワークスペース全体の品質検証 (Quality Gates)

プロジェクト憲章および `AGENTS.md` に定められた必須検証コマンドを全件実行します。

```powershell
# 1. ワークスペース全テスト
cargo test --workspace

# 2. Clippy 警告ゼロ検証
cargo clippy --workspace --all-targets -- -D warnings

# 3. フォーマット適合検証
cargo fmt --check

# 4. フロントエンド型チェック＆バンドル検証
npm run build
```
**期待される結果**:
- すべてのコマンドが exit code 0、エラー 0 件、警告 0 件で完了すること。

---

## 3. CLI 手動動作検証シナリオ (Manual Validation)

ビルドされた独立CLIバイナリを直接実行して動作を検証します。

### シナリオ A: ヘルプ表示
```powershell
cargo run -p exlgrep-cli -- --help
```
**期待される結果**:
- ヘルプオプション一覧およびショートオプション（`-o`, `-f`, `-c`, `-s` 等）が表示され、exit code 0 で終了すること。

### シナリオ B: 標準出力へのCSVストリーミング検索
```powershell
cargo run -p exlgrep-cli -- "テスト" .\fixtures\test.xlsx
```
**期待される結果**:
- サマリー文言を混入させずに、純粋な CSV ヘッダーとデータ行が標準出力に表示されること。

### シナリオ C: ファイル出力（CSV / Excel）
```powershell
# CSV出力
cargo run -p exlgrep-cli -- -o output.csv "キーワード" .\fixtures\test.xlsx

# Excel出力
cargo run -p exlgrep-cli -- -o output.xlsx -f xlsx "キーワード" .\fixtures\test.xlsx
```
**期待される結果**:
- 出力先にファイルが正常生成され、サマリー文言（走査ファイル数、一致件数、出力パス）が表示されること。

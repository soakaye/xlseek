<!-- 処理内容: CLI短縮オプション、位置引数、複数パス、フォーマット自動推論、非同期パイプライン、標準出力CSV出力の実装計画、技術構成、憲章適合性を定義する。入力・出力: 承認済み仕様と調査結果を入力とし、実装計画（Markdown）を出力する。エラー: 入力不備・部分失敗・保存失敗の検証条件を明記する。変更履歴: v1.0.0 2026-09-29 AI Agent 初版作成。v1.1.0 2026-09-29 AI Agent 非同期パイプライン対応。v1.2.0 2026-09-29 AI Agent output省略時の標準出力CSVストリーミング対応。 -->
# Implementation Plan: CLI短縮オプション・位置引数・複数パス・非同期パイプライン・標準出力対応 (CLI Short Options, Positional Args, Async Pipeline & Stdout CSV)

**Branch**: `015-cli-short-options` | **Date**: 2026-09-29 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/015-cli-short-options/spec.md`

## Summary

`exlgrep-cli` コマンドラインツールを拡張し、grepライクな位置引数構文（第1引数: 検索語、第2引数以降: 複数検索パス）、標準的な1文字短縮オプション（`-p`, `-q`, `-f`, `-o`, `-c`, `-r`, `-w`, `-l`, `-h` 等）、出力先省略時の標準出力（stdout）への純粋なCSVストリーミング出力、ファイル出力時のフォーマット自動推論、および複数パス走査サポートを実装する。
CLI内部の実行エンジンにおいて、ディレクトリ探索（プロデューサー）とファイル内検索（コンシューマー）を有界同期チャネル（`sync_channel`）と `rayon` 並列スレッドプールで接続する**非同期パイプラインアーキテクチャ**を導入し、探索の完了を待たずに順次並行してファイル解析を実行し、エラー発生時は即座に stderr へ通知する。

## Technical Context

**Language/Version**: Rust 2021 edition。
**Primary Dependencies**: 既存の `calamine`, `rayon`, `regex`, `walkdir`, `rust_xlsxwriter`, `csv`, `same-file`。引数解析およびスレッド間パイプラインは標準ライブラリ（`std::sync::mpsc::sync_channel`）。新規外部クレートは追加しない。
**Storage**: ローカルExcel入力とCSV/XLSX出力。`-o` 省略時は標準出力（stdout）。
**Testing**: `cargo test` による単体テストおよび実プロセスの結合テスト（標準出力パイプ検証を含む）。
**Target Platform**: Windows (コンソールバイナリ) および macOS / Linux。
**Project Type**: CLIツール（デスクトップアプリと同一パッケージ `exlgrep-cli`）。
**Performance Goals**: ディレクトリ探索とファイル解析の同時並行化により、大規模フォルダー検索時のターンアラウンドタイムを短縮し、マルチコアCPUリソースを即座にフル活用する。
**Constraints**: 憲章原則（I〜V）に完全準拠。ハードコード定数の禁止（定数は `constants.rs` に集約）。4要素ヘッダコメントの徹底。

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| 原則・ゲート | 設計上の対応 | 調査前 / 設計後 |
|---|---|---|
| I. 指定言語と自然な出力 | ドキュメントは日本語。CLIは既定ja、明示指定en。ヘルプ・エラー文言はYAML翻訳カタログに短縮構文を反映 | PASS / PASS |
| II. 定数の外部抽出 | 新規の短縮オプション文字、エラー文面、プレースホルダーはすべて `constants.rs` に定義し、ハードコードを排除 | PASS / PASS |
| III. 4要素ヘッダ | 新規・変更する関数・構造体すべてに詳細説明、型付き引数・戻り値、エラー・panic条件、履歴を漏れなく付与 | PASS / PASS |
| IV. モジュール責務 | 引数解析（`args.rs`）、実行・非同期パイプライン走査（`mod.rs`）、出力保護（`output.rs`）、CSV出力（`export/csv_export.rs`）の責務分割を維持 | PASS / PASS |
| V. 堅牢なエラーと検証 | 引数競合・重複エラー、複数パス走査時の部分失敗（ステータス1）、全パス失敗（ステータス2）を網羅 | PASS / PASS |
| 性能とローカル処理 | 非同期パイプライン（`sync_channel` + `rayon`）でストリーミング並行解析。`same-file` で重複排除。外部通信なし | PASS / PASS |
| 品質ゲート | 実装完了前に `cargo test`, `cargo clippy`, `cargo fmt`, `npm run build` をすべて検証 | PASS / PASS |

## Project Structure

### Documentation (this feature)

```text
specs/015-cli-short-options/
├── spec.md                  # 機能仕様書
├── plan.md                  # 本実装計画書
├── research.md              # Phase 0: 調査・技術決定（非同期パイプライン・標準出力含む）
├── data-model.md            # Phase 1: データモデル・バリデーション定義
├── quickstart.md            # Phase 1: 動作検証ガイド
├── contracts/
│   └── cli.md               # Phase 1: CLIインターフェース契約
└── checklists/
    └── requirements.md      # 品質チェックリスト
```

### Source Code (repository root)

```text
src-tauri/
├── locales/
│   ├── ja.yml               # 短縮オプション・位置引数構文・stdout動作を含むヘルプ更新
│   └── en.yml               # 英語ヘルプ更新
├── src/
│   ├── constants.rs         # 短縮オプション文字、定数定義の追加
│   ├── export/
│   │   └── csv_export.rs    # io::Writeを受け取れるジェネリックCSV出力（stdout対応）
│   └── cli/
│       ├── mod.rs           # 非同期パイプライン走査、エラー即時stderr出力、stdout CSVストリーミング
│       ├── args.rs          # 短縮オプション対応、位置引数抽出、フォーマット自動推論、CliOutputTarget
│       └── output.rs        # ファイル出力時の入力衝突検査・一時保存（stdout時はバイパス）
└── tests/
    └── cli_search.rs        # 位置引数・短縮オプション・複数パス走査・stdoutストリーミングのテスト追加
```

## 実装ステップ計画

1. **定数および翻訳カタログの更新**:
   - `src-tauri/src/constants.rs` に短縮オプション文字および定数を定義。
   - `locales/ja.yml` / `locales/en.yml` の `cli.HELP` を更新。
2. **引数解析ロジックの拡張 (`cli/args.rs`)**:
   - `CliOutputTarget`（`File(PathBuf)` / `Stdout`）の導入。
   - 位置引数抽出、短縮オプション解釈、出力先省略時の `Stdout` + `Csv` 設定。
3. **CSV出力の汎用化 (`export/csv_export.rs`)**:
   - ファイルだけでなく `std::io::stdout()` 等の `Write` トレイトオブジェクトへ直接BOMなしで出力可能な関数を提供。
4. **非同期パイプライン実行制御の実装 (`cli/mod.rs` & `cli/output.rs`)**:
   - プロデューサー（別スレッド）: 複数パスを順次走査、即時エラーstderr出力、チャネル送信。
   - コンシューマー（Rayon並列処理）: ファイル内解析、即時エラーstderr出力、一致結果収集。
   - 出力先判定: `Stdout` の場合は直接 stdout へ CSV 出力し、サマリー文言は抑制。`File` の場合は既存の `OutputGuard` を通して安全に一時保存・公開し、stdout にサマリーを出力。
5. **テスト作成と品質ゲート検証**:
   - 単体テスト・結合テストの追加・実行。
   - `cargo test`, `cargo clippy`, `cargo fmt`, `npm run build` のパス確認。

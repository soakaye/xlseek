<!-- 処理内容: CLIプログラムソースのTauriディレクトリ(src-tauri)からの分離・独立化の実装計画書。 引数・戻り値: 仕様書（spec.md）、調査結果（research.md）、データモデル（data-model.md）を入力とし、実装計画書（Markdown文書）を出力する。 エラー: 憲章違反、依存循環、リグレッションを防止するための計画と品質ゲートを定義する。 変更履歴: v1.0.0 2026-09-29 Antigravity 初版策定。 -->
# Implementation Plan: CLIプログラムソースのTauriディレクトリからの分離・独立化 (Separate CLI Program Source from Tauri Directory)

**Branch**: `016-extract-cli-crate` | **Date**: 2026-09-29 | **Spec**: [spec.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/spec.md)

**Input**: Feature specification from `/specs/016-extract-cli-crate/spec.md`

## Summary

`src-tauri` 内に同居していた独立CLIバイナリ（`exlgrep-cli`）の実装およびCLIテストを `src-tauri` から完全に排除し、リポジトリの Cargo ワークスペースを `crates/core`（共有コアライブラリ）、`crates/cli`（スタンドアローンCLIバイナリ）、`src-tauri`（デスクトップGUI）の3クレート構成に再編成する。  
これにより関心の分離を徹底し、CLIプログラムのTauri非依存・単独ビルドを可能にしつつ、既存のCLI機能仕様（014/015）およびGUI機能を 100% 維持する。

## Technical Context

**Language/Version**: Rust 2021 edition (1.75+)  
**Primary Dependencies**:
- `crates/core`: `calamine`, `rayon`, `regex`, `same-file`, `serde`, `serde_json`, `serde_yaml`, `walkdir`, `rust_xlsxwriter`, `chrono`, `encoding_rs`, `cfb`, `quick-xml`, `zip`
- `crates/cli`: `exlgrep-core` (パス参照), `same-file`, `walkdir`, `rayon`, `regex`
- `src-tauri`: `exlgrep-core` (パス参照), `tauri` v2, `tauri-plugin-dialog`, `tauri-plugin-os`, `tauri-plugin-shell`, `tauri-plugin-i18n`, `open`, `winreg` (Windows)  
**Storage**: N/A (ファイルシステム上の Excel ブック読み取りおよび CSV/Excel 結果ファイル出力)  
**Testing**: `cargo test --workspace` (単体テスト、統合テスト、契約検証テスト)  
**Target Platform**: Windows 10/11, macOS, Linux  
**Project Type**: Rust Cargo Workspace (マルチクレート構成: Library + CLI Binary + Tauri Desktop App)  
**Performance Goals**: 既存の並列探索・解析性能を完全維持（Rayon並列ワーカーおよびストリーミングチャネル走査）  
**Constraints**:
- `src-tauri` 配下にCLIコード（`src/bin/exlgrep-cli.rs`, `src/cli/`）が一切残らないこと
- `crates/cli` は Tauri および GUI プラグインに一切依存しないこと
- 憲章原則（日本語品質、定数一元化、4要素ヘッダコメント、Clippy/fmt適合、テスト検証）を完全遵守すること  
**Scale/Scope**: クレート分割・再配置（3 クレート）、移行対象ファイル数約 15 ファイル、リグレッション 0 件

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| 憲章原則 | 適合状況 | 検証・遵守方針 |
|:---|:---:|:---|
| **原則 I: 指定言語の優先と自然な出力** | **PASS** | 日本語ドキュメント、コミットメッセージ、UI・CLIメッセージの自然な日本語を維持。特殊トークン混入なし。 |
| **原則 II: 定数の外部抽出とハードコードの禁止** | **PASS** | `crates/core/src/constants.rs`, `crates/cli/src/constants.rs`, `src-tauri/src/constants.rs` に各クレートの定数を一元管理。0と""以外のハードコードを禁止。 |
| **原則 III: 厳格な 4 要素ヘッダコメント** | **PASS** | すべての新規・移動先ファイル、モジュール、関数に 4 要素（処理内容、引数/戻り値、エラー、変更履歴）を漏れなく記述。 |
| **原則 IV: 責務に応じたモジュール分割と標準スタイル準拠** | **PASS** | 3クレート構成への明確な責務分離。`cargo fmt --check` および `cargo clippy --workspace --all-targets -- -D warnings` 警告 0 件。 |
| **原則 V: 堅牢なエラーハンドリングとテスト検証** | **PASS** | パニックの排除、Result型による適切なエラー伝播。既存CLIテスト・形状検索テストの完全移行と `cargo test --workspace` 全件合格。 |

## Project Structure

### Documentation (this feature)

```text
specs/016-extract-cli-crate/
├── spec.md              # 仕様書（/speckit-specify, /speckit-clarify）
├── checklists/
│   └── requirements.md  # 仕様品質チェックリスト
├── plan.md              # 実装計画書（本ファイル /speckit-plan）
├── research.md          # 調査結果およびアーキテクチャ意思決定
├── data-model.md        # クレート構成・モジュール依存・データモデル
├── contracts/           # インターフェース契約仕様
│   ├── cli-interface.md # CLI引数・終了ステータス契約
│   └── core-api.md      # 共有コアライブラリ公開API契約
└── quickstart.md        # クレート別ビルド・動作検証手順書
```

### Source Code Layout (repository root)

```text
exgrep/
├── Cargo.toml                  # ワークスペース定義 (crates/core, crates/cli, src-tauri)
├── package.json
├── src/                        # フロントエンド (React / TypeScript)
├── crates/
│   ├── core/                   # 共有コアライブラリ (exlgrep-core)
│   │   ├── Cargo.toml
│   │   ├── locales/
│   │   │   ├── ja.yml
│   │   │   └── en.yml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── constants.rs
│   │   │   ├── i18n.rs
│   │   │   ├── models/
│   │   │   ├── export/
│   │   │   └── search/
│   │   └── tests/
│   │       ├── shape_contract.rs
│   │       ├── shape_export.rs
│   │       └── shape_extract.rs
│   └── cli/                    # 独立CLIバイナリ (exlgrep-cli)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── lib.rs
│       │   ├── constants.rs
│       │   ├── args.rs
│       │   └── output.rs
│       └── tests/
│           ├── cli_search.rs
│           └── cli_output.rs
└── src-tauri/                  # デスクトップGUIアプリケーション (exlgrep)
    ├── Cargo.toml              # CLI定義を削除、exlgrep-core に依存
    ├── tauri.conf.json
    ├── locales/
    └── src/
        ├── main.rs
        ├── lib.rs              # cliモジュール参照を削除
        ├── constants.rs        # GUI専用定数
        ├── i18n.rs
        └── commands/
```

**Structure Decision**:  
標準的な Rust マルチクレートワークスペース構成を採用し、共通コア機能を `crates/core` に、CLIバイナリを `crates/cli` に、デスクトップGUIを `src-tauri` に明確に分離する。

## Complexity Tracking

*Constitution Check において違反・例外事項はありません。*

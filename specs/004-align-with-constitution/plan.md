# Implementation Plan: プロジェクト憲章準拠のプログラム改修 (Align Program with Constitution)

**Branch**: `004-align-with-constitution` | **Date**: 2026-09-26 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/004-align-with-constitution/spec.md`

## Summary

プロジェクト憲章（5大原則：自然かつ正確な日本語出力、定数の外部抽出とハードコード禁止、厳格な4要素ヘッダコメント、モジュール分割と標準スタイル準拠、堅牢なエラーハンドリングとテスト検証）に基づき、全コードベース（RustバックエンドおよびTypeScriptフロントエンド）を包括的に改修・リファクタリングする。

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.80+), TypeScript 5.5, React 18

**Primary Dependencies**: Tauri v2, calamine 0.36, rayon 1.10, regex 1.10, csv 1.3, rust_xlsxwriter 0.65, Tailwind CSS, Lucide React

**Storage**: ローカルファイルシステム（MS-Excelブック: `.xlsx`, `.xlsm`, `.xlsb`, `.xls`、CSV/Excelエクスポート出力）

**Testing**: `cargo test`（Rust単体・統合テスト）、`tsc --noEmit`、`npm run build`

**Target Platform**: クロスプラットフォーム デスクトップ（macOS, Windows, Linux）

**Project Type**: デスクトップアプリケーション（Tauri GUI + Rustバックエンドライブラリ）

**Performance Goals**: 大量・大容量Excelファイルのマルチスレッド並列高速スキャン、UIスレッド非ブロッキング、省メモリ動作

**Constraints**:
- `0` および空文字列 `""` を除くすべての固定値・マジックナンバー・固定文字列の定数化（`src-tauri/src/constants.rs`, `src/constants/index.ts`）
- 定数利用箇所における参照コメントの明記（`// 定数参照: ...`）
- 全ファイル、モジュール、構造体、関数、メソッドに対する4要素ヘッダコメントの100%記載
- `<PAD>`、`<pad>` 等の不要トークンおよび文字化けの完全排除
- `cargo clippy --all-targets -- -D warnings` および `cargo fmt --check` の警告ゼロ維持
- 予期せぬパニック（`unwrap` 等）の排除と `Result` による日本語エラーハンドリング

**Scale/Scope**: バックエンド（`src-tauri/src/` 配下15ファイル）、フロントエンド（`src/` 配下14ファイル）の全コードベース

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **原則I: 自然かつ正確な日本語出力（Japanese-First & Quality）**: [PASS]
  - UI、ダイアログ、トースト、エクスポート項目名、エラーメッセージをすべて自然な日本語に統一。`<PAD>` や不要トークンの混入防止検証を策定。
- **原則II: 定数の外部抽出とハードコードの禁止（No Hardcoded Constants）**: [PASS]
  - バックエンド `src-tauri/src/constants.rs` およびフロントエンド `src/constants/index.ts` を新設。`0` と `""` 以外の全リテラルを集約し、参照コメント記法を確立。
- **原則III: 厳格なヘッダコメントとドキュメンテーション（Comprehensive Header Comments）**: [PASS]
  - 必須4要素（処理詳細、引数/戻り値、エラー/panic条件、変更履歴）を含むヘッダコメント規約を策定。
- **原則IV: 責務に応じたモジュール分割と標準スタイル準拠（Modular Design & Code Standards）**: [PASS]
  - Clippy指摘事項（7件）を修正対象として特定し、`cargo fmt` によるフォーマット整形を義務化。
- **原則V: 堅牢なエラーハンドリングとテスト検証（Robust Error Handling & Testing）**: [PASS]
  - パニックを排除し `Result<T, String>` に統一。既存テスト3件のパス維持に加え、定数検証テストを新設。

## Project Structure

### Documentation (this feature)

```text
specs/004-align-with-constitution/
├── plan.md              # 本ファイル（実装計画書）
├── research.md          # 技術調査および設計決定事項
├── data-model.md        # 定数モデル、ヘッダ規約、エラー契約
├── quickstart.md        # 検証・実行ガイド
├── contracts/           # 各種インターフェース規約
│   ├── constants-contract.md
│   ├── header-comment-contract.md
│   └── tauri-commands-contract.md
└── checklists/
    └── requirements.md  # 仕様品質チェックリスト
```

### Source Code (repository root)

```text
src/
├── constants/
│   └── index.ts                 # フロントエンド定数一元定義
├── components/
│   ├── common/                  # 共通UI（StatusBar, Toast）
│   ├── layout/                  # レイアウト（WindowFrame）
│   ├── preview/                 # セルプレビュー（SpreadsheetGrid等）
│   ├── results/                 # 検索結果（ResultTable）
│   └── search/                  # 検索フォーム（SearchBar）
├── hooks/
│   └── useSearch.ts             # 検索カスタムフック
├── types/
│   └── search.ts                # 型定義
├── App.tsx                      # メインコンポーネント
├── index.css                    # スタイル定義
└── main.tsx                     # エントリーポイント

src-tauri/src/
├── constants.rs                 # バックエンド定数一元定義
├── commands/
│   ├── export_cmd.rs            # エクスポートコマンド
│   ├── mod.rs                   # コマンド再エクスポート
│   ├── preview_cmd.rs           # プレビューコマンド
│   ├── search_cmd.rs            # 検索コマンド
│   └── system_cmd.rs            # システム・アプリ連携コマンド
├── export/
│   ├── csv_export.rs            # CSV出力処理
│   ├── mod.rs                   # エクスポートモジュール
│   └── xlsx_export.rs           # Excel出力処理
├── models/
│   └── mod.rs                   # データ構造定義
├── search/
│   ├── engine.rs                # 検索エンジン・進捗管理
│   ├── mod.rs                   # 検索モジュール
│   ├── parser.rs                # Excelパーサー・スニペット生成
│   └── preview.rs               # セル周辺データ抽出
├── lib.rs                       # アプリケーション初期化・ハンドラ登録
└── main.rs                      # エントリーポイント
```

**Structure Decision**: 既存の Tauri v2 デスクトップアプリケーション構造を維持しつつ、定数一元管理用の `constants.rs` および `src/constants/` を追加。すべての既存モジュールに対して憲章原則（定数参照、4要素ヘッダコメント、Clippy修正）を適用する。

## Complexity Tracking

> 憲章に対する逸脱や例外はありません。すべて憲章の規定通りに実装されます。

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| なし | なし | なし |

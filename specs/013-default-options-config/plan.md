<!-- 処理内容: デフォルト動作設定および出力機能の実装計画。入力: spec.md と調査結果。出力: 設計・検証方針。エラー: 未解決事項または憲章違反があれば計画を停止。変更履歴: v1.0.0 2026-09-28 AI Agent 初版策定。 -->
# Implementation Plan: デフォルト動作設定 (Default Options Configuration)

**Branch**: `013-default-options-config` | **Date**: 2026-09-28 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/013-default-options-config/spec.md`

## Summary

検索時の初期オプション（大文字/小文字区別: OFF、正規表現: OFF、数式: ON、Shape内テキスト: OFF、コメント/メモ: ON、非表示シート: OFF、対象拡張子: .xlsx, .xlsm, .xlsb, .xls）を設定画面からカスタマイズ・保存可能とし、ローカルストレージへ永続化して次回起動時および設定保存時に検索バーへ即座に反映する。また、公式既定値へのリセットボタンを提供する。
検索結果のCSV出力およびExcel出力について、OS標準のファイル保存ダイアログを介して指定パスへ保存し、0件時の抑止やキャンセル時の安全性を含む受入シナリオを整備・保証する。

## Technical Context

**Language/Version**: TypeScript 5.5、React 18、Rust 2021 edition

**Primary Dependencies**: Tauri v2 (`@tauri-apps/api`, `@tauri-apps/plugin-dialog`), `lucide-react`, `tailwindcss`, `rust_xlsxwriter`, `csv`. 追加依存なし。

**Storage**: 既存の言語設定・検索履歴と同様に WebView `localStorage` を使用。外部送信は一切行わない。

**Testing**: `npm test`、`npm run build`、`cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`。

**Target Platform**: Windows、macOS、Linux (Tauri v2 デスクトップアプリケーション)。

**Project Type**: React + Rust によるクロスプラットフォームデスクトップアプリ。

**Performance Goals**: 起動時における設定読込は同期的に行われ、画面トグルのチラつき（FOUC）なく即時初期化されること。

**Constraints**: 完全ローカル動作、最小権限、全定数の外部抽出（原則II）、4要素ヘッダコメント（原則III）、型エラーゼロ・Clippy警告ゼロ。

**Scale/Scope**: 設定画面への検索デフォルト設定項目追加、デフォルト値管理モジュールの新設、`useSearch` および `SearchBar` への初期値バインディング、CSV/Excel保存ダイアログフローの受入確認。

## Constitution Check

*GATE: Phase 0 着手前と Phase 1 完了後に確認。*

| 原則・制約 | Phase 0 前 | Phase 1 後の設計確認 |
|---|---|---|
| I. 指定言語 | 合格。仕様書・UI文言は日本語を既定とし、英語リソースも提供 | 合格。多言語カタログ (`src/i18n`) に新項目キーを追加し、ログは英語で統一 |
| II. 定数の外部抽出 | 合格。全設定キー、初期値、UIラベル定数化を計画 | 合格。フロント定数は `src/constants/index.ts` に集約し参照コメントを記載 |
| III. 4要素ヘッダ | 合格。新設・修正ファイル全てに4要素コメントを適用 | 合格。処理内容、引数/戻り値、エラー/例外、変更履歴を完全網羅 |
| IV. 責務分割・標準スタイル | 合格。設定管理コア、UIコンポーネント、検索フックの責務分離 | 合格。モジュール分割と公式スタイルガイドに準拠 |
| V. 堅牢なエラー処理・テスト | 合格。ストレージ破損時のフォールバックおよび単体テスト計画 | 合格。不正設定値復元テスト・拡張子空防止バリデーションを設計 |
| オフライン・最小権限 | 合格。ローカル端末ストレージ完結、追加権限不要 | 合格。外部通信は一切発生しない |

**ゲート判定**: 憲章違反なし。全てのゲートをパス。

## Project Structure

### Documentation (this feature)

```text
specs/013-default-options-config/
├── spec.md              # 仕様書 (/speckit-specify, /speckit-clarify)
├── checklists/          # 品質検証チェックリスト
│   └── requirements.md
├── plan.md              # 実装計画書 (本ファイル)
├── research.md          # Phase 0 調査結果
├── data-model.md        # Phase 1 データモデル設計
├── quickstart.md        # Phase 1 クイックスタート & 動作検証手順
└── contracts/           # Phase 1 契約仕様書
    └── default-options-contract.md
```

### Source Code Layout

```text
src/
├── constants/
│   └── index.ts                 # デフォルトオプション定数 (初期値、ストレージキー)
├── types/
│   ├── search.ts                # 既存検索型定義
│   └── defaultOptions.ts        # デフォルトオプション型定義
├── default-options-core.ts      # 読込、検証、保存、初期値復元ロジック
├── components/
│   ├── settings/
│   │   └── SettingsDialog.tsx   # デフォルト検索オプション設定UI、リセットボタン
│   ├── search/
│   │   └── SearchBar.tsx        # 検索オプション・拡張子トグル表示
│   └── common/
│       └── StatusBar.tsx        # CSV/Excelエクスポートボタン & ダイアログハンドラ
├── hooks/
│   └── useSearch.ts             # 検索オプション初期値の反映と更新
└── App.tsx                      # ルートコンポーネントにおける設定バインディング
```

**Structure Decision**: 既存のモジュール構成（`constants`, `types`, `components`, `hooks`）を厳格に踏襲し、言語や検索履歴と同様のパターンで `default-options-core.ts` を配置する。

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| なし | 憲章に完全適合しており不要な複雑性はない | 単純なlocalStorage永続化と既存UIへの統合を採用 |

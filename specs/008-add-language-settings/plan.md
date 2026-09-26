# Implementation Plan: 表示言語の自動選択と設定画面

**Branch**: `008-add-language-settings` | **Date**: 2026-09-26 | **Spec**: [spec.md](spec.md)

**Input**: `specs/008-add-language-settings/spec.md`

## Summary

「デフォルト」では端末の優先言語から日英を選び、設定画面でデフォルト・日英の即時切り替えと保存を行う。アプリが作る表示文言は言語別の辞書から描画し、欠けた文言は英語へフォールバックする。検索進捗とエラーは言語に依存しない値で受け渡す。CSV・Excel の見出しと macOS のアプリメニューも表示言語へ揃える。診断ログは表示言語に関係なく英語にする。

## Technical Context

**Language/Version**: TypeScript 5.5、React 18、Rust 2021 edition

**Primary Dependencies**: 既存の Tauri v2、React、ダイアログプラグイン。端末言語の取得に Tauri OS 情報プラグインを追加する。変更する React コンポーネントの単体テストに Vitest と jsdom、TypeScript の静的解析に ESLint と TypeScript 対応設定を追加する。翻訳専用ライブラリは追加しない。

**Storage**: 言語設定（`default`・`ja`・`en`）一件を WebView の `localStorage` に保存。検索対象データと診断ログは既存の保存方式を維持。

**Testing**: デフォルトの言語判定、文言欠落時の英語フォールバック、保存処理、変更する React コンポーネント・フックのフロントエンド単体テスト、Rust の単体テスト、`npm test`、`npm run lint`、`npm run build`、`cargo test`、Clippy、rustfmt、および実機での切り替え・再起動確認。

**Target Platform**: 既存の Windows、macOS、Linux 向けデスクトップアプリ

**Project Type**: Tauri デスクトップアプリ

**Performance Goals**: 設定画面からの言語切り替えを1秒以内に画面へ反映し、仕様の30秒以内の操作目標を満たす。検索速度を変更しない。

**Constraints**: 完全オフライン、既存の検索状態を維持、明示的な日英設定が端末言語より優先、デフォルトは端末言語に追随、保存失敗時のセッション内言語維持、表示言語と英語ログの分離、検索語とセル内容をログへ出さない、定数と4要素ヘッダコメントの憲章準拠。

**Scale/Scope**: 現時点は日本語と英語の2表示言語、設定画面にはデフォルトを加えた3選択肢。メイン画面、About、設定、通知、エラー、進捗、アプリメニュー、CSV・Excel の生成見出しを対象とする。

## Constitution Check

*GATE: Phase 0 着手前と Phase 1 完了後に確認。*

| 原則・制約 | Phase 0 前 | Phase 1 後の設計確認 |
|---|---|---|
| I. 指定言語の優先 | 合格。英語表示と英語ログは仕様で明示済み | 合格。画面辞書と英語ログを分離し、原文保持の範囲も定義 |
| II. 定数の外部抽出 | 合格。固定値を既存の定数ファイルに置く方針 | 合格。言語コード、文言、エラーコード、メニュー・出力ラベルを定数へ集約 |
| III. 4要素ヘッダコメント | 合格。追加・変更するコードに適用 | 合格。実装時に各ファイル・関数・型へ記載し確認する |
| IV. モジュール分割と標準スタイル | 合格。言語状態、画面、Rust 側の出力・エラー責務を分離 | 合格。既存の責務境界を保ち、型チェック、ESLint、Clippy、rustfmtで確認する |
| V. エラー処理とテスト | 合格。保存・端末言語取得の失敗を扱う | 合格。安定したエラーコード、失敗時の動作、変更する React コンポーネント・フックと Rust モジュールの単体テストを定義 |
| オフライン・プライバシー | 合格。端末内の設定のみ利用 | 合格。外部通信を使わず、Excel内容・検索語を翻訳しない |

**ゲート判定**: 違反なし。未解決事項なし。

## Project Structure

### Documentation (this feature)

```text
specs/008-add-language-settings/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── localization-contract.md
└── tasks.md                 # $speckit-tasks で作成
```

### Source Code (repository root)

```text
src/
├── App.tsx                  # 言語状態、設定ダイアログ、通知の統合
├── constants/index.ts       # 日英辞書、言語コード、ログ用定数
├── hooks/useSearch.ts       # 進捗・エラー・通知の意味データを保持
├── locale-core.ts           # 言語判定、設定の読み書き、文言選択
├── hooks/useLocale.tsx      # 言語状態の共有
├── components/settings/SettingsDialog.tsx
└── components/             # 既存の画面、About、検索、プレビュー、結果、ステータス
src-tauri/
├── Cargo.toml               # OS 情報プラグイン
├── capabilities/default.json
└── src/
    ├── constants.rs         # エラーコードと日英のメニュー・出力ラベル、英語ログ定数
    ├── lib.rs               # メニュー更新コマンド
    ├── models/mod.rs        # 言語、進捗、エラー、出力要求の契約
    ├── commands/            # エラーコードを返す既存コマンド
    ├── search/              # 言語非依存の進捗とエラー
    └── export/              # 言語別の CSV・Excel 見出し
design/mainui/index.html    # 設定画面のスタンドアローン試作を同期
tests/                     # 言語ロジックと変更する React コンポーネント・フックの単体テスト
vitest.config.ts           # jsdom を使う単体テスト設定
eslint.config.js           # TypeScript/React の警告0件検査
```

**Structure Decision**: 既存の React/Tauri 構成を維持する。文言は既存の定数モジュールに置き、言語判定と共有状態だけを追加する。Rust のコードは既存の検索・出力・コマンドに沿って変更する。UI 変更時は `syncing-mainui-mock` に従う。

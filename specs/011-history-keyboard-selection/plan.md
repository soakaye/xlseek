<!-- 処理内容: 履歴一覧のキーボード操作を既存画面に追加する実装計画を定義する。引数・戻り値: spec.md、憲章、既存コードを入力とし、変更範囲と品質ゲートを出力する。エラー: 憲章違反または未解決事項があれば計画を進めない。変更履歴: v1.0.0 (2026-09-27, Codex): 初版作成。 -->
# Implementation Plan: 履歴一覧のキーボード操作

**Branch**: `011-history-keyboard-selection` | **Date**: 2026-09-27 | **Spec**: [spec.md](spec.md)

**Input**: `specs/011-history-keyboard-selection/spec.md`

## Summary

検索テキストと検索ディレクトリの履歴ボタンから Tab または下矢印で先頭項目へ移り、一覧内で上下矢印・Tab・Enter・Escape を扱う。Tab は末尾から先頭へ循環する。既存の履歴選択処理、フォーカス離脱処理、IME 保護を利用し、画面内のキー操作だけを拡張する。

## Technical Context

**Language/Version**: TypeScript 5.5、React 18

**Primary Dependencies**: 既存の React、Tauri v2、Vitest、Testing Library。追加依存なし。

**Storage**: 変更なし。既存の端末内検索履歴を表示する。

**Testing**: `tests/search-history-ui.test.tsx` の操作テスト、`npm test`、`npm run lint`、`npm run build`、既定の Rust 品質ゲート、デスクトップ画面でのキー操作確認。

**Target Platform**: 既存の Windows、macOS、Linux デスクトップアプリ。

**Project Type**: React と Rust による Tauri デスクトップアプリ。

**Performance Goals**: 履歴表示後の先頭項目の選択は 2 キー以内。既存の最大 50 件の履歴でも、全項目へキーだけで到達可能。

**Constraints**: 履歴選択は検索を開始しない。IME 変換中のキーは処理しない。空履歴で Tab を妨げない。履歴の保存形式・検索処理・ディレクトリ補完の仕様を維持する。新しい固定値は `src/constants/index.ts` へ置き、参照コメントと 4 要素ヘッダを付ける。

**Scale/Scope**: 履歴一覧 2 種と、それぞれの履歴ボタン。新しい保存データ、IPC、画面、依存パッケージは不要。

## Constitution Check

*GATE: Phase 0 着手前と Phase 1 完了後に確認。*

| 原則・制約 | Phase 0 前 | Phase 1 後の設計確認 |
|---|---|---|
| I. 指定言語 | 合格。仕様・設計文書は日本語 | 合格。新しい UI 文言は不要。既存の表示言語を維持する |
| II. 定数の外部抽出 | 合格。キー名や追加する固定値は定数化する | 合格。追加時は `src/constants/index.ts` を参照し、利用箇所に定数参照コメントを置く |
| III. 4 要素ヘッダ | 合格。変更する関数・ファイルの処理、型、エラー、履歴を記録する | 合格。画面とテスト、モックに必要なヘッダを更新する |
| IV. 責務分割・標準スタイル | 合格。既存の履歴 UI 内でフォーカスとキー処理を扱う | 合格。新たな層を作らず、型検査・ESLint を通す |
| V. エラー処理・テスト | 合格。履歴消失やフォーカス離脱を安全に扱う | 合格。空・1 件・複数件、両履歴、IME、取消しを既存の操作テストで検証する |
| オフライン・最小権限 | 合格。端末内の既存履歴を使用する | 合格。外部通信と新規権限は不要 |

**ゲート判定**: Phase 0 前、Phase 1 後ともに憲章違反と未解決の明確化事項はない。

## Project Structure

### Documentation (this feature)

```text
specs/011-history-keyboard-selection/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── history-keyboard-ui-contract.md
```

### Source Code (repository root)

```text
src/
├── components/search/SearchBar.tsx  # 履歴ボタン、一覧、キー操作とフォーカス
└── constants/index.ts               # 追加するキー名・固定値がある場合

tests/search-history-ui.test.tsx      # 両履歴のキー操作と既存動作の回帰確認
design/mainui/index.html             # スタンドアローン UI モックの同期
```

**Structure Decision**: 既存の `SearchBar` が履歴一覧、選択、閉じる操作を保持しているため、ここでキー操作を完結させる。保存フック、検索コマンド、Rust 側は変更しない。

## Design Sequence

1. 既存の履歴画面テストに、両履歴のボタンからの遷移、Tab 循環、上下矢印、Enter、Escape、空・1 件、IME、既存のマウス選択を検証するケースを追加する。
2. `SearchBar.tsx` の履歴ボタンと項目にキー処理とフォーカス移動を追加する。既存の `selectOption` と `closeList` を使い、補完候補への影響を限定する。表示中の項目が減った場合は有効な項目だけへ移る。
3. `design/mainui/index.html` の履歴 UI に同じキー操作を反映する。
4. 操作テスト、`npm run build`、`npm run lint`、`cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`、モックの構文確認、デスクトップでの操作確認を行う。

<!-- 処理内容: Shape 内テキスト検索の実装方針、品質ゲート、成果物を定義する。引数・戻り値: 入力は spec.md と調査結果、出力は実装計画。エラー: 形式別の解析リスクと検証条件を明記する。変更履歴: v1.0.0 2026-09-27 Codex 初版作成。 -->
# Implementation Plan: Excel Shape 内テキストの検索

**Branch**: `012-search-shape-text` | **Date**: 2026-09-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/012-search-shape-text/spec.md`

## Summary

Shape 検索の独立したオン・オフ項目を追加し、4形式すべてのテキストボックス、図形、グループ内の子図形を検索する。セル値・数式の既存経路を保ち、Shape 抽出をファイル形式ごとの責務に分けて検索結果へ合流させる。結果には Shape 名と取得できた位置を持たせ、Shape 結果を選択した時はアンカーの有無にかかわらずセルプレビューを開かない。実際のアンカーがある場合だけ、セル番地を補助情報として示す。

## Technical Context

**Language/Version**: Rust 2021、TypeScript 5、React 18

**Primary Dependencies**: 既存の calamine 0.36、rayon、regex、Tauri v2。描画パートの読取に ZIP・XML、旧形式の読取に CFB/BIFF・OfficeArt の解析が必要。採用する直接依存は [research.md](research.md) に記す。

**Storage**: ローカル Excel ファイルの読取のみ。検索設定の永続保存は追加しない。結果出力は既存の CSV / Excel。

**Testing**: Rust の形式別 fixture テスト、検索統合テスト、フロントエンドの操作テスト。既存の `cargo test`、`npm test`。

**Target Platform**: 既存の Tauri デスクトップ対象 OS。外部アプリのインストールを前提としない。

**Project Type**: ローカル完結のデスクトップアプリ。

**Performance Goals**: 各ファイルに一致する Shape を含む、ローカル保存の100ファイル・合計100 MB を、計測環境を記録した同一端末で検索し、画面に最初の結果を10秒以内に表示して操作を継続可能にする。

**Constraints**: 4形式で図形とグループ内の子図形を検索する。画像・グラフの文字を混入させない。キャンセル可能で、破損データでも検索全体を落とさない。定数・コメント・単体テストは憲章に従う。

**Scale/Scope**: 既存の4形式、結果一覧・プレビュー・CSV / Excel 出力、日英表示。単一ファイル内で数千の Shape があっても、ファイル単位の並列検索と逐次的な結果配送を維持する。

## Constitution Check

*GATE: Phase 0 前に確認し、Phase 1 の設計後に再確認する。*

| 原則 | Phase 0 前の判定 | Phase 1 後の判定 |
|---|---|---|
| I. 言語 | 日英カタログに項目名・結果種別・出力文言を追加する方針。適合 | 画面と出力の契約に日英表示を記載。適合 |
| II. 定数 | 新しい固定値・形式識別子・翻訳キーは `src/constants/index.ts` と `src-tauri/src/constants.rs` に集約する方針。適合 | モデルと契約に定数参照が必要な箇所を限定。適合 |
| III. ヘッダコメント | 追加・変更するファイル、構造体、関数に4要素を記す方針。適合 | 形式別モジュールの境界とコメント対象を定義。適合 |
| IV. モジュール・標準スタイル | OOXML と BIFF/OfficeArt の解析を分離し、既存検索へ結果を合流させる方針。適合 | 分割箇所と品質ゲートを定義。適合 |
| V. エラー・テスト | 形式別 fixture、破損・中断・出力を検証し、部分的な読取失敗を安全に処理する方針。適合 | quickstart と契約に検証経路を定義。適合 |

**Gate result**: 憲章違反なし。4形式の fixture 準備と `.xls` 解析は実装開始時の最初の検証点とする。

## Project Structure

### Documentation (this feature)

```text
specs/012-search-shape-text/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── shape-search-contract.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
src-tauri/src/
├── search/
│   ├── parser.rs                 # 既存検索と Shape 結果の合流
│   └── shape/                    # 形式別の描画・文字抽出
├── models/mod.rs                 # 検索条件・結果・一致種別
├── export/{csv_export,xlsx_export}.rs
└── constants.rs
src/
├── types/search.ts
├── hooks/useSearch.ts
├── components/search/SearchBar.tsx
├── components/results/ResultTable.tsx
├── components/preview/MetaInfoCard.tsx
└── constants/index.ts
src-tauri/locales/{ja,en}.yml
design/mainui/index.html
tests/
└── fixtures/                    # 4形式の検証用ブック
```

**Structure Decision**: 既存の検索経路に形式別 Shape 抽出器を追加し、既存の結果・出力経路を拡張する。新しい外部サービスや別プロセスは設けない。

## Phase 0: Research decisions

形式別抽出、結果位置の表現、プレビュー分岐、ZIP/バイナリ入力の境界、検証用ブックについて [research.md](research.md) に決定と代替案を記録する。`calamine` 単独では Shape 文字列を取得できないため、独立した描画解析が必要。

## Phase 1: Design and contracts

[data-model.md](data-model.md) に検索条件、Shape 識別子、結果とセル位置の関係を定義する。[shape-search-contract.md](contracts/shape-search-contract.md) に Tauri 検索要求・結果と画面・出力の条件を定義する。[quickstart.md](quickstart.md) に4形式、グループ、検索切替、異常・中断、性能の実行可能な検証手順を記す。

**Post-design gate**: 上の Constitution Check を再評価済み。未解決事項はない。

# Bug Assessment: 英語モードで検索結果タイプ表示が折り返される

- **Slug**: i18n-display-layout
- **Created**: 2026-09-26
- **Source**: pasted text and screenshots
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

報告: 「英語モード時: 検索結果のタイプの表示が乱れている」。添付画像2では、Match type 列の `Cell value (Text / Number)` が狭い領域内で複数行に折り返され、固定高さの複数行にまたがるように見える。

別の「プレビュー時」の指摘はユーザーが誤りとして取り消したため、この評価の対象外とする。

## Symptom

英語モードで `CellValue` の検索結果があると、タイプバッジの長い文言が狭い列幅で折り返される。行高は固定されているため、文言が行内に収まらず読みづらい表示になる。期待動作は、行を崩さずタイプを読める状態で表示すること。

## Reproduction

1. アプリで表示言語を English にする。
2. `CellValue` に一致する検索結果を表示する。
3. 検索結果の Match type 列で `Cell value (Text / Number)` が複数行に折り返されることを確認する。

検索語、ブック、ウィンドウ寸法は画像から特定できないため、具体値は `[NEEDS CLARIFICATION]`。

## Suspected Code Paths

- `src/components/results/ResultTable.tsx:131` — `renderBadge` が一致種別ラベルをバッジとして描画し、折返しを制御していない。
- `src/components/results/ResultTable.tsx:225` — Match type 列の幅を `15%` に固定している。
- `src/components/results/ResultTable.tsx:260` — 仮想化行の高さを `virtualRow.size` に固定しており、折り返しに応じて行高が増えない。
- `src/components/results/ResultTable.tsx:286` — 各行のタイプ表示領域も `15%` 幅で、バッジに利用可能な横幅を制限する。
- `src-tauri/locales/en.yml:75` — 英語の CellValue ラベルは `Cell value (Text / Number)`。
- `src/constants/index.ts:69` — 結果行高は `36px`。

## Root Cause Hypothesis

英語ラベルの幅が Match type 列の利用可能幅を超える一方、バッジに `white-space: nowrap`、省略表示、または列幅に応じた短縮がなく、仮想化行高も固定されているために折り返しが発生している。画像と現在の JSX/CSS 構造が一致しており、確度は **high**。

## Proposed Remediation

**Preferred**: タイプバッジの内容を1行に制限し、列幅内で省略表示する。完全なラベルは `title` とアクセシブルなラベルで読めるようにする。ヘッダーと各行の列幅は同じレイアウト定義を共有し、幅が狭い場合も列の位置がずれないようにする。

必要に応じて Match type 列の幅配分を見直す。ただし、単純に幅を増やすだけだとファイル名や一致内容の列を圧迫するため、狭い画面での省略表示も維持する。

**Files likely to change**:
- `src/components/results/ResultTable.tsx`
- `tests/locale-ui.test.tsx` または `tests/results.test.tsx`
- `design/mainui/index.html` — UI モック同期

**Tests to add or update**:
- 英語の長い CellValue ラベルが1行表示または省略表示となり、仮想化された行高を超えて隣接行へはみ出さないことを確認する。
- 英語・日本語の Match type ヘッダーと各行の列位置が一致することを狭い幅で確認する。
- 省略した場合も完全なラベルをアクセシビリティ情報またはツールチップから取得できることを確認する。

## Risks & Considerations

- 列幅のみを変更すると、一致内容列など他の列が狭くなる可能性がある。
- 省略表示を導入する場合、完全な文言を支援技術やツールチップから利用可能にする必要がある。
- 将来の長い翻訳でも崩れないよう、英語文言だけを短くして解決したものとしないこと。

## Open Questions

- [NEEDS CLARIFICATION: 最小対応ウィンドウ幅と、タイプラベルの省略表示を許容するか。]

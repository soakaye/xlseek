# Bug Assessment: 履歴一覧へ下矢印で移動できない

- **Slug**: history-arrow-navigation
- **Created**: 2026-09-27
- **Source**: ユーザーの報告文
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> 履歴ボタン押下後表示された一覧へはTab以外にも下矢印でも移動しなければならない

## Symptom

履歴ボタンをクリックして履歴一覧を表示した後、Tab を使わず下矢印で最初の履歴項目へ移動できない場合がある。履歴一覧が表示中なら、フォーカスが履歴ボタンまたは対応する入力欄にある状態から下矢印で最初の項目へ移動できることが期待される。

## Reproduction

1. 検索テキストまたは検索ディレクトリの履歴ボタンをクリックして一覧を表示する。
2. Tab を押さずに下矢印を押す。
3. 報告では一覧の先頭項目へ移動できない。実行環境でクリック後にどの要素がフォーカスを保持するかは未確認。

## Suspected Code Paths

- `src/components/search/SearchBar.tsx:174-187` — 下矢印処理は `handleHistoryButtonKeyDown` にあり、履歴ボタン自身が keydown を受けた場合に限り先頭項目へフォーカスを移す。
- `src/components/search/SearchBar.tsx:237-252` — 入力欄の `handleListKeyDown` も下矢印を受けるが、選択インデックスを更新するだけで履歴項目へ DOM フォーカスを移さない。
- `src/components/search/SearchBar.tsx:486,553` — キーハンドラーは履歴ボタンへ個別に割り当てられている。クリック後にボタンがフォーカスを保持しない環境では、下矢印がこの処理へ届かない可能性がある。
- `tests/search-history-ui.test.tsx:74-111` — 下矢印でボタンから先頭項目へ移動するテストはあるが、クリック後も入力欄がフォーカスを保持する条件は検証していない。

## Root Cause Hypothesis

**確信度: medium。** 下矢印による先頭項目へのフォーカス移動が履歴ボタン上の keydown にだけ実装されている。一方、対応入力欄で同じキーを受けた時は `activeOption` の更新だけでフォーカスを移さないため、クリック後も入力欄がフォーカスされたままの環境では報告された症状になると考えられる。ボタンのクリック後に実際にフォーカスがどこへ残るかは実機で要確認。

## Proposed Remediation

**Preferred**: 履歴一覧が表示中に対応する入力欄で下矢印が押された場合も、履歴項目が存在すれば先頭項目へフォーカスを移す。既存の履歴ボタン上の処理と共通化または同じ小さな処理へ委ね、ディレクトリ補完候補の既存操作は変えない。両履歴について「クリックで一覧を開いた直後、Tab を挟まず下矢印で先頭項目へ移動する」回帰テストを追加する。

**Files likely to change**:

- `src/components/search/SearchBar.tsx`
- `tests/search-history-ui.test.tsx`
- `design/mainui/index.html`（モックのキー操作も同期する場合）

**Tests to add or update**:

- 両履歴のボタンをクリックして一覧を開いた後、対応入力欄がフォーカスを保持している条件で下矢印を押すと先頭項目へフォーカスが移ること。
- ボタン自体にフォーカスがある場合の下矢印移動も維持されること。
- 空履歴では下矢印で存在しない項目へ移らず、ディレクトリ補完候補の操作を変えないこと。

## Risks & Considerations

- 入力欄の下矢印処理を補完候補にも一律適用すると、既存のディレクトリ補完の選択動作が変わる可能性がある。履歴表示中に限定する。
- 検索条件、永続化データ、バックエンド API への変更は不要。

## Open Questions

- [NEEDS CLARIFICATION: 再現環境（OS・入力欄／履歴ボタンのどちらにフォーカスが残っているか）を実機で確認する。]

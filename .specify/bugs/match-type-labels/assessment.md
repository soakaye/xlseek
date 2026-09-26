# Bug Assessment: 検索結果の一致種別ラベルを短縮する

- **Slug**: match-type-labels
- **Created**: 2026-09-27
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: low

## Report (verbatim or summarized)

報告: 「match typeは、検索結果では、値/Value, 数式/Formulaとする。一致の詳細情報は現行通り。」

検索結果一覧の CellValue と Formula の表示を、それぞれ日本語では「値」「数式」、英語では「Value」「Formula」にする。一致の詳細情報に表示するラベルは現行のまま維持する。

## Symptom

検索結果一覧で一致種別に詳細向けのラベルが使われ、日本語・英語の CellValue 表示が「セル値 (Text / Number)」「Cell value (Text / Number)」と長い。期待動作は一覧では指定された短いラベルを表示し、詳細カードでは現在のラベルを保つこと。

## Reproduction

1. 表示言語を日本語または英語に設定する。
2. CellValue または Formula に一致する検索結果を表示する。
3. 検索結果一覧と一致詳細情報の一致種別ラベルを確認する。

検索対象のブックや検索語は表示ラベルの確認に不要。

## Suspected Code Paths

- `src/components/results/ResultTable.tsx:142` — 一覧の `renderBadge` が `ui.MATCH_TYPE_CELL_VALUE` と `ui.MATCH_TYPE_FORMULA` を使用している。
- `src/components/preview/MetaInfoCard.tsx:53` — 一致詳細情報も同じ翻訳キーを使用しているため、既存キー自体を短縮すると詳細情報まで変わる。
- `src-tauri/locales/ja.yml:75` — 現在の日本語 CellValue 表示は `セル値 (Text / Number)`。
- `src-tauri/locales/en.yml:75` — 現在の英語 CellValue 表示は `Cell value (Text / Number)`。
- `tests/locale-ui.test.tsx` — 日英の一覧ラベル表示を検証するロケール UI テストがある。

## Root Cause Hypothesis

検索結果一覧と一致詳細カードが同じ翻訳キーを共有しているため、詳細向けに説明を含めたラベルが一覧にも表示されている。表示先ごとの文言要件を翻訳キーで分離すれば、一覧を短縮しつつ詳細表示を維持できる。確度: **high**。

## Proposed Remediation

**Preferred**: 検索結果一覧用に CellValue と Formula の翻訳キーを分け、日英カタログでそれぞれ `値` / `Value` と `数式` / `Formula` を定義する。`ResultTable` のバッジだけ新しいキーを使い、`MetaInfoCard` は既存キーを使い続ける。Comment と HiddenSheet の表示は現行のままとする。

**Files likely to change**:
- `src/components/results/ResultTable.tsx`
- `src-tauri/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `tests/locale-ui.test.tsx`
- `design/mainui/index.html` — 検索結果モックを同期する場合

**Tests to add or update**:
- 日英の検索結果一覧で CellValue と Formula が指定された短いラベルになることを確認する。
- 日英の一致詳細カードでは CellValue と Formula の既存ラベルが維持されることを確認する。
- Comment と HiddenSheet の一覧ラベルが変化しないことを確認する。

## Risks & Considerations

- 既存キーの値を書き換えると詳細カードの表示も変わるため、一覧用のキーを追加して利用箇所を限定する。
- ロケールカタログ間で新規キーが揃わない場合、翻訳フォールバックにより表示が不一致になる可能性がある。
- `design/mainui/index.html` の検索結果モックを更新しないと、実アプリとプロトタイプの表示がずれる。

## Open Questions

- なし。

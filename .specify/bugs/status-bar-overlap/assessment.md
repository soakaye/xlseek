# Bug Assessment: ステータスバーの進捗表示と詳細文が重なる

- **Slug**: status-bar-overlap
- **Created**: 2026-09-27
- **Source**: ユーザーの報告文と添付画像
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> ステータスバーの表示が重なっている。

添付画像では、英語表示でファイル検出中、プログレスバーの右側にある「Discovering (28…)」と、続く「Discovering: 28 files (116 matches, 6.69s) - audit_2026.xlsx」が重なっている。

## Symptom

ファイル検出中の進捗欄の文字が、隣の詳細文の領域へはみ出して読みにくくなる。両者はそれぞれの表示領域内に収まり、重ならないことが期待される。

## Reproduction

1. 英語表示で Excel ファイルの検索を開始する。
2. 総ファイル数が未確定のまま、走査済みファイル数が 1 以上になる状態を観察する。
3. 添付画像の状態では、走査済み 28 ファイル時に進捗欄と詳細文が重なる。

[NEEDS CLARIFICATION: ウィンドウ幅、表示倍率、日本語表示でも発生するかは未確認。実アプリでの再現操作は未実施。]

## Suspected Code Paths

- `src/components/common/StatusBar.tsx:178-203` — 進捗欄を `w-44` の固定幅にし、内部のプログレスバーと可変長の進捗文字列をともに `flex-shrink-0` で配置している。
- `src/components/common/StatusBar.tsx:205-214` — 同じ行の後続要素として詳細文を表示している。詳細文自身には `truncate` があるが、前要素からはみ出す文字は抑制しない。
- `src-tauri/locales/en.yml:126` — 該当する英語の「Discovering」を定義している。
- `design/mainui/index.html:520-534` — 同じ固定幅と縮小不可の組み合わせを持つ UI モック。

## Root Cause Hypothesis

**確信度: high。** `w-44` は 176px だが、内部のバー `w-24` が 96px、`gap-2` が 8px を占め、文字列に残る幅は約 72px となる。「Discovering (28)」はその幅より長く、文字要素は `flex-shrink-0` によって縮まない。親要素にもはみ出しを抑える指定がないため、詳細文の上へ描画される。画像の重なり位置と一致する。

## Proposed Remediation

**Preferred**: `StatusBar.tsx` の進捗欄で、文字要素が固定幅内で縮小・省略されるようにする。総数未確定時は走査済み件数など短い表示にし、詳細な「Discovering: 28 files ...」は既存の詳細文に任せる。英語・日本語の双方と大きな件数でも、進捗欄の文字が隣接領域へ出ない CSS 制約を設ける。UI モックの対応箇所も同期する。

**Files likely to change**:

- `src/components/common/StatusBar.tsx`
- `design/mainui/index.html`
- 該当するフロントエンドテストファイル

**Tests to add or update**:

- 総数未確定かつ走査済みファイル数が 1 以上の状態で、英語・日本語の進捗表示が固定幅を超えて詳細文と重ならないことを、実際のレイアウトで確認する。
- 総数確定後の百分率表示、待機状態、狭いウィンドウ幅でも重なりがないことを確認する。

## Risks & Considerations

- 省略によって進捗欄の情報量が減るが、走査済み件数と検出状態は隣の詳細文でも表示している。
- DOM の文字列だけを検査するテストでは描画上の重なりを検出できない。レイアウトの目視確認または要素の境界を比較する検証が必要。
- API・データ・セキュリティへの影響は見込まれない。

## Open Questions

- [NEEDS CLARIFICATION: 問題発生時のウィンドウ幅と表示倍率は何か。]
- [NEEDS CLARIFICATION: 日本語表示でも同様の重なりが発生するか。]

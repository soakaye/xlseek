# Bug Assessment: Word検索結果プレビュー表示時の指定行即時表示への改善

- **Slug**: word-preview-instant-scroll
- **Created**: 2026-10-16T11:57:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: low

## Report (verbatim or summarized)

> wordの検索結果のプレビュー表示時に指定行までスクロールするのは操作性が劣る。指定行周辺を直ちに表示する。

Word ドキュメント（`.docx`, `.docm`, `.doc`）の検索結果をプレビューする際、対象行・段落までスムーズスクロール（`behavior: "smooth"`）のアニメーションが行われることで、目的の段落が表示されるまでに待たされ操作性が劣っているという指摘です。スムーズスクロールを行わずに、指定行とその周辺を直ちに（即座に）表示することが求められています。

## Symptom

- **観測される振る舞い**:
  検索結果テーブルで Word 文書の行を選択した際、プレビュー領域（`WordTextPreview.tsx`）が文書の先頭から対象段落まで時間をかけてスムーズスクロール（`behavior: "smooth"`）アニメーションを実行し、さらに 50ms の `setTimeout` 遅延も発生するため、対象行と周辺テキストが即座に確認できず操作テンポが損なわれる。
- **期待される振る舞い**:
  Word 文書の検索結果行を選択した際、スクロールアニメーションを伴わずに対象段落の位置へ即時ジャンプ（`behavior: "auto"` / `"instant"`）し、対象段落およびその周辺行が中央に配置された状態で直ちに表示される。

## Reproduction

1. Word 検索を有効にし、数十〜数百行以上の段落が存在する Word 文書（`.docx`, `.docm`, `.doc`）を対象にキーワード検索を実行する。
2. 文書の後半（例: 30段落目以降）でヒットした検索結果行をテーブル上でクリックして選択する。
3. 右ペインのプレビュー領域を観察する。
4. 先頭段落（1行目）が表示された後、対象段落に向かってスムーズスクロールのアニメーションがゆっくり流れ、目的の段落が表示されるまでに遅延が発生することを確認する。

## Suspected Code Paths

- `src/components/preview/WordTextPreview.tsx:51-61`:
  - `useEffect` 内で 50ms の `window.setTimeout` を介して `targetParagraphRef.current?.scrollIntoView({ behavior: "smooth", block: "center" })` を呼び出している。
  - `behavior: "smooth"` によりブラウザ側でスムーズアニメーションが強制され、長大な文書ほどスクロール到達に時間を要する原因となっている。
- `src/constants/index.ts:165-168`:
  - `WORD_UI_CONSTANTS` にプレビューのスクロール挙動（`behavior` や `block` 配置）に関する定数が未定義であり、コンポーネント内にマジックストリング・マジックナンバーが直接記述されている。

## Root Cause Hypothesis

**確信度: 高 (High)**

Word 検索プレビュー機能の実装時（仕様 024）、該当ヒット箇所を目立たせる意図で `scrollIntoView` のオプションに `behavior: "smooth"` が指定され、さらに `setTimeout(..., 50)` による遅延呼び出しが行われていました。
しかし、検索結果を次々と切り替えて確認する実際のユースケースにおいては、スムーズスクロールのアニメーション時間が表示遅延（タイムラグ）として作用し、操作テンポを著しく低下させる要因となっていました。

## Proposed Remediation

**Preferred (推奨方針)**:
1. **即時スクロール（instant/auto）への変更**:
   - `src/components/preview/WordTextPreview.tsx` の `useEffect` において、`window.setTimeout` による 50ms の人為的遅延を廃止する。
   - `scrollIntoView` のオプションを `behavior: "auto"`（または `"instant"`）および `block: "center"` に変更し、プレビューの描画と同時に対象段落およびその周辺行がビューポート中央に即時表示されるようにする。
2. **定数の中央管理（憲法 Principle II の遵守）**:
   - `src/constants/index.ts` の `WORD_UI_CONSTANTS` に `SCROLL_BEHAVIOR: "auto"` および `SCROLL_BLOCK: "center"` を定義し、マジックストリングを完全に排除する。
3. **コンポーネントテストの追加**:
   - `tests/word-preview.test.tsx` を新設し、`WordTextPreview` コンポーネントが描画時に対象段落要素に対して `scrollIntoView` を `behavior: "auto"`, `block: "center"` で即座に呼び出すことを検証する。

**Alternatives (代替案)**:
- **代替案 1（`scrollTop` による手動位置設定）**:
  - `containerRef.current.scrollTop` を対象要素の `offsetTop` とコンテナの高さから計算して直接設定する。
  - *トレードオフ*: 計算処理が複雑化し、フォントやスタイルのリフロータイミングの影響を受けやすくなる。標準の `scrollIntoView({ behavior: "auto", block: "center" })` の方がシンプルかつ堅牢。
- **代替案 2（Excel プレビュー同様にバックエンド側で前後行のみに絞り込む）**:
  - `extract_word_document` で対象段落の前後 N 行のみを返すようにする。
  - *トレードオフ*: Word 文書全体の文脈をスクロールして確認できなくなるため、文書ビューワーとしての閲覧性が損なわれる。またバックエンドと IPC 契約の変更が必要になる。

**Files likely to change**:
- `src/constants/index.ts`
- `src/components/preview/WordTextPreview.tsx`
- `tests/word-preview.test.tsx` (新規追加)

**Tests to add or update**:
- `tests/word-preview.test.tsx`:
  - `WordTextPreview` が対象段落を持つペイロードで描画された際、`scrollIntoView` が `behavior: "auto"`（即時）および `block: "center"` で遅延なく呼ばれることをテストする。
  - ペイロード切り替え時にも即時に対象段落へスクロール位置が更新されることをテストする。

## Risks & Considerations

- **jsdom 環境での `scrollIntoView` モック**:
  - Vitest / jsdom 環境では `Element.prototype.scrollIntoView` がネイティブ実装されていないため、ユニットテスト実行時に `window.HTMLElement.prototype.scrollIntoView = vi.fn()` で適切にモックおよびスパイを配置して呼び出し引数を検証する必要がある。
- **リグレッションのリスク**:
  - 変更範囲は Word ドキュメントのプレビューコンポーネントのみに閉じており、Excel プレビューや検索エンジンコアへの影響・副作用はない。

## Open Questions

- なし（ユーザー確認により、スムーズスクロールアニメーションを廃止し、対象段落および周辺行を直ちに即時ジャンプで中央表示する方針で合意済み）。

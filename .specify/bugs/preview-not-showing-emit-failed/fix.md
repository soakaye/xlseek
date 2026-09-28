# Bug Fix: 大量マッチ時のイベント過負荷解消および Shape プレビュー表示の改善

- **Slug**: preview-not-showing-emit-failed
- **Fixed**: 2026-09-28
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

大量マッチ（700件超）発生時に同期 `emit` で WebView2 のメッセージキューが飽和して `Failed to emit Tauri event` が継続発生する問題を解消するため、50件または25ms間隔のバッチイベント送信（フロントエンドでの配列展開対応含む）を導入しました。また、アンカーセル情報を持つ Shape（図形）ではセルプレビューを読み込み、アンカーなし Shape では空欄とならないよう全文・図形名と専用ガイダンスを表示するように改善しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/constants.rs` | modified | `SEARCH_MATCH_BATCH_SIZE` (50) および `SEARCH_MATCH_BATCH_INTERVAL_MS` (25) を定義し、単体テストを追加 |
| `src-tauri/src/commands/search_cmd.rs` | modified | `execute_search` 内でマッチ結果をバッファリングし、バッチ単位で `EVENT_SEARCH_MATCH` を送信、検索終了時に残バッファをフラッシュ |
| `src-tauri/locales/ja.yml` | modified | ガイダンスキー `ui.PREVIEW_SHAPE_NO_ANCHOR` を追加（日本語） |
| `src-tauri/locales/en.yml` | modified | ガイダンスキー `ui.PREVIEW_SHAPE_NO_ANCHOR` を追加（英語） |
| `src/hooks/useSearch.ts` | modified | `EVENT_NAMES.SEARCH_MATCH` でのバッチ（配列）受信対応、およびアンカーあり Shape（`row_index > 0 && col_index > 0`）での `loadPreview` 呼び出し制御を追加 |
| `src/App.tsx` | modified | アンカーあり Shape およびセルでの通常グリッド表示、アンカーなし Shape での全文・図形名・ガイダンスカード表示の切り替えを追加 |
| `tests/shape-results-ui.test.tsx` | modified | アンカー付き Shape 選択時に `get_cell_preview` が呼び出され、アンカーなし Shape 選択時にプレビュー呼び出しが抑止されることを検証するテストへ更新 |

## Diff Highlights

### 1. バックエンドでのバッチ送信 (`src-tauri/src/commands/search_cmd.rs`)
```rust
let mut batch: Vec<SearchMatch> = Vec::with_capacity(constants::SEARCH_MATCH_BATCH_SIZE);
let mut last_emit = std::time::Instant::now();
// ...
let search_result = execute_search(
    // ...
    |match_item| {
        batch.push(match_item);
        if batch.len() >= constants::SEARCH_MATCH_BATCH_SIZE
            || last_emit.elapsed().as_millis() >= constants::SEARCH_MATCH_BATCH_INTERVAL_MS
        {
            let to_send = std::mem::replace(&mut batch, Vec::with_capacity(constants::SEARCH_MATCH_BATCH_SIZE));
            last_emit = std::time::Instant::now();
            let _ = app_handle_clone.emit(constants::EVENT_SEARCH_MATCH, &to_send);
        }
    },
);
// 残バッファの強制フラッシュ
if !batch.is_empty() {
    let _ = app_handle_clone.emit(constants::EVENT_SEARCH_MATCH, &batch);
}
```

### 2. フロントエンドでのバッチ受領 (`src/hooks/useSearch.ts`)
```typescript
const unlistenMatch = await listen<SearchMatch | SearchMatch[]>(
  EVENT_NAMES.SEARCH_MATCH,
  (event) => {
    if (activeSearchIdRef.current === null) return;
    const incoming = Array.isArray(event.payload) ? event.payload : [event.payload];
    if (incoming.length === 0) return;
    setResults((prev) => [...prev, ...incoming]);
    setResultCount((prev) => prev + incoming.length);
  },
);
```

### 3. アンカー有無による Shape プレビュー判定 (`src/hooks/useSearch.ts`)
```typescript
// Shape の場合、セル番地（アンカー）が存在する場合のみセルプレビューを読み込み
if (item.match_type === "Shape" && (!item.row_index || !item.col_index)) {
  setPreviewData(null);
  setSelectedCell(null);
} else {
  await loadPreview(item.full_path, item.sheet_name, item.row_index, item.col_index);
}
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::test_pipeline_constants` — バッチ定数 `SEARCH_MATCH_BATCH_SIZE` と `SEARCH_MATCH_BATCH_INTERVAL_MS` の妥当性アサーション
- `tests/shape-results-ui.test.tsx` — アンカー付き Shape (`row_index > 0`) 選択時に `get_cell_preview` が呼ばれ、アンカーなし Shape (`row_index === 0`) 選択時に呼び出されないことの検証

## Local Verification

- `npm test`: 11 テストファイル、全 65 件パス (exit 0)
- `npm run build`: TypeScript 型検査 (`tsc`) および Vite バンドル成功 (exit 0)
- `cargo test`: 全 43 件（単体・結合テスト）パス (exit 0)
- `cargo clippy --all-targets -- -D warnings`: コンパイラ・Clippy 警告 0 件 (exit 0)
- `cargo fmt --check`: コードフォーマット完全準拠 (exit 0)

## Deviations from Assessment

特になし。アセスメントで提示したバッチ化戦略およびアンカー有無に基づくプレビュー制御案に従い、最小限のスコープで憲章に準拠して実装・検証を完了しました。

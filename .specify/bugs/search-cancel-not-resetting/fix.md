# Bug Fix: CANCELボタンによる検索中断後のSEARCHボタン復帰と即時キャンセル対応

- **Slug**: search-cancel-not-resetting
- **Fixed**: 2026-09-25T17:44:30+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

`src/hooks/useSearch.ts` で `cancelSearch` 実行時に即座に `progress.state` を `"Cancelled"` に遷移させ、バックエンドから遅延して届く `"Scanning"` イベントを破棄するように改善しました。また、`src-tauri/src/search/parser.rs` および `engine.rs` において、ファイル内走査（シート単位・行単位）でもキャンセルフラグを定期チェックして並列ワーカースレッドが即座に脱出するように改修しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/hooks/useSearch.ts` | modified | `isCancellingRef` の導入、`cancelSearch` 実行時の即時ステータス更新・未フラッシュバッファ反映、中断要求後の遅延 `Scanning` 進捗および新規マッチイベントの無視 |
| `src-tauri/src/search/parser.rs` | modified | `parse_and_search_file` に `cancel_flag: Option<&AtomicBool>` を渡し、シートループおよびセル/数式行ループ（128行ごと）で早期脱出するチェックを追加 |
| `src-tauri/src/search/engine.rs` | modified | `par_iter().for_each` 内で `parse_and_search_file` に `cancel_flag` を渡し、中断要求後の `Scanning` 進捗通知送信を抑止。中断検証用の単体テスト `test_search_engine_cancellation` を追加 |

## Diff Highlights (optional)

```typescript
// src/hooks/useSearch.ts
  const cancelSearch = useCallback(async () => {
    isCancellingRef.current = true;
    setProgress(prev => prev ? { ...prev, state: "Cancelled", current_file: "スキャンが中断されました" } : ...);
    await invoke("cancel_search");
  }, []);

  const uProg = await listen<ScanProgress>("scan-progress", (event) => {
    if (isCancellingRef.current && event.payload.state === "Scanning") return;
    setProgress(event.payload);
  });
```

```rust
// src-tauri/src/search/parser.rs
pub fn parse_and_search_file<P: AsRef<Path>>(
    path: P,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    if (row_idx & 0x7F) == 0 && cancel_flag.map_or(false, |f| f.load(Ordering::Relaxed)) {
        return Ok(matches);
    }
```

## Tests Added or Updated

- `src-tauri/src/search/engine.rs::tests::test_search_engine_cancellation` — 検索コールバック中に中断フラグをセットした際、処理が早期終了して `ScanState::Cancelled` が返ることを検証。

## Local Verification

- Commands run:
  - `cargo test` → 成功 (`test_search_engine_cancellation` を含む全3テスト合格: 3 passed; 0 failed)
  - `npm run build` → 成功 (TypeScript型チェック & Viteバンドル正常終了: 0 errors, built in 9.92s)

## Deviations from Assessment

None.

## Follow-ups

- アプリ起動確認（`npm run tauri dev`）時に、大容量フォルダでの検索中にCANCELボタンを押して即座にSEARCHボタンに戻る挙動を確認する。

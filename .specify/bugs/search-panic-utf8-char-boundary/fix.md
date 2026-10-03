# Bug Fix: 検索実行時のスレッドパニックによる進捗停止 (UTF-8文字境界違反 & calamine XLSパニック)

- **Slug**: search-panic-utf8-char-boundary
- **Fixed**: 2026-09-25T16:26:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

日本語マルチバイト文字列における `make_snippet` の UTF-8 文字境界違反によるパニックを解消するため、文字数単位で安全に探索・スライスするロジックに改修しました。また、`calamine` の一部の旧 XLS パース処理等で発生するライブラリ内部パニックから安全に回復できるよう、各ファイルパースを `std::panic::catch_unwind` で保護し、進捗通知と走査を停止させないよう改修しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/search/parser.rs` | modified | `make_snippet` で `char_indices` を用いて安全に文字境界を特定し、スライス時のパニックを防止 |
| `src-tauri/src/search/engine.rs` | modified | `parse_and_search_file` を `catch_unwind` でラップし、破損・ライブラリパニック発生時もスキップして処理を継続 |
| `src-tauri/src/search/engine.rs` | added test | 日本語文字列に対する `test_snippet_utf8_boundary_safety` 単体テストを追加 |

## Diff Highlights

### 1. `parser.rs`: 安全な文字境界スライスの実装
```rust
// 前方は末尾から最大 20 文字を安全にスライス
let before_chars_count = 20;
let before_byte_len = before_str
    .char_indices()
    .rev()
    .take(before_chars_count)
    .last()
    .map(|(idx, _)| idx)
    .unwrap_or(0);

let before = &before_str[before_byte_len..];
let prefix = if before_byte_len > 0 { "..." } else { "" };

// 後方は先頭から最大 20 文字を安全にスライス
let after_chars_count = 20;
let (after, suffix) = match after_str.char_indices().nth(after_chars_count) {
    Some((idx, _)) => (&after_str[..idx], "..."),
    None => (after_str, ""),
};
```

### 2. `engine.rs`: `catch_unwind` による防御的保護
```rust
let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    parse_and_search_file(file_path, &query, regex_obj.as_ref())
}));

match parse_result {
    Ok(Ok(matches)) => { /* 結果登録 */ }
    Ok(Err(err_msg)) => { eprintln!("Skipping unreadable file: {}", err_msg); }
    Err(_) => { eprintln!("Recovered from panic while parsing file. Skipping safely."); }
}
```

## Tests Added or Updated

- `src-tauri/src/search/engine.rs::test_snippet_utf8_boundary_safety` — 日本語文字列の任意位置（先頭・中間・末尾）に対するスニペット生成が文字境界違反を起こさず正常に `<mark>` ハイライトタグを生成することを担保。

## Local Verification

- Commands run: `cargo test --manifest-path src-tauri\Cargo.toml`
  - 結果: `test result: ok. 2 passed; 0 failed; 0 ignored; finished in 0.61s`
- 全ユニットテストがエラー・パニックなく通過することを確認。

## Deviations from Assessment

None. (アセスメントの Preferred Remediation に忠実に実装)

## Follow-ups

- 今後必要に応じて、より詳細なエラーログを UI のトーストやステータスバーに反映する仕組みを拡張可能。

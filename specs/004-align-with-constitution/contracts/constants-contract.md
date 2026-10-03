# Interface Contract: Constants Management (定数管理規約)

**Feature Branch**: `004-align-with-constitution`  
**Date**: 2026-09-26  
**Status**: Completed  

## 1. 概要
本規約は、プロジェクト憲章原則II（定数の外部抽出とハードコードの禁止）を具体的にコードへ適用するための命名規則、配置場所、および参照コメントの記法を定義する。

## 2. 定数定義の配置
- **Rust バックエンド**: `src-tauri/src/constants.rs`
- **TypeScript フロントエンド**: `src/constants/index.ts` (必要に応じて `src/constants/ui.ts` 等に分割し `index.ts` から再エクスポート)

## 3. 定数命名規則
- **Rust**: `SCREAMING_SNAKE_CASE` (例: `DEFAULT_EXTENSIONS`, `PROGRESS_NOTIFY_INTERVAL_MS`)
- **TypeScript**: `SCREAMING_SNAKE_CASE` または 名前空間オブジェクト下の `SCREAMING_SNAKE_CASE` (例: `UI_CONSTANTS.TOAST_DURATION_MS`)

## 4. 定数参照コメントの標準構文 (MUST)
定数を利用するすべての箇所において、直前または同一行に以下の形式で参照コメントを記載しなければならない。

### Rust での参照コメント例
```rust
// 定数参照: crate::constants::DEFAULT_EXTENSIONS を使用
pub fn default_extensions() -> Vec<String> {
    crate::constants::DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect()
}

// 定数参照: crate::constants::PROGRESS_NOTIFY_INTERVAL_MS を使用
if elapsed.saturating_sub(last) >= crate::constants::PROGRESS_NOTIFY_INTERVAL_MS {
    ...
}
```

### TypeScript での参照コメント例
```typescript
// 定数参照: UI_CONSTANTS.TOAST_DURATION_MS を使用
setTimeout(() => {
  setIsVisible(false);
}, UI_CONSTANTS.TOAST_DURATION_MS);
```

## 5. 例外事項
- 数値の `0` および空文字列 `""` のみ、定数化せず直接リテラルとして記述することを許容する（憲章原則IIで明示的に免除）。
- ただし、ビジネス上の意味が強い `0`（例: 初期ステータスコードなど）やデフォルト文字列については、必要に応じて定数化することを推奨する。

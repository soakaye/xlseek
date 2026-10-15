# Bug Fix: プレビュー画面の場所表示における Windows Verbatim パスプレフィックス除去

- **Slug**: preview-verbatim-path-prefix
- **Fixed**: 2026-10-15T20:08:45+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Windows 環境において検索結果プレビューヘッダー（`PreviewHeader.tsx`）の「場所:」欄およびクリップボードコピー処理に付加されていた Windows Verbatim 拡張プレフィックス（`\\?\` および `//?/`）を除去する表示用パス正規化ユーティリティ `normalizeDisplayPath` を実装し、ローカルパス（`C:\...`）および UNC パス（`\\server\share\...`）が自然な標準形式でプレビュー表示・コピーされるよう修正しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/constants/index.ts` | modified | Verbatim パスプレフィックス定数 `PATH_DISPLAY_CONSTANTS` を追加定義（Principle II 遵守） |
| `src/utils/pathDisplay.ts` | added | ユーザー向け表示パス正規化関数 `normalizeDisplayPath` を新規作成（Principle I, II, III 遵守） |
| `src/components/preview/PreviewHeader.tsx` | modified | `normalizeDisplayPath` を適用し、プレビューの場所表示、ツールチップ、クリップボードコピー、フォルダ起動（`open_in_folder`）に正規化パスを使用 |
| `tests/path-display.test.ts` | added | `normalizeDisplayPath` の各種パターンテスト（12件）を新規作成（Principle V 遵守） |
| `tests/preview-header.test.tsx` | added | `PreviewHeader` のプレビュー場所表示コンポーネントテスト（4件）を新規作成（Principle V 遵守） |

## Diff Highlights (optional)

### `src/utils/pathDisplay.ts` (新規追加)
```typescript
export function normalizeDisplayPath(path: string): string {
  if (!path) return "";
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH) || path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH)) {
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH.length)}`;
  }
  const upper = path.toUpperCase();
  if (upper.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH) || upper.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH)) {
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH.length)}`;
  }
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_BACKSLASH)) {
    return path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_BACKSLASH.length);
  }
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_SLASH)) {
    return path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_SLASH.length);
  }
  return path;
}
```

### `src/components/preview/PreviewHeader.tsx`
```tsx
  const displayPath = normalizeDisplayPath(selectedMatch.full_path);
  ...
  await navigator.clipboard.writeText(displayPath);
  ...
  <span className="text-zinc-400 truncate select-text flex-1" title={displayPath}>
    {displayPath}
  </span>
```

## Tests Added or Updated

- `tests/path-display.test.ts`:
  - `\\?\` バックスラッシュ Verbatim ドライブパス（大文字・小文字）のプレフィックス除去
  - `//?/` スラッシュ Verbatim ドライブパス（大文字・小文字）のプレフィックス除去
  - `\\?\UNC\` および `//?/UNC/`（大文字・小文字）の標準 UNC パス（`\\`）変換
  - 通常の Windows パス、UNC パス、POSIX パス、相対パス、空文字列の透過性検証
- `tests/preview-header.test.tsx`:
  - `\\?\` を含む `selectedMatch` 選択時の場所行表示検証
  - `//?/` を含む `selectedMatch` 選択時の場所行表示検証
  - Verbatim UNC を含む `selectedMatch` 選択時の場所行表示検証
  - POSIX パス選択時の場所行表示検証

## Local Verification

- Commands run:
  - `npx vitest run tests/path-display.test.ts` → 12 passed
  - `npx vitest run tests/preview-header.test.tsx` → 4 passed
  - `npm run build` → passed (tsc 型エラー 0 件, Vite ビルド成功)
  - `npm test` (`vitest run`) → 全 21 テストファイル（142 テスト）すべて合格
  - `cargo test --workspace` → 全テスト合格（58 core, 7 cli, 10 integration）
  - `cargo clippy --workspace --all-targets -- -D warnings` → Clippy 警告 0 件
  - `cargo fmt --check` → フォーマット違反なし

## Deviations from Assessment

なし。アセスメントの推奨方針（Preferred）に完全準拠して実装しました。

## Follow-ups

- なし。

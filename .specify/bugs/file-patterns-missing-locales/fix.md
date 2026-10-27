# Bug Fix: ファイル検索パターン関連の翻訳キーを GUI 辞書カタログに追加

- **Slug**: file-patterns-missing-locales
- **Fixed**: 2026-10-28T01:14:30+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

GUIフロントエンド（`tauri-plugin-i18n` およびフォールバック機構）が参照する `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` に不足していたファイル検索パターン関連の翻訳キー7件および `cli.HELP` 内の `-g, --file-patterns` オプション説明を追加し、`tests/catalog.test.ts` に回帰防止テストを追加しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/locales/ja.yml` | modified | `ui.FILE_PATTERNS_*` 7キーおよび `cli.HELP` 内の `-g` オプション説明を追加 |
| `src-tauri/locales/en.yml` | modified | `ui.FILE_PATTERNS_*` 7キーおよび `cli.HELP` 内の `-g` オプション説明を追加 |
| `tests/catalog.test.ts` | modified | `src-tauri/locales/{en,ja}.yml` におけるファイル検索パターンキーの存在検証テストを追加 |

## Diff Highlights (optional)

```yaml
# src-tauri/locales/{ja,en}.yml
ui.FILE_PATTERNS_LABEL: "検索パターン:"
ui.FILE_PATTERNS_PLACEHOLDER: "*budget*, quarterly_* (カンマ区切り、拡張子は自動補完)"
ui.FILE_PATTERNS_TOOLTIP: "検索対象のファイルパターン。除外パターンの前に評価されます。"
ui.FILE_PATTERNS_CLEAR: "検索パターンをクリア"
ui.FILE_PATTERN_HISTORY_BUTTON: "過去の検索パターン"
ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL: "デフォルトの検索パターン"
ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP: "新しい検索セッションで初期設定されるファイル検索パターン（空の場合は有効な拡張子すべて）"
```

## Tests Added or Updated

- `tests/catalog.test.ts::"shared localization catalogs" -> "defines all file search pattern localization keys in English and Japanese"`:
  `src-tauri/locales/en.yml` および `src-tauri/locales/ja.yml` にファイル検索パターン関連の全7キー（`ui.FILE_PATTERNS_LABEL` 等）が空文字列でなく確実に定義されていることをテストし、将来のキー欠落リグレッションを防止。

## Local Verification

- `npm test`: 28テストファイル、197テスト全件パス（`tests/catalog.test.ts` の新テスト含む）。
- `cargo test --workspace`: 151テスト全件パス。
- `cargo clippy --workspace --all-targets -- -D warnings`: 警告0件。
- `cargo fmt --check`: フォーマット不整合なし。
- `npm run build`: TypeScript型検査およびVite本番バンドル成功（終了コード0）。

## Deviations from Assessment

None（アセスメントの推奨手順どおりに適用）。

## Follow-ups

- 次のステップとして `/speckit-bug-test slug=file-patterns-missing-locales` を実行して検証レポートを作成します。

# Bug Assessment: ファイル検索パターン領域および設定ダイアログのラベル類の翻訳テキストが「Some text could not be translated.」と表示される

- **Slug**: file-patterns-missing-locales
- **Created**: 2026-10-28T01:12:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> 追加部分のメッセージがsome text could not be translatedである

## Symptom

機能 037（ファイル検索パターン指定）で追加されたメイン画面の検索パターン入力領域のラベル（「検索パターン:」）、ツールチップ、プレースホルダーテキスト、履歴ボタンのツールチップ、クリアボタン、および設定ダイアログ（`SettingsDialog`）内の「デフォルトの検索パターン」のラベルとヘルプテキストにおいて、本来の言語リソース（日本語または英語）の文字列が表示されず、フォールバック文言（`common.translationUnavailable`: "Some text could not be translated."）が表示されてしまう。

期待される動作としては、選択された言語（日本語または英語）に応じた適切なローカライズテキストが表示されること。

## Reproduction

1. アプリケーションを起動（`npm run dev` または GUI 実行）する。
2. メイン画面の検索バー直下（検索ディレクトリと無視パターンの間）にあるファイル検索パターン入力領域を確認する。
   - ラベルテキストやツールチップ、プレースホルダー、履歴ボタンのツールチップ等に「Some text could not be translated.」が表示される。
3. 画面右上の歯車アイコンをクリックして「設定」ダイアログを開き、「保存される初期値（Saved Defaults）」タブを選択する。
4. 「デフォルトの検索パターン」入力欄を確認する。
   - タイトルラベルおよび説明ヘルプテキストが「Some text could not be translated.」と表示される。

## Suspected Code Paths

- `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` — フロントエンド（`tauri-plugin-i18n` およびフォールバック用インポート `src/i18n.ts`）が参照する翻訳カタログファイル。機能追加時（`037-file-search-patterns`）に `crates/core/locales/` にのみキーが追加され、`src-tauri/locales/` に必要なキー（`ui.FILE_PATTERNS_LABEL`, `ui.FILE_PATTERNS_PLACEHOLDER`, `ui.FILE_PATTERNS_TOOLTIP`, `ui.FILE_PATTERNS_CLEAR`, `ui.FILE_PATTERN_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP`, および `cli.HELP` 内の `-g, --file-patterns` オプション説明）が未定義のままになっている。
- `src/components/search/SearchBar.tsx:711-780` — `t("ui.FILE_PATTERNS_LABEL")`, `t("ui.FILE_PATTERNS_PLACEHOLDER")`, `t("ui.FILE_PATTERNS_TOOLTIP")`, `t("ui.FILE_PATTERN_HISTORY_BUTTON")` を呼び出している。
- `src/components/search/FilePatternHistoryDropdown.tsx:60-100` — `t("ui.FILE_PATTERNS_LABEL")`, `t("ui.FILE_PATTERNS_CLEAR")` を呼び出している。
- `src/components/settings/SettingsDialog.tsx:604-635` — `t("ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL")`, `t("ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP")` を呼び出している。
- `src/i18n.ts:82-88` — プラグインカタログおよびインポートされた `src-tauri/locales/en.yml` にキーが存在しない場合、フォールバック安全機構により `common.translationUnavailable`（"Some text could not be translated."）を返却する。
- `tests/catalog.test.ts` — `src-tauri/locales/en.yml` と `ja.yml` の間の整合性をテストしているが、ファイル検索パターン関連のキー定義の存在を検証するテストが未追加。

## Root Cause Hypothesis

**確信度: High（高）**

コミット `1721b80`（`feat: implement file search patterns across core, cli, mcp, and gui`）において、ファイル検索パターン関連の翻訳キー（`ui.FILE_PATTERNS_LABEL`, `ui.FILE_PATTERNS_PLACEHOLDER`, `ui.FILE_PATTERNS_TOOLTIP`, `ui.FILE_PATTERNS_CLEAR`, `ui.FILE_PATTERN_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP`）がバックエンドコア用のカタログ `crates/core/locales/{ja,en}.yml` に追加されたものの、GUIフロントエンドおよびTauriプラグイン（`tauri-plugin-i18n`）が直接バンドル・読み込みを行う `src-tauri/locales/{ja,en}.yml` への追加が行われませんでした。
その結果、フロントエンド側の `t(...)` 呼び出し時に `src-tauri/locales/` にキーが見つからず、フォールバック機構によって `Some text could not be translated.` が出力されています。

## Proposed Remediation

**Preferred（推奨）**:

1. **`src-tauri/locales/` の辞書カタログ同期**:
   - `src-tauri/locales/ja.yml` に以下のキーを追加し、`cli.HELP` にも `-g, --file-patterns` を反映する:
     ```yaml
     ui.FILE_PATTERNS_LABEL: "検索パターン:"
     ui.FILE_PATTERNS_PLACEHOLDER: "*budget*, quarterly_*, report.xlsx (カンマ区切り)"
     ui.FILE_PATTERNS_TOOLTIP: "ファイル名検索パターン (カンマ区切り、ワイルドカード * ? 対応、省略時は対象拡張子すべて)"
     ui.FILE_PATTERNS_CLEAR: "検索パターンをクリア"
     ui.FILE_PATTERN_HISTORY_BUTTON: "検索パターンの履歴"
     ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL: "デフォルトの検索パターン"
     ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP: "新しい検索セッションで初期設定されるファイル検索パターン（空の場合は有効な拡張子すべて）"
     ```
   - `src-tauri/locales/en.yml` に以下のキーを追加し、`cli.HELP` にも `-g, --file-patterns` を反映する:
     ```yaml
     ui.FILE_PATTERNS_LABEL: "Search patterns:"
     ui.FILE_PATTERNS_PLACEHOLDER: "*budget*, quarterly_*, report.xlsx (comma-separated)"
     ui.FILE_PATTERNS_TOOLTIP: "Target file glob patterns (comma-separated, wildcards * and ? supported, empty matches all active extensions)"
     ui.FILE_PATTERNS_CLEAR: "Clear search patterns"
     ui.FILE_PATTERN_HISTORY_BUTTON: "Search pattern history"
     ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL: "Default search patterns"
     ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP: "Initial inclusion file search patterns applied to new search sessions (empty matches all enabled extensions)"
     ```
2. **カタログ回帰テストの追加**:
   - `tests/catalog.test.ts` にファイル検索パターン関連キー（`ui.FILE_PATTERNS_LABEL` など）が `src-tauri/locales/{en,ja}.yml` の双方に必ず存在することを検証するテストケースを追加し、将来のリグレッションを防止する。

**Alternatives（代替案）**:
- なし（辞書ファイルへの追加と回帰テストの追加が最も安全かつ確実な修正）。

**Files likely to change**:
- `src-tauri/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `tests/catalog.test.ts`

**Tests to add or update**:
- `tests/catalog.test.ts`:
  - `src-tauri/locales/en.yml` および `src-tauri/locales/ja.yml` にファイル検索パターン関連の全7キーが定義されており、空でないことを検証するテストケースを追加。

## Risks & Considerations

- 既存のキー名・構成との重複や競合はなく、他のUIコンポーネントに対する影響もないため、リスクは極めて低い。
- 修正後に `npm run build`、`npm test`、`cargo test --workspace` を実行して検証を行う。

## Open Questions

なし

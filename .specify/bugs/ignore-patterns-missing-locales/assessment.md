# Bug Assessment: 無視パターン入力領域および設定ダイアログのラベル類の翻訳テキストが表示されない

- **Slug**: ignore-patterns-missing-locales
- **Created**: 2026-10-16T01:00:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> 追加された無視パターン領域の文字列に翻訳テキストが表示されていない。

## Symptom

メインUIの検索バー直下に追加された無視パターン領域のラベル（「除外パターン:」）、ツールチップ、プレースホルダーテキスト、履歴ボタンのツールチップ、クリアボタン、および設定ダイアログ（`SettingsDialog`）内の「デフォルトの除外パターン」のラベルとヘルプテキストにおいて、本来の言語リソース（日本語または英語）の文字列が表示されず、フォールバック文言（`common.translationUnavailable`: "Some text could not be translated."）や未翻訳キーがそのまま表示されてしまう。

期待される動作としては、選択された言語（日本語または英語）に応じた適切なローカライズテキストが表示されること。

## Reproduction

1. アプリケーションを起動（`npm run tauri dev` など）する。
2. メイン画面の検索バー直下にある無視パターン入力領域を確認する。
   - ラベルテキストやツールチップ、プレースホルダー、履歴ボタンのツールチップ等に「Some text could not be translated.」またはキー名が表示される。
3. 画面右上の歯車アイコンをクリックして「設定」ダイアログを開き、「保存される初期値（Saved Defaults）」タブを選択する。
4. 「デフォルトの除外パターン」入力欄を確認する。
   - タイトルラベルおよび説明ヘルプテキストが「Some text could not be translated.」と表示される。

## Suspected Code Paths

- `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` — フロントエンド（`tauri-plugin-i18n` およびフォールバック用インポート `src/i18n.ts`）が参照する翻訳カタログファイル。機能追加時（`025-ignore-file-patterns`）に `crates/core/locales/` にのみキーが追加され、`src-tauri/locales/` に必要なキー（`ui.IGNORE_PATTERNS_LABEL`, `ui.IGNORE_PATTERNS_PLACEHOLDER`, `ui.IGNORE_PATTERNS_TOOLTIP`, `ui.IGNORE_PATTERNS_CLEAR`, `ui.IGNORE_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP`）が未定義のままになっている。
- `src/components/search/SearchBar.tsx:645-715` — `t("ui.IGNORE_PATTERNS_LABEL")`, `t("ui.IGNORE_PATTERNS_PLACEHOLDER")`, `t("ui.IGNORE_PATTERNS_TOOLTIP")`, `t("ui.IGNORE_HISTORY_BUTTON")` を呼び出している。
- `src/components/search/IgnoreHistoryDropdown.tsx:50-80` — `t("ui.IGNORE_PATTERNS_LABEL")`, `t("ui.IGNORE_PATTERNS_CLEAR")` を呼び出している。
- `src/components/settings/SettingsDialog.tsx:302-330` — `t("ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL")`, `t("ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP")` を呼び出している。
- `src/i18n.ts:82-88` — プラグインカタログおよびインポートされた `src-tauri/locales/en.yml` にキーが存在しない場合、フォールバック安全機構により `common.translationUnavailable`（"Some text could not be translated."）を返却する。
- `tests/catalog.test.ts` — `src-tauri/locales/en.yml` と `ja.yml` の間の整合性をテストしているが、無視パターン関連のキー定義の存在を検証するテストが未追加。

## Root Cause Hypothesis

**確信度: High（高）**

コミット `ebfcf3d51a67b4e7273476019da9705ce6969480`（`[Spec Kit] Implement ignore file patterns with gitignore matching and history`）において、無視パターン関連の翻訳キー（`ui.IGNORE_PATTERNS_LABEL`, `ui.IGNORE_PATTERNS_PLACEHOLDER`, `ui.IGNORE_PATTERNS_TOOLTIP`, `ui.IGNORE_PATTERNS_CLEAR`, `ui.IGNORE_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP`）がバックエンドコア用のカタログ `crates/core/locales/{ja,en}.yml` に追加されたものの、GUIフロントエンドおよびTauriプラグイン（`tauri-plugin-i18n`）が直接バンドル・読み込みを行う `src-tauri/locales/{ja,en}.yml` への追加が行われませんでした。
その結果、フロントエンド側の `t(...)` 呼び出し時に `src-tauri/locales/` にキーが見つからず、フォールバック機構によって `Some text could not be translated.`（または未翻訳キー）が出力されています。

## Proposed Remediation

**Preferred（推奨）**:

1. **`src-tauri/locales/` の辞書カタログ同期**:
   - `src-tauri/locales/ja.yml` に以下のキーを追加する:
     ```yaml
     ui.IGNORE_PATTERNS_LABEL: "除外パターン:"
     ui.IGNORE_PATTERNS_PLACEHOLDER: "~$*, *.tmp, backup/ (カンマ区切り)"
     ui.IGNORE_PATTERNS_TOOLTIP: "除外するファイルまたはフォルダのパターン (.gitignore構文)"
     ui.IGNORE_PATTERNS_CLEAR: "除外パターンをクリア"
     ui.IGNORE_HISTORY_BUTTON: "除外パターンの履歴"
     ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL: "デフォルトの除外パターン"
     ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP: "新しい検索セッションで初期設定される除外パターン（カンマ区切り）"
     ```
   - `src-tauri/locales/en.yml` に以下のキーを追加する:
     ```yaml
     ui.IGNORE_PATTERNS_LABEL: "Ignore patterns:"
     ui.IGNORE_PATTERNS_PLACEHOLDER: "~$*, *.tmp, backup/ (comma-separated)"
     ui.IGNORE_PATTERNS_TOOLTIP: "Patterns of files or folders to ignore (.gitignore syntax)"
     ui.IGNORE_PATTERNS_CLEAR: "Clear ignore patterns"
     ui.IGNORE_HISTORY_BUTTON: "Ignore pattern history"
     ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL: "Default ignore patterns"
     ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP: "Initial ignore patterns applied to new search sessions (comma-separated)"
     ```
2. **カタログ回帰テストの追加**:
   - `tests/catalog.test.ts` に無視パターン関連キー（`ui.IGNORE_PATTERNS_LABEL` など）が `src-tauri/locales/{en,ja}.yml` の双方に必ず存在することを検証するテストケースを追加し、将来のリグレッションを防止する。

**Alternatives（代替案）**:
- ビルド時に `crates/core/locales/` から `src-tauri/locales/` へカタログを自動コピーするビルドスクリプトを作成する。
  - *トレードオフ*: バグ修正のスコープを超えてビルドパイプラインに変更が及ぶため、今回の直接修正・テスト追加が最も低リスクかつ迅速。

**Files likely to change**:
- `src-tauri/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `tests/catalog.test.ts`

**Tests to add or update**:
- `tests/catalog.test.ts`:
  - `src-tauri/locales/en.yml` および `src-tauri/locales/ja.yml` に無視パターン関連の全7キーが定義されており、空でないことを検証するテストを追加。

## Risks & Considerations

- 既存のキー名・構成との重複や競合はなく、他のUIコンポーネントに対する影響もないため、リスクは極めて低い。
- 修正後に `npm run build`、`npm test`、`cargo test --workspace` を実行して検証を行う。

## Open Questions

なし

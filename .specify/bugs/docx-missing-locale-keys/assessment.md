# Bug Assessment: docx オプションおよび検索結果バッジで翻訳不能メッセージが表示される

- **Slug**: docx-missing-locale-keys
- **Created**: 2026-10-15T19:39:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> docx オプション関連/検索結果が Some text could not be translated と表示されている。

## Symptom

設定画面（`SettingsDialog`）内の「Word文書も検索対象に含める」オプションのラベル・説明文、および検索結果一覧（`ResultTable`）の一致種別バッジ（WordText）において、本来の言語リソース文字列（「Word文書も検索対象に含める (.docx, .docm, .doc)」「有効にすると、Word文書内の本文・表・コメント・図形テキストも検索します」「Word」など）が表示されず、フォールバック文言である「Some text could not be translated.」が表示されてしまう。

期待される動作としては、選択された言語（日本語または英語）に応じた適切なラベルが表示されること。

## Reproduction

1. アプリケーションを起動（`npm run tauri dev` など）する。
2. 画面右上の歯車アイコンをクリックして「設定」ダイアログを開き、「保存設定」タブを選択する。
3. デフォルト検索オプション欄の「Word文書も検索対象に含める」オプションを確認する。
   - チェックボックス横のタイトルおよび説明文が「Some text could not be translated.」になっている。
4. Word文書検索を有効化し、Wordファイル（`.docx`, `.docm`, `.doc`）が含まれるフォルダをキーワード検索する。
5. 検索結果一覧テーブル（`ResultTable`）にヒットした行の一致種別バッジを確認する。
   - バッジのテキストが「Some text could not be translated.」と表示される。

## Suspected Code Paths

- `src-tauri/locales/en.yml` および `src-tauri/locales/ja.yml` — フロントエンド（`tauri-plugin-i18n` およびフォールバック用インポート）が参照するカタログファイル。Word検索機能追加時に `crates/core/locales/` にのみキーが追加され、`src-tauri/locales/` に必要なキー（`ui.OPTION_ENABLE_WORD_SEARCH`, `ui.OPTION_ENABLE_WORD_SEARCH_DESC`, `ui.MATCH_TYPE_WORD_TEXT`, `ui.RESULT_MATCH_TYPE_WORD_TEXT`, `export.match.wordText`）が未定義のままになっている。
- `src/components/settings/SettingsDialog.tsx:295-296` — `t("ui.OPTION_ENABLE_WORD_SEARCH")` および `t("ui.OPTION_ENABLE_WORD_SEARCH_DESC")` を呼び出している。
- `src/components/results/ResultTable.tsx:198` — `t("ui.RESULT_MATCH_TYPE_WORD_TEXT")` を呼び出している。
- `src/i18n.ts:82-88` — `resolveTranslation` 関数において、プラグインカタログおよびインポートされた `src-tauri/locales/en.yml` にキーが存在しない場合、`common.translationUnavailable`（"Some text could not be translated."）にフォールバックする。
- `tests/catalog.test.ts` — `src-tauri/locales/en.yml` と `ja.yml` 間のキー整合性のみを検証しており、`crates/core/locales/` との同期やUI上で使用されているキーの存在検証が漏れている。

## Root Cause Hypothesis

**確信度: High（高）**

コミット `119f5fd235a6c32375e518f3579d81fe218cc6ee`（`add docx code`）において、Word検索機能に関連する翻訳キー（`ui.OPTION_ENABLE_WORD_SEARCH`, `ui.OPTION_ENABLE_WORD_SEARCH_DESC`, `ui.MATCH_TYPE_WORD_TEXT`, `ui.RESULT_MATCH_TYPE_WORD_TEXT`, `export.match.wordText`）がバックエンドコア用のカタログ `crates/core/locales/{en,ja}.yml` に追加されたものの、GUIフロントエンドおよびTauriプラグインが直接読み込む `src-tauri/locales/{en,ja}.yml` への追加が行われませんでした。
その結果、フロントエンド側の `t(...)` 呼び出し時に `src-tauri/locales/` にキーが見つからず、`i18n.ts` のフォールバック安全機構によって `common.translationUnavailable`（"Some text could not be translated."）が出力されています。

## Proposed Remediation

**Preferred（推奨）**:

1. **`src-tauri/locales/` の辞書更新**:
   - `src-tauri/locales/en.yml` に以下のキーを追加する:
     ```yaml
     export.match.wordText: Word
     ui.OPTION_ENABLE_WORD_SEARCH: "Include Word documents (.docx, .docm, .doc)"
     ui.OPTION_ENABLE_WORD_SEARCH_DESC: "Search text, tables, comments, and shape text inside Word documents"
     ui.MATCH_TYPE_WORD_TEXT: Word Text
     ui.RESULT_MATCH_TYPE_WORD_TEXT: Word
     ```
   - `src-tauri/locales/ja.yml` に以下のキーを追加する:
     ```yaml
     export.match.wordText: Word
     ui.OPTION_ENABLE_WORD_SEARCH: "Word文書も検索対象に含める (.docx, .docm, .doc)"
     ui.OPTION_ENABLE_WORD_SEARCH_DESC: "有効にすると、Word文書内の本文・表・コメント・図形テキストも検索します"
     ui.MATCH_TYPE_WORD_TEXT: Word本文
     ui.RESULT_MATCH_TYPE_WORD_TEXT: Word
     ```
2. **カタログ同期テストの強化**:
   - `tests/catalog.test.ts` を拡張し、`crates/core/locales/` と `src-tauri/locales/` 間で共通するキーの乖離がないか、また今回追加されたWord関連キーが両ロケールに存在することを自動検証するテストケースを追加する。

**Alternatives（代替案）**:
- *共通YAMLディレクトリの一元化*: ビルドスクリプトまたはシンボリックリンク等で `crates/core/locales/` と `src-tauri/locales/` を共有する。
  - *トレードオフ*: 現行アーキテクチャでは `tauri-plugin-i18n` のディレクトリ制約や Cargo クレートの独立性要件があり、設定変更のスコープが大きくなる。

**Files likely to change**:
- `src-tauri/locales/en.yml`
- `src-tauri/locales/ja.yml`
- `tests/catalog.test.ts`

**Tests to add or update**:
- `tests/catalog.test.ts`:
  - `src-tauri/locales/{en,ja}.yml` に `ui.OPTION_ENABLE_WORD_SEARCH`、`ui.OPTION_ENABLE_WORD_SEARCH_DESC`、`ui.RESULT_MATCH_TYPE_WORD_TEXT` が定義されていることの検証
  - `crates/core/locales/` と `src-tauri/locales/` の共通キーの整合性検証

## Risks & Considerations

- 既存のキー名・構成との衝突や既存コンポーネントへの破壊的影響はなく、リスクは極めて低い。
- `npm run build` および `cargo test` による型・リソースチェックを通過することを確認する。

## Open Questions

なし

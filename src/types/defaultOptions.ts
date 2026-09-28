/**
 * @fileoverview デフォルト検索オプションの型定義 (src/types/defaultOptions.ts)
 *
 * ## 処理内容
 * ユーザーが設定ダイアログからカスタマイズ可能で、ローカルストレージに永続化される
 * デフォルト検索オプションのデータ型を定義する。
 * 憲章原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版策定。
 */

/**
 * デフォルト検索オプション型
 */
export interface DefaultSearchOptions {
  match_case: boolean;
  use_regex: boolean;
  include_formula: boolean;
  include_shape: boolean;
  include_comment: boolean;
  include_hidden: boolean;
  extensions: string[];
}

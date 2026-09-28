/**
 * @fileoverview デフォルト検索オプション管理コアモジュール (src/default-options-core.ts)
 *
 * ## 処理内容
 * ユーザーがカスタマイズしたデフォルト検索オプションの検証、ローカルストレージへの永続化、
 * 読み込み、および公式規定値へのリセット処理を提供する。
 * 憲章原則II（定数の一元管理）、原則III（ヘッダコメント）、原則V（堅牢なエラーハンドリング）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版策定。
 */

import { DEFAULT_SEARCH_OPTIONS, DEFAULT_OPTIONS_STORAGE_KEY, FILE_EXTENSIONS } from "./constants";
import { DefaultSearchOptions } from "./types/defaultOptions";

/**
 * ## 処理内容
 * 引数の値が `DefaultSearchOptions` 型の要件を満たすかを検証する型ガード。
 *
 * ## 引数
 * @param value - 検証対象の未知の値 (`unknown`)
 *
 * ## 戻り値
 * @returns 妥当であれば `true`、それ以外は `false`
 *
 * ## エラー / 例外発生条件
 * 例外は送出せず、不正値には `false` を返す。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版作成。
 */
export function isDefaultSearchOptions(value: unknown): value is DefaultSearchOptions {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }
  const candidate = value as Record<string, unknown>;

  if (
    typeof candidate.match_case !== "boolean" ||
    typeof candidate.use_regex !== "boolean" ||
    typeof candidate.include_formula !== "boolean" ||
    typeof candidate.include_shape !== "boolean" ||
    typeof candidate.include_comment !== "boolean" ||
    typeof candidate.include_hidden !== "boolean"
  ) {
    return false;
  }

  if (!Array.isArray(candidate.extensions) || candidate.extensions.length === 0) {
    return false;
  }

  // 定数参照: FILE_EXTENSIONS.DEFAULT_LIST 内の拡張子のみを許可
  const allowed = new Set<string>(FILE_EXTENSIONS.DEFAULT_LIST);
  const allValid = candidate.extensions.every(
    (ext) => typeof ext === "string" && allowed.has(ext)
  );

  return allValid;
}

/**
 * ## 処理内容
 * 公式規定のデフォルト検索オプションの新しいディープコピーオブジェクトを取得する。
 *
 * ## 引数
 * なし
 *
 * ## 戻り値
 * @returns `DefaultSearchOptions`: 公式規定のデフォルトオプション
 *
 * ## エラー / 例外発生条件
 * なし。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版作成。
 */
export function resetDefaultSearchOptions(): DefaultSearchOptions {
  // 定数参照: DEFAULT_SEARCH_OPTIONS
  return {
    match_case: DEFAULT_SEARCH_OPTIONS.match_case,
    use_regex: DEFAULT_SEARCH_OPTIONS.use_regex,
    include_formula: DEFAULT_SEARCH_OPTIONS.include_formula,
    include_shape: DEFAULT_SEARCH_OPTIONS.include_shape,
    include_comment: DEFAULT_SEARCH_OPTIONS.include_comment,
    include_hidden: DEFAULT_SEARCH_OPTIONS.include_hidden,
    extensions: [...DEFAULT_SEARCH_OPTIONS.extensions],
  };
}

/**
 * ## 処理内容
 * ローカルストレージからデフォルト検索オプションを読み込む。
 * ストレージ未保存、パース失敗、または不正スキーマの場合は規定のデフォルト値を返す。
 *
 * ## 引数
 * なし
 *
 * ## 戻り値
 * @returns `DefaultSearchOptions`: 検証済みのデフォルト検索オプション
 *
 * ## エラー / 例外発生条件
 * ストレージ読み取りエラー時も例外を外部に送出せず、規定値へフォールバックする。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版作成。
 */
export function loadDefaultSearchOptions(): DefaultSearchOptions {
  try {
    // 定数参照: DEFAULT_OPTIONS_STORAGE_KEY
    const raw = typeof window !== "undefined" && window.localStorage
      ? window.localStorage.getItem(DEFAULT_OPTIONS_STORAGE_KEY)
      : null;

    if (!raw) {
      return resetDefaultSearchOptions();
    }

    const parsed: unknown = JSON.parse(raw);
    if (isDefaultSearchOptions(parsed)) {
      return parsed;
    }

    return resetDefaultSearchOptions();
  } catch {
    return resetDefaultSearchOptions();
  }
}

/**
 * ## 処理内容
 * デフォルト検索オプションをローカルストレージへ保存する。
 *
 * ## 引数
 * @param options - 保存対象の `DefaultSearchOptions`
 *
 * ## 戻り値
 * @returns 正常に保存できた場合は `true`、書き込み失敗時は `false`
 *
 * ## エラー / 例外発生条件
 * QuotaExceededError などの例外発生時はキャッチして `false` を返し、アプリのクラッシュを防止する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-28, AI Agent): 初版作成。
 */
export function saveDefaultSearchOptions(options: DefaultSearchOptions): boolean {
  if (!isDefaultSearchOptions(options)) {
    return false;
  }
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      // 定数参照: DEFAULT_OPTIONS_STORAGE_KEY
      window.localStorage.setItem(DEFAULT_OPTIONS_STORAGE_KEY, JSON.stringify(options));
      return true;
    }
    return false;
  } catch {
    return false;
  }
}

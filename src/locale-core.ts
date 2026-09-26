/**
 * ## 処理内容
 * 表示言語の保存値検証と端末ロケール判定を行う。
 * ## 引数・戻り値
 * ロケール文字列または保存値を受け取り、対応する型付き言語値を返す。
 * ## エラー
 * 不正値と取得失敗は default または英語へ安全にフォールバックする。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 日英ロケールの判定と設定保存を追加。
 */
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, LANGUAGE_STORAGE_KEY } from "./constants";

export type LanguagePreference = (typeof LANGUAGE_PREFERENCES)[keyof typeof LANGUAGE_PREFERENCES];
export type DisplayLanguage = (typeof DISPLAY_LANGUAGES)[keyof typeof DISPLAY_LANGUAGES];

/**
 * ## 処理内容
 * 保存値がサポートする言語設定のいずれかを判定する。
 * ## 引数・戻り値
 * 未検証値を受け取り、対応値なら真を返す。
 * ## エラー
 * 型判定のみで例外は発生しない。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定値の検証を追加。
 */
export const isLanguagePreference = (value: unknown): value is LanguagePreference =>
  Object.values(LANGUAGE_PREFERENCES).includes(value as LanguagePreference);

/**
 * ## 処理内容
 * 未検証値を言語設定へ正規化する。
 * ## 引数・戻り値
 * 任意値を受け取り、対応する設定または default を返す。
 * ## エラー
 * 例外は発生しない。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 不正設定の既定値化を追加。
 */
export const getLanguagePreference = (value: unknown): LanguagePreference =>
  isLanguagePreference(value) ? value : LANGUAGE_PREFERENCES.DEFAULT;

/**
 * ## 処理内容
 * OS の BCP-47 ロケールを対応表示言語へ解決する。
 * ## 引数・戻り値
 * ロケール文字列または nullish 値を受け取り、ja または en を返す。
 * ## エラー
 * 未対応・欠落ロケールは英語へフォールバックする。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 日本語タグ検出を追加。
 */
export const resolveLanguage = (locale: string | null | undefined): DisplayLanguage =>
  locale?.toLowerCase().split(/[-_]/, 1)[0] === DISPLAY_LANGUAGES.JA
    ? DISPLAY_LANGUAGES.JA
    : DISPLAY_LANGUAGES.EN;

/**
 * ## 処理内容
 * localStorage から保存済みの言語設定を安全に読み取る。
 * ## 引数・戻り値
 * 引数なし。検証済み設定または default を返す。
 * ## エラー
 * 読み取り例外と不正値は default へフォールバックする。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 設定読み取りを追加。
 */
export const readLanguagePreference = (): LanguagePreference => {
  try {
    return getLanguagePreference(window.localStorage.getItem(LANGUAGE_STORAGE_KEY));
  } catch {
    return LANGUAGE_PREFERENCES.DEFAULT;
  }
};

/**
 * ## 処理内容
 * 言語設定を localStorage へ保存する。
 * ## 引数・戻り値
 * 対応済み設定を受け取り、保存成功なら真を返す。
 * ## エラー
 * 書き込み例外を捕捉し、偽を返す。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 設定保存を追加。
 */
export const saveLanguagePreference = (value: LanguagePreference): boolean => {
  try {
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, value);
    return true;
  } catch {
    return false;
  }
};

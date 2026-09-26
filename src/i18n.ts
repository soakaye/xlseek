/**
 * ## 処理内容
 * tauri-plugin-i18n の翻訳APIを React から利用し、英語カタログによるキー単位のフォールバックと差込値の展開を行う。
 * ## 引数・戻り値
 * 言語、翻訳キー、プラグインの翻訳結果、英語カタログ、差込値を受け取り、表示文字列を返す。
 * ## エラー
 * プラグイン読込・言語切替・YAML解析の失敗を捕捉し、同梱英語または定義済み代替文言を返す。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): tauri-plugin-i18n アダプタと英語フォールバックを追加。
 */
import I18n from "@razein97/tauri-plugin-i18n";
import { createContext, useCallback, useContext } from "react";
import { parse } from "yaml";
import englishCatalogSource from "../src-tauri/locales/en.yml?raw";
import { DisplayLanguage } from "./locale-core";
import { I18N_CONSTANTS } from "./constants";

const INTERPOLATION_PATTERN = /\{([A-Za-z0-9_]+)\}/g;

/**
 * ## 処理内容
 * YAML の英語翻訳カタログから文字列値だけを抽出する。
 * ## 引数・戻り値
 * YAML テキストを受け取り、ドット区切りキーと翻訳文の対応表を返す。
 * ## エラー
 * YAML が不正またはルートがマップでない場合は空の対応表を返す。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): フロントエンド用英語カタログ読込を追加。
 */
const parseEnglishCatalog = (source: string): Record<string, string> => {
  try {
    const parsed: unknown = parse(source);
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) return {};
    return Object.fromEntries(
      Object.entries(parsed).filter((entry): entry is [string, string] => typeof entry[1] === "string"),
    );
  } catch {
    return {};
  }
};

const englishCatalog = parseEnglishCatalog(englishCatalogSource);
let activePluginLanguage: DisplayLanguage | null = null;
const LocaleContext = createContext<DisplayLanguage>("en");

export type TranslationKey = `${string}.${string}`;
export type TranslationValues = Readonly<Record<string, string | number>>;

/**
 * ## 処理内容
 * 現在の表示言語を子コンポーネントの翻訳フックへ提供する。
 * ## 引数・戻り値
 * 表示言語と React 子要素を受け取り、言語コンテキストを提供する要素を返す。
 * ## エラー
 * React の描画エラーは呼出元へ伝播する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): React 言語コンテキストを追加。
 */
export const LocaleProvider = LocaleContext.Provider;

/**
 * ## 処理内容
 * 翻訳キーを対象言語、英語カタログ、必須代替キーの順に解決し、プレースホルダーを展開する。
 * ## 引数・戻り値
 * 表示言語、キー、プラグインの結果、英語カタログ、任意の差込値を受け取り、翻訳文を返す。
 * ## エラー
 * キー欠落時は `common.translationUnavailable` または定義済み英語文を返し、例外を送出しない。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): キー単位の英語フォールバックを追加。
 */
export const resolveTranslation = (
  language: DisplayLanguage,
  key: string,
  pluginText: string,
  fallbackCatalog: Readonly<Record<string, string>> = englishCatalog,
  values: TranslationValues = {},
  pluginLanguage: DisplayLanguage | null = language,
): string => {
  const text = pluginLanguage === language && pluginText !== key
    ? pluginText
    : fallbackCatalog[key]
      ?? fallbackCatalog[I18N_CONSTANTS.TRANSLATION_UNAVAILABLE_KEY]
      ?? I18N_CONSTANTS.TRANSLATION_UNAVAILABLE_TEXT;
  return text.replace(INTERPOLATION_PATTERN, (_match, name: string) => String(values[name] ?? ""));
};

/**
 * ## 処理内容
 * tauri-plugin-i18n のカタログを読み込み、初期表示言語を設定する。
 * ## 引数・戻り値
 * `DisplayLanguage` を受け取り、成功時は true、プラグイン失敗時は false を返す。
 * ## エラー
 * プラグイン例外を捕捉し、英語ログを出して起動を継続する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): 起動時の翻訳初期化を追加。
 */
export const initializeI18n = async (language: DisplayLanguage): Promise<boolean> => {
  try {
    await I18n.getInstance().load();
    await I18n.setLocale(language);
    activePluginLanguage = language;
    return true;
  } catch {
    activePluginLanguage = null;
    console.error(I18N_CONSTANTS.PLUGIN_LOAD_FAILED_LOG);
    return false;
  }
};

/**
 * ## 処理内容
 * プラグインの現在言語を更新する。
 * ## 引数・戻り値
 * 表示言語コードを受け取り、成功時 true、失敗時 false を返す。
 * ## エラー
 * プラグイン例外を捕捉し、英語診断ログを出す。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): 表示言語切替APIを追加。
 */
export const setI18nLocale = async (language: DisplayLanguage): Promise<boolean> => {
  try {
    await I18n.setLocale(language);
    activePluginLanguage = language;
    return true;
  } catch {
    activePluginLanguage = null;
    console.error(I18N_CONSTANTS.PLUGIN_LOCALE_FAILED_LOG);
    return false;
  }
};

/**
 * ## 処理内容
 * プラグインの現在言語を使って翻訳し、同梱英語カタログへフォールバックする。
 * ## 引数・戻り値
 * 表示言語、翻訳キー、任意の差込値を受け取り、React に表示する文字列を返す。
 * ## エラー
 * プラグイン翻訳中の例外は英語カタログからの取得へ置き換える。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): React 用翻訳関数を追加。
 */
export const t = (
  language: DisplayLanguage,
  key: TranslationKey,
  values: TranslationValues = {},
): string => {
  let pluginText: string = key;
  try {
    pluginText = I18n.getInstance().translate(key);
  } catch {
    // 翻訳呼出し失敗時は英語カタログを使用する。
  }
  return resolveTranslation(language, key, pluginText, englishCatalog, values, activePluginLanguage);
};

/**
 * ## 処理内容
 * React コンポーネント向けに現在言語を束縛した翻訳関数を返す。
 * ## 引数・戻り値
 * 引数なし。ドット区切りキーと差込値を受ける翻訳関数を返す。
 * ## エラー
 * プラグインの翻訳失敗は同梱英語カタログへフォールバックする。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): コンポーネント用翻訳フックを追加。
 */
export const useTranslation = () => {
  const language = useContext(LocaleContext);
  return useCallback((key: TranslationKey, values?: TranslationValues) => t(language, key, values), [language]);
};

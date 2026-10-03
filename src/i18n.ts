/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Integrates tauri-plugin-i18n translation API with React, providing key-level fallback to bundled English catalog and variable interpolation.
 *
 * ## Arguments & Returns
 * Accepts language code, translation key, plugin result, fallback catalog, and interpolation parameters, returning translated display strings.
 *
 * ## Errors / Exceptions
 * Catches plugin loading, locale switching, and YAML parsing errors, falling back safely to bundled English catalog or default fallback message.
 */
import I18n from "@razein97/tauri-plugin-i18n";
import { createContext, useCallback, useContext } from "react";
import { parse } from "yaml";
import englishCatalogSource from "../src-tauri/locales/en.yml?raw";
import { DisplayLanguage } from "./locale-core";
import { I18N_CONSTANTS } from "./constants";

const INTERPOLATION_PATTERN = /\{([A-Za-z0-9_]+)\}/g;

/**
 * ## Description
 * Extracts string key-value mappings from the English YAML translation catalog.
 *
 * ## Arguments & Returns
 * @param source - Raw YAML string
 * @returns Map of dot-separated translation keys to English string values
 *
 * ## Errors / Exceptions
 * Returns an empty record if YAML parsing fails or root is not an object.
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
 * ## Description
 * Provides the current display language to child component translation hooks.
 */
export const LocaleProvider = LocaleContext.Provider;

/**
 * ## Description
 * Resolves translation keys in priority order (active language -> English fallback catalog -> unavailable message) and interpolates values.
 *
 * ## Arguments & Returns
 * @param language - Target display language
 * @param key - Translation key
 * @param pluginText - Text returned from tauri-plugin-i18n
 * @param fallbackCatalog - Fallback catalog (defaults to bundled English)
 * @param values - Interpolation parameters
 * @param pluginLanguage - Active plugin language
 * @returns Resolved translated string
 *
 * ## Errors / Exceptions
 * Falls back to `common.translationUnavailable` or standard placeholder if key is missing; never throws.
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
 * ## Description
 * Loads translation catalogs from tauri-plugin-i18n and sets the initial display language.
 *
 * ## Arguments & Returns
 * @param language - Initial `DisplayLanguage`
 * @returns Promise resolving to `true` on success, `false` on failure
 *
 * ## Errors / Exceptions
 * Catches plugin initialization errors, logs in English, and allows app startup to proceed.
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
 * ## Description
 * Updates the active locale in tauri-plugin-i18n.
 *
 * ## Arguments & Returns
 * @param language - Target display language code
 * @returns Promise resolving to `true` on success, `false` on failure
 *
 * ## Errors / Exceptions
 * Catches plugin errors, logs diagnostic information in English, and returns `false`.
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
 * ## Description
 * Translates a key using the active plugin locale, falling back to the bundled English catalog.
 *
 * ## Arguments & Returns
 * @param language - Current display language
 * @param key - Translation key
 * @param values - Interpolation parameters
 * @returns Formatted translation string
 *
 * ## Errors / Exceptions
 * Catches plugin translation errors and substitutes fallback text from English catalog.
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
    // On translation failure, fall back to English catalog
  }
  return resolveTranslation(language, key, pluginText, englishCatalog, values, activePluginLanguage);
};

/**
 * ## Description
 * React hook returning a translation function bound to the current context language.
 *
 * ## Arguments & Returns
 * No arguments. Returns `(key, values) => string` translation function.
 *
 * ## Errors / Exceptions
 * Plugin translation failures fall back to the bundled English catalog.
 */
export const useTranslation = () => {
  const language = useContext(LocaleContext);
  return useCallback((key: TranslationKey, values?: TranslationValues) => t(language, key, values), [language]);
};

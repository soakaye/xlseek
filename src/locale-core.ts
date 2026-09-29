/**
 * ## Description
 * Performs display language preference validation and system locale resolution.
 *
 * ## Arguments & Returns
 * Accepts locale strings or persisted values and returns corresponding typed language identifiers.
 *
 * ## Errors / Exceptions
 * Invalid values and acquisition failures fall back safely to default or English.
 */
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, LANGUAGE_STORAGE_KEY } from "./constants";

export type LanguagePreference = (typeof LANGUAGE_PREFERENCES)[keyof typeof LANGUAGE_PREFERENCES];
export type DisplayLanguage = (typeof DISPLAY_LANGUAGES)[keyof typeof DISPLAY_LANGUAGES];

/**
 * ## Description
 * Checks whether an unverified value matches one of the supported language preference options.
 *
 * ## Arguments & Returns
 * @param value - Unvalidated candidate value
 * @returns `true` if valid, `false` otherwise
 *
 * ## Errors / Exceptions
 * Pure type guard; does not throw exceptions.
 */
export const isLanguagePreference = (value: unknown): value is LanguagePreference =>
  Object.values(LANGUAGE_PREFERENCES).includes(value as LanguagePreference);

/**
 * ## Description
 * Normalizes an unverified value to a valid language preference.
 *
 * ## Arguments & Returns
 * @param value - Any input value
 * @returns Corresponding preference or default
 *
 * ## Errors / Exceptions
 * Never throws exceptions.
 */
export const getLanguagePreference = (value: unknown): LanguagePreference =>
  isLanguagePreference(value) ? value : LANGUAGE_PREFERENCES.DEFAULT;

/**
 * ## Description
 * Resolves an OS BCP-47 locale tag to a supported display language.
 *
 * ## Arguments & Returns
 * @param locale - BCP-47 locale tag or nullish value
 * @returns Resolved DisplayLanguage ("ja" or "en")
 *
 * ## Errors / Exceptions
 * Unsupported or missing locales fall back to English.
 */
export const resolveLanguage = (locale: string | null | undefined): DisplayLanguage =>
  locale?.toLowerCase().split(/[-_]/, 1)[0] === DISPLAY_LANGUAGES.JA
    ? DISPLAY_LANGUAGES.JA
    : DISPLAY_LANGUAGES.EN;

/**
 * ## Description
 * Safely reads the saved language preference from localStorage.
 *
 * ## Arguments & Returns
 * No arguments. Returns validated preference or default.
 *
 * ## Errors / Exceptions
 * Storage read errors and invalid values fall back to default.
 */
export const readLanguagePreference = (): LanguagePreference => {
  try {
    return getLanguagePreference(window.localStorage.getItem(LANGUAGE_STORAGE_KEY));
  } catch {
    return LANGUAGE_PREFERENCES.DEFAULT;
  }
};

/**
 * ## Description
 * Saves the language preference to localStorage.
 *
 * ## Arguments & Returns
 * @param value - Language preference to save
 * @returns `true` on success, `false` on write error
 *
 * ## Errors / Exceptions
 * Catches storage write errors and returns `false`.
 */
export const saveLanguagePreference = (value: LanguagePreference): boolean => {
  try {
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, value);
    return true;
  } catch {
    return false;
  }
};

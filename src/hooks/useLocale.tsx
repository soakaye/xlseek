/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Resolves the display language from stored preference and OS locale at startup,
 * and persists language preference changes.
 *
 * ## Arguments & Returns
 * No arguments. Returns display language, preference setting, and selection updater function.
 *
 * ## Errors / Exceptions
 * Absorbs OS locale retrieval and localStorage errors to maintain application startup.
 */
import { useCallback, useEffect, useState } from "react";
import { locale } from "@tauri-apps/plugin-os";
import { DisplayLanguage, LanguagePreference, readLanguagePreference, resolveLanguage, saveLanguagePreference } from "../locale-core";
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, TIMING_CONSTANTS } from "../constants";
import { initializeI18n, setI18nLocale } from "../i18n";

/**
 * ## Description
 * Races an asynchronous initialization promise against `TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS`
 * and resolves to a fallback value if the promise stalls or rejects.
 *
 * ## Arguments & Returns
 * @param task - Promise to await
 * @param fallback - Fallback value returned on timeout or error
 * @returns Resolved value from `task` or `fallback`
 *
 * ## Errors / Exceptions
 * Never rejects; catches errors and timeouts to return `fallback`.
 */
const withInitTimeout = async <T,>(task: Promise<T>, fallback: T): Promise<T> => {
  let timerId: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      task,
      new Promise<T>((resolve) => {
        // Constant reference: TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS
        timerId = setTimeout(() => resolve(fallback), TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS);
      }),
    ]);
  } catch {
    return fallback;
  } finally {
    if (timerId !== undefined) {
      clearTimeout(timerId);
    }
  }
};

/**
 * ## Description
 * Reads stored preferences and OS locale, managing shared display language and preference mutation.
 *
 * ## Arguments & Returns
 * No arguments. Returns current display language, preference setting, async selection function, and readiness state.
 *
 * ## Errors / Exceptions
 * Catches OS and localStorage errors, falling back safely to default or English.
 */
export const useLocale = () => {
  const [preference, setPreference] = useState<LanguagePreference>(LANGUAGE_PREFERENCES.DEFAULT);
  const [language, setLanguage] = useState<DisplayLanguage>(DISPLAY_LANGUAGES.EN);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let cancelled = false;
    const initialize = async () => {
      const stored = readLanguagePreference();
      const browserLocale = typeof navigator !== "undefined" && navigator.language ? navigator.language : DISPLAY_LANGUAGES.EN;
      const systemLanguage = (await withInitTimeout(locale(), browserLocale)) ?? browserLocale;
      const resolvedLanguage = stored === LANGUAGE_PREFERENCES.DEFAULT ? resolveLanguage(systemLanguage) : stored;
      await withInitTimeout(initializeI18n(resolvedLanguage), false);
      if (!cancelled) {
        setPreference(stored);
        setLanguage(resolvedLanguage);
        setReady(true);
      }
    };
    void initialize();
    return () => { cancelled = true; };
  }, []);

  const selectLanguage = useCallback(async (selected: LanguagePreference) => {
    let nextLanguage: DisplayLanguage = selected === LANGUAGE_PREFERENCES.DEFAULT ? DISPLAY_LANGUAGES.EN : selected;
    if (selected === LANGUAGE_PREFERENCES.DEFAULT) {
      try { nextLanguage = resolveLanguage(await locale()); } catch { nextLanguage = DISPLAY_LANGUAGES.EN; }
    }
    await setI18nLocale(nextLanguage);
    setPreference(selected);
    setLanguage(nextLanguage);
    return saveLanguagePreference(selected);
  }, []);

  return { language, preference, selectLanguage, ready };
};

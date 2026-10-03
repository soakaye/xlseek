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
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES } from "../constants";
import { initializeI18n, setI18nLocale } from "../i18n";

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
      let systemLanguage: string = DISPLAY_LANGUAGES.EN;
      try { systemLanguage = (await locale()) ?? DISPLAY_LANGUAGES.EN; } catch { /* Continue with English. */ }
      const resolvedLanguage = stored === LANGUAGE_PREFERENCES.DEFAULT ? resolveLanguage(systemLanguage) : stored;
      await initializeI18n(resolvedLanguage);
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

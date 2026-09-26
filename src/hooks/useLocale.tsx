/**
 * ## 処理内容
 * 起動時に保存設定と OS ロケールから表示言語を決め、設定変更を保存する。
 * ## 引数・戻り値
 * 引数なし。表示言語、選択設定、選択関数を返す。
 * ## エラー
 * OS ロケール取得と localStorage の失敗を吸収し、アプリ起動を継続する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): OS 既定と明示設定を統合。
 */
import { useCallback, useEffect, useState } from "react";
import { locale } from "@tauri-apps/plugin-os";
import { DisplayLanguage, LanguagePreference, readLanguagePreference, resolveLanguage, saveLanguagePreference } from "../locale-core";
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, setActiveLanguage } from "../constants";

/**
 * ## 処理内容
 * 保存設定と OS ロケールを読み取り、共有表示言語と設定変更操作を管理する。
 * ## 引数・戻り値
 * 引数なし。現在の表示言語、選択設定、非同期選択関数を返す。
 * ## エラー
 * OS と localStorage の失敗を捕捉し、default または英語で起動を継続する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): OS ロケールと言語設定のフックを追加。
 */
export const useLocale = () => {
  const [preference, setPreference] = useState<LanguagePreference>(LANGUAGE_PREFERENCES.DEFAULT);
  const [language, setLanguage] = useState<DisplayLanguage>(DISPLAY_LANGUAGES.EN);

  useEffect(() => {
    let cancelled = false;
    const initialize = async () => {
      const stored = readLanguagePreference();
      let systemLanguage: string = DISPLAY_LANGUAGES.EN;
      try { systemLanguage = (await locale()) ?? DISPLAY_LANGUAGES.EN; } catch { /* Continue with English. */ }
      if (!cancelled) {
        setPreference(stored);
        setLanguage(stored === LANGUAGE_PREFERENCES.DEFAULT ? resolveLanguage(systemLanguage) : stored);
      }
    };
    void initialize();
    return () => { cancelled = true; };
  }, []);

  setActiveLanguage(language);
  const selectLanguage = useCallback(async (selected: LanguagePreference) => {
    let nextLanguage: DisplayLanguage = selected === LANGUAGE_PREFERENCES.DEFAULT ? DISPLAY_LANGUAGES.EN : selected;
    if (selected === LANGUAGE_PREFERENCES.DEFAULT) {
      try { nextLanguage = resolveLanguage(await locale()); } catch { nextLanguage = DISPLAY_LANGUAGES.EN; }
    }
    setPreference(selected);
    setLanguage(nextLanguage);
    setActiveLanguage(nextLanguage);
    return saveLanguagePreference(selected);
  }, []);

  return { language, preference, selectLanguage };
};

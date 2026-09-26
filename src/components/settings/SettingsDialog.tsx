/**
 * ## 処理内容
 * 表示言語を既定・日本語・英語から選ぶアクセシブルな設定ダイアログ。
 * ## 引数・戻り値
 * 表示状態、現在の設定、言語変更・閉じるコールバックを受け、React 要素を返す。
 * ## エラー
 * 設定保存の失敗は呼び出し元が通知し、ダイアログは例外を送出しない。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定画面を追加。
 */
import React from "react";
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES } from "../../constants";
import { DisplayLanguage, LanguagePreference } from "../../locale-core";
import { useTranslation } from "../../i18n";

interface SettingsDialogProps {
  isOpen: boolean;
  preference: LanguagePreference;
  language: DisplayLanguage;
  onSelect: (value: LanguagePreference) => void;
  onClose: () => void;
}

/**
 * ## 処理内容
 * 言語選択肢と現在の表示言語を、キーボード操作可能なダイアログで表示する。
 * ## 引数・戻り値
 * `SettingsDialogProps` を受け取り、開いていればダイアログ要素、閉じていれば null を返す。
 * ## エラー
 * 保存失敗は親へ委譲し、Esc と閉じる操作は閉じるコールバックを呼ぶ。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定画面を追加。
 */
export const SettingsDialog: React.FC<SettingsDialogProps> = ({ isOpen, preference, language, onSelect, onClose }) => {
  const t = useTranslation();
  if (!isOpen) return null;
  const choices: Array<[LanguagePreference, string]> = [
    [LANGUAGE_PREFERENCES.DEFAULT, t("ui.LANGUAGE_DEFAULT")],
    [LANGUAGE_PREFERENCES.JA, t("ui.LANGUAGE_JA")],
    [LANGUAGE_PREFERENCES.EN, t("ui.LANGUAGE_EN")],
  ];
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70" onMouseDown={(event) => { if (event.target === event.currentTarget) onClose(); }} onKeyDown={(event) => { if (event.key === "Escape") onClose(); }}>
      <section role="dialog" aria-modal="true" aria-labelledby="settings-title" className="w-[min(28rem,90vw)] rounded-lg border border-zinc-700 bg-zinc-900 p-5 text-zinc-100 shadow-2xl">
        <div className="mb-4 flex items-center justify-between">
          <h2 id="settings-title" className="text-base font-semibold">{t("ui.SETTINGS_TITLE")}</h2>
          <button type="button" aria-label={t("about.BUTTON_CLOSE")} onClick={onClose} className="rounded px-2 py-1 hover:bg-zinc-700">×</button>
        </div>
        <p className="mb-4 text-sm text-zinc-400">{t("ui.SETTINGS_DESCRIPTION")}</p>
        <fieldset>
          <legend className="mb-2 text-sm font-medium">{t("ui.DISPLAY_LANGUAGE")}</legend>
          <div className="space-y-2">
            {choices.map(([value, label]) => (
              <label key={value} className="flex cursor-pointer items-center gap-2 rounded border border-zinc-700 px-3 py-2 hover:bg-zinc-800">
                <input type="radio" name="display-language" value={value} checked={preference === value} onChange={() => onSelect(value)} />
                <span>{label}</span>
              </label>
            ))}
          </div>
        </fieldset>
        <p className="mt-4 text-xs text-zinc-400">{t("ui.CURRENT_LANGUAGE")}: {language === DISPLAY_LANGUAGES.JA ? t("ui.LANGUAGE_JA") : t("ui.LANGUAGE_EN")}</p>
      </section>
    </div>
  );
};

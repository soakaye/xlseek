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
import React, { useEffect, useState } from "react";
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, SEARCH_HISTORY_CONSTANTS } from "../../constants";
import { DisplayLanguage, LanguagePreference } from "../../locale-core";
import { useTranslation } from "../../i18n";

/**
 * 処理内容: 設定ダイアログへ渡す表示状態と言語・履歴上限を定義する。
 * 引数・戻り値: 言語設定、履歴件数、および値更新・閉じるコールバックを保持する。
 * エラー: 保存処理の失敗は各コールバックの呼び出し元で通知する。
 * 変更履歴: v1.0.0 (2026-09-26, AI Agent): 言語設定プロパティを定義。v1.1.0 (2026-09-27, Codex): 履歴件数を追加。
 */
interface SettingsDialogProps {
  isOpen: boolean;
  preference: LanguagePreference;
  language: DisplayLanguage;
  onSelect: (value: LanguagePreference) => void;
  onClose: () => void;
  maxEntries: number;
  onSetMaxEntries: (value: number) => void;
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
 * - v1.1.0 (2026-09-27, Codex): 検索履歴の保存件数設定を追加。
 */
export const SettingsDialog: React.FC<SettingsDialogProps> = ({ isOpen, preference, language, onSelect, onClose, maxEntries, onSetMaxEntries }) => {
  const t = useTranslation();
  const [entryCount, setEntryCount] = useState(String(maxEntries));
  useEffect(() => { setEntryCount(String(maxEntries)); }, [maxEntries, isOpen]);
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
        <fieldset className="mt-4">
          <label htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID} className="mb-2 block text-sm font-medium">{t("ui.HISTORY_LIMIT_LABEL")}</label>
          <input id={SEARCH_HISTORY_CONSTANTS.INPUT_ID} type="number" min={SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES} max={SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES} step={SEARCH_HISTORY_CONSTANTS.STEP} value={entryCount} onChange={(event) => setEntryCount(event.target.value)} className="w-full rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm" />
          <p className="mt-1 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
          <button type="button" disabled={!/^\d+$/.test(entryCount) || Number(entryCount) > SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES} onClick={() => onSetMaxEntries(Number(entryCount))} className="mt-2 rounded bg-emerald-700 px-3 py-1.5 text-sm disabled:opacity-50">{t("ui.HISTORY_LIMIT_SAVE")}</button>
        </fieldset>
        <p className="mt-4 text-xs text-zinc-400">{t("ui.CURRENT_LANGUAGE")}: {language === DISPLAY_LANGUAGES.JA ? t("ui.LANGUAGE_JA") : t("ui.LANGUAGE_EN")}</p>
      </section>
    </div>
  );
};

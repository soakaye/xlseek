/**
 * ## 処理内容
 * 表示言語、検索履歴、およびデフォルト検索オプションを設定するアクセシブルなダイアログ。
 * ## 引数・戻り値
 * 表示状態、現在の設定、各設定更新・閉じるコールバックを受け、React 要素を返す。
 * ## エラー
 * 設定保存の失敗は呼び出し元が通知し、ダイアログは例外を送出しない。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定画面を追加。
 * - v1.1.0 (2026-09-27, Codex): 検索履歴の保存件数設定を追加。
 * - v1.2.0 (2026-09-28, AI Agent): デフォルト検索オプション設定および初期値リセットボタンを追加。
 */
import React, { useEffect, useState } from "react";
import {
  DISPLAY_LANGUAGES,
  FILE_EXTENSIONS,
  LANGUAGE_PREFERENCES,
  SEARCH_HISTORY_CONSTANTS,
  SEARCH_LABELS,
} from "../../constants";
import { DisplayLanguage, LanguagePreference } from "../../locale-core";
import { useTranslation } from "../../i18n";
import { DefaultSearchOptions } from "../../types/defaultOptions";
import { resetDefaultSearchOptions } from "../../default-options-core";

// 定数参照: 初期デフォルト検索オプションの固定参照（レンダー毎のオブジェクト再生成による無限再描画ループを防止）
const FALLBACK_DEFAULT_OPTIONS: DefaultSearchOptions = resetDefaultSearchOptions();

/**
 * 処理内容: 設定ダイアログへ渡す表示状態、言語、履歴上限、デフォルト検索オプションを定義する。
 * 引数・戻り値: 各種設定値および値更新・閉じるコールバックを保持する。
 * エラー: 保存処理の失敗は各コールバックの呼び出し元で通知する。
 * 変更履歴:
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定プロパティを定義。
 * - v1.1.0 (2026-09-27, Codex): 履歴件数を追加。
 * - v1.2.0 (2026-09-28, AI Agent): デフォルト検索オプションプロパティを追加。
 */
interface SettingsDialogProps {
  isOpen: boolean;
  preference: LanguagePreference;
  language: DisplayLanguage;
  onSelect: (value: LanguagePreference) => void;
  onClose: () => void;
  maxEntries: number;
  onSetMaxEntries: (value: number) => void;
  defaultOptions?: DefaultSearchOptions;
  onSaveDefaultOptions?: (options: DefaultSearchOptions) => void;
}

/**
 * ## 処理内容
 * 言語選択肢、検索履歴上限、およびデフォルト検索オプション設定をキーボード操作可能なダイアログで表示する。
 * ## 引数・戻り値
 * `SettingsDialogProps` を受け取り、開いていればダイアログ要素、閉じていれば null を返す。
 * ## エラー
 * 保存失敗は親へ委譲し、Esc と閉じる操作は閉じるコールバックを呼ぶ。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 言語設定画面を追加。
 * - v1.1.0 (2026-09-27, Codex): 検索履歴の保存件数設定を追加。
 * - v1.2.0 (2026-09-28, AI Agent): デフォルト検索オプション編集・リセット・保存を追加。
 * - v1.2.1 (2026-09-29, Antigravity): defaultOptions 未指定時のオブジェクト再生成による無限再描画ループを修正。
 */
export const SettingsDialog: React.FC<SettingsDialogProps> = ({
  isOpen,
  preference,
  language,
  onSelect,
  onClose,
  maxEntries,
  onSetMaxEntries,
  defaultOptions = FALLBACK_DEFAULT_OPTIONS,
  onSaveDefaultOptions,
}) => {
  const t = useTranslation();
  const [entryCount, setEntryCount] = useState(String(maxEntries));
  const [optDraft, setOptDraft] = useState<DefaultSearchOptions>(defaultOptions ?? FALLBACK_DEFAULT_OPTIONS);

  useEffect(() => {
    setEntryCount(String(maxEntries));
  }, [maxEntries, isOpen]);

  useEffect(() => {
    setOptDraft(defaultOptions ?? FALLBACK_DEFAULT_OPTIONS);
  }, [defaultOptions, isOpen]);

  if (!isOpen) return null;

  const choices: Array<[LanguagePreference, string]> = [
    [LANGUAGE_PREFERENCES.DEFAULT, t("ui.LANGUAGE_DEFAULT")],
    [LANGUAGE_PREFERENCES.JA, t("ui.LANGUAGE_JA")],
    [LANGUAGE_PREFERENCES.EN, t("ui.LANGUAGE_EN")],
  ];

  const handleToggleExtension = (ext: string) => {
    setOptDraft((prev) => {
      const exists = prev.extensions.includes(ext);
      const updated = exists
        ? prev.extensions.filter((item) => item !== ext)
        : [...prev.extensions, ext];
      return { ...prev, extensions: updated };
    });
  };

  const handleResetDefaults = () => {
    setOptDraft(resetDefaultSearchOptions());
  };

  const handleSaveOptions = () => {
    if (optDraft.extensions.length === 0) return;
    if (onSaveDefaultOptions) {
      onSaveDefaultOptions(optDraft);
    }
  };

  const isExtensionValid = optDraft.extensions.length > 0;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 overflow-y-auto p-4"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onClose();
      }}
      onKeyDown={(event) => {
        if (event.key === "Escape") onClose();
      }}
    >
      <section
        role="dialog"
        aria-modal="true"
        aria-labelledby="settings-title"
        className="w-[min(32rem,95vw)] max-h-[90vh] overflow-y-auto rounded-lg border border-zinc-700 bg-zinc-900 p-5 text-zinc-100 shadow-2xl"
      >
        <div className="mb-4 flex items-center justify-between">
          <h2 id="settings-title" className="text-base font-semibold">
            {t("ui.SETTINGS_TITLE")}
          </h2>
          <button
            type="button"
            aria-label={t("about.BUTTON_CLOSE")}
            onClick={onClose}
            className="rounded px-2 py-1 hover:bg-zinc-700"
          >
            ×
          </button>
        </div>
        <p className="mb-4 text-sm text-zinc-400">{t("ui.SETTINGS_DESCRIPTION")}</p>

        {/* 1. 表示言語設定 */}
        <fieldset>
          <legend className="mb-2 text-sm font-medium">{t("ui.DISPLAY_LANGUAGE")}</legend>
          <div className="space-y-2">
            {choices.map(([value, label]) => (
              <label
                key={value}
                className="flex cursor-pointer items-center gap-2 rounded border border-zinc-700 px-3 py-2 hover:bg-zinc-800"
              >
                <input
                  type="radio"
                  name="display-language"
                  value={value}
                  checked={preference === value}
                  onChange={() => onSelect(value)}
                />
                <span>{label}</span>
              </label>
            ))}
          </div>
        </fieldset>

        {/* 2. 検索履歴件数設定 */}
        <fieldset className="mt-4">
          <label
            htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID}
            className="mb-2 block text-sm font-medium"
          >
            {t("ui.HISTORY_LIMIT_LABEL")}
          </label>
          <input
            id={SEARCH_HISTORY_CONSTANTS.INPUT_ID}
            type="number"
            min={SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES}
            max={SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES}
            step={SEARCH_HISTORY_CONSTANTS.STEP}
            value={entryCount}
            onChange={(event) => setEntryCount(event.target.value)}
            className="w-full rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm"
          />
          <p className="mt-1 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
          <button
            type="button"
            disabled={
              !/^\d+$/.test(entryCount) ||
              Number(entryCount) > SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES
            }
            onClick={() => onSetMaxEntries(Number(entryCount))}
            className="mt-2 rounded bg-emerald-700 px-3 py-1.5 text-sm disabled:opacity-50"
          >
            {t("ui.HISTORY_LIMIT_SAVE")}
          </button>
        </fieldset>

        {/* 3. デフォルト検索オプション設定 */}
        <fieldset className="mt-6 border-t border-zinc-800 pt-4">
          <legend className="mb-1 text-sm font-medium">{t("ui.DEFAULT_OPTIONS_TITLE")}</legend>
          <p className="mb-3 text-xs text-zinc-400">{t("ui.DEFAULT_OPTIONS_DESCRIPTION")}</p>

          <div className="space-y-2 text-xs">
            {/* 大文字/小文字 */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.match_case}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, match_case: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t("ui.OPTION_MATCH_CASE")}</span>
            </label>

            {/* 正規表現 */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.use_regex}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, use_regex: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t("ui.OPTION_USE_REGEX")}</span>
            </label>

            {/* 数式 */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.include_formula}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, include_formula: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t("ui.OPTION_INCLUDE_FORMULA")}</span>
            </label>

            {/* Shape内テキスト */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.include_shape}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, include_shape: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t(SEARCH_LABELS.INCLUDE_SHAPE)}</span>
            </label>

            {/* コメント/メモ */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.include_comment}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, include_comment: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t("ui.OPTION_INCLUDE_COMMENT")}</span>
            </label>

            {/* 非表示シート */}
            <label className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800">
              <input
                type="checkbox"
                checked={optDraft.include_hidden}
                onChange={(e) =>
                  setOptDraft((prev) => ({ ...prev, include_hidden: e.target.checked }))
                }
                className="accent-excel rounded cursor-pointer"
              />
              <span>{t("ui.OPTION_INCLUDE_HIDDEN")}</span>
            </label>

            {/* 対象拡張子選択 */}
            <div className="mt-3 pt-2">
              <span className="block mb-1.5 text-zinc-300 font-medium">
                {t("ui.LABEL_TARGET_EXTENSIONS")}
              </span>
              <div className="flex gap-2">
                {FILE_EXTENSIONS.DEFAULT_LIST.map((ext) => {
                  const isSelected = optDraft.extensions.includes(ext);
                  return (
                    <button
                      key={ext}
                      type="button"
                      onClick={() => handleToggleExtension(ext)}
                      className={`px-2.5 py-1 rounded text-xs font-mono border transition ${
                        isSelected
                          ? "bg-emerald-950/80 text-emerald-300 border-emerald-600 font-medium"
                          : "bg-zinc-800 text-zinc-500 border-zinc-700 hover:text-zinc-300"
                      }`}
                    >
                      {ext}
                    </button>
                  );
                })}
              </div>
              {!isExtensionValid && (
                <p className="mt-1 text-[11px] text-amber-400">
                  {t("ui.DEFAULT_OPTIONS_EXTENSIONS_REQUIRED")}
                </p>
              )}
            </div>
          </div>

          <div className="mt-4 flex items-center justify-between gap-3">
            <button
              type="button"
              onClick={handleResetDefaults}
              className="rounded border border-zinc-700 bg-zinc-800 px-3 py-1.5 text-xs text-zinc-300 hover:bg-zinc-700 transition"
            >
              {t("ui.DEFAULT_OPTIONS_RESET")}
            </button>
            <button
              type="button"
              disabled={!isExtensionValid}
              onClick={handleSaveOptions}
              className="rounded bg-emerald-700 px-3 py-1.5 text-xs font-medium text-white hover:bg-emerald-600 disabled:opacity-50 transition"
            >
              {t("ui.DEFAULT_OPTIONS_SAVE")}
            </button>
          </div>
        </fieldset>

        <p className="mt-4 text-xs text-zinc-400">
          {t("ui.CURRENT_LANGUAGE")}:{" "}
          {language === DISPLAY_LANGUAGES.JA ? t("ui.LANGUAGE_JA") : t("ui.LANGUAGE_EN")}
        </p>
      </section>
    </div>
  );
};

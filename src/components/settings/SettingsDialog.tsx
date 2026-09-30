/**
 * ## Description
 * Accessible dialog for configuring display language, search history limits, and default search options.
 *
 * ## Arguments & Returns
 * Accepts open state, active preferences, update handlers, and close callback; returns React modal element.
 *
 * ## Errors / Exceptions
 * Persistence failures are notified by callers via toast; dialog does not throw exceptions.
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

// Constant reference: Stable fallback default search options to prevent infinite re-render loops from object recreation
const FALLBACK_DEFAULT_OPTIONS: DefaultSearchOptions = resetDefaultSearchOptions();

/**
 * Description: Defines property types passed to SettingsDialog.
 * Arguments & Returns: Holds settings state and update/close callbacks.
 * Errors: Storage failures are handled by callback callers.
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
 * ## Description
 * Renders language options, history limit configuration, and default search options in a keyboard-accessible modal dialog.
 *
 * ## Arguments & Returns
 * Accepts `SettingsDialogProps`; returns rendered dialog when open, null otherwise.
 *
 * ## Errors / Exceptions
 * Delegates storage failures to parents; Esc key and close button trigger onClose callback.
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

        {/* 1. Display Language Preference */}
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

        {/* 2. Search History Limit */}
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

        {/* 3. Default Search Options */}
        <fieldset className="mt-6 border-t border-zinc-800 pt-4">
          <legend className="mb-1 text-sm font-medium">{t("ui.DEFAULT_OPTIONS_TITLE")}</legend>
          <p className="mb-3 text-xs text-zinc-400">{t("ui.DEFAULT_OPTIONS_DESCRIPTION")}</p>

          <div className="space-y-2 text-xs">
            {/* Match case */}
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

            {/* Regex */}
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

            {/* Formula */}
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

            {/* Shape text */}
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

            {/* Comment */}
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

            {/* Hidden sheet */}
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

            {/* Target extension selector */}
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

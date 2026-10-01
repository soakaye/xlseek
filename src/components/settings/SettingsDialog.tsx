/**
 * Description: Provides immediate language selection and a separate draft-based search settings page.
 * Arguments & Returns: Accepts committed settings and callbacks; returns an accessible modal or null when closed.
 * Errors: Save callback failures retain the draft; all close routes discard it and storage errors remain caller-handled.
 */
import React, { useEffect, useRef, useState } from "react";
import {
  BURST_WORKER_CHOICES, DISPLAY_LANGUAGES, DIRECTORY_SEARCH_CONSTANTS, FILE_EXTENSIONS, KEYBOARD_KEYS, LANGUAGE_PREFERENCES,
  SEARCH_HISTORY_CONSTANTS, SEARCH_LABELS, SETTINGS_DIALOG_IDS, SETTINGS_KEYBOARD_KEYS, SETTINGS_PAGES, SETTINGS_TRANSLATION_KEYS,
} from "../../constants";
import { DisplayLanguage, LanguagePreference } from "../../locale-core";
import { TranslationKey, useTranslation } from "../../i18n";
import { DefaultSearchOptions } from "../../types/defaultOptions";
import { resetDefaultSearchOptions } from "../../default-options-core";

// Constant reference: DEFAULT_SEARCH_OPTIONS supplies fallback state for isolated dialog renders.
const FALLBACK_DEFAULT_OPTIONS = resetDefaultSearchOptions();

/**
 * Description: Describes the saved preference values submitted by this dialog.
 * Arguments & Returns: Carries the limit, options, and remembered valid worker count; has no return value.
 * Errors: Draft validation occurs before this value is submitted.
 */
export interface SettingsSubmission {
  maxEntries: number;
  defaultOptions: DefaultSearchOptions;
  rememberedBurstWorkers: number | null;
}

/**
 * Description: Defines settings dialog inputs and persistence callbacks.
 * Arguments & Returns: Accepts committed state and handlers; no return value.
 * Errors: onSaveSettings returns false on failure so the dialog can retain its draft.
 */
interface SettingsDialogProps {
  isOpen: boolean;
  preference: LanguagePreference;
  language: DisplayLanguage;
  onSelect: (value: LanguagePreference) => void;
  onClose: () => void;
  maxEntries: number;
  defaultOptions?: DefaultSearchOptions;
  rememberedBurstWorkers?: number | null;
  onSaveSettings?: (settings: SettingsSubmission) => boolean;
}

/**
 * Description: Renders localized immediate and saved settings pages with validated draft state and accessible modal controls.
 * Arguments & Returns: Accepts SettingsDialogProps; returns the open modal or null.
 * Errors: Catches unexpected synchronous save callback errors and leaves the dialog open for recovery.
 */
export const SettingsDialog: React.FC<SettingsDialogProps> = ({
  isOpen, preference, language, onSelect, onClose, maxEntries,
  defaultOptions = FALLBACK_DEFAULT_OPTIONS,
  rememberedBurstWorkers = null, onSaveSettings = () => true,
}) => {
  const t = useTranslation();
  const [page, setPage] = useState<string>(SETTINGS_PAGES.IMMEDIATE);
  const [historyInput, setHistoryInput] = useState(String(maxEntries));
  const [options, setOptions] = useState<DefaultSearchOptions>({ ...defaultOptions, extensions: [...defaultOptions.extensions] });
  const [workerChoice, setWorkerChoice] = useState<string>(defaultOptions.burst_workers === null ? DIRECTORY_SEARCH_CONSTANTS.AUTOMATIC : BURST_WORKER_CHOICES.CUSTOM);
  const [workerInput, setWorkerInput] = useState(String(defaultOptions.burst_workers ?? rememberedBurstWorkers ?? ""));
  const [rememberedWorkers, setRememberedWorkers] = useState<number | null>(rememberedBurstWorkers ?? defaultOptions.burst_workers);
  const [saving, setSaving] = useState(false);
  const savingRef = useRef(false);
  const dialogRef = useRef<HTMLElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);
  const savedDraftSeedRef = useRef({
    maxEntries,
    defaultOptions: { ...defaultOptions, extensions: [...defaultOptions.extensions] },
    rememberedBurstWorkers,
  });

  useEffect(() => {
    if (isOpen) return;
    savedDraftSeedRef.current = {
      maxEntries,
      defaultOptions: { ...defaultOptions, extensions: [...defaultOptions.extensions] },
      rememberedBurstWorkers,
    };
  }, [defaultOptions, isOpen, maxEntries, rememberedBurstWorkers]);

  useEffect(() => {
    if (!isOpen) return;
    const seed = savedDraftSeedRef.current;
    previousFocusRef.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    setPage(SETTINGS_PAGES.IMMEDIATE);
    setHistoryInput(String(seed.maxEntries));
    setOptions({ ...seed.defaultOptions, extensions: [...seed.defaultOptions.extensions] });
    setWorkerChoice(seed.defaultOptions.burst_workers === null ? DIRECTORY_SEARCH_CONSTANTS.AUTOMATIC : BURST_WORKER_CHOICES.CUSTOM);
    setWorkerInput(String(seed.defaultOptions.burst_workers ?? seed.rememberedBurstWorkers ?? ""));
    setRememberedWorkers(seed.rememberedBurstWorkers ?? seed.defaultOptions.burst_workers);
    savingRef.current = false;
    setSaving(false);
    const frame = window.requestAnimationFrame(() => dialogRef.current?.querySelector<HTMLElement>("button, input")?.focus());
    return () => {
      window.cancelAnimationFrame(frame);
      previousFocusRef.current?.focus();
    };
  }, [isOpen]);

  if (!isOpen) return null;

  const historyValid = /^\d+$/.test(historyInput) && Number(historyInput) >= SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES && Number(historyInput) <= SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES;
  const workerActive = options.directory_mode === DIRECTORY_SEARCH_CONSTANTS.BURST && workerChoice === BURST_WORKER_CHOICES.CUSTOM;
  const workerValid = !workerActive || (/^\d+$/.test(workerInput) && Number(workerInput) >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN && Number(workerInput) <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX);
  const extensionsValid = options.extensions.length > 0;
  const valid = historyValid && workerValid && extensionsValid;
  const choices: Array<[LanguagePreference, string]> = [
    [LANGUAGE_PREFERENCES.DEFAULT, t("ui.LANGUAGE_DEFAULT")],
    [LANGUAGE_PREFERENCES.JA, t("ui.LANGUAGE_JA")],
    [LANGUAGE_PREFERENCES.EN, t("ui.LANGUAGE_EN")],
  ];

  /**
   * Description: Closes the dialog and discards draft state unless a save is in progress.
   * Arguments & Returns: Takes no arguments; returns void.
   * Errors: Does nothing while saving to avoid discarding the candidate being committed.
   */
  const close = (): void => {
    if (savingRef.current) return;
    onClose();
  };

  /**
   * Description: Validates and submits all saved-page values exactly once.
   * Arguments & Returns: Takes no arguments; returns void.
   * Errors: Invalid input or callback failure keeps the dialog and draft open.
   */
  const save = (): void => {
    if (savingRef.current || !valid) return;
    savingRef.current = true;
    setSaving(true);
    const selectedWorkers = workerActive
      ? Number(workerInput)
      : workerChoice === BURST_WORKER_CHOICES.CUSTOM ? rememberedWorkers : null;
    const submitted = {
      ...options,
      burst_workers: selectedWorkers,
      extensions: [...options.extensions],
    };
    try {
      if (onSaveSettings({ maxEntries: Number(historyInput), defaultOptions: submitted, rememberedBurstWorkers: rememberedWorkers })) {
        onClose();
      } else {
        savingRef.current = false;
        setSaving(false);
      }
    } catch {
      savingRef.current = false;
      setSaving(false);
    }
  };

  /**
   * Description: Keeps keyboard focus within the open modal and handles Escape as cancellation.
   * Arguments & Returns: Accepts a keyboard event; returns void.
   * Errors: Missing focusable controls leave focus on the dialog without throwing.
   */
  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>): void => {
    if (event.key === KEYBOARD_KEYS.ESCAPE) {
      event.preventDefault();
      close();
      return;
    }
    if ([SETTINGS_KEYBOARD_KEYS.LEFT, SETTINGS_KEYBOARD_KEYS.RIGHT, SETTINGS_KEYBOARD_KEYS.HOME, SETTINGS_KEYBOARD_KEYS.END].includes(event.key as typeof SETTINGS_KEYBOARD_KEYS[keyof typeof SETTINGS_KEYBOARD_KEYS]) && (event.target as HTMLElement).getAttribute("role") === "tab") {
      event.preventDefault();
      const tabs = [...(dialogRef.current?.querySelectorAll<HTMLElement>("[role='tab']") ?? [])];
      const currentIndex = tabs.indexOf(event.target as HTMLElement);
      const nextIndex = event.key === SETTINGS_KEYBOARD_KEYS.HOME ? 0
        : event.key === SETTINGS_KEYBOARD_KEYS.END ? tabs.length - 1
        : (currentIndex + (event.key === SETTINGS_KEYBOARD_KEYS.RIGHT ? 1 : tabs.length - 1)) % tabs.length;
      tabs[nextIndex]?.click();
      tabs[nextIndex]?.focus();
      return;
    }
    if (event.key !== KEYBOARD_KEYS.TAB) return;
    const elements = dialogRef.current?.querySelectorAll<HTMLElement>("button:not(:disabled), input:not(:disabled), [tabindex='0']");
    if (!elements?.length) return;
    const first = elements[0];
    const last = elements[elements.length - 1];
    if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
    else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
  };

  /**
   * Description: Toggles one supported extension in the unsaved options draft.
   * Arguments & Returns: Accepts the extension string; returns void.
   * Errors: Unknown values can only arrive from fixed supported extension controls.
   */
  const toggleExtension = (extension: string): void => {
    setOptions((current) => ({ ...current, extensions: current.extensions.includes(extension)
      ? current.extensions.filter((item) => item !== extension)
      : [...current.extensions, extension] }));
  };

  /**
   * Description: Restores only default search options in the draft and clears remembered worker input.
   * Arguments & Returns: Takes no arguments; returns void.
   * Errors: Does not persist; history and language draft state remain unchanged.
   */
  const resetDraft = (): void => {
    const reset = resetDefaultSearchOptions();
    setOptions(reset);
    setWorkerChoice(DIRECTORY_SEARCH_CONSTANTS.AUTOMATIC);
    setWorkerInput("");
    setRememberedWorkers(null);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4" onMouseDown={(event) => { if (event.target === event.currentTarget) close(); }} onKeyDown={handleKeyDown}>
      <section ref={dialogRef} role="dialog" aria-modal="true" aria-labelledby={SETTINGS_DIALOG_IDS.TITLE} tabIndex={-1} className="flex max-h-[min(90vh,56rem)] w-[min(32rem,95vw)] flex-col overflow-hidden rounded-lg border border-zinc-700 bg-zinc-900 text-zinc-100 shadow-2xl">
        <header className="shrink-0 border-b border-zinc-800 p-5">
          <div className="mb-3 flex items-center justify-between">
            <h2 id={SETTINGS_DIALOG_IDS.TITLE} className="text-base font-semibold">{t("ui.SETTINGS_TITLE")}</h2>
            <button type="button" aria-label={t(SETTINGS_TRANSLATION_KEYS.CLOSE)} disabled={saving} onClick={close} className="rounded px-2 py-1 hover:bg-zinc-700 disabled:opacity-50">×</button>
          </div>
          <div id={SETTINGS_DIALOG_IDS.PAGE_TABS} role="tablist" aria-label={t("ui.SETTINGS_TITLE")} className="grid grid-cols-2 gap-2">
            {[[SETTINGS_PAGES.IMMEDIATE, SETTINGS_TRANSLATION_KEYS.IMMEDIATE_PAGE], [SETTINGS_PAGES.SAVED, SETTINGS_TRANSLATION_KEYS.SAVED_PAGE]].map(([value, label]) => (
              <button key={value} type="button" role="tab" id={`${SETTINGS_DIALOG_IDS.TAB_PREFIX}${value}`} aria-controls={value === SETTINGS_PAGES.IMMEDIATE ? SETTINGS_DIALOG_IDS.IMMEDIATE_PANEL : SETTINGS_DIALOG_IDS.SAVED_PANEL} aria-selected={page === value} tabIndex={page === value ? 0 : -1} disabled={saving} onClick={() => setPage(value)} className={`rounded px-3 py-2 text-sm ${page === value ? "bg-zinc-700 text-white" : "bg-zinc-800 text-zinc-300"}`}>
                {t(label as typeof SETTINGS_TRANSLATION_KEYS[keyof typeof SETTINGS_TRANSLATION_KEYS])}
              </button>
            ))}
          </div>
        </header>

        <main className="min-h-0 flex-1 overflow-y-auto p-5">
          {page === SETTINGS_PAGES.IMMEDIATE ? (
            <section role="tabpanel" id={SETTINGS_DIALOG_IDS.IMMEDIATE_PANEL} aria-labelledby={`${SETTINGS_DIALOG_IDS.TAB_PREFIX}${SETTINGS_PAGES.IMMEDIATE}`}>
              <p className="mb-4 text-sm text-zinc-400">{t(SETTINGS_TRANSLATION_KEYS.IMMEDIATE_DESCRIPTION)}</p>
              <fieldset>
                <legend className="mb-2 text-sm font-medium">{t("ui.DISPLAY_LANGUAGE")}</legend>
                <div className="space-y-2">{choices.map(([value, label]) => <label key={value} className="flex cursor-pointer items-center gap-2 rounded border border-zinc-700 px-3 py-2 hover:bg-zinc-800"><input type="radio" name="display-language" value={value} checked={preference === value} onChange={() => onSelect(value)} /><span>{label}</span></label>)}</div>
              </fieldset>
              <p className="mt-4 text-xs text-zinc-400">{t("ui.CURRENT_LANGUAGE")}: {language === DISPLAY_LANGUAGES.JA ? t("ui.LANGUAGE_JA") : t("ui.LANGUAGE_EN")}</p>
            </section>
          ) : (
            <section role="tabpanel" id={SETTINGS_DIALOG_IDS.SAVED_PANEL} aria-labelledby={`${SETTINGS_DIALOG_IDS.TAB_PREFIX}${SETTINGS_PAGES.SAVED}`} aria-describedby={SETTINGS_DIALOG_IDS.SAVED_DESCRIPTION}>
              <p id={SETTINGS_DIALOG_IDS.SAVED_DESCRIPTION} className="mb-4 text-sm text-zinc-400">{t(SETTINGS_TRANSLATION_KEYS.SAVED_DESCRIPTION)}</p>
              <div className="mb-5">
                <div className="flex items-center justify-between gap-4">
                  <label htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID} className="text-sm font-medium">{t("ui.HISTORY_LIMIT_LABEL")}</label>
                  <input id={SEARCH_HISTORY_CONSTANTS.INPUT_ID} type="number" min={SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES} max={SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES} step={SEARCH_HISTORY_CONSTANTS.STEP} value={historyInput} disabled={saving} aria-invalid={!historyValid} aria-describedby={`${SETTINGS_DIALOG_IDS.HISTORY_HELP} ${SETTINGS_DIALOG_IDS.HISTORY_ERROR}`} onChange={(event) => setHistoryInput(event.target.value)} className="w-24 rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none" />
                </div>
                <p id={SETTINGS_DIALOG_IDS.HISTORY_HELP} className="mt-2 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
                {!historyValid && <p id={SETTINGS_DIALOG_IDS.HISTORY_ERROR} role="alert" className="mt-2 text-xs text-amber-400">{t(SETTINGS_TRANSLATION_KEYS.HISTORY_RANGE_ERROR)}</p>}
              </div>

              <fieldset className="mt-6 border-t border-zinc-800 pt-4">
                <legend className="mb-1 text-sm font-medium">{t("ui.DEFAULT_OPTIONS_TITLE")}</legend>
                <p className="mb-3 text-xs text-zinc-400">{t("ui.DEFAULT_OPTIONS_DESCRIPTION")}</p>
                <fieldset className="mb-3"><legend className="mb-2 text-xs font-medium">{t("ui.DIRECTORY_SEARCH_MODE")}</legend>
                  {[[DIRECTORY_SEARCH_CONSTANTS.SEQUENTIAL, "ui.DIRECTORY_SEARCH_SEQUENTIAL"], [DIRECTORY_SEARCH_CONSTANTS.BURST, "ui.DIRECTORY_SEARCH_BURST"]].map(([value, label]) => <label key={value} className="mr-3 inline-flex items-center gap-2"><input type="radio" name="directory-search-mode" checked={options.directory_mode === value} disabled={saving} onChange={() => setOptions((current) => ({ ...current, directory_mode: value as DefaultSearchOptions["directory_mode"] }))} /><span>{t(label as TranslationKey)}</span></label>)}
                </fieldset>
                <fieldset className="mb-3"><legend className="mb-2 text-xs font-medium">{t("ui.BURST_WORKER_CHOICE")}</legend>
                  {[[DIRECTORY_SEARCH_CONSTANTS.AUTOMATIC, "ui.BURST_WORKERS_AUTOMATIC"], [BURST_WORKER_CHOICES.CUSTOM, "ui.BURST_WORKERS_CUSTOM"]].map(([value, label]) => <label key={value} className="mr-3 inline-flex items-center gap-2"><input type="radio" name="burst-worker-choice" value={value} checked={workerChoice === value} disabled={saving || options.directory_mode !== DIRECTORY_SEARCH_CONSTANTS.BURST} onChange={() => { setWorkerChoice(value); if (value === DIRECTORY_SEARCH_CONSTANTS.AUTOMATIC) setOptions((current) => ({ ...current, burst_workers: null })); }} /><span>{t(label as TranslationKey)}</span></label>)}
                  <input aria-label={t("ui.BURST_WORKER_CHOICE")} type="number" min={DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN} max={DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX} step={SEARCH_HISTORY_CONSTANTS.STEP} value={workerInput} disabled={saving || options.directory_mode !== DIRECTORY_SEARCH_CONSTANTS.BURST || workerChoice !== BURST_WORKER_CHOICES.CUSTOM} aria-invalid={workerActive && !workerValid} aria-describedby={SETTINGS_DIALOG_IDS.WORKER_ERROR} onChange={(event) => { const value = event.target.value; setWorkerInput(value); if (/^\d+$/.test(value) && Number(value) >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN && Number(value) <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX) { setRememberedWorkers(Number(value)); setOptions((current) => ({ ...current, burst_workers: Number(value) })); } }} className="ml-3 w-24 rounded border border-zinc-700 bg-zinc-800 px-2 py-1" />
                  {workerActive && !workerValid && <p id={SETTINGS_DIALOG_IDS.WORKER_ERROR} role="alert" className="mt-1 text-amber-400">{t("ui.BURST_WORKERS_RANGE_ERROR")}</p>}
                </fieldset>
                <div className="space-y-2 text-xs">{[
                  ["match_case", "ui.OPTION_MATCH_CASE"], ["use_regex", "ui.OPTION_USE_REGEX"], ["include_formula", "ui.OPTION_INCLUDE_FORMULA"], ["include_shape", SEARCH_LABELS.INCLUDE_SHAPE], ["include_comment", "ui.OPTION_INCLUDE_COMMENT"], ["include_hidden", "ui.OPTION_INCLUDE_HIDDEN"],
                ].map(([field, label]) => <label key={field} className="flex cursor-pointer items-center gap-2 rounded border border-zinc-800 bg-zinc-850 px-3 py-2 hover:bg-zinc-800"><input type="checkbox" disabled={saving} checked={Boolean(options[field as keyof DefaultSearchOptions])} onChange={(event) => setOptions((current) => ({ ...current, [field]: event.target.checked }))} /><span>{t(label as TranslationKey)}</span></label>)}
                  <div><span className="mb-1.5 block font-medium text-zinc-300">{t("ui.LABEL_TARGET_EXTENSIONS")}</span><div className="flex gap-2">{FILE_EXTENSIONS.DEFAULT_LIST.map((extension) => <button key={extension} type="button" disabled={saving} aria-pressed={options.extensions.includes(extension)} onClick={() => toggleExtension(extension)} className={`rounded border px-2.5 py-1 font-mono ${options.extensions.includes(extension) ? "border-emerald-600 bg-emerald-950/80 text-emerald-300" : "border-zinc-700 bg-zinc-800 text-zinc-500"}`}>{extension}</button>)}</div>{!extensionsValid && <p role="alert" className="mt-1 text-amber-400">{t("ui.DEFAULT_OPTIONS_EXTENSIONS_REQUIRED")}</p>}</div>
                </div>
              </fieldset>
            </section>
          )}
        </main>

        <footer className="flex shrink-0 items-center justify-between gap-3 border-t border-zinc-800 p-4">
          {page === SETTINGS_PAGES.IMMEDIATE ? <button type="button" disabled={saving} onClick={close} className="ml-auto rounded bg-zinc-700 px-3 py-1.5 text-sm">{t(SETTINGS_TRANSLATION_KEYS.CLOSE)}</button> : <>
            <button type="button" disabled={saving} onClick={resetDraft} className="rounded border border-zinc-700 bg-zinc-800 px-3 py-1.5 text-xs">{t("ui.DEFAULT_OPTIONS_RESET")}</button>
            <div className="flex gap-2"><button type="button" disabled={saving} onClick={close} className="rounded bg-zinc-700 px-3 py-1.5 text-sm">{t(SETTINGS_TRANSLATION_KEYS.CANCEL)}</button><button type="button" disabled={!valid || saving} onClick={save} className="rounded bg-emerald-700 px-3 py-1.5 text-sm font-medium disabled:opacity-50">{t(SETTINGS_TRANSLATION_KEYS.SAVE)}</button></div>
          </>}
        </footer>
      </section>
    </div>
  );
};

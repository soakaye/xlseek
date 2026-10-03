/**
 * Copyright (c) 2026 soakaye
 *
 * Renders nonfatal directory discovery problems for the active search.
 * Arguments: accepts issue records and the active UI language; returns a warning list or null.
 * Errors: translation lookup is total and rendering does not throw for path text.
 */
import React from "react";
import { SearchIssue } from "../../types/search";
import { DisplayLanguage } from "../../locale-core";
import { t } from "../../i18n";

/**
 * Shows folders that could not be read while continuing to display search results.
 * Arguments: `issues` is the list emitted by discovery; `language` selects translated labels.
 * Returns: an accessible warning panel, or null when there are no issues.
 * Errors: no exceptions are expected; paths are rendered as text.
 */
export const SearchWarnings: React.FC<{ issues: SearchIssue[]; language: DisplayLanguage }> = ({ issues, language }) => {
  if (issues.length === 0) return null;
  const uniqueIssues = Array.from(new Map(issues.map((issue) => [`${issue.path}:${issue.code}`, issue])).values());
  return (
    <section className="border-b border-amber-800/70 bg-amber-950/30 px-4 py-2 text-xs text-amber-200" role="status">
      {/* Constant reference: translation keys are defined in src-tauri/locales/en.yml and ja.yml. */}
      <p className="mb-1 font-medium">{t(language, "ui.SEARCH_WARNING_TITLE")}</p>
      <ul className="max-h-20 list-inside list-disc overflow-y-auto">
        {uniqueIssues.map((issue) => <li key={`${issue.path}:${issue.code}`} className="break-all">{t(language, "ui.SEARCH_WARNING_PATH", { path: issue.path })}</li>)}
      </ul>
    </section>
  );
};

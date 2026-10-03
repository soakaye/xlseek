/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Package detail view component (src/components/about/PackageDetail.tsx)
 *
 * ## Description
 * Manages the right pane (60% width) in the About dialog "Open Source Licenses" tab.
 * Renders selected package metadata (name, version, SPDX license, author, repository URL)
 * and independently scrollable full license text.
 * Integrates clipboard copy button and visual feedback.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), Principle III (comprehensive documentation), and Principle IV (modular design).
 */

import React from "react";
import { Copy, Check } from "lucide-react";
import { PackageLicenseRecord } from "../../types/license";
import { useTranslation } from "../../i18n";

export interface PackageDetailProps {
  package: PackageLicenseRecord | null;
  onCopyLicense: (text: string) => void;
  isCopied: boolean;
}

/**
 * ## Description
 * Package detail view component.
 *
 * ## Arguments
 * @param props - PackageDetailProps
 *
 * ## Returns
 * @returns Rendered element
 *
 * ## Errors / Exceptions
 * Shows guidance placeholder when package is null.
 */
export const PackageDetail: React.FC<PackageDetailProps> = ({
  package: pkg,
  onCopyLicense,
  isCopied,
}) => {
  const t = useTranslation();
  if (!pkg) {
    return (
      <div className="flex-1 flex items-center justify-center p-8 text-center text-xs text-zinc-500 bg-[#18181b]">
        {t("about.SELECT_PACKAGE_PROMPT")}
      </div>
    );
  }

  const lineCount = pkg.license_text.split("\n").length;

  return (
    <div className="flex-1 flex flex-col bg-[#18181b] min-w-0 overflow-hidden">
      {/* Detail header */}
      <div className="p-4 border-b border-zinc-800 flex-shrink-0 bg-[#161618]">
        <div className="flex items-start justify-between gap-3 mb-2">
          <div>
            <div className="flex items-center gap-2">
              <h4 className="text-base font-semibold text-zinc-100">{pkg.name}</h4>
              <span className="text-xs font-mono text-zinc-400 bg-zinc-800 px-2 py-0.5 rounded border border-zinc-700">
                v{pkg.version}
              </span>
              <span className="text-[11px] font-medium px-2 py-0.5 rounded bg-emerald-950/60 text-emerald-300 border border-emerald-800">
                {pkg.license}
              </span>
            </div>
          </div>

          {/* Copy button */}
          <button
            type="button"
            onClick={() => onCopyLicense(pkg.license_text)}
            className={`px-3 py-1.5 rounded-lg border text-xs font-medium flex items-center gap-1.5 transition cursor-pointer flex-shrink-0 ${
              isCopied
                ? "bg-emerald-950 text-emerald-300 border-emerald-600 shadow"
                : "bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white border-zinc-700"
            }`}
          >
            {isCopied ? (
              <>
                <Check className="w-3.5 h-3.5 text-emerald-400" />
                <span>{t("about.BUTTON_COPIED")}</span>
              </>
            ) : (
              <>
                <Copy className="w-3.5 h-3.5 text-zinc-400" />
                <span>{t("about.BUTTON_COPY_LICENSE")}</span>
              </>
            )}
          </button>
        </div>

        {/* Metadata (Author, Repository) */}
        <div className="space-y-1 text-xs text-zinc-400">
          {pkg.author && (
            <div className="flex items-baseline gap-2">
              <span className="text-zinc-500 font-medium w-28 flex-shrink-0">
                {t("about.AUTHOR_LABEL")}
              </span>
              <span className="text-zinc-300 truncate">{pkg.author}</span>
            </div>
          )}
          {pkg.repository && (
            <div className="flex items-baseline gap-2">
              <span className="text-zinc-500 font-medium w-28 flex-shrink-0">
                {t("about.REPOSITORY_LABEL")}
              </span>
              <span className="text-zinc-300 font-mono text-[11px] truncate flex items-center gap-1">
                {pkg.repository}
              </span>
            </div>
          )}
        </div>
      </div>

      {/* License text scrollview */}
      <div className="flex-1 flex flex-col p-4 min-h-0 bg-[#141416]">
        <div className="text-[11px] text-zinc-400 font-medium mb-1.5 flex items-center justify-between">
          <span>{t("about.LICENSE_TEXT_LABEL")}</span>
          <span className="text-[10px] text-zinc-500 font-mono">
            {lineCount} {t("ui.UNIT_ROWS")}
          </span>
        </div>
        <div className="flex-1 overflow-y-auto bg-[#1a1a1d] border border-zinc-800 rounded-lg p-3.5">
          <pre className="font-mono text-xs text-zinc-300 whitespace-pre-wrap leading-relaxed select-text">
            {pkg.license_text}
          </pre>
        </div>
      </div>
    </div>
  );
};

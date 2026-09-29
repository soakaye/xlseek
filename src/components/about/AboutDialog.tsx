/**
 * @fileoverview About dialog top-level component (src/components/about/AboutDialog.tsx)
 *
 * ## Description
 * Displays basic application metadata (name, version, description, copyright) and open-source license
 * information for all third-party packages used across the application in a modal dialog.
 * Handles tab navigation (App info / Open Source Licenses), keyboard ESC and backdrop dismissal,
 * and integration of child components (PackageList, PackageDetail).
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), Principle III (comprehensive documentation), and Principle IV (modular design).
 */

import React, { useState, useEffect, useMemo, useCallback } from "react";
import { X, FileCode2, ShieldCheck } from "lucide-react";
import { PackageLicenseRecord, AboutTabType } from "../../types/license";
import { LAYOUT_CONSTANTS } from "../../constants";
import defaultLicensesJson from "../../constants/licenses.json";
import { PackageList } from "./PackageList";
import { PackageDetail } from "./PackageDetail";
import { useTranslation } from "../../i18n";

export interface AboutDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onShowToast: (key: import("../../i18n").TranslationKey, values?: import("../../i18n").TranslationValues) => void;
  licenses?: PackageLicenseRecord[];
}

/**
 * ## Description
 * About modal dialog component.
 *
 * ## Arguments
 * @param props - AboutDialogProps
 *
 * ## Returns
 * @returns Rendered dialog element or null if not open
 *
 * ## Errors / Exceptions
 * Clipboard copy errors are caught and surfaced via toast notifications without crashing the UI.
 */
export const AboutDialog: React.FC<AboutDialogProps> = ({
  isOpen,
  onClose,
  onShowToast,
  licenses = defaultLicensesJson as PackageLicenseRecord[],
}) => {
  const t = useTranslation();
  const [activeTab, setActiveTab] = useState<AboutTabType>("about");
  const [searchKeyword, setSearchKeyword] = useState<string>("");
  const [selectedPackageId, setSelectedPackageId] = useState<string | null>(null);
  const [copyFeedback, setCopyFeedback] = useState<boolean>(false);

  // Set initial package selection when opening
  useEffect(() => {
    if (isOpen) {
      if (licenses.length > 0 && !selectedPackageId) {
        setSelectedPackageId(licenses[0].id);
      }
    }
  }, [isOpen, licenses, selectedPackageId]);

  // Subscribe to ESC key to dismiss dialog
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  // Filter package list based on search keyword
  const filteredPackages = useMemo(() => {
    const keyword = searchKeyword.trim().toLowerCase();
    if (!keyword) {
      return licenses;
    }
    return licenses.filter(
      (pkg) =>
        pkg.name.toLowerCase().includes(keyword) ||
        pkg.license.toLowerCase().includes(keyword) ||
        pkg.version.toLowerCase().includes(keyword)
    );
  }, [licenses, searchKeyword]);

  // Sync selected package ID with filtered results
  useEffect(() => {
    if (filteredPackages.length > 0) {
      const exists = filteredPackages.some((p) => p.id === selectedPackageId);
      if (!exists) {
        setSelectedPackageId(filteredPackages[0].id);
      }
    } else {
      setSelectedPackageId(null);
    }
  }, [filteredPackages, selectedPackageId]);

  // Active package record
  const selectedPackage = useMemo(() => {
    if (!selectedPackageId) return null;
    return filteredPackages.find((p) => p.id === selectedPackageId) || null;
  }, [filteredPackages, selectedPackageId]);

  // Copy full license text to clipboard
  const handleCopyLicense = useCallback(
    async (text: string) => {
      try {
        await navigator.clipboard.writeText(text);
        setCopyFeedback(true);
        const pkgName = selectedPackage ? selectedPackage.name : "";
        onShowToast("about.TOAST_COPIED", { packageName: pkgName });
        setTimeout(() => {
          setCopyFeedback(false);
        }, LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS);
      } catch {
        console.error("[AboutDialog] Failed to copy license text");
        onShowToast("about.TOAST_COPY_FAILED");
      }
    },
    [onShowToast, selectedPackage]
  );

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm select-none p-4"
      onClick={onClose}
    >
      {/* Modal dialog shell */}
      <div
        className="w-full max-w-4xl h-[620px] bg-[#18181b] border border-zinc-700/80 rounded-xl shadow-2xl flex flex-col overflow-hidden text-zinc-100 animate-in fade-in zoom-in-95 duration-150"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal header bar */}
        <div className="h-14 px-5 bg-[#141416] border-b border-zinc-800 flex items-center justify-between flex-shrink-0">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-excel/20 border border-excel/40 flex items-center justify-center text-excel-light">
              <ShieldCheck className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-base font-semibold text-zinc-100 flex items-center gap-2">
                {t("about.TITLE")}
                <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300 border border-zinc-700">
                  v{t("about.APP_VERSION")}
                </span>
              </h2>
            </div>
          </div>

          {/* Tab switcher */}
          <div className="flex items-center gap-1 bg-[#202024] p-1 rounded-lg border border-zinc-800">
            <button
              type="button"
              onClick={() => setActiveTab("about")}
              className={`px-3 py-1 text-xs font-medium rounded-md transition-all cursor-pointer ${
                activeTab === "about"
                  ? "bg-zinc-800 text-zinc-100 shadow-sm border border-zinc-700/60"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {t("about.TAB_ABOUT")}
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("licenses")}
              className={`px-3 py-1 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 ${
                activeTab === "licenses"
                  ? "bg-zinc-800 text-zinc-100 shadow-sm border border-zinc-700/60"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              <FileCode2 className="w-3.5 h-3.5" />
              <span>{t("about.TAB_LICENSES")}</span>
              <span className="text-[10px] font-mono px-1.5 py-0.2 rounded-full bg-zinc-700/80 text-zinc-300">
                {licenses.length}
              </span>
            </button>
          </div>

          {/* Close button (X) */}
          <button
            type="button"
            onClick={onClose}
            title={t("about.BUTTON_CLOSE")}
            className="p-1.5 text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800 rounded-lg transition cursor-pointer"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Modal content body */}
        <div className="flex-1 overflow-hidden flex flex-col bg-[#161618]">
          {activeTab === "about" ? (
            /* Tab 1: Application General Info */
            <div className="flex-1 overflow-y-auto p-8 flex flex-col items-center justify-center text-center">
              <div className="w-20 h-20 rounded-2xl bg-gradient-to-br from-emerald-500/20 to-excel/10 border border-emerald-500/30 flex items-center justify-center mb-5 shadow-lg shadow-emerald-950/20">
                <ShieldCheck className="w-10 h-10 text-emerald-400" />
              </div>

              <h3 className="text-2xl font-bold text-zinc-100 mb-1">
                {t("about.APP_NAME")}
              </h3>
              <p className="text-xs font-mono text-zinc-400 mb-4 bg-zinc-900/80 px-3 py-1 rounded-full border border-zinc-800">
                Version {t("about.APP_VERSION")}
              </p>

              <p className="text-sm text-zinc-300 max-w-lg mb-6 leading-relaxed">
                {t("about.APP_DESCRIPTION")}
              </p>

              <div className="w-full max-w-md bg-[#1c1c20] border border-zinc-800 rounded-lg p-4 text-xs text-zinc-400 space-y-2 mb-6">
                <div className="flex justify-between items-center py-1 border-b border-zinc-800/80">
                  <span className="text-zinc-500">{t("about.COPYRIGHT_LABEL")}</span>
                  <span className="font-mono text-zinc-300">{t("about.COPYRIGHT")}</span>
                </div>
                <div className="flex justify-between items-center py-1 border-b border-zinc-800/80">
                  <span className="text-zinc-500">{t("about.LICENSE_LABEL")}</span>
                  <span className="font-medium text-emerald-400">{t("about.APP_LICENSE_LABEL")}</span>
                </div>
                <div className="flex justify-between items-center py-1">
                  <span className="text-zinc-500">{t("about.THIRD_PARTY_LABEL")}</span>
                  <span className="font-mono text-zinc-300">
                    {licenses.length}
                    {t("about.PACKAGE_COUNT_SUFFIX")}
                  </span>
                </div>
              </div>

              <button
                type="button"
                onClick={() => setActiveTab("licenses")}
                className="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white rounded-lg border border-zinc-700 text-xs font-medium transition flex items-center gap-2 cursor-pointer"
              >
                <FileCode2 className="w-4 h-4 text-emerald-400" />
                <span>{t("about.PACKAGE_LIST_LINK")}</span>
              </button>
            </div>
          ) : (
            /* Tab 2: Open Source Licenses (Master-Detail) */
            <div className="flex-1 flex overflow-hidden min-h-0">
              <PackageList
                packages={filteredPackages}
                totalPackageCount={licenses.length}
                selectedPackageId={selectedPackageId}
                onSelectPackage={setSelectedPackageId}
                searchKeyword={searchKeyword}
                onSearchChange={setSearchKeyword}
              />
              <PackageDetail
                package={selectedPackage}
                onCopyLicense={handleCopyLicense}
                isCopied={copyFeedback}
              />
            </div>
          )}
        </div>

        {/* Modal footer */}
        <div className="h-12 px-5 bg-[#141416] border-t border-zinc-800 flex items-center justify-end flex-shrink-0">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white text-xs font-medium rounded-lg border border-zinc-700 transition cursor-pointer"
          >
            {t("about.BUTTON_CLOSE")}
          </button>
        </div>
      </div>
    </div>
  );
};

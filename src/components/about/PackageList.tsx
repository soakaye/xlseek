/**
 * @fileoverview Package list component (src/components/about/PackageList.tsx)
 *
 * ## Description
 * Manages the left pane (40% width) in the About dialog "Open Source Licenses" tab.
 * Renders real-time incremental search bar and package rows, displaying package name,
 * version, license type, and origin source badge (Rust/npm).
 * Handles active item highlighting and click selection notifications.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), Principle III (comprehensive documentation), and Principle IV (modular design).
 */

import React from "react";
import { X } from "lucide-react";
import { PackageLicenseRecord } from "../../types/license";
import { useTranslation } from "../../i18n";

export interface PackageListProps {
  packages: PackageLicenseRecord[];
  totalPackageCount: number;
  selectedPackageId: string | null;
  onSelectPackage: (id: string) => void;
  searchKeyword: string;
  onSearchChange: (keyword: string) => void;
}

/**
 * ## Description
 * Package list master pane component.
 *
 * ## Arguments
 * @param props - PackageListProps
 *
 * ## Returns
 * @returns Rendered element
 *
 * ## Errors / Exceptions
 * Displays empty notice when no packages match filter; never throws.
 */
export const PackageList: React.FC<PackageListProps> = ({
  packages,
  totalPackageCount,
  selectedPackageId,
  onSelectPackage,
  searchKeyword,
  onSearchChange,
}) => {
  const t = useTranslation();

  return (
    <div className="w-[40%] border-r border-zinc-800 flex flex-col bg-[#141416] min-w-0">
      {/* Search and package count header */}
      <div className="p-3 border-b border-zinc-800 flex flex-col gap-2 flex-shrink-0 bg-[#161618]">
        <div className="flex items-center justify-between text-xs text-zinc-400">
          <span className="font-medium text-zinc-300">{t("about.TAB_LICENSES")}</span>
          <span className="text-[11px] font-mono text-zinc-500">
            {packages.length} / {totalPackageCount}
          </span>
        </div>
        <div className="relative">
          <input
            type="text"
            value={searchKeyword}
            onChange={(e) => onSearchChange(e.target.value)}
            placeholder={t("about.SEARCH_PLACEHOLDER")}
            className="w-full pl-3 pr-8 py-1.5 bg-[#202024] border border-zinc-700/80 rounded-lg text-xs text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-excel-light transition"
          />
          {searchKeyword && (
            <button
              type="button"
              onClick={() => onSearchChange("")}
              className="absolute right-2 top-2 text-zinc-400 hover:text-zinc-200"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          )}
        </div>
      </div>

      {/* Package rows list */}
      <div className="flex-1 overflow-y-auto divide-y divide-zinc-800/60 min-h-0">
        {packages.length === 0 ? (
          <div className="p-6 text-center text-xs text-zinc-500">
            {t("about.NO_PACKAGES_FOUND")}
          </div>
        ) : (
          packages.map((pkg) => {
            const isSelected = pkg.id === selectedPackageId;
            return (
              <button
                key={pkg.id}
                type="button"
                onClick={() => onSelectPackage(pkg.id)}
                className={`w-full text-left p-3 transition flex flex-col gap-1 cursor-pointer ${
                  isSelected
                    ? "bg-zinc-800/90 border-l-2 border-emerald-500"
                    : "hover:bg-zinc-800/40"
                }`}
              >
                <div className="flex items-center justify-between gap-2">
                  <span className="font-medium text-xs text-zinc-200 truncate" title={pkg.name}>
                    {pkg.name}
                  </span>
                  <span
                    className={`text-[10px] font-mono px-1.5 py-0.2 rounded border flex-shrink-0 ${
                      pkg.source === "rust"
                        ? "bg-amber-950/40 text-amber-300 border-amber-800/60"
                        : "bg-blue-950/40 text-blue-300 border-blue-800/60"
                    }`}
                  >
                    {pkg.source === "rust"
                      ? t("about.SOURCE_RUST_LABEL")
                      : t("about.SOURCE_NPM_LABEL")}
                  </span>
                </div>
                <div className="flex items-center justify-between gap-2 text-[11px] text-zinc-400">
                  <span className="font-mono text-zinc-500">v{pkg.version}</span>
                  <span className="font-mono text-emerald-400/90 truncate" title={pkg.license}>
                    {pkg.license}
                  </span>
                </div>
              </button>
            );
          })
        )}
      </div>
    </div>
  );
};

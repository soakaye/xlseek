/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Third-party licenses and About dialog type definitions (src/types/license.ts)
 *
 * ## Description
 * Defines license record types for open-source software (OSS) used in the application,
 * basic application metadata types, and state management types for the About dialog.
 * Complies with Constitution Principle I (English documentation), Principle III (comprehensive documentation), and Principle IV (modular design).
 *
 * ## Type Definitions
 * - `PackageLicenseRecord`: Individual license record for third-party packages (Rust / npm)
 * - `AppMetaInfo`: Application version, description, and copyright metadata
 * - `AboutTabType`: Active tab in About dialog ("about" | "licenses")
 * - `AboutDialogState`: State management for About dialog UI, selection, and search
 */

/**
 * Individual license information record for a third-party package
 *
 * ## Field Definitions
 * - `id`: Unique identifier (`{name}@{version}`)
 * - `name`: Package name (crate or npm package name)
 * - `version`: Package version string
 * - `source`: Package source origin ("rust": backend crate, "npm": frontend package)
 * - `license`: SPDX license identifier or license name (e.g. "MIT", "Apache-2.0")
 * - `author`: Author or copyright notice (null if not available)
 * - `repository`: Repository or official site URL (null if not available)
 * - `license_text`: Full license text provided by the upstream author
 */
export interface PackageLicenseRecord {
  id: string;
  name: string;
  version: string;
  source: "rust" | "npm";
  license: string;
  author: string | null;
  repository: string | null;
  license_text: string;
}

/**
 * Basic application metadata interface
 *
 * ## Field Definitions
 * - `name`: Application name ("Excel Seek")
 * - `version`: Application version string (e.g. "0.1.0")
 * - `description`: Application description summary
 * - `copyright`: Full application copyright notice string
 * - `license`: Application distribution license ("MIT License")
 */
export interface AppMetaInfo {
  name: string;
  version: string;
  description: string;
  copyright: string;
  license: string;
}

/**
 * Active tab union type for About dialog
 * - `"about"`: App basic info, overview, and copyright
 * - `"licenses"`: Open-source licenses list and detail view
 */
export type AboutTabType = "about" | "licenses";

/**
 * About dialog UI state interface
 *
 * ## Field Definitions
 * - `isOpen`: Dialog modal visibility flag
 * - `activeTab`: Currently active tab
 * - `searchKeyword`: Search filter query for license list
 * - `selectedPackageId`: ID of the package selected for detailed view in the right pane
 * - `copyFeedback`: Temporary visual feedback flag when clipboard copy succeeds
 */
export interface AboutDialogState {
  isOpen: boolean;
  activeTab: AboutTabType;
  searchKeyword: string;
  selectedPackageId: string | null;
  copyFeedback: boolean;
}

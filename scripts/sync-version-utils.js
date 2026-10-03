/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Pure string transformation and file synchronization utilities for keeping
 * repository version configurations in sync across Rust, Tauri, React, and i18n catalogs.
 *
 * ## Arguments & Returns
 * Exports validator functions, string transformer functions, and an asynchronous
 * file synchronization function that accepts the repository root path and target version.
 *
 * ## Errors / Exceptions
 * Propagates TypeError on invalid semver or malformed file content. Propagates Node.js
 * filesystem or child process errors when reading, writing, or executing cargo commands.
 */

import { spawnSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import {
  CARGO_CHECK_SUBCOMMAND,
  CARGO_COMMAND,
  CARGO_PACKAGE_VERSION_PATTERN,
  CARGO_WORKSPACE_FLAG,
  CLI_CARGO_TOML_PATH,
  CONSTANTS_APP_VERSION_PATTERN,
  CONSTANTS_INDEX_PATH,
  CORE_CARGO_TOML_PATH,
  DESIGN_HTML_APP_HEADER_VERSION_PATTERN,
  DESIGN_HTML_APP_MODAL_VERSION_PATTERN,
  DESIGN_INDEX_HTML_PATH,
  INVALID_SEMVER_MESSAGE,
  LOCALE_APP_VERSION_PATTERN,
  LOCALES_CORE_EN_PATH,
  LOCALES_CORE_JA_PATH,
  LOCALES_TAURI_EN_PATH,
  LOCALES_TAURI_JA_PATH,
  PATTERN_NOT_MATCHED_MESSAGE,
  SEMVER_PATTERN,
  TAURI_CARGO_TOML_PATH,
  TAURI_CONF_PATH,
  TAURI_CONF_VERSION_PATTERN,
  UTF8_ENCODING,
  VERSION_PREFIX_PATTERN,
} from "./sync-version-constants.js";

/**
 * Validates and normalizes a semantic version string.
 *
 * ## Description
 * Strips any leading 'v' character and validates that the remaining string conforms
 * to the SemVer 2.0.0 specification pattern.
 *
 * ## Arguments & Returns
 * - `version` {unknown}: Candidate version string or value to validate.
 * - Returns {string}: Normalized semantic version string.
 *
 * ## Errors / Exceptions
 * - Throws {TypeError}: When the input is not a string or fails the SemVer regex.
 */
export function validateSemver(version) {
  // Constant reference: INVALID_SEMVER_MESSAGE.
  if (typeof version !== "string") {
    throw new TypeError(`${INVALID_SEMVER_MESSAGE}: expected string, received ${typeof version}`);
  }
  // Constant reference: VERSION_PREFIX_PATTERN.
  const trimmed = version.trim().replace(VERSION_PREFIX_PATTERN, "");
  // Constant references: SEMVER_PATTERN and INVALID_SEMVER_MESSAGE.
  if (!SEMVER_PATTERN.test(trimmed)) {
    throw new TypeError(`${INVALID_SEMVER_MESSAGE}: "${version}"`);
  }
  return trimmed;
}

/**
 * Updates the version field inside tauri.conf.json content.
 *
 * ## Description
 * Replaces the top-level "version" property in the JSON string with the new version.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of tauri.conf.json.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated tauri.conf.json content.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When the version pattern is not found in the content.
 */
export function updateTauriConf(content, newVersion) {
  // Constant reference: TAURI_CONF_VERSION_PATTERN.
  if (!TAURI_CONF_VERSION_PATTERN.test(content)) {
    // Constant reference: PATTERN_NOT_MATCHED_MESSAGE.
    throw new Error(`${PATTERN_NOT_MATCHED_MESSAGE} (tauri.conf.json version)`);
  }
  return content.replace(TAURI_CONF_VERSION_PATTERN, `$1${newVersion}$2`);
}

/**
 * Updates the package version entry inside a Cargo.toml manifest content.
 *
 * ## Description
 * Replaces the `version = "..."` entry within the `[package]` table with the new version.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of Cargo.toml.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated Cargo.toml content.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When the `[package]` version entry is not found.
 */
export function updateCargoToml(content, newVersion) {
  // Constant reference: CARGO_PACKAGE_VERSION_PATTERN.
  if (!CARGO_PACKAGE_VERSION_PATTERN.test(content)) {
    // Constant reference: PATTERN_NOT_MATCHED_MESSAGE.
    throw new Error(`${PATTERN_NOT_MATCHED_MESSAGE} (Cargo.toml [package] version)`);
  }
  return content.replace(CARGO_PACKAGE_VERSION_PATTERN, `$1${newVersion}$2`);
}

/**
 * Updates the APP_VERSION constant inside src/constants/index.ts content.
 *
 * ## Description
 * Replaces the APP_VERSION property in the APP_CONSTANTS object literal.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of src/constants/index.ts.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated TypeScript source content.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When APP_VERSION is not found in the content.
 */
export function updateConstantsIndex(content, newVersion) {
  // Constant reference: CONSTANTS_APP_VERSION_PATTERN.
  if (!CONSTANTS_APP_VERSION_PATTERN.test(content)) {
    // Constant reference: PATTERN_NOT_MATCHED_MESSAGE.
    throw new Error(`${PATTERN_NOT_MATCHED_MESSAGE} (src/constants/index.ts APP_VERSION)`);
  }
  return content.replace(CONSTANTS_APP_VERSION_PATTERN, `$1${newVersion}$2`);
}

/**
 * Updates the about.APP_VERSION entry inside a locale YAML file content.
 *
 * ## Description
 * Replaces the `about.APP_VERSION: ...` line with the new version string.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of the locale YAML file.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated YAML content.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When `about.APP_VERSION:` is not found in the content.
 */
export function updateLocaleYaml(content, newVersion) {
  // Constant reference: LOCALE_APP_VERSION_PATTERN.
  if (!LOCALE_APP_VERSION_PATTERN.test(content)) {
    // Constant reference: PATTERN_NOT_MATCHED_MESSAGE.
    throw new Error(`${PATTERN_NOT_MATCHED_MESSAGE} (locale YAML about.APP_VERSION)`);
  }
  return content.replace(LOCALE_APP_VERSION_PATTERN, `$1${newVersion}`);
}

/**
 * Updates version mentions inside the standalone UI design mockup HTML.
 *
 * ## Description
 * Replaces version strings in the About dialog mockup section of design/mainui/index.html.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of design/mainui/index.html.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated HTML content.
 *
 * ## Errors / Exceptions
 * - None. Returns unchanged content if no version pattern matches.
 */
export function updateDesignHtml(content, newVersion) {
  // Constant references: DESIGN_HTML_APP_HEADER_VERSION_PATTERN and DESIGN_HTML_APP_MODAL_VERSION_PATTERN.
  return content
    .replace(DESIGN_HTML_APP_HEADER_VERSION_PATTERN, `$1${newVersion}`)
    .replace(DESIGN_HTML_APP_MODAL_VERSION_PATTERN, `$1${newVersion}`);
}

/**
 * Reads, transforms, and synchronizes all project files to a specified version.
 *
 * ## Description
 * Iterates through all designated target files in the repository, validates the target
 * semver, applies the corresponding transformation, writes the updated content, and
 * optionally triggers `cargo check --workspace` to synchronize Cargo.lock.
 *
 * ## Arguments & Returns
 * - `repoRoot` {string}: Absolute path to the repository root directory.
 * - `targetVersion` {string}: Candidate version to validate and apply.
 * - `options` {Object} [options]: Optional configuration.
 *   - `options.dryRun` {boolean}: When true, skips writing files to disk.
 *   - `options.skipCargoLock` {boolean}: When true, skips running `cargo check`.
 * - Returns {Promise<Array<string>>}: List of relative paths for all synchronized files.
 *
 * ## Errors / Exceptions
 * - Throws {TypeError}: When semver validation fails.
 * - Throws {Error}: When any file transformation fails or I/O encounters an error.
 */
export async function syncAllFiles(repoRoot, targetVersion, options = {}) {
  const version = validateSemver(targetVersion);
  const dryRun = Boolean(options.dryRun);
  const skipCargoLock = Boolean(options.skipCargoLock);

  const fileTransforms = [
    // Constant reference: TAURI_CONF_PATH.
    { relativePath: TAURI_CONF_PATH, transform: updateTauriConf },
    // Constant reference: CORE_CARGO_TOML_PATH.
    { relativePath: CORE_CARGO_TOML_PATH, transform: updateCargoToml },
    // Constant reference: CLI_CARGO_TOML_PATH.
    { relativePath: CLI_CARGO_TOML_PATH, transform: updateCargoToml },
    // Constant reference: TAURI_CARGO_TOML_PATH.
    { relativePath: TAURI_CARGO_TOML_PATH, transform: updateCargoToml },
    // Constant reference: CONSTANTS_INDEX_PATH.
    { relativePath: CONSTANTS_INDEX_PATH, transform: updateConstantsIndex },
    // Constant reference: LOCALES_CORE_EN_PATH.
    { relativePath: LOCALES_CORE_EN_PATH, transform: updateLocaleYaml },
    // Constant reference: LOCALES_CORE_JA_PATH.
    { relativePath: LOCALES_CORE_JA_PATH, transform: updateLocaleYaml },
    // Constant reference: LOCALES_TAURI_EN_PATH.
    { relativePath: LOCALES_TAURI_EN_PATH, transform: updateLocaleYaml },
    // Constant reference: LOCALES_TAURI_JA_PATH.
    { relativePath: LOCALES_TAURI_JA_PATH, transform: updateLocaleYaml },
    // Constant reference: DESIGN_INDEX_HTML_PATH.
    { relativePath: DESIGN_INDEX_HTML_PATH, transform: updateDesignHtml },
  ];

  const updatedPaths = [];

  for (const { relativePath, transform } of fileTransforms) {
    const absolutePath = path.resolve(repoRoot, relativePath);
    // Constant reference: UTF8_ENCODING.
    const originalContent = await readFile(absolutePath, UTF8_ENCODING);
    const updatedContent = transform(originalContent, version);

    if (!dryRun && updatedContent !== originalContent) {
      // Constant reference: UTF8_ENCODING.
      await writeFile(absolutePath, updatedContent, UTF8_ENCODING);
    }
    updatedPaths.push(relativePath);
  }

  if (!dryRun && !skipCargoLock) {
    // Constant references: CARGO_COMMAND, CARGO_CHECK_SUBCOMMAND, CARGO_WORKSPACE_FLAG.
    const result = spawnSync(CARGO_COMMAND, [CARGO_CHECK_SUBCOMMAND, CARGO_WORKSPACE_FLAG], {
      cwd: repoRoot,
      stdio: "pipe",
    });
    if (result.status === 0) {
      updatedPaths.push("Cargo.lock");
    }
  }

  return updatedPaths;
}

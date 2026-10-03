#!/usr/bin/env node
/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Executable script invoked during the `npm version` lifecycle hook or directly
 * from the command line to synchronize the version string across all project configuration
 * files (Cargo manifests, Tauri config, frontend constants, i18n catalogs, and mockups).
 *
 * ## Arguments & Returns
 * - Optional command-line argument: `[target-version]` (defaults to `process.env.npm_package_version`
 *   or the version field in `package.json`).
 * - Exits with status code 0 on successful synchronization; nonzero on validation or I/O failure.
 *
 * ## Errors / Exceptions
 * Catches unhandled errors, logs a diagnostic message with error details to stderr, and exits with code 1.
 */

import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  PACKAGE_JSON_PATH,
  SYNC_LOG_PREFIX,
  SYNC_SUCCESS_MESSAGE,
  UTF8_ENCODING,
} from "./sync-version-constants.js";
import { syncAllFiles, validateSemver } from "./sync-version-utils.js";

/**
 * Resolves the target version from CLI args, environment variables, or package.json.
 *
 * ## Description
 * Inspects command line arguments first, then `process.env.npm_package_version` (set by
 * `npm version`), and falls back to reading `package.json`.
 *
 * ## Arguments & Returns
 * - `repoRoot` {string}: Absolute path to the repository root directory.
 * - Returns {Promise<string>}: Validated target version string.
 *
 * ## Errors / Exceptions
 * - Propagates filesystem errors if package.json cannot be read or parsed.
 * - Throws TypeError if the resolved version is not a valid semver.
 */
async function resolveTargetVersion(repoRoot) {
  const cliArg = process.argv[2];
  if (cliArg) {
    return validateSemver(cliArg);
  }

  const envVersion = process.env.npm_package_version;
  if (envVersion) {
    return validateSemver(envVersion);
  }

  // Constant references: PACKAGE_JSON_PATH and UTF8_ENCODING.
  const packageJsonContent = await readFile(path.resolve(repoRoot, PACKAGE_JSON_PATH), UTF8_ENCODING);
  const parsed = JSON.parse(packageJsonContent);
  return validateSemver(parsed.version);
}

/**
 * Main entry point for the version synchronization command.
 *
 * ## Description
 * Resolves repository root, determines target version, executes syncAllFiles, and logs results.
 *
 * ## Arguments & Returns
 * - None.
 *
 * ## Errors / Exceptions
 * - Exits process with code 1 on failure.
 */
async function main() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const repoRoot = path.resolve(scriptDir, "..");

  try {
    const targetVersion = await resolveTargetVersion(repoRoot);
    const updatedFiles = await syncAllFiles(repoRoot, targetVersion);

    // Constant references: SYNC_LOG_PREFIX and SYNC_SUCCESS_MESSAGE.
    console.log(`${SYNC_LOG_PREFIX} ${SYNC_SUCCESS_MESSAGE} v${targetVersion}:`);
    for (const filePath of updatedFiles) {
      console.log(`  ✓ ${filePath}`);
    }
  } catch (error) {
    // Constant reference: SYNC_LOG_PREFIX.
    console.error(`${SYNC_LOG_PREFIX} Failed:`, error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

main();

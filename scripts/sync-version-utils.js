/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Pure string transformation, Cargo execution, and Git staging verification utilities
 * for keeping the repository version configuration in sync across the Rust workspace.
 *
 * ## Arguments & Returns
 * Exports validator functions, string transformer functions, and an asynchronous
 * file synchronization function that accepts the repository root path and target version.
 *
 * ## Errors / Exceptions
 * Propagates TypeError on invalid semver or malformed file content. Propagates Node.js
 * filesystem or child process errors when reading, writing, or executing Cargo and Git commands.
 */

import { spawnSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import {
  CARGO_CHECK_SUBCOMMAND,
  CARGO_COMMAND,
  CARGO_LOCK_PATH,
  CARGO_LOCK_VERIFY_FAILED_MESSAGE,
  CARGO_WORKSPACE_FLAG,
  CARGO_WORKSPACE_PACKAGE_VERSION_PATTERN,
  GIT_ADD_SUBCOMMAND,
  GIT_CACHED_FLAG,
  GIT_COMMAND,
  GIT_DIFF_SUBCOMMAND,
  GIT_NAME_ONLY_FLAG,
  GIT_STAGE_VERIFY_FAILED_MESSAGE,
  INVALID_SEMVER_MESSAGE,
  PATTERN_NOT_MATCHED_MESSAGE,
  ROOT_CARGO_TOML_PATH,
  SEMVER_PATTERN,
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
 * Updates the workspace package version entry inside the root Cargo.toml manifest content.
 *
 * ## Description
 * Replaces the `version = "..."` entry within the `[workspace.package]` table with the new version.
 *
 * ## Arguments & Returns
 * - `content` {string}: Existing content of the root Cargo.toml.
 * - `newVersion` {string}: Validated semantic version string.
 * - Returns {string}: Updated Cargo.toml content.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When the `[workspace.package]` version entry is not found.
 */
export function updateWorkspaceCargoToml(content, newVersion) {
  // Constant reference: CARGO_WORKSPACE_PACKAGE_VERSION_PATTERN.
  if (!CARGO_WORKSPACE_PACKAGE_VERSION_PATTERN.test(content)) {
    // Constant reference: PATTERN_NOT_MATCHED_MESSAGE.
    throw new Error(`${PATTERN_NOT_MATCHED_MESSAGE} (Cargo.toml [workspace.package] version)`);
  }
  return content.replace(CARGO_WORKSPACE_PACKAGE_VERSION_PATTERN, `$1${newVersion}$2`);
}

/**
 * Verifies that Cargo.lock reflects the target version for workspace crates.
 *
 * ## Description
 * Reads Cargo.lock from disk and verifies that the target version is present for member packages.
 *
 * ## Arguments & Returns
 * - `repoRoot` {string}: Absolute path to repository root.
 * - `targetVersion` {string}: Target semantic version expected in Cargo.lock.
 * - Returns {Promise<boolean>}: Resolves true if verified.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When Cargo.lock cannot be read or does not contain the target version.
 */
export async function verifyCargoLock(repoRoot, targetVersion) {
  // Constant references: CARGO_LOCK_PATH and UTF8_ENCODING.
  const lockFilePath = path.resolve(repoRoot, CARGO_LOCK_PATH);
  const content = await readFile(lockFilePath, UTF8_ENCODING);
  const expectedLine = `version = "${targetVersion}"`;
  if (!content.includes(expectedLine)) {
    // Constant reference: CARGO_LOCK_VERIFY_FAILED_MESSAGE.
    throw new Error(`${CARGO_LOCK_VERIFY_FAILED_MESSAGE}: expected '${expectedLine}' in ${CARGO_LOCK_PATH}`);
  }
  return true;
}

/**
 * Verifies that required files are successfully staged in the Git index with no unstaged modifications.
 *
 * ## Description
 * Executes `git diff --cached --name-only` to ensure required paths are staged, and checks
 * `git diff <path>` to guarantee there are no remaining unstaged changes.
 *
 * ## Arguments & Returns
 * - `repoRoot` {string}: Absolute path to repository root.
 * - `requiredPaths` {Array<string>}: List of relative paths required to be cleanly staged.
 * - Returns {boolean}: Returns true if all files are staged cleanly.
 *
 * ## Errors / Exceptions
 * - Throws {Error}: When Git diff commands fail or a required file is missing from index or has unstaged diff.
 */
export function verifyGitStaged(repoRoot, requiredPaths) {
  // Constant references: GIT_COMMAND, GIT_DIFF_SUBCOMMAND, GIT_CACHED_FLAG, GIT_NAME_ONLY_FLAG, UTF8_ENCODING.
  const cachedResult = spawnSync(GIT_COMMAND, [GIT_DIFF_SUBCOMMAND, GIT_CACHED_FLAG, GIT_NAME_ONLY_FLAG], {
    cwd: repoRoot,
    encoding: UTF8_ENCODING,
  });

  if (cachedResult.status !== 0) {
    // Constant reference: GIT_STAGE_VERIFY_FAILED_MESSAGE.
    throw new Error(`${GIT_STAGE_VERIFY_FAILED_MESSAGE}: git diff --cached exited with status ${cachedResult.status}`);
  }

  const stagedFiles = cachedResult.stdout.split(/\r?\n/).map((s) => s.trim()).filter(Boolean);

  for (const relPath of requiredPaths) {
    if (!stagedFiles.includes(relPath)) {
      // Constant reference: GIT_STAGE_VERIFY_FAILED_MESSAGE.
      throw new Error(`${GIT_STAGE_VERIFY_FAILED_MESSAGE}: ${relPath} is not staged in git index`);
    }

    // Check unstaged diff for this file
    const unstagedResult = spawnSync(GIT_COMMAND, [GIT_DIFF_SUBCOMMAND, relPath], {
      cwd: repoRoot,
      encoding: UTF8_ENCODING,
    });
    if (unstagedResult.status === 0 && unstagedResult.stdout.trim().length > 0) {
      // Constant reference: GIT_STAGE_VERIFY_FAILED_MESSAGE.
      throw new Error(`${GIT_STAGE_VERIFY_FAILED_MESSAGE}: ${relPath} has unstaged modifications after staging`);
    }
  }

  return true;
}

/**
 * Synchronizes minimal version manifests across the repository.
 *
 * ## Description
 * Validates the target SemVer, updates the root Cargo.toml workspace package version,
 * runs `cargo check --workspace` to update Cargo.lock, verifies the lockfile content,
 * stages both Cargo.toml and Cargo.lock, and verifies that they are cleanly staged in Git.
 *
 * ## Arguments & Returns
 * - `repoRoot` {string}: Absolute path to the repository root directory.
 * - `targetVersion` {string}: Candidate version to validate and apply.
 * - `options` {Object} [options]: Optional configuration.
 *   - `options.dryRun` {boolean}: When true, skips writing files to disk and Git operations.
 *   - `options.skipCargoLock` {boolean}: When true, skips running `cargo check` and lockfile operations.
 * - Returns {Promise<Array<string>>}: List of relative paths for all synchronized files.
 *
 * ## Errors / Exceptions
 * - Throws {TypeError}: When semver validation fails.
 * - Throws {Error}: When file transformation, Cargo execution, or Git staging encounters an error.
 */
export async function syncAllFiles(repoRoot, targetVersion, options = {}) {
  const version = validateSemver(targetVersion);
  const dryRun = Boolean(options.dryRun);
  const skipCargoLock = Boolean(options.skipCargoLock);

  // Constant references: ROOT_CARGO_TOML_PATH and UTF8_ENCODING.
  const rootCargoPath = path.resolve(repoRoot, ROOT_CARGO_TOML_PATH);
  const originalContent = await readFile(rootCargoPath, UTF8_ENCODING);
  const updatedContent = updateWorkspaceCargoToml(originalContent, version);

  if (!dryRun && updatedContent !== originalContent) {
    await writeFile(rootCargoPath, updatedContent, UTF8_ENCODING);
  }

  const updatedPaths = [ROOT_CARGO_TOML_PATH];

  if (!dryRun && !skipCargoLock) {
    // Constant references: CARGO_COMMAND, CARGO_CHECK_SUBCOMMAND, CARGO_WORKSPACE_FLAG.
    const cargoResult = spawnSync(CARGO_COMMAND, [CARGO_CHECK_SUBCOMMAND, CARGO_WORKSPACE_FLAG], {
      cwd: repoRoot,
      stdio: "pipe",
    });

    if (cargoResult.status !== 0) {
      // Constant reference: CARGO_LOCK_VERIFY_FAILED_MESSAGE.
      throw new Error(`${CARGO_LOCK_VERIFY_FAILED_MESSAGE}: cargo check --workspace failed with status ${cargoResult.status}`);
    }

    // Verify Cargo.lock contains the target version
    await verifyCargoLock(repoRoot, version);
    // Constant reference: CARGO_LOCK_PATH.
    updatedPaths.push(CARGO_LOCK_PATH);

    // Stage both Cargo.toml and Cargo.lock
    // Constant references: GIT_COMMAND and GIT_ADD_SUBCOMMAND.
    const addResult = spawnSync(GIT_COMMAND, [GIT_ADD_SUBCOMMAND, ...updatedPaths], {
      cwd: repoRoot,
      stdio: "pipe",
    });

    if (addResult.status !== 0) {
      // Constant reference: GIT_STAGE_VERIFY_FAILED_MESSAGE.
      throw new Error(`${GIT_STAGE_VERIFY_FAILED_MESSAGE}: git add failed with status ${addResult.status}`);
    }

    // Verify Git staging index
    verifyGitStaged(repoRoot, updatedPaths);
  }

  return updatedPaths;
}

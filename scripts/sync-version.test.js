/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Unit and integration tests for minimal version synchronization utilities and CLI entry point.
 * Verifies semver validation, root Cargo.toml workspace transformation, lockfile verification,
 * and workspace synchronization in isolated temporary workspaces.
 *
 * ## Arguments & Returns
 * None. Executed via `node --test scripts/sync-version.test.js`.
 *
 * ## Errors / Exceptions
 * Assertion errors on contract failure. Cleanly removes temporary directories even on failure.
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  ALL_VERSION_TARGET_PATHS,
  ROOT_CARGO_TOML_PATH,
  UTF8_ENCODING,
} from "./sync-version-constants.js";
import {
  syncAllFiles,
  updateWorkspaceCargoToml,
  validateSemver,
} from "./sync-version-utils.js";

const SCRIPT_PATH = fileURLToPath(new URL("./sync-version.js", import.meta.url));

test("validateSemver: accepts valid semver and strips leading 'v'", () => {
  assert.equal(validateSemver("1.0.0"), "1.0.0");
  assert.equal(validateSemver("v1.2.3"), "1.2.3");
  assert.equal(validateSemver("  v2.0.1  "), "2.0.1");
  assert.equal(validateSemver("1.0.0-beta.1"), "1.0.0-beta.1");
  assert.equal(validateSemver("0.1.0"), "0.1.0");
});

test("validateSemver: rejects non-string and malformed inputs", () => {
  assert.throws(() => validateSemver(null), TypeError);
  assert.throws(() => validateSemver(undefined), TypeError);
  assert.throws(() => validateSemver(123), TypeError);
  assert.throws(() => validateSemver(""), TypeError);
  assert.throws(() => validateSemver("not-a-semver"), TypeError);
  assert.throws(() => validateSemver("1.0"), TypeError);
  assert.throws(() => validateSemver("1.0.0.0"), TypeError);
});

test("updateWorkspaceCargoToml: replaces workspace package version in Cargo.toml string", () => {
  const input = '[workspace]\nmembers = ["crates/core"]\n\n[workspace.package]\nversion = "0.1.0"\nauthors = ["soakaye"]\n';
  const expected = '[workspace]\nmembers = ["crates/core"]\n\n[workspace.package]\nversion = "1.2.3"\nauthors = ["soakaye"]\n';
  assert.equal(updateWorkspaceCargoToml(input, "1.2.3"), expected);
});

test("updateWorkspaceCargoToml: throws when workspace package version pattern is not matched", () => {
  const inputWithoutWorkspacePackage = '[workspace]\nmembers = ["crates/core"]\n';
  assert.throws(() => updateWorkspaceCargoToml(inputWithoutWorkspacePackage, "1.2.3"), /Failed to match/);

  const inputWithOldPackageOnly = '[package]\nname = "foo"\nversion = "0.1.0"\n';
  assert.throws(() => updateWorkspaceCargoToml(inputWithOldPackageOnly, "1.2.3"), /Failed to match/);
});

test("syncAllFiles: updates root Cargo.toml in a temporary workspace", async () => {
  const tempDir = await mkdtemp(path.join(tmpdir(), "xlseek-sync-test-"));
  try {
    const rootCargoPath = path.join(tempDir, ROOT_CARGO_TOML_PATH);
    const mockContent = '[workspace]\nmembers = ["crates/core"]\n\n[workspace.package]\nversion = "0.1.0"\nauthors = ["soakaye"]\n';
    await writeFile(rootCargoPath, mockContent, UTF8_ENCODING);

    // Test dry run
    const dryRunResult = await syncAllFiles(tempDir, "1.2.3", { dryRun: true, skipCargoLock: true });
    assert.equal(dryRunResult.length, ALL_VERSION_TARGET_PATHS.length);
    const unchangedCargo = await readFile(rootCargoPath, UTF8_ENCODING);
    assert.match(unchangedCargo, /version = "0.1.0"/);

    // Test actual synchronization without cargo/git execution
    const syncResult = await syncAllFiles(tempDir, "1.2.3", { dryRun: false, skipCargoLock: true });
    assert.equal(syncResult.length, ALL_VERSION_TARGET_PATHS.length);

    const updatedCargo = await readFile(rootCargoPath, UTF8_ENCODING);
    assert.match(updatedCargo, /version = "1.2.3"/);
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
});

test("updateWorkspaceCargoToml: preserves header comments and existing metadata", () => {
  const input = '# Workspace manifest for xlseek\n# Copyright (c) 2026 soakaye\n\n[workspace]\nmembers = ["crates/core"]\n\n[workspace.package]\nversion = "0.1.0"\nauthors = ["soakaye"]\nedition = "2021"\n';
  const expected = '# Workspace manifest for xlseek\n# Copyright (c) 2026 soakaye\n\n[workspace]\nmembers = ["crates/core"]\n\n[workspace.package]\nversion = "1.2.3"\nauthors = ["soakaye"]\nedition = "2021"\n';
  assert.equal(updateWorkspaceCargoToml(input, "1.2.3"), expected);
});

test("sync-version CLI: fails with nonzero status on invalid version argument", () => {
  const result = spawnSync("node", [SCRIPT_PATH, "invalid-version-string"], {
    encoding: "utf8",
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Invalid semver/);
});

/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Unit and integration tests for version synchronization utilities and CLI entry point.
 * Verifies semver validation, string transformation regexes across all file formats,
 * and workspace file synchronization in isolated temporary workspaces.
 *
 * ## Arguments & Returns
 * None. Executed via `node --test scripts/sync-version.test.js`.
 *
 * ## Errors / Exceptions
 * Assertion errors on contract failure. Cleanly removes temporary directories even on failure.
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  ALL_VERSION_TARGET_PATHS,
  CLI_CARGO_TOML_PATH,
  CONSTANTS_INDEX_PATH,
  CORE_CARGO_TOML_PATH,
  DESIGN_INDEX_HTML_PATH,
  LOCALES_CORE_EN_PATH,
  LOCALES_CORE_JA_PATH,
  LOCALES_TAURI_EN_PATH,
  LOCALES_TAURI_JA_PATH,
  TAURI_CARGO_TOML_PATH,
  TAURI_CONF_PATH,
  UTF8_ENCODING,
} from "./sync-version-constants.js";
import {
  syncAllFiles,
  updateCargoToml,
  updateConstantsIndex,
  updateDesignHtml,
  updateLocaleYaml,
  updateTauriConf,
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

test("updateTauriConf: replaces version property in JSON string", () => {
  const input = '{\n  "productName": "xlseek",\n  "version": "0.1.0",\n  "identifier": "com.xlseek.desktop"\n}';
  const expected = '{\n  "productName": "xlseek",\n  "version": "1.2.3",\n  "identifier": "com.xlseek.desktop"\n}';
  assert.equal(updateTauriConf(input, "1.2.3"), expected);
  assert.throws(() => updateTauriConf('{"name": "foo"}', "1.2.3"), /Failed to match/);
});

test("updateCargoToml: replaces package version in Cargo.toml string", () => {
  const input = '[package]\nname = "xlseek-core"\nversion = "0.1.0"\ndescription = "Core"\n';
  const expected = '[package]\nname = "xlseek-core"\nversion = "1.2.3"\ndescription = "Core"\n';
  assert.equal(updateCargoToml(input, "1.2.3"), expected);
  assert.throws(() => updateCargoToml('[dependencies]\nversion = "0.1.0"\n', "1.2.3"), /Failed to match/);
});

test("updateConstantsIndex: replaces APP_VERSION in TypeScript constants", () => {
  const input = 'export const APP_CONSTANTS = { ROOT_ELEMENT_ID: "root", APP_VERSION: "0.1.0" } as const;';
  const expected = 'export const APP_CONSTANTS = { ROOT_ELEMENT_ID: "root", APP_VERSION: "1.2.3" } as const;';
  assert.equal(updateConstantsIndex(input, "1.2.3"), expected);
  assert.throws(() => updateConstantsIndex('export const X = 1;', "1.2.3"), /Failed to match/);
});

test("updateLocaleYaml: replaces about.APP_VERSION in YAML files", () => {
  const input = 'ui.TITLE: "Title"\nabout.APP_VERSION: 0.1.0\nui.OTHER: "Other"';
  const expected = 'ui.TITLE: "Title"\nabout.APP_VERSION: 1.2.3\nui.OTHER: "Other"';
  assert.equal(updateLocaleYaml(input, "1.2.3"), expected);
  assert.throws(() => updateLocaleYaml('ui.TITLE: "Title"', "1.2.3"), /Failed to match/);
});

test("updateDesignHtml: replaces mockup version tags while preserving third-party versions", () => {
  const input = 'アプリについて\n<span class="font-mono">v0.1.0</span>\nExcel Seek\n<p>Version 0.1.0</p>\ncalamine v0.26.1';
  const expected = 'アプリについて\n<span class="font-mono">v1.2.3</span>\nExcel Seek\n<p>Version 1.2.3</p>\ncalamine v0.26.1';
  assert.equal(updateDesignHtml(input, "1.2.3"), expected);
});

test("syncAllFiles: updates all project files in a temporary workspace", async () => {
  const tempDir = await mkdtemp(path.join(tmpdir(), "xlseek-sync-test-"));
  try {
    // Create directory structure
    for (const relPath of ALL_VERSION_TARGET_PATHS) {
      await mkdir(path.dirname(path.join(tempDir, relPath)), { recursive: true });
    }

    // Populate mock files
    await writeFile(path.join(tempDir, TAURI_CONF_PATH), '{"version": "0.1.0"}', UTF8_ENCODING);
    await writeFile(path.join(tempDir, CORE_CARGO_TOML_PATH), '[package]\nversion = "0.1.0"\n', UTF8_ENCODING);
    await writeFile(path.join(tempDir, CLI_CARGO_TOML_PATH), '[package]\nversion = "0.1.0"\n', UTF8_ENCODING);
    await writeFile(path.join(tempDir, TAURI_CARGO_TOML_PATH), '[package]\nversion = "0.1.0"\n', UTF8_ENCODING);
    await writeFile(path.join(tempDir, CONSTANTS_INDEX_PATH), 'APP_VERSION: "0.1.0"', UTF8_ENCODING);
    await writeFile(path.join(tempDir, LOCALES_CORE_EN_PATH), "about.APP_VERSION: 0.1.0", UTF8_ENCODING);
    await writeFile(path.join(tempDir, LOCALES_CORE_JA_PATH), "about.APP_VERSION: 0.1.0", UTF8_ENCODING);
    await writeFile(path.join(tempDir, LOCALES_TAURI_EN_PATH), "about.APP_VERSION: 0.1.0", UTF8_ENCODING);
    await writeFile(path.join(tempDir, LOCALES_TAURI_JA_PATH), "about.APP_VERSION: 0.1.0", UTF8_ENCODING);
    await writeFile(path.join(tempDir, DESIGN_INDEX_HTML_PATH), "アプリについて\n<span>v0.1.0</span>\nExcel Seek\n<p>Version 0.1.0</p>", UTF8_ENCODING);

    // Test dry run
    const dryRunResult = await syncAllFiles(tempDir, "1.2.3", { dryRun: true, skipCargoLock: true });
    assert.equal(dryRunResult.length, ALL_VERSION_TARGET_PATHS.length);
    const unchangedTauriConf = await readFile(path.join(tempDir, TAURI_CONF_PATH), UTF8_ENCODING);
    assert.match(unchangedTauriConf, /"version": "0.1.0"/);

    // Test actual synchronization
    const syncResult = await syncAllFiles(tempDir, "1.2.3", { dryRun: false, skipCargoLock: true });
    assert.equal(syncResult.length, ALL_VERSION_TARGET_PATHS.length);

    // Verify all files were updated to 1.2.3
    const updatedTauriConf = await readFile(path.join(tempDir, TAURI_CONF_PATH), UTF8_ENCODING);
    assert.match(updatedTauriConf, /"version": "1.2.3"/);

    const updatedCargo = await readFile(path.join(tempDir, CORE_CARGO_TOML_PATH), UTF8_ENCODING);
    assert.match(updatedCargo, /version = "1.2.3"/);

    const updatedConstants = await readFile(path.join(tempDir, CONSTANTS_INDEX_PATH), UTF8_ENCODING);
    assert.match(updatedConstants, /APP_VERSION: "1.2.3"/);

    const updatedEnLocale = await readFile(path.join(tempDir, LOCALES_CORE_EN_PATH), UTF8_ENCODING);
    assert.match(updatedEnLocale, /about\.APP_VERSION: 1\.2\.3/);

    const updatedDesign = await readFile(path.join(tempDir, DESIGN_INDEX_HTML_PATH), UTF8_ENCODING);
    assert.equal(updatedDesign, "アプリについて\n<span>v1.2.3</span>\nExcel Seek\n<p>Version 1.2.3</p>");
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
});

test("sync-version CLI: fails with nonzero status on invalid version argument", () => {
  const result = spawnSync("node", [SCRIPT_PATH, "invalid-version-string"], {
    encoding: "utf8",
  });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Invalid semver/);
});

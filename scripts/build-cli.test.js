/**
 * ## Description
 * Verifies target selection, executable and artifact paths, and the CLI build
 * hook's failure when Cargo does not produce the requested target artifact.
 *
 * ## Arguments & Returns
 * Takes no command-line arguments. Node's test runner reports test results and
 * returns a nonzero process status when an assertion fails.
 *
 * ## Errors / Exceptions
 * Assertions fail for contract violations; temporary workspace setup and child
 * process failures are surfaced by the test runner after cleanup.
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtemp, mkdir, rm, writeFile, chmod } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  CLI_BINARY_NAME,
  GENERATED_BINARIES_DIRECTORY,
  TAURI_TARGET_TRIPLE_ENV_VAR,
} from "./build-constants.js";

// Test fixtures: real supported Rust triples and deliberately invalid inputs.
const WINDOWS_TARGET = "x86_64-pc-windows-msvc";
const MAC_TARGET = "aarch64-apple-darwin";
const LINUX_TARGET = "x86_64-unknown-linux-gnu";
const INVALID_TARGET = "invalid triple";
const TEST_WORKSPACE_PREFIX = "xlseek-sidecar-test-";
const BUILD_SCRIPT_PATH = fileURLToPath(new URL("./build-cli.js", import.meta.url));

/**
 * Loads the production target and path helpers after selecting a focused test.
 * Takes no arguments; returns a Promise of the helper module. Import failures
 * reject the Promise and fail the requesting test.
 */
async function loadBuildHelpers() {
  return import("./build-cli-utils.js");
}

/**
 * Runs a callback in a disposable workspace with a successful fake Cargo.
 * Takes an async callback receiving the absolute workspace root and fake Cargo
 * directory; returns its value. Setup/callback errors propagate, and the
 * workspace is removed in a finally block even when an assertion fails.
 */
async function withFakeCargoWorkspace(callback) {
  const workspaceRoot = await mkdtemp(path.join(tmpdir(), TEST_WORKSPACE_PREFIX));
  try {
    const fakeCargoDirectory = path.join(workspaceRoot, "bin");
    await mkdir(fakeCargoDirectory);
    if (process.platform === "win32") {
      await writeFile(
        path.join(fakeCargoDirectory, "cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
      );
    } else {
      const fakeCargoPath = path.join(fakeCargoDirectory, "cargo");
      await writeFile(fakeCargoPath, "#!/bin/sh\nexit 0\n");
      await chmod(fakeCargoPath, 0o755);
    }
    return await callback(workspaceRoot, fakeCargoDirectory);
  } finally {
    await rm(workspaceRoot, { recursive: true, force: true });
  }
}

/**
 * Verifies that the Tauri build target takes precedence over a supplied host.
 * Takes no arguments, returns a Promise<void>, and fails if the helper ignores
 * the Tauri environment variable.
 */
test("target triple: Tauri environment takes precedence", async () => {
  const { resolveTargetTriple } = await loadBuildHelpers();
  // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
  assert.equal(
    resolveTargetTriple({ [TAURI_TARGET_TRIPLE_ENV_VAR]: WINDOWS_TARGET }, LINUX_TARGET),
    WINDOWS_TARGET,
  );
});

/**
 * Verifies that an absent or empty Tauri target uses the supplied host triple.
 * Takes no arguments, returns a Promise<void>, and fails on incorrect fallback.
 */
test("target triple: absent or empty Tauri value uses host fallback", async () => {
  const { resolveTargetTriple } = await loadBuildHelpers();
  assert.equal(resolveTargetTriple({}, MAC_TARGET), MAC_TARGET);
  // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
  assert.equal(resolveTargetTriple({ [TAURI_TARGET_TRIPLE_ENV_VAR]: "" }, LINUX_TARGET), LINUX_TARGET);
});

/**
 * Verifies empty and malformed target triples are rejected before path creation.
 * Takes no arguments, returns a Promise<void>, and fails if invalid input passes.
 */
test("target triple: empty and malformed inputs fail", async () => {
  const { resolveTargetTriple } = await loadBuildHelpers();
  assert.throws(() => resolveTargetTriple({}, ""));
  // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
  assert.throws(() => resolveTargetTriple({ [TAURI_TARGET_TRIPLE_ENV_VAR]: INVALID_TARGET }, LINUX_TARGET));
  assert.throws(() => resolveTargetTriple({ [TAURI_TARGET_TRIPLE_ENV_VAR]: 0 }, LINUX_TARGET), TypeError);
  assert.throws(() => resolveTargetTriple({ [TAURI_TARGET_TRIPLE_ENV_VAR]: undefined }, LINUX_TARGET), TypeError);
});

/**
 * Verifies Windows executable naming and extensionless Unix naming.
 * Takes no arguments, returns a Promise<void>, and fails on incorrect names.
 */
test("executable name: Windows gets .exe", async () => {
  const { getExecutableName } = await loadBuildHelpers();
  // Constant reference: CLI_BINARY_NAME.
  assert.equal(getExecutableName(WINDOWS_TARGET), `${CLI_BINARY_NAME}.exe`);
  assert.equal(getExecutableName(MAC_TARGET), CLI_BINARY_NAME);
  assert.equal(getExecutableName(LINUX_TARGET), CLI_BINARY_NAME);
});

/**
 * Verifies target-specific Cargo release and Tauri Sidecar staging paths.
 * Takes no arguments, returns a Promise<void>, and fails if paths omit the
 * triple, release directory, Windows extension, or staging directory.
 */
test("artifact paths: Cargo and Sidecar use the selected target", async () => {
  const { getCargoArtifactPath, getSidecarPath } = await loadBuildHelpers();
  const repositoryRoot = path.resolve("fixture-repository");
  // Constant references: CLI_BINARY_NAME and GENERATED_BINARIES_DIRECTORY.
  assert.equal(
    getCargoArtifactPath(WINDOWS_TARGET, repositoryRoot),
    path.join(repositoryRoot, "target", WINDOWS_TARGET, "release", `${CLI_BINARY_NAME}.exe`),
  );
  assert.equal(
    getSidecarPath(WINDOWS_TARGET, repositoryRoot),
    path.join(repositoryRoot, GENERATED_BINARIES_DIRECTORY, `${CLI_BINARY_NAME}-${WINDOWS_TARGET}.exe`),
  );
  assert.equal(
    getCargoArtifactPath(LINUX_TARGET, repositoryRoot),
    path.join(repositoryRoot, "target", LINUX_TARGET, "release", CLI_BINARY_NAME),
  );
  assert.equal(
    getSidecarPath(MAC_TARGET, repositoryRoot),
    path.join(repositoryRoot, GENERATED_BINARIES_DIRECTORY, `${CLI_BINARY_NAME}-${MAC_TARGET}`),
  );
});

/**
 * Verifies the build hook reports a missing target-specific Cargo artifact.
 * Takes no arguments, returns a Promise<void>, and fails if the hook reports
 * success or omits the target-aware artifact path from its error diagnostics.
 */
test("missing source artifact: build hook fails with target path", async () => {
  await withFakeCargoWorkspace(async (workspaceRoot, fakeCargoDirectory) => {
    const pathKey = Object.keys(process.env).find((key) => key.toLowerCase() === "path") ?? "PATH";
    const buildEnvironment = {
      ...process.env,
      // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
      [TAURI_TARGET_TRIPLE_ENV_VAR]: WINDOWS_TARGET,
      [pathKey]: `${fakeCargoDirectory}${path.delimiter}${process.env[pathKey] ?? ""}`,
    };
    const result = spawnSync(process.execPath, [BUILD_SCRIPT_PATH], {
      cwd: workspaceRoot,
      env: buildEnvironment,
      encoding: "utf8",
    });

    assert.equal(result.status, 1, result.error?.message ?? result.stdout);
    const diagnostics = `${result.stdout}\n${result.stderr}`;
    assert.match(diagnostics, /Build artifact not found/i);
    assert.match(diagnostics, /x86_64-pc-windows-msvc/);
    assert.match(diagnostics, /target[\\/]x86_64-pc-windows-msvc[\\/]release/);
  });
});

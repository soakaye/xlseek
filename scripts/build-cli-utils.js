/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Resolves the build target and derives target-specific Cargo and Tauri
 * Sidecar artifact names and paths without performing I/O.
 *
 * ## Arguments & Returns
 * This module exports pure functions accepting environment data, Rust target
 * triples, and repository paths; each returns a validated triple or string.
 *
 * ## Errors / Exceptions
 * Invalid or absent target triples throw TypeError. Path helpers may also
 * propagate TypeError from node:path when the repository root is not a string.
 */

import path from "node:path";
import {
  CARGO_RELEASE_DIRECTORY_NAME,
  CARGO_TARGET_DIRECTORY_NAME,
  CLI_BINARY_NAME,
  GENERATED_BINARIES_DIRECTORY,
  INVALID_TARGET_TRIPLE_MESSAGE,
  SIDECAR_TARGET_SEPARATOR,
  TARGET_TRIPLE_PATTERN,
  TAURI_TARGET_TRIPLE_ENV_VAR,
  WINDOWS_EXECUTABLE_EXTENSION,
  WINDOWS_TARGET_OS_TOKEN,
} from "./build-constants.js";

/**
 * Checks that a value is a nonempty, path-safe Rust target triple.
 * Takes an unknown value and returns the validated string. Throws TypeError
 * for absent, non-string, or malformed values.
 */
function validateTargetTriple(value) {
  // Constant references: TARGET_TRIPLE_PATTERN and INVALID_TARGET_TRIPLE_MESSAGE.
  if (typeof value !== "string" || !TARGET_TRIPLE_PATTERN.test(value)) {
    throw new TypeError(`${INVALID_TARGET_TRIPLE_MESSAGE}: ${String(value)}`);
  }
  return value;
}

/**
 * Selects Tauri's explicit target when present and nonempty, otherwise the host target.
 * Takes an environment object and a host triple string; returns the validated
 * target string. Throws TypeError if an explicit or fallback target is malformed
 * or absent.
 */
export function resolveTargetTriple(environment, hostTriple) {
  // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
  const hasExplicitTarget = Object.hasOwn(environment ?? {}, TAURI_TARGET_TRIPLE_ENV_VAR);
  const explicitTarget = environment?.[TAURI_TARGET_TRIPLE_ENV_VAR];
  return validateTargetTriple(!hasExplicitTarget || explicitTarget === "" ? hostTriple : explicitTarget);
}

/**
 * Derives the executable filename for a Rust target triple.
 * Takes a target triple string and returns the CLI filename. Throws TypeError
 * if the target is malformed or absent.
 */
export function getExecutableName(targetTriple) {
  const validTarget = validateTargetTriple(targetTriple);
  // Constant references: SIDECAR_TARGET_SEPARATOR, WINDOWS_TARGET_OS_TOKEN, WINDOWS_EXECUTABLE_EXTENSION, and CLI_BINARY_NAME.
  const isWindows = validTarget.split(SIDECAR_TARGET_SEPARATOR).includes(WINDOWS_TARGET_OS_TOKEN);
  return `${CLI_BINARY_NAME}${isWindows ? WINDOWS_EXECUTABLE_EXTENSION : ""}`;
}

/**
 * Derives the Cargo release artifact path for the selected target.
 * Takes a target triple string and repository root path string; returns an
 * artifact path string. Throws TypeError for malformed targets or invalid roots.
 */
export function getCargoArtifactPath(targetTriple, repositoryRoot) {
  // Constant references: CARGO_TARGET_DIRECTORY_NAME and CARGO_RELEASE_DIRECTORY_NAME.
  return path.join(
    repositoryRoot,
    CARGO_TARGET_DIRECTORY_NAME,
    validateTargetTriple(targetTriple),
    CARGO_RELEASE_DIRECTORY_NAME,
    getExecutableName(targetTriple),
  );
}

/**
 * Derives the target-suffixed Tauri Sidecar staging path.
 * Takes a target triple string and repository root path string; returns a
 * staging path string. Throws TypeError for malformed targets or invalid roots.
 */
export function getSidecarPath(targetTriple, repositoryRoot) {
  const validTarget = validateTargetTriple(targetTriple);
  const { name, ext } = path.parse(getExecutableName(validTarget));
  // Constant references: GENERATED_BINARIES_DIRECTORY and SIDECAR_TARGET_SEPARATOR.
  return path.join(
    repositoryRoot,
    GENERATED_BINARIES_DIRECTORY,
    `${name}${SIDECAR_TARGET_SEPARATOR}${validTarget}${ext}`,
  );
}

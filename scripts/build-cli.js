/**
 * ## Description
 * Builds xlseek-cli for Tauri's selected Rust target (or the local Rust host)
 * and stages its target-suffixed executable for Tauri Sidecar bundling.
 *
 * ## Arguments & Returns
 * Takes no CLI arguments. Reads TAURI_ENV_TARGET_TRIPLE when provided and
 * returns process exit status 0 after staging, or 1 after a failure.
 *
 * ## Errors / Exceptions
 * Reports invalid or unavailable target metadata, Cargo failure, missing
 * target-specific output, and directory or copy failures with artifact paths
 * when the target has been resolved.
 */

import { execFileSync, execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import {
  BUILD_ARTIFACT_MISSING_MESSAGE,
  BUILD_ERROR_MESSAGE,
  BUILD_LOG_PREFIX,
  BUILD_SOURCE_PATH_LABEL,
  BUILD_STAGE_PATH_LABEL,
  BUILD_START_MESSAGE,
  BUILD_SUCCESS_MESSAGE,
  CARGO_BINARY_FLAG,
  CARGO_BUILD_SUBCOMMAND,
  CARGO_COMMAND,
  CARGO_PACKAGE_FLAG,
  CARGO_RELEASE_FLAG,
  CARGO_TARGET_FLAG,
  CLI_BINARY_NAME,
  CLI_CARGO_PACKAGE_NAME,
  COMMAND_ARGUMENT_SEPARATOR,
  HOST_TARGET_NOT_FOUND_MESSAGE,
  RUSTC_COMMAND,
  RUSTC_HOST_LINE_PREFIX,
  RUSTC_OUTPUT_LINE_SEPARATOR,
  RUSTC_VERBOSE_VERSION_FLAG,
  TAURI_TARGET_TRIPLE_ENV_VAR,
} from "./build-constants.js";
import {
  getCargoArtifactPath,
  getSidecarPath,
  resolveTargetTriple,
} from "./build-cli-utils.js";

/**
 * Reads the Rust host target from rustc's verbose version metadata.
 * Takes no arguments and returns a Rust target triple string. Throws if rustc
 * cannot execute or its output does not contain a host target line.
 */
function getHostTargetTriple() {
  // Constant references: RUSTC_COMMAND, RUSTC_VERBOSE_VERSION_FLAG, RUSTC_OUTPUT_LINE_SEPARATOR, and RUSTC_HOST_LINE_PREFIX.
  const rustcOutput = execFileSync(RUSTC_COMMAND, [RUSTC_VERBOSE_VERSION_FLAG], {
    encoding: "utf8",
  });
  const hostLine = rustcOutput
    .split(RUSTC_OUTPUT_LINE_SEPARATOR)
    .find((line) => line.startsWith(RUSTC_HOST_LINE_PREFIX));

  if (!hostLine) {
    // Constant reference: HOST_TARGET_NOT_FOUND_MESSAGE.
    throw new Error(HOST_TARGET_NOT_FOUND_MESSAGE);
  }

  // Constant reference: RUSTC_HOST_LINE_PREFIX.
  return hostLine.slice(RUSTC_HOST_LINE_PREFIX.length).trim();
}

let targetTriple;
let sourceArtifact;
let stagedArtifact;

try {
  // Constant reference: TAURI_TARGET_TRIPLE_ENV_VAR.
  const hasTauriTarget = Object.hasOwn(process.env, TAURI_TARGET_TRIPLE_ENV_VAR)
    && process.env[TAURI_TARGET_TRIPLE_ENV_VAR] !== "";
  targetTriple = resolveTargetTriple(
    process.env,
    hasTauriTarget ? undefined : getHostTargetTriple(),
  );

  const repositoryRoot = path.resolve();
  sourceArtifact = getCargoArtifactPath(targetTriple, repositoryRoot);
  stagedArtifact = getSidecarPath(targetTriple, repositoryRoot);

  // Constant references: BUILD_LOG_PREFIX and BUILD_START_MESSAGE.
  console.log(`${BUILD_LOG_PREFIX} ${BUILD_START_MESSAGE} ${targetTriple}`);

  // Constant references: CARGO_COMMAND, CARGO_BUILD_SUBCOMMAND, CARGO_RELEASE_FLAG, CARGO_PACKAGE_FLAG, CLI_CARGO_PACKAGE_NAME, CARGO_BINARY_FLAG, CLI_BINARY_NAME, CARGO_TARGET_FLAG, and COMMAND_ARGUMENT_SEPARATOR.
  const cargoCommand = [
    CARGO_COMMAND,
    CARGO_BUILD_SUBCOMMAND,
    CARGO_RELEASE_FLAG,
    CARGO_PACKAGE_FLAG,
    CLI_CARGO_PACKAGE_NAME,
    CARGO_BINARY_FLAG,
    CLI_BINARY_NAME,
    CARGO_TARGET_FLAG,
    targetTriple,
  ].join(COMMAND_ARGUMENT_SEPARATOR);
  execSync(cargoCommand, { stdio: "inherit" });

  if (!fs.existsSync(sourceArtifact)) {
    // Constant reference: BUILD_ARTIFACT_MISSING_MESSAGE.
    throw new Error(`${BUILD_ARTIFACT_MISSING_MESSAGE}: ${sourceArtifact}`);
  }

  fs.mkdirSync(path.dirname(stagedArtifact), { recursive: true });
  fs.copyFileSync(sourceArtifact, stagedArtifact);

  // Constant references: BUILD_LOG_PREFIX and BUILD_SUCCESS_MESSAGE.
  console.log(`${BUILD_LOG_PREFIX} ${BUILD_SUCCESS_MESSAGE}: ${stagedArtifact}`);
} catch (error) {
  // Constant references: BUILD_LOG_PREFIX and BUILD_ERROR_MESSAGE.
  console.error(`${BUILD_LOG_PREFIX} ${BUILD_ERROR_MESSAGE}: ${error instanceof Error ? error.message : String(error)}`);
  if (targetTriple) {
    // Constant references: BUILD_SOURCE_PATH_LABEL and BUILD_STAGE_PATH_LABEL.
    console.error(`${BUILD_LOG_PREFIX} ${BUILD_SOURCE_PATH_LABEL}: ${sourceArtifact}`);
    console.error(`${BUILD_LOG_PREFIX} ${BUILD_STAGE_PATH_LABEL}: ${stagedArtifact}`);
  }
  process.exitCode = 1;
}


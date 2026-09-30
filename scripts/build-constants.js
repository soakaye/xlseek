/**
 * ## Description
 * Centralizes fixed package, target environment, and artifact path values used
 * by the CLI Sidecar build helpers.
 *
 * ## Inputs & Outputs
 * Takes no inputs and exports named string constants for the Cargo package,
 * executable, Tauri target environment variable, Sidecar base path, generated
 * binaries directory, and naming/validation components.
 *
 * ## Errors / Exceptions
 * This declaration-only module performs no I/O and throws no runtime errors
 * during normal module loading.
 */

export const CLI_CARGO_PACKAGE_NAME = "xlseek-cli";
export const CLI_BINARY_NAME = "xlseek-cli";
export const TAURI_TARGET_TRIPLE_ENV_VAR = "TAURI_ENV_TARGET_TRIPLE";
export const SIDECAR_BASE_PATH = "binaries/xlseek-cli";
export const GENERATED_BINARIES_DIRECTORY = "src-tauri/binaries";
export const WINDOWS_TARGET_OS_TOKEN = "windows";
export const WINDOWS_EXECUTABLE_EXTENSION = ".exe";
export const CARGO_TARGET_DIRECTORY_NAME = "target";
export const CARGO_RELEASE_DIRECTORY_NAME = "release";
export const SIDECAR_TARGET_SEPARATOR = "-";
export const INVALID_TARGET_TRIPLE_MESSAGE = "Invalid Rust target triple";
export const TARGET_TRIPLE_PATTERN = /^[A-Za-z0-9_]+(?:-[A-Za-z0-9_]+){2,}$/;
export const RUSTC_COMMAND = "rustc";
export const RUSTC_VERBOSE_VERSION_FLAG = "-vV";
export const RUSTC_HOST_LINE_PREFIX = "host:";
export const RUSTC_OUTPUT_LINE_SEPARATOR = /\r?\n/;
export const HOST_TARGET_NOT_FOUND_MESSAGE = "Rust host target triple not found in rustc output";
export const CARGO_COMMAND = "cargo";
export const CARGO_BUILD_SUBCOMMAND = "build";
export const CARGO_RELEASE_FLAG = "--release";
export const CARGO_PACKAGE_FLAG = "-p";
export const CARGO_BINARY_FLAG = "--bin";
export const CARGO_TARGET_FLAG = "--target";
export const COMMAND_ARGUMENT_SEPARATOR = " ";
export const BUILD_LOG_PREFIX = "[build-cli]";
export const BUILD_START_MESSAGE = "Building CLI for target";
export const BUILD_SUCCESS_MESSAGE = "Staged CLI Sidecar";
export const BUILD_ERROR_MESSAGE = "Error";
export const BUILD_ARTIFACT_MISSING_MESSAGE = "Build artifact not found";
export const BUILD_SOURCE_PATH_LABEL = "Expected Cargo artifact";
export const BUILD_STAGE_PATH_LABEL = "Expected Sidecar path";

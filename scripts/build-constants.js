/**
 * ## Description
 * Centralizes fixed package, target environment, and artifact path values used
 * by the CLI Sidecar build helpers.
 *
 * ## Inputs & Outputs
 * Takes no inputs and exports named string constants for the Cargo package,
 * executable, Tauri target environment variable, Sidecar base path, and
 * generated binaries directory.
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

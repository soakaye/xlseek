/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Centralizes file paths, regular expressions, command names, and diagnostic messages
 * used by the minimal version synchronization script suite.
 *
 * ## Arguments & Returns
 * None. Exports named constants for repository file locations, regex patterns,
 * command names, and log messages.
 *
 * ## Errors / Exceptions
 * This declaration-only module performs no I/O and throws no runtime errors.
 */

export const PACKAGE_JSON_PATH = "package.json";
export const ROOT_CARGO_TOML_PATH = "Cargo.toml";
export const CARGO_LOCK_PATH = "Cargo.lock";

export const ALL_VERSION_TARGET_PATHS = [
  ROOT_CARGO_TOML_PATH,
];

export const SEMVER_PATTERN = /^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;
export const VERSION_PREFIX_PATTERN = /^v/;

export const CARGO_WORKSPACE_PACKAGE_VERSION_PATTERN = /^(\s*\[workspace\.package\][\s\S]*?^\s*version\s*=\s*")[^"]+(")/m;

export const CARGO_COMMAND = "cargo";
export const CARGO_CHECK_SUBCOMMAND = "check";
export const CARGO_WORKSPACE_FLAG = "--workspace";

export const GIT_COMMAND = "git";
export const GIT_ADD_SUBCOMMAND = "add";
export const GIT_DIFF_SUBCOMMAND = "diff";
export const GIT_CACHED_FLAG = "--cached";
export const GIT_NAME_ONLY_FLAG = "--name-only";

export const SYNC_LOG_PREFIX = "[sync-version]";
export const SYNC_SUCCESS_MESSAGE = "Successfully synchronized project version to";
export const INVALID_SEMVER_MESSAGE = "Invalid semver version format";
export const FILE_NOT_FOUND_MESSAGE = "Target file not found";
export const PATTERN_NOT_MATCHED_MESSAGE = "Failed to match version pattern in file";
export const CARGO_LOCK_VERIFY_FAILED_MESSAGE = "Cargo.lock version verification failed";
export const GIT_STAGE_VERIFY_FAILED_MESSAGE = "Git staging verification failed";
export const UTF8_ENCODING = "utf8";

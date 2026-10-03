/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Centralizes file paths, regular expressions, and diagnostic messages used
 * by the version synchronization script suite.
 *
 * ## Arguments & Returns
 * None. Exports named constants for repository file locations, regex patterns,
 * command names, and log messages.
 *
 * ## Errors / Exceptions
 * This declaration-only module performs no I/O and throws no runtime errors.
 */

export const PACKAGE_JSON_PATH = "package.json";
export const TAURI_CONF_PATH = "src-tauri/tauri.conf.json";
export const CORE_CARGO_TOML_PATH = "crates/core/Cargo.toml";
export const CLI_CARGO_TOML_PATH = "crates/cli/Cargo.toml";
export const TAURI_CARGO_TOML_PATH = "src-tauri/Cargo.toml";
export const CONSTANTS_INDEX_PATH = "src/constants/index.ts";
export const LOCALES_CORE_EN_PATH = "crates/core/locales/en.yml";
export const LOCALES_CORE_JA_PATH = "crates/core/locales/ja.yml";
export const LOCALES_TAURI_EN_PATH = "src-tauri/locales/en.yml";
export const LOCALES_TAURI_JA_PATH = "src-tauri/locales/ja.yml";
export const DESIGN_INDEX_HTML_PATH = "design/mainui/index.html";

export const ALL_VERSION_TARGET_PATHS = [
  TAURI_CONF_PATH,
  CORE_CARGO_TOML_PATH,
  CLI_CARGO_TOML_PATH,
  TAURI_CARGO_TOML_PATH,
  CONSTANTS_INDEX_PATH,
  LOCALES_CORE_EN_PATH,
  LOCALES_CORE_JA_PATH,
  LOCALES_TAURI_EN_PATH,
  LOCALES_TAURI_JA_PATH,
  DESIGN_INDEX_HTML_PATH,
];

export const SEMVER_PATTERN = /^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;
export const VERSION_PREFIX_PATTERN = /^v/;

export const CARGO_PACKAGE_VERSION_PATTERN = /^(\s*\[package\][\s\S]*?^\s*version\s*=\s*")[^"]+(")/m;
export const TAURI_CONF_VERSION_PATTERN = /("version"\s*:\s*")[^"]+(")/;
export const CONSTANTS_APP_VERSION_PATTERN = /(APP_VERSION\s*:\s*")[^"]+(")/;
export const LOCALE_APP_VERSION_PATTERN = /(about\.APP_VERSION\s*:\s*)[^\r\n]+/;
export const DESIGN_HTML_APP_HEADER_VERSION_PATTERN = /(アプリについて[\s\S]*?<span[^>]*>\s*v)[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?/g;
export const DESIGN_HTML_APP_MODAL_VERSION_PATTERN = /(Excel Seek[\s\S]*?<p[^>]*>\s*Version\s+)[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?/g;

export const CARGO_COMMAND = "cargo";
export const CARGO_CHECK_SUBCOMMAND = "check";
export const CARGO_WORKSPACE_FLAG = "--workspace";

export const SYNC_LOG_PREFIX = "[sync-version]";
export const SYNC_SUCCESS_MESSAGE = "Successfully synchronized all project version entries to";
export const INVALID_SEMVER_MESSAGE = "Invalid semver version format";
export const FILE_NOT_FOUND_MESSAGE = "Target file not found";
export const PATTERN_NOT_MATCHED_MESSAGE = "Failed to match version pattern in file";
export const UTF8_ENCODING = "utf8";

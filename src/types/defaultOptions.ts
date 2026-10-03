/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Default search options type definition (src/types/defaultOptions.ts)
 *
 * ## Description
 * Defines the data type for default search options that can be customized by
 * the user via the settings dialog and persisted in local storage.
 * Complies with Constitution Principle III (comprehensive documentation).
 */

/**
 * Default search options interface
 */
export interface DefaultSearchOptions {
  match_case: boolean;
  use_regex: boolean;
  include_formula: boolean;
  include_shape: boolean;
  include_comment: boolean;
  include_hidden: boolean;
  extensions: string[];
  directory_mode: "sequential" | "burst";
  burst_workers: number | null;
}

/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview ESLint flat configuration (eslint.config.js)
 *
 * ## Description
 * Configures ESLint rules, TypeScript ESLint plugin, and React hooks rules.
 *
 * ## Arguments & Returns
 * None. Exports ESLint FlatConfig array.
 *
 * ## Errors / Exceptions
 * None.
 */

import tseslint from "typescript-eslint";
import reactHooks from "eslint-plugin-react-hooks";

export default tseslint.config(
  { ignores: ["dist/**", "target/**", "src-tauri/**", "node_modules/**"] },
  ...tseslint.configs.recommended,
  {
    files: ["**/*.{ts,tsx}"],
    plugins: { "react-hooks": reactHooks },
    rules: { ...reactHooks.configs.recommended.rules },
  },
);

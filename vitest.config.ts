/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Vitest test runner configuration (vitest.config.ts)
 *
 * ## Description
 * Configures Vitest test environment (jsdom), globals, and coverage settings.
 *
 * ## Arguments & Returns
 * None. Exports Vitest UserConfig.
 *
 * ## Errors / Exceptions
 * None.
 */

import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  test: {
    environment: "jsdom",
    environmentOptions: { jsdom: { url: "http://localhost" } },
    include: ["tests/**/*.test.{ts,tsx}"],
  },
});

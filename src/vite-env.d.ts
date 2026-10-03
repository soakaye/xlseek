/**
 * Copyright (c) 2026 soakaye
 *
 * ## Description
 * Provides Vite environment types and raw resource import types to TypeScript.
 *
 * ## Arguments & Returns
 * Type declarations only; types `*.yml?raw` imports as string.
 *
 * ## Errors / Exceptions
 * TypeScript will reject raw resource imports if this type declaration is missing.
 */
/// <reference types="vite/client" />

declare module "*?raw" {
  const source: string;
  export default source;
}

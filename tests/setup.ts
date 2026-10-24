/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Vitest test setup (tests/setup.ts)
 *
 * ## Description
 * Polyfills window.localStorage for jsdom in Node 22+ environments.
 * Complies with Constitution Principle I, II, and III.
 *
 * ## Arguments & Returns
 * None.
 *
 * ## Errors / Exceptions
 * None.
 */

if (typeof window !== "undefined") {
  const store = new Map<string, string>();
  const mockStorage: Storage = {
    get length() {
      return store.size;
    },
    clear: () => store.clear(),
    getItem: (key: string) => store.get(key) ?? null,
    key: (index: number) => [...store.keys()][index] ?? null,
    removeItem: (key: string) => {
      store.delete(key);
    },
    setItem: (key: string, value: string) => {
      store.set(key, String(value));
    },
  };
  Object.defineProperty(window, "localStorage", {
    value: mockStorage,
    writable: true,
    configurable: true,
  });
  Object.defineProperty(globalThis, "localStorage", {
    value: mockStorage,
    writable: true,
    configurable: true,
  });
}

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

/**
 * ## Description
 * Sets chunk size warning limit reflecting the known size of lazy-loaded About chunk including licenses.
 *
 * ## Arguments & Returns
 * None. Returns Rollup chunk size limit in KiB.
 *
 * ## Errors / Exceptions
 * Setting too small a value will trigger false warning diagnostics on the license JSON chunk.
 */
const LICENSE_CATALOG_CHUNK_SIZE_KB = 3500;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // Prevent Vite from watching Rust target and crates directories to avoid EBUSY errors
      ignored: ["**/src-tauri/**", "**/target/**", "**/crates/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome105", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    // Constant reference: LICENSE_CATALOG_CHUNK_SIZE_KB. About dialog is lazy loaded and contains full license JSON.
    chunkSizeWarningLimit: LICENSE_CATALOG_CHUNK_SIZE_KB,
  },
});

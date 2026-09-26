import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

/**
 * ## 処理内容
 * ライセンス一覧を含む遅延読込 About チャンクの既知サイズをビルド警告上限へ反映する。
 * ## 引数・戻り値
 * 引数なし。Rollup が比較するチャンク警告上限を KiB 単位で返す。
 * ## エラー
 * 小さすぎる値ではライセンス JSON のチャンクについて既知のサイズ警告が発生する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): About のライセンス一覧に合わせて上限を定義。
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
      // ViteがRustのtargetディレクトリやsrc-tauri配下を監視してEBUSYエラーを起こすのを防止
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome105", "safari13"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    // 定数参照: LICENSE_CATALOG_CHUNK_SIZE_KB。About は遅延読込され、ライセンス一覧を含む。
    chunkSizeWarningLimit: LICENSE_CATALOG_CHUNK_SIZE_KB,
  },
});

/**
 * ## 処理内容
 * Vite の環境型と raw 資源インポート型を TypeScript に提供する。
 * ## 引数・戻り値
 * 宣言のみを含み、`.yml?raw` インポートを文字列として型付けする。
 * ## エラー
 * 型宣言が不足した場合は TypeScript が raw 資源インポートを拒否する。
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, Codex): YAML raw インポート型を追加。
 */
/// <reference types="vite/client" />

declare module "*?raw" {
  const source: string;
  export default source;
}

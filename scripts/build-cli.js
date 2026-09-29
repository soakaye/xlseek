/**
 * 処理内容: exlgrep-cli バイナリをリリースモードでビルドする。
 * 引数・戻り値: コマンドライン引数なし。プロセス終了ステータス (0: 成功, 1: 失敗)。
 * エラー: Cargo ビルド失敗時に例外を捕捉し、標準エラー出力へ通知して exit(1) する。
 * 変更履歴:
 *   - v1.0.0 (2026-09-29, Antigravity): 初版作成。クロスプラットフォームな CLI ビルドスクリプトを実装。
 *   - v1.1.0 (2026-09-29, Antigravity): CLIクレート分離に伴いパッケージ対象を exlgrep-cli に更新。
 */

import { execSync } from "child_process";
import fs from "fs";
import path from "path";

try {
  console.log("[build-cli] Building exlgrep-cli in release mode...");
  execSync("cargo build --release -p exlgrep-cli --bin exlgrep-cli", { stdio: "inherit" });

  const isWindows = process.platform === "win32";
  const binName = isWindows ? "exlgrep-cli.exe" : "exlgrep-cli";
  const srcBin = path.resolve("target", "release", binName);

  if (!fs.existsSync(srcBin)) {
    throw new Error(`Build artifact not found: ${srcBin}`);
  }

  console.log(`[build-cli] Successfully built ${binName} at ${srcBin}`);
} catch (error) {
  console.error("[build-cli] Error:", error.message || error);
  process.exit(1);
}


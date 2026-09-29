/**
 * ## Description
 * Builds the xlseek-cli binary in release mode.
 *
 * ## Arguments & Returns
 * Takes no CLI arguments. Returns process exit status (0 on success, 1 on failure).
 *
 * ## Errors / Exceptions
 * Catches errors on Cargo build failures, logs diagnostics to stderr, and exits with code 1.
 */

import { execSync } from "child_process";
import fs from "fs";
import path from "path";

try {
  console.log("[build-cli] Building xlseek-cli in release mode...");
  execSync("cargo build --release -p xlseek-cli --bin xlseek-cli", { stdio: "inherit" });

  const isWindows = process.platform === "win32";
  const binName = isWindows ? "xlseek-cli.exe" : "xlseek-cli";
  const srcBin = path.resolve("target", "release", binName);

  if (!fs.existsSync(srcBin)) {
    throw new Error(`Build artifact not found: ${srcBin}`);
  }

  console.log(`[build-cli] Successfully built ${binName} at ${srcBin}`);
} catch (error) {
  console.error("[build-cli] Error:", error.message || error);
  process.exit(1);
}


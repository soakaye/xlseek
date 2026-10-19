//! # MCP Server Binary Entry Point (crates/mcp/src/main.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Command-line entry point for the `xlseek-mcp` binary executable.
//! Parses runtime arguments (`--allowed-dir`, `--help`, `--version`), configures the security
//! context, and enters the standard input/output JSON-RPC 2.0 loop.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use std::io::{self, BufReader};
use std::path::PathBuf;
use xlseek_mcp::constants;
use xlseek_mcp::security::McpSecurityContext;
use xlseek_mcp::McpServer;

/// Prints help summary to standard output.
///
/// ## Arguments
/// None.
///
/// ## Returns
/// None.
///
/// ## Errors
/// Does not panic.
fn print_help() {
    println!(
        "{} - Model Context Protocol server for Excel Seek\n\n\
        USAGE:\n  \
          {} [OPTIONS]\n\n\
        OPTIONS:\n  \
          {} <PATH>    Permit file operations only within designated root directory (can be repeated)\n  \
          {}, {}             Display this help message\n  \
          {}, {}          Display version information\n\n\
        ENVIRONMENT VARIABLES:\n  \
          {}    Comma-separated list of allowed directory roots",
        constants::SERVER_NAME,
        constants::SERVER_NAME,
        constants::CLI_FLAG_ALLOWED_DIR,
        constants::CLI_FLAG_HELP_SHORT,
        constants::CLI_FLAG_HELP_LONG,
        constants::CLI_FLAG_VERSION_SHORT,
        constants::CLI_FLAG_VERSION_LONG,
        constants::ENV_ALLOWED_DIRS,
    );
}

/// Main executable entry point.
///
/// ## Arguments
/// Receives OS process command-line arguments.
///
/// ## Returns
/// Process exit code represented as `std::process::ExitCode`.
///
/// ## Errors
/// Catches I/O and configuration errors, writing diagnostics strictly to `stderr`
/// so `stdout` remains pristine for JSON-RPC framing.
fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut allowed_dirs = Vec::new();

    let mut idx = 1;
    while idx < args.len() {
        let arg = &args[idx];
        // Constant reference: constants::CLI_FLAG_HELP_SHORT
        // Constant reference: constants::CLI_FLAG_HELP_LONG
        if arg == constants::CLI_FLAG_HELP_SHORT || arg == constants::CLI_FLAG_HELP_LONG {
            print_help();
            return std::process::ExitCode::SUCCESS;
        }
        // Constant reference: constants::CLI_FLAG_VERSION_SHORT
        // Constant reference: constants::CLI_FLAG_VERSION_LONG
        if arg == constants::CLI_FLAG_VERSION_SHORT || arg == constants::CLI_FLAG_VERSION_LONG {
            println!("{} {}", constants::SERVER_NAME, constants::SERVER_VERSION);
            return std::process::ExitCode::SUCCESS;
        }
        // Constant reference: constants::CLI_FLAG_ALLOWED_DIR
        if arg == constants::CLI_FLAG_ALLOWED_DIR {
            if idx + 1 < args.len() {
                idx += 1;
                allowed_dirs.push(PathBuf::from(&args[idx]));
            } else {
                eprintln!(
                    "Error: {} requires a directory path argument",
                    constants::CLI_FLAG_ALLOWED_DIR
                );
                return std::process::ExitCode::FAILURE;
            }
        }
        idx += 1;
    }

    let mut security = McpSecurityContext::from_env();
    if !allowed_dirs.is_empty() {
        let mut merged = security.allowed_directories().to_vec();
        merged.extend(allowed_dirs);
        security = McpSecurityContext::new(merged);
    }

    let server = McpServer::new(security);
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = BufReader::new(stdin.lock());
    let writer = stdout.lock();

    match server.run(reader, writer) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fatal MCP server error: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

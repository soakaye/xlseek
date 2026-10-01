//! # CLI Output Safety & Publication
//!
//! ## Description
//! Protects search inputs, temporarily stages results, and publishes completed export files
//! to the target destination.
//!
//! ## Arguments / Returns
//! Receives target path, overwrite flag, search matches, export format, language, and translation catalogs,
//! returning the saved file path.
//!
//! ## Errors
//! Returns `Err` on input collision, output conflict, export error, publication failure, or file system error.

use crate::constants;
use same_file::Handle;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use xlseek_core::models::{ExportFormat, SearchMatch};

/// Retains initial output file identity and overwrite policy.
///
/// ## Arguments / Returns
/// Accepts target output path and overwrite flag, returning an inspectable `OutputGuard`.
///
/// ## Errors
/// Returns `Err` if destination already exists and overwrite is false, or if file is symlink / metadata is unreadable.
pub struct OutputGuard {
    output: PathBuf,
    initial_identity: Option<Handle>,
    overwrite: bool,
}

impl OutputGuard {
    /// Validates whether the destination is safe for publication and records the initial file identity.
    ///
    /// ## Arguments / Returns
    /// Accepts output `PathBuf` and overwrite flag, returning `Result<Self, String>`.
    ///
    /// ## Errors
    /// Returns `Err` on symlink, non-regular file, existing file without overwrite flag, or metadata retrieval failure.
    pub fn new(output: PathBuf, overwrite: bool) -> Result<Self, String> {
        let initial_identity = match std::fs::symlink_metadata(&output) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_OUTPUT_PATH,
                    output.display()
                ));
            }
            Ok(_) if !overwrite => {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_EXISTS,
                    output.display()
                ));
            }
            Ok(_) => Some(Handle::from_path(&output).map_err(|error| error.to_string())?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.to_string()),
        };
        Ok(Self {
            output,
            initial_identity,
            overwrite,
        })
    }

    /// Verifies that a discovered search input path is not identical to the target output destination.
    ///
    /// ## Arguments / Returns
    /// Accepts input `&Path`, returning `Ok(())` if distinct or `Err` if paths collide.
    ///
    /// ## Errors
    /// Returns `Err` if file handles match (including hard links) or canonical paths are identical.
    pub fn check_input(&self, input: &Path) -> Result<(), String> {
        if let Some(output_identity) = &self.initial_identity {
            let input_identity = Handle::from_path(input).map_err(|error| error.to_string())?;
            if output_identity == &input_identity {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_INPUT_COLLISION,
                    input.display()
                ));
            }
        } else if std::fs::canonicalize(input).ok().as_deref() == Some(self.output.as_path()) {
            return Err(format!(
                "{}: {}",
                constants::ERR_CLI_INPUT_COLLISION,
                input.display()
            ));
        }
        Ok(())
    }

    /// Exports search matches to a temporary file in the destination's parent directory, checks for conflicts, and publishes.
    ///
    /// ## Arguments / Returns
    /// Accepts search matches, format, language, and translation catalogs, returning the published `PathBuf`.
    ///
    /// ## Errors
    /// Returns `Err` on temporary directory creation, write failure, conflict verification, hard-link, or rename failure.
    pub fn publish(
        &self,
        matches: &[SearchMatch],
        format: ExportFormat,
        language: &str,
        catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    ) -> Result<PathBuf, String> {
        let parent = self
            .output
            .parent()
            .ok_or_else(|| constants::ERR_CLI_OUTPUT_PATH.to_string())?;
        let temporary_directory = TemporaryDirectory::create(parent)?;
        let temporary_file = temporary_directory.path.join(constants::CLI_TEMP_FILE_NAME);
        let temporary_file_text = temporary_file.to_string_lossy();
        match format {
            ExportFormat::Csv => xlseek_core::export::export_to_csv(
                &temporary_file_text,
                matches,
                language,
                catalogs,
            )?,
            ExportFormat::Xlsx => xlseek_core::export::export_to_xlsx(
                &temporary_file_text,
                matches,
                language,
                catalogs,
            )?,
        }
        self.verify_output_identity()?;
        if self.overwrite {
            std::fs::rename(&temporary_file, &self.output)
                .map_err(|error| format!("{}: {}", constants::ERR_CLI_OUTPUT, error))?;
        } else {
            std::fs::hard_link(&temporary_file, &self.output)
                .map_err(|error| format!("{}: {}", constants::ERR_CLI_OUTPUT, error))?;
        }
        Ok(self.output.clone())
    }

    /// Verifies that the destination file identity has not changed since execution began.
    ///
    /// ## Arguments / Returns
    /// Borrows `self` and returns `Ok(())` if destination identity matches expected state.
    ///
    /// ## Errors
    /// Returns `Err` if file was unexpectedly created, replaced, deleted, or metadata read fails.
    fn verify_output_identity(&self) -> Result<(), String> {
        match (
            &self.initial_identity,
            std::fs::symlink_metadata(&self.output),
        ) {
            (None, Err(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            (Some(expected), Ok(metadata))
                if metadata.is_file() && !metadata.file_type().is_symlink() =>
            {
                let actual = Handle::from_path(&self.output).map_err(|error| error.to_string())?;
                if expected == &actual {
                    Ok(())
                } else {
                    Err(constants::ERR_CLI_OUTPUT_CHANGED.to_string())
                }
            }
            _ => Err(constants::ERR_CLI_OUTPUT_CHANGED.to_string()),
        }
    }
}

/// RAII manager that creates an exclusive temporary directory next to the output destination and cleans it up on drop.
struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    /// Creates a non-colliding temporary directory next to the target output path.
    ///
    /// ## Arguments / Returns
    /// Accepts parent path and returns RAII cleanup guard `Result<Self, String>`.
    ///
    /// ## Errors
    /// Returns `Err` if maximum attempt limit is reached or directory creation fails.
    fn create(parent: &Path) -> Result<Self, String> {
        for attempt in 0..constants::CLI_TEMP_CREATE_ATTEMPTS {
            let path = parent.join(format!(
                "{}{}-{}",
                constants::CLI_TEMP_DIRECTORY_PREFIX,
                std::process::id(),
                attempt
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
        }
        Err(constants::ERR_CLI_TEMP_LIMIT.to_string())
    }
}

impl Drop for TemporaryDirectory {
    /// Cleans up temporary directory contents when scope ends.
    ///
    /// ## Arguments / Returns
    /// Borrows `&mut self`; returns nothing.
    ///
    /// ## Errors
    /// Any removal failure is silently ignored in destructor.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

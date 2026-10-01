//! Description: Provides search directory resolution, read validation, and completion candidates.
//! Arguments/Returns: Resolves validated paths or returns candidates from user input and home directory.
//! Errors: Returns typed errors for invalid, nonexistent, or unreadable paths.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Description: Classifies failures occurring during directory validation.
/// Arguments/Returns: None. Holds error variants.
/// Errors: None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    NotFound,
    PermissionDenied,
    SearchFailed,
}

/// Description: Expands home shorthand prefix in input path and returns search path.
/// Arguments/Returns: Accepts input string and optional home path, returns resolved PathBuf.
/// Errors: Returns SearchFailed if home shorthand is present but home path is not provided.
pub fn resolve_search_path(input: &str, home_dir: Option<&Path>) -> Result<PathBuf, PathError> {
    #[cfg(windows)]
    let prefixes = [
        crate::constants::HOME_PATH_PREFIX_WINDOWS,
        crate::constants::HOME_PATH_PREFIX_UNIX,
    ];
    #[cfg(not(windows))]
    let prefixes = [crate::constants::HOME_PATH_PREFIX_UNIX];

    if let Some(remainder) = prefixes
        .iter()
        .find_map(|prefix| input.strip_prefix(prefix))
    {
        let home = home_dir.ok_or(PathError::SearchFailed)?;
        return Ok(home.join(remainder));
    }

    Ok(PathBuf::from(input))
}

/// Description: Verifies that the search target directory exists and can be read.
/// Arguments/Returns: Accepts Path to validate; returns Ok(()) on success, or classified error on failure.
/// Errors: Converts nonexistent, non-directory, permission denied, or other I/O errors into PathError.
pub fn validate_search_directory(path: &Path) -> Result<(), PathError> {
    let mut entries = fs::read_dir(path).map_err(classify_io_error)?;
    if let Some(entry) = entries.next() {
        entry.map_err(classify_io_error)?;
    }
    Ok(())
}

/// Description: Converts std::io::Error into the public PathError type.
/// Arguments/Returns: Accepts std::io::Error, returns PathError.
/// Errors: None. Unknown I/O errors are classified as SearchFailed.
fn classify_io_error(error: io::Error) -> PathError {
    match error.kind() {
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory => PathError::NotFound,
        io::ErrorKind::PermissionDenied => PathError::PermissionDenied,
        _ => PathError::SearchFailed,
    }
}

/// Description: Lists matching directories directly under parent of absolute path or home shorthand.
/// Arguments/Returns: Accepts path input and optional home path; returns string array of candidate paths.
/// Errors: Returns empty vector on unreadable, invalid, or unsupported input; never panics.
pub fn complete_directory_path(input: &str, home_dir: Option<&Path>) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }
    let is_home_input = input.starts_with(crate::constants::HOME_PATH_PREFIX_UNIX)
        || input.starts_with(crate::constants::HOME_PATH_PREFIX_WINDOWS);
    let path = match resolve_search_path(input, home_dir) {
        Ok(path) => path,
        Err(_) => return Vec::new(),
    };
    if !is_supported_completion_path(&path, is_home_input) {
        return Vec::new();
    }

    let trailing_separator = input.ends_with(std::path::MAIN_SEPARATOR)
        || (cfg!(windows) && (input.ends_with('/') || input.ends_with('\\')));
    let (parent, name_prefix) = if trailing_separator {
        (path.as_path(), "")
    } else {
        let Some(parent) = path.parent() else {
            return Vec::new();
        };
        let Some(file_name) = path.file_name() else {
            return Vec::new();
        };
        (parent, &*file_name.to_string_lossy())
    };

    let Ok(entries) = fs::read_dir(parent) else {
        return Vec::new();
    };
    let mut candidates = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let is_directory = entry.file_type().ok()?.is_dir();
            let name = entry.file_name().to_string_lossy().into_owned();
            (is_directory && name.starts_with(name_prefix))
                .then(|| (name, parent.join(entry.file_name())))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.0.cmp(&right.0));
    candidates
        .into_iter()
        .take(crate::constants::MAX_PATH_COMPLETION_RESULTS)
        .map(|(_, candidate)| {
            if is_home_input {
                if let Some(home) = home_dir {
                    if let Ok(relative) = candidate.strip_prefix(home) {
                        let prefix =
                            if input.starts_with(crate::constants::HOME_PATH_PREFIX_WINDOWS) {
                                crate::constants::HOME_PATH_PREFIX_WINDOWS
                            } else {
                                crate::constants::HOME_PATH_PREFIX_UNIX
                            };
                        return format!("{prefix}{}", relative.display());
                    }
                }
            }
            candidate.to_string_lossy().into_owned()
        })
        .collect()
}

/// ## Description
/// Checks whether completion target path is an absolute path or has a safe home-shorthand prefix.
///
/// ## Arguments
/// - `path`: `&Path` - Path to check
/// - `is_home_input`: `bool` - Flag indicating whether input originated from home shorthand
///
/// ## Returns
/// - `bool`: true if valid and safe prefix, false otherwise
///
/// ## Errors / Exceptions
/// None. Unsupported paths return false.
fn is_supported_completion_path(path: &Path, is_home_input: bool) -> bool {
    if !path.is_absolute() {
        return false;
    }
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};
        if is_home_input {
            return true;
        }
        matches!(
            path.components().next(),
            Some(Component::Prefix(prefix))
                if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::UNC(..))
        )
    }
    #[cfg(not(windows))]
    {
        let _ = is_home_input;
        true
    }
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    use super::is_supported_completion_path;
    use super::{
        complete_directory_path, resolve_search_path, validate_search_directory, PathError,
    };
    use std::fs;
    #[cfg(windows)]
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_DIRECTORY_COUNT: usize = 12;
    const EXPECTED_COMPLETION_COUNT: usize = 10;

    /// Description: Creates a test-only temporary directory to verify path behavior on real filesystem.
    /// Arguments/Returns: None. Returns created temporary PathBuf.
    /// Errors: Panics if temporary directory creation fails.
    fn temp_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("xlseek-path-tests-{unique}"));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("temporary directory should be created");
        path
    }

    /// Description: Verifies that home shorthand resolves to the specified home directory.
    /// Arguments/Returns: None. Validates resolved path.
    /// Errors: Panics if resolved path does not match expectation.
    #[test]
    fn resolves_home_shorthand_to_the_home_directory() {
        let root = temp_dir();
        let home = root.join("home");
        fs::create_dir_all(&home).expect("home directory should be created");

        assert_eq!(
            resolve_search_path("~/reports", Some(&home)).expect("path should resolve"),
            home.join("reports")
        );
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies rejection of missing search targets and regular files.
    /// Arguments/Returns: None. Validates error variant from validation function.
    /// Errors: Panics if invalid target is not rejected.
    #[test]
    fn rejects_missing_and_non_directory_search_targets() {
        let root = temp_dir();
        let file = root.join("report.xlsx");
        fs::write(&file, "workbook").expect("file should be created");

        assert!(matches!(
            validate_search_directory(&root.join("missing")),
            Err(PathError::NotFound)
        ));
        assert!(matches!(
            validate_search_directory(&file),
            Err(PathError::NotFound)
        ));
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies returning only matching directories under parent in sorted order.
    /// Arguments/Returns: None. Validates candidate list.
    /// Errors: Panics if candidate list includes files or is out of order.
    #[test]
    fn returns_sorted_matching_directories_and_excludes_files() {
        let root = temp_dir();
        for name in ["Alpha", "Alpine", "Beta"] {
            fs::create_dir(root.join(name)).expect("directory should be created");
        }
        fs::write(root.join("Almanac.xlsx"), "workbook").expect("file should be created");

        assert_eq!(
            complete_directory_path(&root.join("Al").to_string_lossy(), None),
            vec![
                root.join("Alpha").to_string_lossy().into_owned(),
                root.join("Alpine").to_string_lossy().into_owned(),
            ]
        );
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies sorting candidates by name before limiting to max results.
    /// Arguments/Returns: None. Validates prefix candidates within limit.
    /// Errors: Panics if count exceeds limit or ordering is wrong.
    #[test]
    fn sorts_before_limiting_completion_results() {
        let root = temp_dir();
        for index in 0..TEST_DIRECTORY_COUNT {
            fs::create_dir(root.join(format!("dir{index:02}")))
                .expect("directory should be created");
        }
        let input = format!("{}{}", root.display(), std::path::MAIN_SEPARATOR);
        let actual = complete_directory_path(&input, None);
        let expected = (0..EXPECTED_COMPLETION_COUNT)
            .map(|index| {
                root.join(format!("dir{index:02}"))
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies that completion results for home shorthand retain the shorthand notation.
    /// Arguments/Returns: None. Validates candidate string.
    /// Errors: Panics if replaced with resolved real path.
    #[test]
    fn preserves_home_shorthand_in_completion_results() {
        let root = temp_dir();
        let home = root.join("home");
        fs::create_dir_all(home.join("Alpha")).expect("directory should be created");

        assert_eq!(
            complete_directory_path("~/Al", Some(&home)),
            vec!["~/Alpha".to_string()]
        );
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies that directory names containing spaces and non-ASCII characters are preserved.
    /// Arguments/Returns: None. Validates candidate string exact match.
    /// Errors: Panics if candidates are missing or corrupted.
    #[test]
    fn preserves_spaces_and_japanese_directory_names() {
        let root = temp_dir();
        let child = root.join("日本 語");
        fs::create_dir(&child).expect("directory should be created");
        let input = root.join("日本").to_string_lossy().into_owned();

        assert_eq!(
            complete_directory_path(&input, None),
            vec![child.to_string_lossy().into_owned()]
        );
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies listing items directly under target when input has trailing separator.
    /// Arguments/Returns: None. Validates candidate list.
    /// Errors: Panics if trailing separator is treated as name prefix.
    #[test]
    fn treats_trailing_separator_as_an_empty_name_prefix() {
        let root = temp_dir();
        let child = root.join("child");
        fs::create_dir(&child).expect("child directory should be created");
        let input = format!("{}{}", root.display(), std::path::MAIN_SEPARATOR);

        assert_eq!(
            complete_directory_path(&input, None),
            vec![child.to_string_lossy().into_owned()]
        );
        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies that relative paths are excluded from directory completion.
    /// Arguments/Returns: None. Validates empty candidate vector.
    /// Errors: Panics if candidates are returned for relative path.
    #[test]
    fn ignores_relative_completion_input() {
        assert!(complete_directory_path("reports/Al", None).is_empty());
    }

    /// Description: Verifies that readable directory passes preliminary validation.
    /// Arguments/Returns: None. Checks validation result on existing directory.
    /// Errors: Panics if readable directory is rejected.
    #[test]
    fn accepts_readable_search_directory() {
        let root = temp_dir();
        assert_eq!(validate_search_directory(&root), Ok(()));
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(windows)]
    /// Description: Verifies completion for Windows-supported path types, excluding relative/device paths.
    /// Arguments/Returns: None. Validates candidates for various path forms.
    /// Errors: Panics if unsupported Windows paths produce candidates.
    #[test]
    fn accepts_windows_absolute_paths_and_ignores_unsupported_prefixes() {
        let home = Path::new(r"C:\Users\test");
        assert!(is_supported_completion_path(
            Path::new(r"C:\Reports\Al"),
            false
        ));
        assert!(is_supported_completion_path(
            Path::new(r"\\server\share\Reports\Al"),
            false
        ));
        assert!(!is_supported_completion_path(
            Path::new(r"C:Reports"),
            false
        ));
        assert!(!is_supported_completion_path(Path::new(r"\Reports"), false));
        assert!(!is_supported_completion_path(
            Path::new(r"\\.\PhysicalDrive0"),
            false
        ));
        assert!(!is_supported_completion_path(
            Path::new(r"\\?\C:\Reports"),
            false
        ));
        assert_eq!(
            resolve_search_path(r"~\Reports", Some(home)),
            Ok(home.join("Reports"))
        );
        assert!(is_supported_completion_path(
            &resolve_search_path(r"~\Reports", Some(home)).unwrap(),
            true
        ));
    }
}

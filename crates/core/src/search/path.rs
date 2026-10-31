//! Copyright (c) 2026 soakaye
//!
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

/// Description: Classifies failures occurring during path string tokenization.
/// Arguments/Returns: None. Holds error variants.
/// Errors: None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathParseError {
    EmptyInput,
    UnclosedQuote,
}

/// Description: Represents the outcome of resolving, validating, and partitioning search paths.
/// Arguments/Returns: Contains valid roots and invalid path strings paired with their errors.
/// Errors: None.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathResolutionResult {
    pub valid_roots: Vec<PathBuf>,
    pub invalid_paths: Vec<(String, PathError)>,
}

/// Description: Parses comma-separated path string into individual path tokens, respecting double-quoted segments.
/// Arguments/Returns: Accepts `input: &str`; returns `Result<Vec<String>, PathParseError>`.
/// Errors: Returns `PathParseError::UnclosedQuote` if a quoted segment is unclosed, or `EmptyInput` if no paths exist.
pub fn parse_search_paths(input: &str) -> Result<Vec<String>, PathParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PathParseError::EmptyInput);
    }

    let mut paths = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = trimmed.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            crate::constants::PATH_QUOTE_CHAR => {
                if in_quotes {
                    // Check for escaped quote ("" or \")
                    if chars.peek() == Some(&crate::constants::PATH_QUOTE_CHAR) {
                        chars.next();
                        current.push(crate::constants::PATH_QUOTE_CHAR);
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            '\\' if in_quotes && chars.peek() == Some(&crate::constants::PATH_QUOTE_CHAR) => {
                chars.next();
                current.push(crate::constants::PATH_QUOTE_CHAR);
            }
            crate::constants::MULTI_PATH_DELIMITER if !in_quotes => {
                let token = current.trim();
                if !token.is_empty() {
                    paths.push(token.to_string());
                }
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }

    if in_quotes {
        return Err(PathParseError::UnclosedQuote);
    }

    let token = current.trim();
    if !token.is_empty() {
        paths.push(token.to_string());
    }

    if paths.is_empty() {
        return Err(PathParseError::EmptyInput);
    }

    Ok(paths)
}

/// Description: Normalizes an existing filesystem path to its canonical form and removes OS-internal verbatim prefixes.
/// Arguments/Returns: Accepts `path: &Path`; returns `PathBuf` as clean canonical absolute path.
/// Errors: None; falls back to resolving relative segments if canonicalization fails.
pub fn normalize_canonical_path(path: &Path) -> PathBuf {
    let canonical = path.canonicalize().unwrap_or_else(|_| {
        let mut components = Vec::new();
        for component in path.components() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    components.pop();
                }
                c => components.push(c),
            }
        }
        components.iter().collect::<PathBuf>()
    });
    let raw_str = canonical.to_string_lossy();
    let display_str = crate::export::normalize_export_path(&raw_str);
    PathBuf::from(display_str.into_owned())
}

/// Description: Removes exact duplicate paths and prunes descendant subpaths from a set of paths.
/// Arguments/Returns: Accepts `paths: Vec<PathBuf>`; returns deduplicated and pruned `Vec<PathBuf>`.
/// Errors: None. Uses component-level comparison (`starts_with`) safely.
pub fn deduplicate_and_prune_subpaths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique_paths = Vec::new();
    for path in paths {
        if !unique_paths.contains(&path) {
            unique_paths.push(path);
        }
    }

    if unique_paths.len() <= 1 {
        return unique_paths;
    }

    // Cache directory status to avoid repeated filesystem stat calls across O(N^2) comparisons
    let path_entries: Vec<(PathBuf, bool)> = unique_paths
        .into_iter()
        .map(|p| {
            let is_dir = p.is_dir();
            (p, is_dir)
        })
        .collect();

    let mut pruned = Vec::new();
    for (candidate, _) in &path_entries {
        let has_ancestor = path_entries.iter().any(|(other, other_is_dir)| {
            if other == candidate {
                return false;
            }
            if !*other_is_dir {
                return false;
            }
            candidate.starts_with(other)
        });
        if !has_ancestor && !pruned.contains(candidate) {
            pruned.push(candidate.clone());
        }
    }
    pruned
}

/// Description: Resolves, validates, partitions, and deduplicates target paths against the filesystem.
/// Arguments/Returns: Accepts slice of path strings and optional home directory; returns `PathResolutionResult`.
/// Errors: None; classifies inaccessible or invalid paths into `invalid_paths` rather than panicking.
pub fn resolve_and_partition_paths(
    paths: &[String],
    home_dir: Option<&Path>,
) -> PathResolutionResult {
    let mut valid_roots = Vec::new();
    let mut invalid_paths = Vec::new();

    for raw_path in paths {
        let resolved = match resolve_search_path(raw_path, home_dir) {
            Ok(p) => p,
            Err(e) => {
                invalid_paths.push((raw_path.clone(), e));
                continue;
            }
        };

        match validate_search_directory(&resolved) {
            Ok(()) => {
                let canonical = normalize_canonical_path(&resolved);
                if !valid_roots.contains(&canonical) {
                    valid_roots.push(canonical);
                }
            }
            Err(e) => {
                invalid_paths.push((raw_path.clone(), e));
            }
        }
    }

    let valid_roots = deduplicate_and_prune_subpaths(valid_roots);

    PathResolutionResult {
        valid_roots,
        invalid_paths,
    }
}

/// Description: Expands home shorthand prefix or exact shorthand in input path and returns search path.
/// Arguments/Returns: Accepts input string and optional home path, returns resolved PathBuf.
/// Errors: Returns SearchFailed if home shorthand is present but home path cannot be determined.
pub fn resolve_search_path(input: &str, home_dir: Option<&Path>) -> Result<PathBuf, PathError> {
    // Constant reference: crate::constants::HOME_PATH_SHORTHAND_EXACT
    let is_exact_home = input == crate::constants::HOME_PATH_SHORTHAND_EXACT;

    #[cfg(windows)]
    let prefixes = [
        crate::constants::HOME_PATH_PREFIX_WINDOWS,
        crate::constants::HOME_PATH_PREFIX_UNIX,
    ];
    #[cfg(not(windows))]
    let prefixes = [crate::constants::HOME_PATH_PREFIX_UNIX];

    let remainder_opt = if is_exact_home {
        Some("")
    } else {
        prefixes
            .iter()
            .find_map(|prefix| input.strip_prefix(prefix))
    };

    if let Some(remainder) = remainder_opt {
        let fallback_home: Option<PathBuf> =
            std::env::var_os(crate::constants::HOME_ENVIRONMENT_VARIABLE)
                .or_else(|| std::env::var_os(crate::constants::WINDOWS_HOME_ENVIRONMENT_VARIABLE))
                .map(PathBuf::from);

        let home = home_dir
            .map(Path::to_path_buf)
            .or(fallback_home)
            .ok_or(PathError::SearchFailed)?;

        if remainder.is_empty() {
            return Ok(home);
        }
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
        complete_directory_path, parse_search_paths, resolve_and_partition_paths,
        resolve_search_path, validate_search_directory, PathError, PathParseError,
    };
    use std::fs;
    #[cfg(windows)]
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_DIRECTORY_COUNT: usize = 12;
    const EXPECTED_COMPLETION_COUNT: usize = 10;

    static TEST_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// Description: Creates an isolated, test-only temporary directory to verify path behavior on real filesystem.
    /// Arguments/Returns: None. Returns created temporary PathBuf with process ID and atomic counter suffix.
    /// Errors: Panics if temporary directory creation fails.
    fn temp_dir() -> PathBuf {
        let count = TEST_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "xlseek-path-tests-{}-{unique}-{count}",
            std::process::id()
        ));
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

    /// Description: Verifies comma splitting, whitespace trimming, and empty token removal.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_parse_search_paths_basic() {
        let input = "  /dir/a , /dir/b  ,  ";
        let parsed = parse_search_paths(input).unwrap();
        assert_eq!(parsed, vec!["/dir/a", "/dir/b"]);
    }

    /// Description: Verifies that quoted paths with spaces and commas are preserved as single path units.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_parse_search_paths_quoted() {
        let input = r#""/path/with, comma", "/path with spaces", /normal/path"#;
        let parsed = parse_search_paths(input).unwrap();
        assert_eq!(
            parsed,
            vec!["/path/with, comma", "/path with spaces", "/normal/path"]
        );
    }

    /// Description: Verifies escaped quotes handling in both CSV style and backslash style.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_parse_search_paths_escaped_quotes() {
        let input = r#""/path/with""quote", "/path/with\"backslash""#;
        let parsed = parse_search_paths(input).unwrap();
        assert_eq!(parsed, vec!["/path/with\"quote", "/path/with\"backslash"]);
    }

    /// Description: Verifies unclosed quote error.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_parse_search_paths_unclosed_quote() {
        let input = r#""/unclosed/path, /other/path"#;
        assert_eq!(
            parse_search_paths(input),
            Err(PathParseError::UnclosedQuote)
        );
    }

    /// Description: Verifies empty input error.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_parse_search_paths_empty() {
        assert_eq!(
            parse_search_paths("   , ,  "),
            Err(PathParseError::EmptyInput)
        );
    }

    /// Description: Verifies resolve_and_partition_paths separates valid directories from invalid ones.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_resolve_and_partition_paths() {
        let valid_dir = temp_dir();
        let invalid_path = "/non_existent_path_xyz_12345".to_string();
        let paths = vec![
            valid_dir.to_string_lossy().to_string(),
            invalid_path.clone(),
        ];

        let result = resolve_and_partition_paths(&paths, None);
        assert_eq!(result.valid_roots.len(), 1);
        assert_eq!(result.invalid_paths.len(), 1);
        assert_eq!(result.invalid_paths[0].0, invalid_path);
        assert_eq!(result.invalid_paths[0].1, PathError::NotFound);

        let _ = fs::remove_dir_all(valid_dir);
    }

    /// Description: Verifies that exact duplicates and descendant subpaths are pruned while preserving ancestors.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_deduplicate_and_prune_subpaths() {
        let parent = temp_dir();
        let child = parent.join("child");
        let grand_child = child.join("grand");
        fs::create_dir_all(&grand_child).expect("directories should be created");

        let paths = vec![
            parent.clone(),
            child.clone(),
            grand_child.clone(),
            parent.clone(),
        ];
        let pruned = super::deduplicate_and_prune_subpaths(paths);
        assert_eq!(pruned, vec![parent.clone()]);

        let _ = fs::remove_dir_all(parent);
    }

    /// Description: Verifies that order of parent and child does not affect pruning result (order independence).
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_subpath_order_independence() {
        let parent = temp_dir();
        let child = parent.join("child");
        fs::create_dir_all(&child).expect("directory should be created");

        // Child specified before parent
        let paths = vec![child.clone(), parent.clone()];
        let pruned = super::deduplicate_and_prune_subpaths(paths);
        assert_eq!(pruned, vec![parent.clone()]);

        let _ = fs::remove_dir_all(parent);
    }

    /// Description: Verifies that sibling directories with shared prefix are NOT falsely pruned (component boundary check).
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_sibling_prefix_collision_not_pruned() {
        let root = temp_dir();
        let report_dir = root.join("report");
        let report_archive = root.join("report_archive");
        fs::create_dir_all(&report_dir).expect("report dir created");
        fs::create_dir_all(&report_archive).expect("report archive created");

        let paths = vec![report_dir.clone(), report_archive.clone()];
        let pruned = super::deduplicate_and_prune_subpaths(paths);
        assert_eq!(pruned.len(), 2);
        assert!(pruned.contains(&report_dir));
        assert!(pruned.contains(&report_archive));

        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies that an invalid subpath still emits warning and valid ancestor is preserved.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_invalid_subpath_warning_preserved() {
        let parent = temp_dir();
        let missing_child = parent.join("missing_sub");
        let paths = vec![
            parent.to_string_lossy().to_string(),
            missing_child.to_string_lossy().to_string(),
        ];

        let result = resolve_and_partition_paths(&paths, None);
        assert_eq!(result.valid_roots.len(), 1);
        assert_eq!(result.invalid_paths.len(), 1);
        assert_eq!(
            result.invalid_paths[0].0,
            missing_child.to_string_lossy().to_string()
        );

        let _ = fs::remove_dir_all(parent);
    }

    /// Description: Verifies that relative segments and trailing separators normalize to clean canonical paths without verbatim prefix.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_relative_and_verbatim_normalization() {
        let root = temp_dir();
        let child = root.join("child");
        fs::create_dir_all(&child).expect("directory created");

        let relative_input = child.join("..").join("child");
        let normalized = super::normalize_canonical_path(&relative_input);

        let expected_clean = super::normalize_canonical_path(&child);
        assert_eq!(normalized, expected_clean);
        // Ensure no Windows verbatim prefix
        assert!(!normalized.to_string_lossy().starts_with(r"\\?\"));

        let _ = fs::remove_dir_all(root);
    }

    /// Description: Verifies SC-004: Deduplication and subpath pruning completes in under 50ms for 50 paths.
    /// Arguments/Returns: None.
    /// Errors: Panics on test failure.
    #[test]
    fn test_deduplicate_subpaths_performance_50_paths() {
        let root = temp_dir();
        let mut paths = Vec::with_capacity(50);
        for index in 0..50 {
            let dir = root.join(format!("dir_{index:02}"));
            fs::create_dir_all(&dir).expect("dir created");
            paths.push(dir);
        }

        let start = std::time::Instant::now();
        let pruned = super::deduplicate_and_prune_subpaths(paths);
        let elapsed = start.elapsed();

        assert_eq!(pruned.len(), 50);
        assert!(
            elapsed.as_millis() < 50,
            "Pruning 50 paths took {:?}",
            elapsed
        );

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

//! # CLI Argument Parser
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Validates command-line arguments and converts them into search requests or help requests.
//! Supports short options (`-p`, `-q`, etc.), positional arguments (1st arg: query, 2nd+ args: path(s)),
//! multiple input paths, format inference, and stdout mode (when `-o` is omitted).
//!
//! ## Arguments / Returns
//! Receives operating system arguments and returns `ParseOutcome` or a user-facing error string.
//!
//! ## Errors
//! Returns `Err` on non-Unicode arguments, missing/duplicate/unknown arguments, or invalid paths/queries.

use crate::constants;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use xlseek_core::models::{DirectorySearchMode, ExportFormat, SearchQuery};

/// Represents the output destination target for CLI search results (file path or stdout streaming).
///
/// ## Arguments / Returns
/// `File` holds the destination `PathBuf`, while `Stdout` represents standard output when destination is omitted.
///
/// ## Errors
/// Simple value carrier that produces no errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliOutputTarget {
    File(PathBuf),
    Stdout,
}

/// Retains search and output settings for a single CLI invocation.
///
/// ## Arguments / Returns
/// Stores input path list, output target, search query options, export format, language, and overwrite flag.
///
/// ## Errors
/// Encapsulates validated values only and produces no errors after construction.
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub query: SearchQuery,
    pub input_paths: Vec<PathBuf>,
    pub output_target: CliOutputTarget,
    pub format: ExportFormat,
    pub language: String,
    pub overwrite: bool,
}

/// Represents whether parsed arguments specify an execution request or a help request.
///
/// ## Arguments / Returns
/// `Run` contains `CliOptions`, while `Help` contains the translated help text in the requested language.
///
/// ## Errors
/// Simple value carrier that produces no errors.
pub enum ParseOutcome {
    Run(CliOptions),
    Help(String),
}

/// Parses operating system arguments according to CLI contracts and validates input/output paths and SearchQuery.
/// Handles short options, positional arguments, format inference from extensions, and stdout mode when output is omitted.
///
/// ## Arguments / Returns
/// Accepts OS arguments iterator and translation catalogs, returning `Result<ParseOutcome, String>`.
///
/// ## Errors
/// Returns `Err` on invalid options, paths, extensions, regex syntax, duplicate definitions, or missing translations.
pub fn parse_args<I, S>(
    args: I,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<ParseOutcome, String>
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let mut values = BTreeMap::new();
    let mut positional_args = Vec::new();
    let mut help = false;
    let mut after_terminator = false;

    let mut iterator = args.into_iter().map(|value| {
        value
            .into()
            .into_string()
            .map_err(|_| constants::ERR_CLI_NON_UNICODE.to_string())
    });

    while let Some(argument) = iterator.next() {
        let argument = argument?;

        // Treat everything after "--" terminator as positional arguments
        if after_terminator {
            positional_args.push(argument);
            continue;
        }

        // Check for option terminator "--"
        // Constant reference: constants::CLI_OPTION_TERMINATOR
        if argument == constants::CLI_OPTION_TERMINATOR {
            after_terminator = true;
            continue;
        }

        // Check for help option
        // Constant reference: constants::CLI_SHORT_HELP, constants::CLI_LONG_HELP
        if argument == constants::CLI_SHORT_HELP || argument == constants::CLI_LONG_HELP {
            if help {
                return Err(format!("{}: {}", constants::ERR_CLI_DUPLICATE, argument));
            }
            help = true;
            continue;
        }

        // Long options (--name or --name=value)
        // Constant reference: constants::CLI_OPTION_PREFIX
        if argument.starts_with(constants::CLI_OPTION_PREFIX) {
            let (name, inline_value) =
                match argument.split_once(constants::CLI_ASSIGNMENT_SEPARATOR) {
                    Some((name, value)) => (name.to_string(), Some(value.to_string())),
                    None => (argument, None),
                };
            let option_name = name
                .strip_prefix(constants::CLI_OPTION_PREFIX)
                .ok_or_else(|| format!("{}: {name}", constants::ERR_CLI_UNKNOWN))?;

            // Constant reference: constants::CLI_OVERWRITE_FIELD
            if option_name == constants::CLI_OVERWRITE_FIELD {
                if inline_value.is_some()
                    || values
                        .insert(option_name.to_string(), String::new())
                        .is_some()
                {
                    return Err(format!("{}: {name}", constants::ERR_CLI_DUPLICATE));
                }
                continue;
            }

            // Constant reference: constants::CLI_VALUE_OPTIONS
            if !constants::CLI_VALUE_OPTIONS.contains(&option_name) {
                return Err(format!("{}: {name}", constants::ERR_CLI_UNKNOWN));
            }
            if values.contains_key(option_name) {
                return Err(format!("{}: {name}", constants::ERR_CLI_DUPLICATE));
            }

            let value = match inline_value {
                Some(value) => value,
                None => iterator
                    .next()
                    .ok_or_else(|| format!("{}: {name}", constants::ERR_CLI_REQUIRED))??,
            };
            values.insert(option_name.to_string(), value);
            continue;
        }

        // Short options (-x or -x=value)
        // Constant reference: constants::CLI_SHORT_OPTION_PREFIX
        if argument.starts_with(constants::CLI_SHORT_OPTION_PREFIX)
            && argument != constants::CLI_SHORT_OPTION_PREFIX
        {
            let (short_flag, inline_value) =
                match argument.split_once(constants::CLI_ASSIGNMENT_SEPARATOR) {
                    Some((flag, value)) => (flag.to_string(), Some(value.to_string())),
                    None => (argument, None),
                };

            // Constant reference: constants::CLI_SHORT_TO_LONG_OPTIONS
            let (_, long_field) = constants::CLI_SHORT_TO_LONG_OPTIONS
                .iter()
                .find(|(short, _)| *short == short_flag)
                .ok_or_else(|| format!("{}: {short_flag}", constants::ERR_CLI_UNKNOWN))?;

            if *long_field == constants::CLI_HELP_FIELD {
                if help {
                    return Err(format!("{}: {}", constants::ERR_CLI_DUPLICATE, short_flag));
                }
                help = true;
                continue;
            }

            if *long_field == constants::CLI_OVERWRITE_FIELD {
                if inline_value.is_some()
                    || values
                        .insert((*long_field).to_string(), String::new())
                        .is_some()
                {
                    return Err(format!("{}: {short_flag}", constants::ERR_CLI_DUPLICATE));
                }
                continue;
            }

            if values.contains_key(*long_field) {
                return Err(format!("{}: {short_flag}", constants::ERR_CLI_DUPLICATE));
            }

            let value = match inline_value {
                Some(value) => value,
                None => iterator
                    .next()
                    .ok_or_else(|| format!("{}: {short_flag}", constants::ERR_CLI_REQUIRED))??,
            };
            values.insert((*long_field).to_string(), value);
            continue;
        }

        // Positional argument
        positional_args.push(argument);
    }

    // Process help request
    if help {
        if !positional_args.is_empty()
            || values
                .keys()
                .any(|key| key != constants::CLI_LANGUAGE_FIELD)
        {
            return Err(constants::ERR_CLI_HELP_OPTIONS.to_string());
        }
        let language = values
            .get(constants::CLI_LANGUAGE_FIELD)
            .map(String::as_str)
            .unwrap_or(constants::CLI_DEFAULT_LANGUAGE);
        validate_language(language)?;
        let help = xlseek_core::i18n::resolve_catalog_text(
            catalogs,
            language,
            constants::CLI_TRANSLATION_HELP_KEY,
        )
        .ok_or_else(|| constants::ERR_TRANSLATION_MISSING.to_string())?;
        return Ok(ParseOutcome::Help(help));
    }

    // Resolve query and input path(s) (mutual exclusion between positional and named args)
    let (query_str, input_paths) = match positional_args.as_slice() {
        [] => {
            // No positional args: --query and --path are required
            let query = values
                .get(constants::CLI_QUERY_FIELD)
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "{}: --{}",
                        constants::ERR_CLI_REQUIRED,
                        constants::CLI_QUERY_FIELD
                    )
                })?;
            let path_arg = values.get(constants::CLI_PATH_FIELD).ok_or_else(|| {
                format!(
                    "{}: --{}",
                    constants::ERR_CLI_REQUIRED,
                    constants::CLI_PATH_FIELD
                )
            })?;
            let parsed_paths = xlseek_core::search::path::parse_search_paths(path_arg)
                .map_err(|_| constants::ERR_CLI_INPUT_PATH.to_string())?;
            let mut resolved_paths = Vec::with_capacity(parsed_paths.len());
            for p in parsed_paths {
                resolved_paths.push(resolve_input_path(&p)?);
            }
            (query, resolved_paths)
        }
        [first_pos] => {
            // Single positional argument: 1st argument is query
            if values.contains_key(constants::CLI_QUERY_FIELD) {
                return Err(format!(
                    "{}: --{}",
                    constants::ERR_CLI_DUPLICATE,
                    constants::CLI_QUERY_FIELD
                ));
            }
            let path_arg = values.get(constants::CLI_PATH_FIELD).ok_or_else(|| {
                format!(
                    "{}: --{}",
                    constants::ERR_CLI_REQUIRED,
                    constants::CLI_PATH_FIELD
                )
            })?;
            let parsed_paths = xlseek_core::search::path::parse_search_paths(path_arg)
                .map_err(|_| constants::ERR_CLI_INPUT_PATH.to_string())?;
            let mut resolved_paths = Vec::with_capacity(parsed_paths.len());
            for p in parsed_paths {
                resolved_paths.push(resolve_input_path(&p)?);
            }
            (first_pos.clone(), resolved_paths)
        }
        [first_pos, remaining_paths @ ..] => {
            // Two or more positional arguments: 1st is query, 2nd+ are paths
            if values.contains_key(constants::CLI_QUERY_FIELD) {
                return Err(format!(
                    "{}: --{}",
                    constants::ERR_CLI_DUPLICATE,
                    constants::CLI_QUERY_FIELD
                ));
            }
            if values.contains_key(constants::CLI_PATH_FIELD) {
                return Err(format!(
                    "{}: --{}",
                    constants::ERR_CLI_DUPLICATE,
                    constants::CLI_PATH_FIELD
                ));
            }
            let mut paths = Vec::with_capacity(remaining_paths.len());
            for p in remaining_paths {
                paths.push(resolve_input_path(p)?);
            }
            (first_pos.clone(), paths)
        }
    };

    if query_str.trim().is_empty() {
        return Err(constants::ERR_CLI_EMPTY_QUERY.to_string());
    }

    // Resolve output destination and export format (automatic format inference)
    let output_arg_opt = values.get(constants::CLI_OUTPUT_FIELD);
    let format_arg_opt = values.get(constants::CLI_FORMAT_FIELD).map(String::as_str);

    let (output_target, format) = match output_arg_opt {
        Some(output_arg) => {
            let output_path = resolve_output_path(output_arg)?;
            let ext = output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default();

            let format = match format_arg_opt {
                Some(constants::CLI_FORMAT_CSV) => {
                    if ext.eq_ignore_ascii_case(constants::CLI_FORMAT_CSV) {
                        ExportFormat::Csv
                    } else {
                        return Err(constants::ERR_CLI_FORMAT.to_string());
                    }
                }
                Some(constants::CLI_FORMAT_XLSX) => {
                    if ext.eq_ignore_ascii_case(constants::CLI_FORMAT_XLSX) {
                        ExportFormat::Xlsx
                    } else {
                        return Err(constants::ERR_CLI_FORMAT.to_string());
                    }
                }
                Some(_) => return Err(constants::ERR_CLI_FORMAT.to_string()),
                None => {
                    // Automatic format inference
                    if ext.eq_ignore_ascii_case(constants::CLI_FORMAT_CSV) {
                        ExportFormat::Csv
                    } else if ext.eq_ignore_ascii_case(constants::CLI_FORMAT_XLSX) {
                        ExportFormat::Xlsx
                    } else {
                        return Err(constants::ERR_CLI_FORMAT.to_string());
                    }
                }
            };
            (CliOutputTarget::File(output_path), format)
        }
        None => {
            // Default to stdout streaming mode when output is omitted. Format is CSV only.
            let format = match format_arg_opt {
                Some(constants::CLI_FORMAT_CSV) | None => ExportFormat::Csv,
                _ => return Err(constants::ERR_CLI_FORMAT.to_string()),
            };
            (CliOutputTarget::Stdout, format)
        }
    };

    let language = values
        .get(constants::CLI_LANGUAGE_FIELD)
        .map(String::as_str)
        .unwrap_or(constants::CLI_DEFAULT_LANGUAGE);
    validate_language(language)?;

    let extensions = values
        .get(constants::CLI_EXTENSIONS_FIELD)
        .map(|value| {
            value
                .split(constants::CLI_LIST_SEPARATOR)
                .map(str::trim)
                .map(|extension| {
                    let extension = extension.trim_start_matches(constants::CLI_EXTENSION_PREFIX);
                    format!(
                        "{}{}",
                        constants::CLI_EXTENSION_PREFIX,
                        extension.to_ascii_lowercase()
                    )
                })
                .filter(|extension| extension.len() > constants::CLI_EXTENSION_PREFIX.len())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            constants::DEFAULT_EXTENSIONS
                .iter()
                .map(|value| value.to_string())
                .collect()
        });

    if extensions.is_empty()
        || extensions
            .iter()
            .any(|ext| !constants::DEFAULT_EXTENSIONS.contains(&ext.as_str()))
    {
        return Err(constants::ERR_CLI_EXTENSIONS.to_string());
    }

    for input_path in &input_paths {
        if input_path.is_file() {
            let extension = input_path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default();
            let extension = format!(
                "{}{}",
                constants::CLI_EXTENSION_PREFIX,
                extension.to_ascii_lowercase()
            );
            if !extensions.contains(&extension) {
                return Err(constants::ERR_CLI_EXTENSIONS.to_string());
            }
        }
    }

    let match_case = parse_bool(
        &values,
        constants::CLI_MATCH_CASE_FIELD,
        constants::CLI_DEFAULT_FALSE,
    )?;
    let use_regex = parse_bool(
        &values,
        constants::CLI_REGEX_FIELD,
        constants::CLI_DEFAULT_FALSE,
    )?;
    let include_value = parse_bool(
        &values,
        constants::CLI_VALUES_FIELD,
        constants::DEFAULT_INCLUDE_VALUE,
    )?;
    let include_formula = parse_bool(
        &values,
        constants::CLI_FORMULAS_FIELD,
        constants::CLI_DEFAULT_INCLUDE_FORMULA,
    )?;
    let include_comment = parse_bool(
        &values,
        constants::CLI_COMMENTS_FIELD,
        constants::CLI_DEFAULT_INCLUDE_COMMENT,
    )?;
    let include_shape = parse_bool(
        &values,
        constants::CLI_SHAPES_FIELD,
        constants::CLI_DEFAULT_INCLUDE_SHAPE,
    )?;
    let include_hidden = parse_bool(
        &values,
        constants::CLI_HIDDEN_SHEETS_FIELD,
        constants::CLI_DEFAULT_INCLUDE_HIDDEN,
    )?;

    // Constant reference: constants::CLI_DIRECTORY_MODE_FIELD, constants::CLI_BURST_WORKERS_FIELD.
    let requested_workers = values
        .get(constants::CLI_BURST_WORKERS_FIELD)
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|_| constants::ERR_CLI_DIRECTORY_MODE.to_string())
        })
        .transpose()?;
    if requested_workers.is_some_and(|count| {
        !(constants::BURST_WORKERS_MIN..=constants::BURST_WORKERS_MAX).contains(&count)
    }) {
        return Err(constants::ERR_CLI_DIRECTORY_MODE.to_string());
    }
    let directory_mode = match values
        .get(constants::CLI_DIRECTORY_MODE_FIELD)
        .map(String::as_str)
    {
        Some(constants::DIRECTORY_MODE_SEQUENTIAL) if requested_workers.is_some() => {
            return Err(constants::ERR_CLI_DIRECTORY_MODE.to_string());
        }
        Some(constants::DIRECTORY_MODE_SEQUENTIAL) => DirectorySearchMode::Sequential,
        Some(constants::DIRECTORY_MODE_BURST) => DirectorySearchMode::Burst,
        Some(_) => return Err(constants::ERR_CLI_DIRECTORY_MODE.to_string()),
        None if requested_workers.is_some() => DirectorySearchMode::Burst,
        None => DirectorySearchMode::Sequential,
    };

    if !(include_value || include_formula || include_comment || include_shape) {
        return Err(constants::ERR_CLI_NO_SEARCH_TYPES.to_string());
    }

    if use_regex {
        regex::RegexBuilder::new(&query_str)
            .case_insensitive(!match_case)
            .build()
            .map_err(|_| constants::ERR_CLI_REGEX.to_string())?;
    }

    let representative_dir = input_paths
        .first()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default();

    Ok(ParseOutcome::Run(CliOptions {
        query: SearchQuery {
            keyword: query_str,
            target_dir: representative_dir,
            directory_mode,
            burst_workers: requested_workers,
            match_case,
            include_value,
            use_regex,
            include_formula,
            include_comment,
            include_shape,
            include_hidden,
            extensions,
        },
        input_paths,
        output_target,
        format,
        language: language.to_string(),
        overwrite: values.contains_key(constants::CLI_OVERWRITE_FIELD),
    }))
}

/// Parses boolean option from explicit map value or defaults.
///
/// ## Arguments / Returns
/// Accepts value map, option name, and default boolean, returning the parsed boolean.
///
/// ## Errors
/// Returns `Err` if value is neither "true" nor "false".
fn parse_bool(
    values: &BTreeMap<String, String>,
    name: &str,
    default: bool,
) -> Result<bool, String> {
    match values.get(name).map(String::as_str) {
        None => Ok(default),
        Some(constants::CLI_BOOLEAN_TRUE) => Ok(true),
        Some(constants::CLI_BOOLEAN_FALSE) => Ok(false),
        _ => Err(format!("{}: --{name}", constants::ERR_CLI_VALUE)),
    }
}

/// Normalizes path to absolute path and validates it as existing file or directory.
///
/// ## Arguments / Returns
/// Accepts user input `&str` and returns existing canonical `PathBuf`.
///
/// ## Errors
/// Returns `Err` on non-existence, non-file/directory, or canonicalization failure.
fn resolve_input_path(input: &str) -> Result<PathBuf, String> {
    let path = expand_home(input)?;
    let path =
        std::fs::canonicalize(path).map_err(|_| constants::ERR_CLI_INPUT_PATH.to_string())?;
    if !path.is_file() && !path.is_dir() {
        return Err(constants::ERR_CLI_INPUT_PATH.to_string());
    }
    Ok(path)
}

/// Validates destination output path as an absolute file path within an existing parent directory.
///
/// ## Arguments / Returns
/// Accepts user input `&str` and returns canonicalized parent `PathBuf`.
///
/// ## Errors
/// Returns `Err` if missing parent, missing filename, or existing output is directory / symlink.
fn resolve_output_path(input: &str) -> Result<PathBuf, String> {
    let path = expand_home(input)?;
    let file_name = path
        .file_name()
        .ok_or_else(|| constants::ERR_CLI_OUTPUT_PATH.to_string())?;
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or(Path::new(constants::CLI_CURRENT_DIRECTORY));
    let parent =
        std::fs::canonicalize(parent).map_err(|_| constants::ERR_CLI_OUTPUT_PATH.to_string())?;
    let output = parent.join(file_name);
    if let Ok(metadata) = std::fs::symlink_metadata(&output) {
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(constants::ERR_CLI_OUTPUT_PATH.to_string());
        }
    }
    Ok(output)
}

/// Expands leading `~` in path using home directory environment variables.
///
/// ## Arguments / Returns
/// Accepts input path `&str` and returns expanded `PathBuf`.
///
/// ## Errors
/// Returns `Err` if home directory environment variable is absent.
fn expand_home(input: &str) -> Result<PathBuf, String> {
    // Constant reference: constants::HOME_PATH_PREFIX_UNIX, constants::HOME_ENVIRONMENT_VARIABLE
    if let Some(suffix) = input.strip_prefix(constants::HOME_PATH_PREFIX_UNIX) {
        let home = std::env::var_os(constants::HOME_ENVIRONMENT_VARIABLE)
            .or_else(|| std::env::var_os(constants::WINDOWS_HOME_ENVIRONMENT_VARIABLE))
            .ok_or_else(|| constants::ERR_CLI_HOME.to_string())?;
        return Ok(PathBuf::from(home).join(suffix));
    }
    Ok(PathBuf::from(input))
}

/// Validates that language code corresponds to supported translation locales.
///
/// ## Arguments / Returns
/// Accepts language code `&str` and returns `Ok(())` if valid.
///
/// ## Errors
/// Returns `Err` if language is not "ja" or "en".
fn validate_language(language: &str) -> Result<(), String> {
    if language == constants::CLI_LOCALE_JA || language == constants::CLI_LOCALE_EN {
        Ok(())
    } else {
        Err(constants::ERR_CLI_LANGUAGE.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_args, CliOutputTarget, ParseOutcome};
    use crate::constants;
    use xlseek_core::models::ExportFormat;

    /// Resolves the absolute path to the repository root.
    ///
    /// ## Arguments / Returns
    /// Takes no arguments; returns `PathBuf`.
    ///
    /// ## Errors
    /// Panics if repository root cannot be determined.
    fn repo_root() -> std::path::PathBuf {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        if manifest.join("../../tests/fixtures").exists() {
            manifest.join("../..")
        } else {
            manifest
                .parent()
                .expect("repository root must exist")
                .to_path_buf()
        }
    }

    /// Verifies parsing of required CLI arguments and fixed default parameters.
    ///
    /// ## Arguments / Returns
    /// Tests argument resolution against expected SearchQuery defaults.
    ///
    /// ## Errors
    /// Panics if catalogs or arguments fail to parse.
    #[test]
    fn parses_required_arguments_and_fixed_defaults() {
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");
        let output = std::env::temp_dir().join(constants::CLI_TEST_OUTPUT_NAME);
        let args = [
            "--path".to_string(),
            input.to_string_lossy().into_owned(),
            "--query".to_string(),
            constants::CLI_TEST_HYPHEN_QUERY.to_string(),
            "--format".to_string(),
            constants::CLI_FORMAT_CSV.to_string(),
            "--output".to_string(),
            output.to_string_lossy().into_owned(),
        ];
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, constants::CLI_TEST_HYPHEN_QUERY);
        assert_eq!(options.input_paths.len(), 1);
        assert_eq!(
            options.input_paths[0],
            std::fs::canonicalize(&input).unwrap()
        );
        assert_eq!(
            options.output_target,
            CliOutputTarget::File(
                std::fs::canonicalize(output.parent().unwrap())
                    .unwrap()
                    .join(output.file_name().unwrap())
            )
        );
        assert_eq!(options.format, ExportFormat::Csv);
        assert!(options.query.include_value);
        assert!(options.query.include_formula);
        assert!(options.query.include_comment);
        assert!(!options.query.include_shape);
        assert!(!options.query.include_hidden);
        assert_eq!(options.language, constants::CLI_DEFAULT_LANGUAGE);
        assert!(!options.overwrite);
    }

    /// Tests positional arguments (query, multiple paths) and default stdout streaming mode.
    ///
    /// ## Arguments / Returns
    /// Verifies extracted query, input paths, stdout output target, and default CSV format.
    ///
    /// ## Errors
    /// Panics if assertions fail.
    #[test]
    fn parses_positional_arguments_and_stdout() {
        let root = repo_root();
        let input1 = root.join("tests/fixtures/sample_report.xlsx");
        let input2 = root.join("tests/fixtures");
        let args = [
            "search_keyword".to_string(),
            input1.to_string_lossy().into_owned(),
            input2.to_string_lossy().into_owned(),
        ];
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, "search_keyword");
        assert_eq!(options.input_paths.len(), 2);
        assert_eq!(options.output_target, CliOutputTarget::Stdout);
        assert_eq!(options.format, ExportFormat::Csv);
    }

    /// Verifies parsing of short options (-q, -p, -o, -f, -w, -c, -r, -l) and format auto-inference.
    ///
    /// ## Arguments / Returns
    /// Tests short options resolution and XLSX format inference from output file extension.
    ///
    /// ## Errors
    /// Panics if parsing fails.
    #[test]
    fn parses_short_options_and_infers_format() {
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");
        let output = std::env::temp_dir().join("result.xlsx");
        let args = [
            "-q".to_string(),
            "needle".to_string(),
            "-p".to_string(),
            input.to_string_lossy().into_owned(),
            "-o".to_string(),
            output.to_string_lossy().into_owned(),
            "-w".to_string(),
            "-c=true".to_string(),
            "-l".to_string(),
            "en".to_string(),
        ];
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, "needle");
        assert_eq!(options.format, ExportFormat::Xlsx);
        assert!(options.overwrite);
        assert!(options.query.match_case);
        assert_eq!(options.language, "en");
    }

    /// Verifies rejection when positional arguments duplicate named options.
    ///
    /// ## Arguments / Returns
    /// Verifies duplicate error responses.
    ///
    /// ## Errors
    /// Panics if duplicate input is erroneously accepted.
    #[test]
    fn rejects_duplicate_positional_and_named() {
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");

        // Duplicate query
        let args1 = [
            "pos_query".to_string(),
            "-q".to_string(),
            "named_query".to_string(),
            "-p".to_string(),
            input.to_string_lossy().into_owned(),
        ];
        assert!(parse_args(args1, &catalogs).is_err());

        // Duplicate path
        let args2 = [
            "pos_query".to_string(),
            input.to_string_lossy().into_owned(),
            "-p".to_string(),
            input.to_string_lossy().into_owned(),
        ];
        assert!(parse_args(args2, &catalogs).is_err());
    }

    /// Verifies that unknown short flags are rejected.
    ///
    /// ## Arguments / Returns
    /// Tests rejection of unknown flag.
    ///
    /// ## Errors
    /// Panics if unknown flag is accepted.
    #[test]
    fn rejects_unknown_short_option() {
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let args = ["-z".to_string(), "val".to_string()];
        assert!(parse_args(args, &catalogs).is_err());
    }

    /// Verifies that `--help` with `--language=en` succeeds without requiring search arguments.
    ///
    /// ## Arguments / Returns
    /// Verifies returned help text starts with English usage.
    ///
    /// ## Errors
    /// Panics if help request fails or runs search.
    #[test]
    fn help_accepts_language_without_search_options() {
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let result = parse_args(
            [
                constants::CLI_LONG_HELP.to_string(),
                format!("--{}=en", constants::CLI_LANGUAGE_FIELD),
            ],
            &catalogs,
        )
        .unwrap();
        let ParseOutcome::Help(help) = result else {
            panic!("help request must not run search");
        };
        assert!(help.starts_with(constants::CLI_TEST_ENGLISH_USAGE_PREFIX));
    }

    /// Verifies that a Burst worker count can select Burst without a mode option.
    ///
    /// ## Arguments / Returns
    /// Parses a directory search request and asserts it is accepted.
    ///
    /// ## Errors
    /// Panics if valid count-only Burst syntax is rejected.
    #[test]
    fn burst_worker_count_without_mode_is_accepted() {
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");
        let result = parse_args(
            [
                format!("--{}", constants::CLI_QUERY_FIELD),
                constants::CLI_TEST_HYPHEN_QUERY.to_string(),
                format!("--{}", constants::CLI_PATH_FIELD),
                input.to_string_lossy().into_owned(),
                "--burst-workers".to_string(),
                "6".to_string(),
            ],
            &catalogs,
        );
        let ParseOutcome::Run(options) = result.unwrap() else {
            panic!("count-only request must start a search");
        };
        assert_eq!(
            options.query.directory_mode,
            xlseek_core::models::DirectorySearchMode::Burst
        );
        assert_eq!(options.query.burst_workers, Some(6));
    }

    /// Rejects invalid worker counts and a count conflicting with explicit Sequential mode.
    ///
    /// ## Arguments / Returns
    /// Parses invalid requests and asserts each returns a usage error.
    ///
    /// ## Errors
    /// Panics if any invalid option combination is accepted.
    #[test]
    fn rejects_invalid_burst_worker_counts_and_sequential_conflicts() {
        let catalogs = xlseek_core::i18n::load_embedded_catalogs().unwrap();
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");
        for count in ["1", "33", "2.5"] {
            let result = parse_args(
                [
                    "--query".to_string(),
                    constants::CLI_TEST_HYPHEN_QUERY.to_string(),
                    "--path".to_string(),
                    input.to_string_lossy().into_owned(),
                    "--burst-workers".to_string(),
                    count.to_string(),
                ],
                &catalogs,
            );
            assert!(result.is_err());
        }
        let conflict = parse_args(
            [
                "--query".to_string(),
                constants::CLI_TEST_HYPHEN_QUERY.to_string(),
                "--path".to_string(),
                input.to_string_lossy().into_owned(),
                "--directory-mode".to_string(),
                "sequential".to_string(),
                "--burst-workers".to_string(),
                "2".to_string(),
            ],
            &catalogs,
        );
        assert!(conflict.is_err());
    }
}

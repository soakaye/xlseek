//! ## 処理内容
//! コマンドライン引数を検証し、検索要求またはヘルプ要求へ変換する。
//! ## 引数・戻り値
//! OS引数を受け取り、`ParseOutcome` または利用者向けエラー文字列を返す。
//! ## エラー
//! 非Unicode、不足・重複・未知の引数、無効なパスや検索条件はErrとする。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): CLI引数解析を追加。

use crate::constants;
use crate::models::{ExportFormat, SearchQuery};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// ## 処理内容
/// CLI一回分の検索および保存設定を保持する。
/// ## 引数・戻り値
/// 入力・出力パス、検索条件、形式、言語、上書き指定を格納する。
/// ## エラー
/// 検証済み値のみ保持し、構築後にエラーを発生させない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI要求型を追加。
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub query: SearchQuery,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub format: ExportFormat,
    pub language: String,
    pub overwrite: bool,
}

/// ## 処理内容
/// 引数解析結果が実行要求かヘルプ要求かを表す。
/// ## 引数・戻り値
/// Runは`CliOptions`、Helpは選択言語の翻訳本文を保持する。
/// ## エラー
/// 値保持のみでエラーを発生させない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 引数解析結果型を追加。
pub enum ParseOutcome {
    Run(CliOptions),
    Help(String),
}

/// ## 処理内容
/// OS引数をCLI契約に従って解析し、入力・出力パスとSearchQueryを検証する。
/// ## 引数・戻り値
/// OS引数イテレーター、翻訳辞書を受け、`Result<ParseOutcome, String>` を返す。
/// ## エラー
/// 不正オプション、パス、拡張子、正規表現、翻訳欠落ではErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 標準ライブラリによる引数解析を追加。
pub fn parse_args<I, S>(
    args: I,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<ParseOutcome, String>
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let mut values = BTreeMap::new();
    let mut help = false;
    let mut iterator = args.into_iter().map(|value| {
        value
            .into()
            .into_string()
            .map_err(|_| constants::ERR_CLI_NON_UNICODE.to_string())
    });
    while let Some(argument) = iterator.next() {
        let argument = argument?;
        if argument == constants::CLI_SHORT_HELP || argument == constants::CLI_LONG_HELP {
            if help {
                return Err(format!("{}: {}", constants::ERR_CLI_DUPLICATE, argument));
            }
            help = true;
            continue;
        }
        let (name, inline_value) = match argument.split_once(constants::CLI_ASSIGNMENT_SEPARATOR) {
            Some((name, value)) => (name.to_string(), Some(value.to_string())),
            None => (argument, None),
        };
        let option_name = name
            .strip_prefix(constants::CLI_OPTION_PREFIX)
            .ok_or_else(|| format!("{}: {name}", constants::ERR_CLI_UNKNOWN))?;
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
    }
    if help {
        if values
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
        let help = crate::i18n::resolve_catalog_text(
            catalogs,
            language,
            constants::CLI_TRANSLATION_HELP_KEY,
        )
        .ok_or_else(|| constants::ERR_TRANSLATION_MISSING.to_string())?;
        return Ok(ParseOutcome::Help(help));
    }
    for required in constants::CLI_REQUIRED_OPTIONS {
        if !values.contains_key(required) {
            return Err(format!("{}: --{required}", constants::ERR_CLI_REQUIRED));
        }
    }

    let input_arg = get(&values, constants::CLI_PATH_FIELD)?;
    let input_path = resolve_input_path(input_arg)?;
    let output_arg = get(&values, constants::CLI_OUTPUT_FIELD)?;
    let output_path = resolve_output_path(output_arg)?;
    let format = match get(&values, constants::CLI_FORMAT_FIELD)? {
        constants::CLI_FORMAT_CSV
            if output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case(constants::CLI_FORMAT_CSV)) =>
        {
            ExportFormat::Csv
        }
        constants::CLI_FORMAT_XLSX
            if output_path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case(constants::CLI_FORMAT_XLSX)) =>
        {
            ExportFormat::Xlsx
        }
        _ => return Err(constants::ERR_CLI_FORMAT.to_string()),
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
    let query = get(&values, constants::CLI_QUERY_FIELD)?.to_string();
    if query.trim().is_empty() {
        return Err(constants::ERR_CLI_EMPTY_QUERY.to_string());
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
    if !(include_value || include_formula || include_comment || include_shape) {
        return Err(constants::ERR_CLI_NO_SEARCH_TYPES.to_string());
    }
    if use_regex {
        regex::RegexBuilder::new(&query)
            .case_insensitive(!match_case)
            .build()
            .map_err(|_| constants::ERR_CLI_REGEX.to_string())?;
    }
    Ok(ParseOutcome::Run(CliOptions {
        query: SearchQuery {
            keyword: query,
            target_dir: input_path.to_string_lossy().into_owned(),
            match_case,
            include_value,
            use_regex,
            include_formula,
            include_comment,
            include_shape,
            include_hidden,
            extensions,
        },
        input_path,
        output_path,
        format,
        language: language.to_string(),
        overwrite: values.contains_key(constants::CLI_OVERWRITE_FIELD),
    }))
}

/// ## 処理内容
/// 必須文字列オプションの値を取得する。
/// ## 引数・戻り値
/// 解析済み値辞書とオプション名を受け、借用文字列を返す。
/// ## エラー
/// 値が欠落している場合はErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 引数値取得を追加。
fn get<'a>(values: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str, String> {
    values
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| format!("{}: --{name}", constants::ERR_CLI_REQUIRED))
}

/// ## 処理内容
/// 真偽値オプションを明示値または既定値から解析する。
/// ## 引数・戻り値
/// 値辞書、オプション名、bool既定値を受け、boolを返す。
/// ## エラー
/// `true` と `false` 以外の指定でErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 真偽値解析を追加。
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

/// ## 処理内容
/// パスを絶対化し、入力ファイルまたはディレクトリとして検証する。
/// ## 引数・戻り値
/// 利用者入力 `&str` を受け、既存の絶対`PathBuf`を返す。
/// ## エラー
/// 不存在、非ファイル/ディレクトリ、canonicalize失敗でErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 検索入力パス検証を追加。
fn resolve_input_path(input: &str) -> Result<PathBuf, String> {
    let path = expand_home(input)?;
    let path =
        std::fs::canonicalize(path).map_err(|_| constants::ERR_CLI_INPUT_PATH.to_string())?;
    if !path.is_file() && !path.is_dir() {
        return Err(constants::ERR_CLI_INPUT_PATH.to_string());
    }
    Ok(path)
}

/// ## 処理内容
/// 出力先を既存親ディレクトリ内の絶対ファイルパスとして検証する。
/// ## 引数・戻り値
/// 利用者入力 `&str` を受け、親を正規化した`PathBuf`を返す。
/// ## エラー
/// 親がない、ファイル名がない、既存出力がsymlink/ディレクトリの場合はErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 出力パス検証を追加。
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

/// ## 処理内容
/// `~` で始まるパスをOSホーム環境変数から展開する。
/// ## 引数・戻り値
/// 入力パス`&str`を受け、展開した`PathBuf`を返す。
/// ## エラー
/// ホーム環境変数がない場合はErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): ホーム省略パス対応を追加。
fn expand_home(input: &str) -> Result<PathBuf, String> {
    if let Some(suffix) = input.strip_prefix(constants::HOME_PATH_PREFIX_UNIX) {
        let home = std::env::var_os(constants::HOME_ENVIRONMENT_VARIABLE)
            .or_else(|| std::env::var_os(constants::WINDOWS_HOME_ENVIRONMENT_VARIABLE))
            .ok_or_else(|| constants::ERR_CLI_HOME.to_string())?;
        return Ok(PathBuf::from(home).join(suffix));
    }
    Ok(PathBuf::from(input))
}

/// ## 処理内容
/// 言語コードが翻訳対応言語のいずれかであることを確認する。
/// ## 引数・戻り値
/// 言語コード`&str`を受け、妥当ならunitを返す。
/// ## エラー
/// ja/en以外でErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 言語検証を追加。
fn validate_language(language: &str) -> Result<(), String> {
    if language == constants::CLI_LOCALE_JA || language == constants::CLI_LOCALE_EN {
        Ok(())
    } else {
        Err(constants::ERR_CLI_LANGUAGE.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_args, ParseOutcome};
    use crate::constants;

    /// ## 処理内容
    /// 必須引数と固定既定値からCLI検索要求を構築し、ハイフン始まりの検索語を保つ。
    /// ## 引数・戻り値
    /// 引数なし。解析結果と既定SearchQuery各項目を検証する。
    /// ## エラー
    /// カタログ・パス・解析・期待値が不正ならテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): CLI既定値解析テストを追加。
    #[test]
    fn parses_required_arguments_and_fixed_defaults() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
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
        let catalogs = crate::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, constants::CLI_TEST_HYPHEN_QUERY);
        assert!(options.query.include_value);
        assert!(options.query.include_formula);
        assert!(options.query.include_comment);
        assert!(!options.query.include_shape);
        assert!(!options.query.include_hidden);
        assert_eq!(options.language, constants::CLI_DEFAULT_LANGUAGE);
        assert!(!options.overwrite);
    }

    /// ## 処理内容
    /// 英語ヘルプ指定を検索引数なしで受け付け、検索実行を要求しない。
    /// ## 引数・戻り値
    /// 引数なし。ヘルプ文面が英語であることを検証する。
    /// ## エラー
    /// 翻訳欠落または解析結果不一致でテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 言語付きヘルプテストを追加。
    #[test]
    fn help_accepts_language_without_search_options() {
        let catalogs = crate::i18n::load_embedded_catalogs().unwrap();
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
}

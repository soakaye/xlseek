//! ## 処理内容
//! コマンドライン引数を検証し、検索要求またはヘルプ要求へ変換する。
//! 短縮オプション（-p, -q等）、位置引数（第1引数: クエリ、第2引数以降: パス）、
//! 複数パス指定、フォーマット推論、および標準出力モード（-o 省略時）をサポートする。
//! ## 引数・戻り値
//! OS引数を受け取り、`ParseOutcome` または利用者向けエラー文字列を返す。
//! ## エラー
//! 非Unicode、不足・重複・未知の引数、無効なパスや検索条件はErrとする。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): CLI引数解析を追加。
//! - v1.1.0 (2026-09-29, AI Agent): 短縮オプション、位置引数、複数パス、フォーマット自動推論、標準出力モード対応。

use crate::constants;
use exlgrep_core::models::{ExportFormat, SearchQuery};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// ## 処理内容
/// CLI検索結果の出力先ターゲット（ファイル保存または標準出力ストリーミング）を表す。
/// ## 引数・戻り値
/// Fileは保存先PathBufを保持し、Stdoutは出力先省略時の標準出力を表す。
/// ## エラー
/// 値保持のみでエラーを発生させない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, AI Agent): 出力先ターゲット列挙型を追加。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliOutputTarget {
    File(PathBuf),
    Stdout,
}

/// ## 処理内容
/// CLI一回分の検索および保存設定を保持する。
/// ## 引数・戻り値
/// 入力パス一覧、出力先ターゲット、検索条件、形式、言語、上書き指定を格納する。
/// ## エラー
/// 検証済み値のみ保持し、構築後にエラーを発生させない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): CLI要求型を追加。
/// - v1.1.0 (2026-09-29, AI Agent): 複数入力パスおよび出力先ターゲット対応へ更新。
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub query: SearchQuery,
    pub input_paths: Vec<PathBuf>,
    pub output_target: CliOutputTarget,
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
/// 短縮オプション、位置引数、拡張子からのフォーマット自動推論、出力省略時のStdout設定を行う。
/// ## 引数・戻り値
/// OS引数イテレーター、翻訳辞書を受け、`Result<ParseOutcome, String>` を返す。
/// ## エラー
/// 不正オプション、パス、拡張子、正規表現、重複指定、翻訳欠落ではErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 標準ライブラリによる引数解析を追加。
/// - v1.1.0 (2026-09-29, AI Agent): 短縮オプション、位置引数、複数パス、自動推論、標準出力対応。
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

        // 終端記号 "--" 以降はすべて位置引数として扱う
        if after_terminator {
            positional_args.push(argument);
            continue;
        }

        // オプション終端記号 "--" の判定
        // 定数参照: constants::CLI_OPTION_TERMINATOR
        if argument == constants::CLI_OPTION_TERMINATOR {
            after_terminator = true;
            continue;
        }

        // ヘルプオプションの判定
        // 定数参照: constants::CLI_SHORT_HELP, constants::CLI_LONG_HELP
        if argument == constants::CLI_SHORT_HELP || argument == constants::CLI_LONG_HELP {
            if help {
                return Err(format!("{}: {}", constants::ERR_CLI_DUPLICATE, argument));
            }
            help = true;
            continue;
        }

        // ロングオプション (--name または --name=value)
        // 定数参照: constants::CLI_OPTION_PREFIX
        if argument.starts_with(constants::CLI_OPTION_PREFIX) {
            let (name, inline_value) =
                match argument.split_once(constants::CLI_ASSIGNMENT_SEPARATOR) {
                    Some((name, value)) => (name.to_string(), Some(value.to_string())),
                    None => (argument, None),
                };
            let option_name = name
                .strip_prefix(constants::CLI_OPTION_PREFIX)
                .ok_or_else(|| format!("{}: {name}", constants::ERR_CLI_UNKNOWN))?;

            // 定数参照: constants::CLI_OVERWRITE_FIELD
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

            // 定数参照: constants::CLI_VALUE_OPTIONS
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

        // ショートオプション (-x または -x=value)
        // 定数参照: constants::CLI_SHORT_OPTION_PREFIX
        if argument.starts_with(constants::CLI_SHORT_OPTION_PREFIX)
            && argument != constants::CLI_SHORT_OPTION_PREFIX
        {
            let (short_flag, inline_value) =
                match argument.split_once(constants::CLI_ASSIGNMENT_SEPARATOR) {
                    Some((flag, value)) => (flag.to_string(), Some(value.to_string())),
                    None => (argument, None),
                };

            // 定数参照: constants::CLI_SHORT_TO_LONG_OPTIONS
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

        // 位置引数
        positional_args.push(argument);
    }

    // ヘルプ要求の処理
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
        let help = exlgrep_core::i18n::resolve_catalog_text(
            catalogs,
            language,
            constants::CLI_TRANSLATION_HELP_KEY,
        )
        .ok_or_else(|| constants::ERR_TRANSLATION_MISSING.to_string())?;
        return Ok(ParseOutcome::Help(help));
    }

    // クエリおよびパスの解決（位置引数と名前付きオプションの排他検査）
    let (query_str, input_paths) = match positional_args.as_slice() {
        [] => {
            // 位置引数なし: --query と --path が必須
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
            let resolved = resolve_input_path(path_arg)?;
            (query, vec![resolved])
        }
        [first_pos] => {
            // 位置引数が1つ: 第1引数はクエリ
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
            let resolved = resolve_input_path(path_arg)?;
            (first_pos.clone(), vec![resolved])
        }
        [first_pos, remaining_paths @ ..] => {
            // 位置引数が2つ以上: 第1引数はクエリ、第2引数以降はパス
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

    // 出力先およびフォーマットの解決（フォーマット自動推論）
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
                    // フォーマット自動推論
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
            // 出力省略時は標準出力（stdout）モード。フォーマットは CSV のみ対応。
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
    // 定数参照: constants::HOME_PATH_PREFIX_UNIX, constants::HOME_ENVIRONMENT_VARIABLE
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
    use super::{parse_args, CliOutputTarget, ParseOutcome};
    use crate::constants;
    use exlgrep_core::models::ExportFormat;

    /// ## 処理内容
    /// リポジトリルートの絶対パスを解決する。
    /// ## 引数・戻り値
    /// 引数なし。`PathBuf` を返す。
    /// ## エラー
    /// ルートが見つからない場合は panic する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Antigravity): クレート分離に伴うルートパス解決。
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

    /// ## 処理内容
    /// 必須引数と固定既定値からCLI検索要求を構築し、ハイフン始まりの検索語を保つ。
    /// ## 引数・戻り値
    /// 引数なし。解析結果と既定SearchQuery各項目を検証する。
    /// ## エラー
    /// カタログ・パス・解析・期待値が不正ならテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): CLI既定値解析テストを追加。
    /// - v1.1.0 (2026-09-29, AI Agent): input_paths, output_target に追従。
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
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
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

    /// ## 処理内容
    /// 位置引数（第1=クエリ、第2以降=複数パス）と出力先省略時の標準出力（stdout）モード判定を検証する。
    /// ## 引数・戻り値
    /// 引数なし。位置引数からクエリ・複数パスが抽出され、output_target が Stdout、format が Csv になることを確認する。
    /// ## エラー
    /// 解析結果が不一致の場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, AI Agent): 位置引数と標準出力テストを追加。
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
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, "search_keyword");
        assert_eq!(options.input_paths.len(), 2);
        assert_eq!(options.output_target, CliOutputTarget::Stdout);
        assert_eq!(options.format, ExportFormat::Csv);
    }

    /// ## 処理内容
    /// 短縮オプション（-q, -p, -o, -f, -w, -c, -r, -l, -e）およびフォーマット自動推論を検証する。
    /// ## 引数・戻り値
    /// 引数なし。短縮オプションの値が反映され、拡張子からXlsx形式が自動推論されることを確認する。
    /// ## エラー
    /// 解析結果が不一致の場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, AI Agent): 短縮オプションとフォーマット推論テストを追加。
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
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
        let ParseOutcome::Run(options) = parse_args(args, &catalogs).unwrap() else {
            panic!("valid request must run");
        };
        assert_eq!(options.query.keyword, "needle");
        assert_eq!(options.format, ExportFormat::Xlsx);
        assert!(options.overwrite);
        assert!(options.query.match_case);
        assert_eq!(options.language, "en");
    }

    /// ## 処理内容
    /// 位置引数と名前付き引数（-q, -p）の重複指定時にエラーが返されることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。重複エラーが返されることを確認する。
    /// ## エラー
    /// 想定通りのエラーが返されない場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, AI Agent): 重複指定拒否テストを追加。
    #[test]
    fn rejects_duplicate_positional_and_named() {
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
        let root = repo_root();
        let input = root.join("tests/fixtures/sample_report.xlsx");

        // クエリ重複
        let args1 = [
            "pos_query".to_string(),
            "-q".to_string(),
            "named_query".to_string(),
            "-p".to_string(),
            input.to_string_lossy().into_owned(),
        ];
        assert!(parse_args(args1, &catalogs).is_err());

        // パス重複
        let args2 = [
            "pos_query".to_string(),
            input.to_string_lossy().into_owned(),
            "-p".to_string(),
            input.to_string_lossy().into_owned(),
        ];
        assert!(parse_args(args2, &catalogs).is_err());
    }

    /// ## 処理内容
    /// 未知の短縮オプションが指定された場合にエラーが返されることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。未知オプションエラーが返されることを確認する。
    /// ## エラー
    /// 想定通りのエラーが返されない場合にテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, AI Agent): 未知短縮オプション拒否テストを追加。
    #[test]
    fn rejects_unknown_short_option() {
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
        let args = ["-z".to_string(), "val".to_string()];
        assert!(parse_args(args, &catalogs).is_err());
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
        let catalogs = exlgrep_core::i18n::load_embedded_catalogs().unwrap();
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

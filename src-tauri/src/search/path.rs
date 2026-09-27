//! 処理内容: 検索ディレクトリの解決・読取検証・候補取得を提供する。
//! 入出力: 利用者入力とホームディレクトリから検証済みパスまたは候補を返す。
//! エラー: 不正・不存在・読取拒否のパスは型付きエラーで返す。
//! 変更履歴: v1.0.0 (2026-09-27, Codex): パス機能テストを先行追加。v1.1.0 (2026-09-27, Codex): パス検証・解決・補完を実装。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// 処理内容: ディレクトリ検証時に発生する失敗を分類する。
/// 引数・戻り値: なし。各エラー種別を保持する。
/// エラー: なし。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): パス検証エラーを追加。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    NotFound,
    PermissionDenied,
    SearchFailed,
}

/// 処理内容: 入力パスのホーム省略表記を展開して検索用パスを返す。
/// 引数・戻り値: 入力文字列と任意のホームパスを受け、解決した PathBuf を返す。
/// エラー: 省略表記にホームパスが必要なのに渡されない場合は SearchFailed を返す。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): ホーム省略表記の解決を追加。
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

/// 処理内容: 検索対象ディレクトリを開いて読取可能であることを確認する。
/// 引数・戻り値: 検証する Path を受け、成功時は unit、失敗時は分類済みエラーを返す。
/// エラー: 不存在・非ディレクトリ・権限拒否・その他の I/O エラーを PathError に変換する。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): 検索前の読取検証を追加。
pub fn validate_search_directory(path: &Path) -> Result<(), PathError> {
    let mut entries = fs::read_dir(path).map_err(classify_io_error)?;
    if let Some(entry) = entries.next() {
        entry.map_err(classify_io_error)?;
    }
    Ok(())
}

/// 処理内容: I/O エラーを検索パスの公開エラー種別へ変換する。
/// 引数・戻り値: std::io::Error を受け、PathError を返す。
/// エラー: なし。未知の I/O エラーは SearchFailed として扱う。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): I/O エラー分類を追加。
fn classify_io_error(error: io::Error) -> PathError {
    match error.kind() {
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory => PathError::NotFound,
        io::ErrorKind::PermissionDenied => PathError::PermissionDenied,
        _ => PathError::SearchFailed,
    }
}

/// 処理内容: 絶対パスまたはホーム省略表記の親直下から一致するディレクトリを列挙する。
/// 引数・戻り値: パス入力と任意のホームパスを受け、表示用候補の文字列配列を返す。
/// エラー: 対象外・読取不可・不正な入力では空配列を返し、例外は送出しない。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): ディレクトリ補完を追加。
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

/// 処理内容: 補完対象パスが絶対パス、またはホーム省略表記由来で安全な接頭辞か確認する。
/// 引数・戻り値: 展開後 Path と省略表記判定を受け、対象なら true を返す。
/// エラー: なし。対象外の形式は false を返す。
/// 変更履歴: v1.0.0 (2026-09-27, Codex): 補完対象パス判定を追加。
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
        return matches!(
            path.components().next(),
            Some(Component::Prefix(prefix))
                if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::UNC(_))
        );
    }
    #[cfg(not(windows))]
    {
        let _ = is_home_input;
        true
    }
}

#[cfg(test)]
mod tests {
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

    /// 処理内容: テスト専用の一時ディレクトリを作り、実ファイルシステムでパス動作を確認する。
    /// 引数・戻り値: なし。作成した一時ディレクトリを返す。
    /// エラー: 一時ディレクトリを作れない場合はテストを失敗させる。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): パステスト用一時領域を追加。
    fn temp_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("exlgrep-path-tests-{unique}"));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("temporary directory should be created");
        path
    }

    /// 処理内容: ホーム省略表記を指定したホームディレクトリへ解決する。
    /// 引数・戻り値: なし。解決結果の実パスを検証する。
    /// エラー: 期待するパスへ解決されない場合は失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): ホームパス解決テストを追加。
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

    /// 処理内容: 存在しない検索対象と通常ファイルを拒否する。
    /// 引数・戻り値: なし。検証関数のエラー種別を確認する。
    /// エラー: 対象が拒否されない場合は失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 検索パス拒否テストを追加。
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

    /// 処理内容: 親直下の一致するディレクトリだけを整列して返す。
    /// 引数・戻り値: なし。候補一覧を検証する。
    /// エラー: 通常ファイルや順序違いが候補に含まれる場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 候補列挙の振る舞いテストを追加。
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

    /// 処理内容: 候補を名前順に並べた後で最大件数に制限する。
    /// 引数・戻り値: なし。上限内の先頭候補を検証する。
    /// エラー: 返却数超過または順序違いで失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 候補上限テストを追加。
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

    /// 処理内容: ホーム省略表記の補完候補を同じ表記で返す。
    /// 引数・戻り値: なし。表示用候補の文字列を検証する。
    /// エラー: 実パス表記へ置換される場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): ホーム表記維持テストを追加。
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

    /// 処理内容: 空白と日本語を含むディレクトリ名を候補として維持する。
    /// 引数・戻り値: なし。候補文字列の完全一致を検証する。
    /// エラー: 候補が欠落・変質する場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): Unicode パス候補テストを追加。
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

    /// 処理内容: 末尾区切り文字がある入力では入力先の直下を列挙する。
    /// 引数・戻り値: なし。候補一覧を検証する。
    /// エラー: 末尾区切り文字を名前として扱う場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 末尾区切り文字のテストを追加。
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

    /// 処理内容: 相対パスを補完対象から除外する。
    /// 引数・戻り値: なし。候補が空であることを検証する。
    /// エラー: 相対パスから候補が返る場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 相対入力拒否テストを追加。
    #[test]
    fn ignores_relative_completion_input() {
        assert!(complete_directory_path("reports/Al", None).is_empty());
    }

    /// 処理内容: 検索可能なディレクトリを受付前検証で受け付ける。
    /// 引数・戻り値: なし。既存ディレクトリの検証結果を確認する。
    /// エラー: 読取可能なディレクトリが拒否された場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): 読取可能パスの検証テストを追加。
    #[test]
    fn accepts_readable_search_directory() {
        let root = temp_dir();
        assert_eq!(validate_search_directory(&root), Ok(()));
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(windows)]
    /// 処理内容: Windows の対象パス種別だけを補完し、相対・デバイス系を除く。
    /// 引数・戻り値: なし。各パス形式の候補有無を検証する。
    /// エラー: 非対応の Windows パスに候補が出た場合に失敗する。
    /// 変更履歴: v1.0.0 (2026-09-27, Codex): Windows パス種別テストを追加。
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

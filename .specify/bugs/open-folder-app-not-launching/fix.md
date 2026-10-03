# Bug Fix: ファイルの場所を開くでファイル管理アプリが起動しない問題

- **Slug**: open-folder-app-not-launching
- **Fixed**: 2026-10-02T18:19:55+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

`open_file_in_folder_linux` において、`dbus-send` の実行引数に `--print-reply` および `--reply-timeout=2000` を追加して D-Bus メソッド呼び出しの成否を正確に検知するようにしました。
さらに、D-Bus `FileManager1` が利用できない環境向けに、ファイル選択オプション付きのダイレクトファイルマネージャ起動フォールバック（`nautilus --select` / `dolphin --select` / `nemo --no-desktop`）に加え、WSL (Windows Subsystem for Linux) 環境で動作する Windows エクスプローラー連携（`wslpath -w` 経由の `explorer.exe /select,<win_path>` 起動）を追加し、後続の `gio open` / `xdg-open` / `open::that` へ確実にフォールバックするように改修しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/constants.rs` | modified | `DBUS_ARG_PRINT_REPLY`, `DBUS_ARG_REPLY_TIMEOUT`, `CMD_NAUTILUS`, `CMD_DOLPHIN`, `CMD_NEMO`, `ARG_SELECT`, `ARG_NO_DESKTOP`, `CMD_EXPLORER`, `CMD_WSL_EXPLORER`, `ARG_EXPLORER_SELECT_PREFIX`, `CMD_WSLPATH`, `WSLPATH_ARG_WINDOWS` 定数および定数検証テストを追加 |
| `src-tauri/src/commands/system_cmd.rs` | modified | `build_dbus_show_items_args`, `convert_wsl_path_to_windows`, `try_open_in_wsl_explorer` の新設、`open_file_in_folder_linux` での `--print-reply` 付与・ダイレクトファイルマネージャ・WSL Explorer フォールバックの追加、単体テストの追加 |

## Diff Highlights

```rust
// src-tauri/src/commands/system_cmd.rs
#[cfg(target_os = "linux")]
pub fn convert_wsl_path_to_windows(path: &Path) -> Option<String> {
    let output = std::process::Command::new(crate::constants::CMD_WSLPATH)
        .arg(crate::constants::WSLPATH_ARG_WINDOWS)
        .arg(path)
        .output()
        .ok()?;

    if output.status.success() {
        let win_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !win_path.is_empty() {
            return Some(win_path);
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn try_open_in_wsl_explorer(path: &Path) -> Result<(), String> {
    let win_path = convert_wsl_path_to_windows(path).ok_or_else(|| {
        crate::constants::ERR_FOLDER_OPEN.to_string()
    })?;

    let select_arg = format!(
        "{}{}",
        crate::constants::ARG_EXPLORER_SELECT_PREFIX,
        win_path
    );

    let spawn_res = std::process::Command::new(crate::constants::CMD_WSL_EXPLORER)
        .arg(&select_arg)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    if spawn_res.is_ok() {
        return Ok(());
    }

    let parent_path = path.parent().unwrap_or(path);
    if let Some(win_parent) = convert_wsl_path_to_windows(parent_path) {
        let dir_spawn = std::process::Command::new(crate::constants::CMD_WSL_EXPLORER)
            .arg(&win_parent)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();

        if dir_spawn.is_ok() {
            return Ok(());
        }
    }

    Err(crate::constants::ERR_FOLDER_OPEN.to_string())
}
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_linux_desktop_constants`: 追加された Linux / WSL / Explorer デスクトップ定数の値が正しいことを検証。
- `src-tauri/src/commands/system_cmd.rs::tests::test_build_dbus_show_items_args`: `build_dbus_show_items_args` が `--print-reply`、`--reply-timeout=2000`、宛先、メソッド、URI 形式を正しく含むことを検証。
- `src-tauri/src/commands/system_cmd.rs::tests::test_convert_wsl_path_to_windows`: WSL 環境下で `wslpath` による Windows パスへの変換が機能することを検証。
- `src-tauri/src/commands/system_cmd.rs::tests::test_open_file_in_folder_linux_existing`: 実ファイルに対して利用可能なデスクトップオープナー経由で処理が完遂することを検証。

## Local Verification

- `cargo test --workspace`: PASS (全 71 件のテストすべて合格)
- `cargo test -p xlseek --lib`: PASS (16 passed)
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS (警告 0 件)
- `cargo fmt --check`: PASS (フォーマット差分 0 件)
- `npm run build`: PASS (TypeScript 型エラー 0 件、バンドル成功)
- `npx vitest run --pool=threads`: PASS (19 ファイル、125 件全パス)

## Deviations from Assessment

Extended remediation scope to include WSL (Windows Subsystem for Linux) environment support via `wslpath -w` and `explorer.exe /select,...`, as Ubuntu on WSL lacks native Linux GUI file manager daemons and relies on Windows Explorer integration.

## Follow-ups

- 次の検証ステップ: `/speckit-bug-test slug=open-folder-app-not-launching`

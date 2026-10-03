# Research & Technical Decisions: Fix Open Folder Functionality on Ubuntu

**Feature**: `023-fix-ubuntu-open-folder`
**Date**: 2026-10-02

## 1. Background & Problem Analysis

In `xlseek`, the preview panel provides an "Open in Folder" button to locate the found file on the host operating system (`src-tauri/src/commands/system_cmd.rs` -> `open_in_folder`).
- **Windows**: Invokes `explorer.exe /select,<path>`.
- **macOS**: Invokes `open -R <file_path>`.
- **Linux / Ubuntu**: Currently has no dedicated platform block and falls straight into fallback:
  ```rust
  if let Some(parent) = path.parent() {
      open::that(parent)
  }
  ```
  In many Linux environments (including Ubuntu GNOME, XFCE, headless or containerized sessions, or where the `open` crate's detection fails), `open::that(parent)` can fail, spawn improperly, or fail to highlight the target file. Furthermore, the user confirmed (in clarification session 2026-10-02) that the system must attempt to highlight the target file first on Linux before falling back to opening the parent folder.

## 2. Research Findings & Options for Linux Folder Opening

### Option 1: org.freedesktop.FileManager1 DBus Interface (`dbus-send`)
- **How it works**: The Freedesktop standard defines `org.freedesktop.FileManager1.ShowItems` method:
  ```bash
  dbus-send --session --dest=org.freedesktop.FileManager1 --type=method_call \
    /org/freedesktop/FileManager1 org.freedesktop.FileManager1.ShowItems \
    array:string:"file://<encoded_path>" string:""
  ```
- **Supported File Managers**: GNOME Files (Nautilus), KDE Dolphin, PCManFM-Qt, Nemo. This is the official desktop standard for highlighting files in a file manager on Linux.
- **Pros**:
  - Highlights the specific file in Nautilus on Ubuntu out of the box.
  - Returns exit code 0 on success.
- **Cons**:
  - DBus session may not have an active `FileManager1` provider running if the user uses a minimal WM (like i3, Sway, openbox, or headless environment).
  - Requires file URI format (`file://...`).

### Option 2: `gio open <parent_path>`
- **How it works**: `gio open <path_or_uri>` is the modern replacement for `gvfs-open` and part of `glib-networking` / `libglib2.0-bin`, installed by default on almost all GNOME/Ubuntu and Debian-based systems.
- **Pros**:
  - Standard command on Ubuntu desktop (`/usr/bin/gio`).
  - Reliably opens directories with the user's default registered file manager.
- **Cons**:
  - `gio open` does not support selecting/highlighting a file; it opens whatever application is associated with that path/URI (for a directory, the file manager).

### Option 3: `xdg-open <parent_path>`
- **How it works**: Standard XDG utility across Linux desktops.
- **Pros**: Universally recognized fallback across Linux distributions.
- **Cons**: Does not highlight files, only opens the directory. Sometimes `xdg-open` can hang if invoked incorrectly or without detaching stdio.

### Option 4: `open::that(parent)` (Current Fallback)
- **How it works**: Uses the Rust `open` crate (`open v5.4.4`).
- **Pros**: Cross-platform fallback.
- **Cons**: May fail depending on available tools in PATH or environment variables.

## 3. Decision Matrix & Strategy

| Method | Highlights File? | Environment Requirements | Role in Implementation |
|---|---|---|---|
| **DBus `FileManager1.ShowItems`** | Yes | Active desktop session, DBus | **Primary Attempt (Highlight)** |
| **`gio open <parent>`** | No (opens folder) | GNOME / GLib environment | **Secondary Fallback** |
| **`xdg-open <parent>`** | No (opens folder) | XDG tools installed | **Tertiary Fallback** |
| **`open::that(parent)`** | No (opens folder) | General system handler | **Final Fallback** |

### Implementation Design
On Linux (`#[cfg(target_os = "linux")]`):
1. Construct file URI: `file://` + percent-encoded absolute path (or canonicalized path).
2. Execute `dbus-send` calling `org.freedesktop.FileManager1.ShowItems`.
   If it exits with success (code 0), return `Ok(())`.
3. If DBus call fails or is not available, retrieve `path.parent().unwrap_or(path)`.
4. Try `gio open <parent>`:
   If `gio open` succeeds (exit code 0), return `Ok(())`.
5. Try `xdg-open <parent>`:
   If `xdg-open` succeeds (exit code 0), return `Ok(())`.
6. Finally, try `open::that(parent)`.
7. If all attempts fail, return `Err(ERR_FOLDER_OPEN)`.

## 4. Constitution & Constants Compliance
- All command names (`dbus-send`, `gio`, `xdg-open`), arguments, and DBus interface names must be defined in `src-tauri/src/constants.rs` with explicit constant references.
- All new/modified functions must include comprehensive Rustdoc comments describing functionality, parameters, return types, and errors.
- Unit tests must be written to verify URI construction, argument formatting, and error handling.

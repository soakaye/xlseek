# Data Model: Fix Open Folder Functionality on Ubuntu

**Feature**: `023-fix-ubuntu-open-folder`
**Date**: 2026-10-02

## 1. Overview

This feature enhances the system command layer to open the containing folder of a search result on Linux/Ubuntu. While no new persistent database models or user-configurable records are required, several in-memory data structures and command parameters define the interface between the frontend, Tauri IPC, and the operating system.

## 2. In-Memory Entities & Schemas

### 2.1 File Path & URI Representation
- **Input**: `file_path: String` (received via Tauri command argument `open_in_folder`)
- **Path Validation**: Must resolve to a valid existing file on the filesystem via `std::path::Path::new(&file_path).exists()`.
- **File URI**: Formatted as `file://<percent_encoded_path>` for DBus `org.freedesktop.FileManager1.ShowItems`.
  - Prefix: `file://`
  - Escaped: Absolute filesystem path percent-encoded to preserve non-ASCII characters (e.g. Japanese kanji/kana, spaces, percent signs).

### 2.2 Error Codes (`ErrorCode` Enum)
Defined in `crates/core/src/models/mod.rs` and returned in `CommandError`:

| Code | Value (`snake_case`) | Condition | User Impact |
|---|---|---|---|
| `PathNotFound` | `"path_not_found"` | File or folder does not exist at invocation time | Toast: Path not found |
| `FolderOpenFailed` | `"folder_open_failed"` | All desktop file manager opening strategies failed | Toast: Could not open folder |
| `InternalError` | `"internal_error"` | Unexpected runtime failure in blocking task | Toast: Internal error |

### 2.3 Operating System Dispatch Entity (Internal)

```text
LinuxFolderOpenStrategy (Priority Pipeline):
1. DBusShowItems:
   - Command: "dbus-send"
   - Arguments:
       "--session",
       "--dest=org.freedesktop.FileManager1",
       "--type=method_call",
       "/org/freedesktop/FileManager1",
       "org.freedesktop.FileManager1.ShowItems",
       "array:string:file://<path>",
       "string:"
   - Outcome: If exit code == 0 -> Success (Item highlighted in file manager).

2. GioOpenParent:
   - Command: "gio"
   - Arguments: "open", "<parent_directory>"
   - Outcome: If exit code == 0 -> Success (Parent directory opened).

3. XdgOpenParent:
   - Command: "xdg-open"
   - Arguments: "<parent_directory>"
   - Outcome: If exit code == 0 -> Success (Parent directory opened).

4. OpenCrateFallback:
   - Function: open::that("<parent_directory>")
   - Outcome: If Ok(()) -> Success.

5. Exhaustion:
   - Return Err(CommandError { code: ErrorCode::FolderOpenFailed })
```

## 3. Validation Rules
1. **Empty / Non-Existent Path**: If `file_path` is empty or does not exist, return `ErrorCode::PathNotFound` immediately before spawning blocking tasks or child processes.
2. **Safety Against Shell Injection**: `std::process::Command` must be used with individual argument slices (`.args([...])`), never string interpolation in a shell (`sh -c`).

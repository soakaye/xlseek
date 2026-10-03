# Contract: open_in_folder Tauri IPC Command

**Feature**: `023-fix-ubuntu-open-folder`
**Type**: Tauri IPC Command Contract

## 1. Command Definition

- **Command Name**: `open_in_folder`
- **Constant Identifier**: `COMMANDS.OPEN_IN_FOLDER` (`"open_in_folder"`)
- **Backend Function**: `crate::commands::system_cmd::open_in_folder`

## 2. Request

Invoked from frontend (React/TypeScript) via `@tauri-apps/api/core::invoke`:

```typescript
await invoke<void>("open_in_folder", {
  filePath: "/absolute/path/to/target_file.xlsx"
});
```

### Parameters
| Name | Type | Required | Description |
|---|---|---|---|
| `filePath` | `string` | Yes | Absolute path to the file whose containing directory should be opened |

## 3. Response

### Success
- Returns `void` / Rust `Ok(())` (HTTP status code / IPC success).
- Host system launches default file manager displaying containing folder (and highlighting the file if supported on the platform).

### Errors
Returns structured `CommandError`:

```typescript
interface CommandError {
  code: "invalid_regex" | "path_not_found" | "workbook_open_failed" | "preview_failed" | "export_failed" | "app_launch_failed" | "folder_open_failed" | "permission_denied" | "search_failed" | "internal_error";
}
```

#### Error Scenarios:
1. **Path Not Found**:
   - `code`: `"path_not_found"`
   - Condition: `filePath` does not exist on the filesystem at invocation time.
2. **Folder Open Failed**:
   - `code`: `"folder_open_failed"`
   - Condition: All attempts to open the folder (DBus, GIO, XDG, open crate) failed or returned non-zero status.
3. **Internal Error**:
   - `code`: `"internal_error"`
   - Condition: Async runtime thread join failure.

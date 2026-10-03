# Feature Specification: Fix Open Folder Functionality on Ubuntu

**Feature Branch**: `023-fix-ubuntu-open-folder`

**Created**: 2026-10-02

**Status**: Draft

**Input**: User description: "Ubuntu でフォルダを開く機能が動作しない問題に対応する"

## Clarifications

### Session 2026-10-02

- Q: When opening the containing folder on Linux, what should be the primary behavior regarding the target file within the directory? → A: Attempt file selection/highlighting first, falling back cleanly to opening the parent folder.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reveal Found File in Default File Manager on Linux / Ubuntu (Priority: P1)

As an xlseek user running Ubuntu (or a compatible Linux desktop environment),
When I search for Excel files and select an action to open the containing folder of a search result,
I want the system to reliably launch the default desktop file manager (e.g., Nautilus/Files) showing the folder or highlighting the target file,
So that I can immediately manage, inspect, or copy the file without getting a silent failure or an error.

**Why this priority**:
Currently, on Windows and macOS, the "Open Folder" action navigates directly to the enclosing directory (and highlights the file where supported). On Linux/Ubuntu, folder opening fails or does not execute reliably, breaking parity with other platforms and disrupting the user workflow after searching. Fixing this is essential to delivering a functional Linux desktop experience.

**Independent Test**:
Can be fully tested on an Ubuntu desktop session by searching for an Excel file, triggering the "Open in Folder" action on a search result, and verifying that the file manager opens with the enclosing directory.

**Acceptance Scenarios**:

1. **Given** a search result pointing to an existing file on Ubuntu,
   **When** the user triggers "Open in Folder",
   **Then** the desktop environment opens the containing directory in the system's default file manager with the target file highlighted if selection is supported, and the application reports success without showing an error notification.
2. **Given** a search result pointing to an existing file on Ubuntu where desktop file selection is not available,
   **When** the user triggers "Open in Folder",
   **Then** the file manager falls back cleanly to opening the parent directory without an error.

---

### User Story 2 - Resilient Fallback and Clear Error Feedback on Linux (Priority: P2)

As a user on an Ubuntu or Linux system with a non-standard or lightweight desktop configuration,
When I trigger "Open in Folder" for a file,
I want the application to gracefully try reliable fallback mechanisms (such as standard desktop openers or URI handlers) and provide a clear error message if no file manager or desktop handler can be invoked,
So that I know why the action failed rather than experiencing an unhandled silent failure.

**Why this priority**:
Linux environments vary significantly (GNOME/Nautilus, KDE/Dolphin, XFCE/Thunar, headless/WSL without desktop bus). The system must handle edge cases gracefully and avoid hanging or leaving zombie processes.

**Independent Test**:
Can be tested in an environment where primary desktop file manager integration fails or is missing, verifying that the system tries fallback desktop opening tools, and if all fail, surfaces a descriptive error notification.

**Acceptance Scenarios**:

1. **Given** a desktop environment without advanced file selection bus support,
   **When** the user triggers "Open in Folder",
   **Then** the system falls back to opening the parent directory using standard desktop opening utilities.
2. **Given** a target file that has been moved, renamed, or deleted after the search was executed,
   **When** the user clicks "Open in Folder",
   **Then** the application does not crash or freeze, but immediately notifies the user that the file or directory no longer exists.

---

### Edge Cases

- **File deleted or moved after search**: The user triggers "Open in Folder" for a file that was deleted after the search ran. The application must detect that the path does not exist and return an appropriate "Path not found" error without attempting to launch the file manager.
- **Special characters and spaces in path**: The target file or directory contains spaces, multibyte characters (Japanese, etc.), quotes, or symbols. The opening command must correctly handle the path without misinterpreting spaces as separate arguments or suffering shell injection risks.
- **Root or top-level paths**: The target file is in the root directory or has no parent directory beyond root. The system must safely handle the root path without error.
- **Non-standard desktop environments or missing default file manager**: In minimal Linux environments without a registered default file manager, the system must capture the failure cleanly and notify the user with an actionable error.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide an "Open in Folder" action on search result items that functions across all supported desktop platforms, including Ubuntu / Linux.
- **FR-002**: On Ubuntu / Linux, the system MUST attempt to reveal and highlight the target file in the desktop file manager first, and cleanly fall back to opening the containing parent folder if highlighting is unsupported.
- **FR-003**: The system MUST preserve path safety and integrity, properly handling paths containing spaces, Japanese/multibyte characters, and special characters on Linux.
- **FR-004**: If the target file or enclosing folder no longer exists at invocation time, the system MUST display a clear "Path not found" notification and avoid launching any external process.
- **FR-005**: If the desktop file manager cannot be launched or returns a failure status on Linux, the system MUST capture the error and display an informative error notification to the user rather than failing silently.
- **FR-006**: The system MUST execute external file manager launching asynchronously so that the user interface remains responsive and never freezes.

### Key Entities

- **File Path**: The absolute path representing the target Excel file located on the local filesystem.
- **Containing Folder**: The parent directory of the target file path to be displayed in the file manager.
- **Desktop File Manager**: The operating system's default graphical file manager application (e.g., Nautilus on Ubuntu GNOME, Windows Explorer on Windows, Finder on macOS).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of "Open in Folder" actions for existing files on a standard Ubuntu desktop session successfully launch the file manager showing the target directory (with file highlighted where supported) within 1.5 seconds of user invocation.
- **SC-002**: Zero user interface freezes or deadlocks occur when launching the folder view, regardless of whether the external process succeeds or fails.
- **SC-003**: When a target file is deleted prior to clicking "Open in Folder", the user receives an accurate error message within 500 milliseconds, with zero orphan or lingering background processes spawned.
- **SC-004**: Existing "Open in Folder" functionality on Windows and macOS continues to work without regression.

## Assumptions

- The target platform is an Ubuntu / Linux desktop environment with a graphical session and a desktop file manager installed (such as GNOME Files/Nautilus, which is standard on Ubuntu).
- The user has the necessary filesystem permissions to read the enclosing folder and target file.
- When opening on Linux, file selection highlighting is attempted via available desktop mechanisms; if highlighting cannot be performed, opening the parent folder is guaranteed.

# Feature Specification: Bundle xlseek-cli as a Tauri Sidecar

**Feature Branch**: `018-tauri-cli-sidecar`

**Created**: 2026-09-30

**Status**: Draft

**Input**: User description: "Include `xlseek-cli` as a Tauri Sidecar."

## Clarifications

### Session 2026-09-30

- Q: Does this feature include launching the bundled `xlseek-cli` from the Tauri application? → A: Bundle it in the distribution only; launching it from the application is out of scope.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Distribute the desktop application and CLI together (Priority: P1)

Desktop application users can get the corresponding `xlseek-cli` together with the app and use the CLI functionality provided by the desktop release without finding and installing a separate CLI.

**Why this priority**: Providing the app and CLI in the same distribution reduces separate installation effort and version mismatches.

**Independent Test**: Inspect each supported desktop distribution and verify that it contains the matching `xlseek-cli`. After installation, launch the bundled CLI directly and verify its help output and existing search behavior.

**Acceptance Scenarios**:

1. **Given** a desktop application is distributed for a supported OS and CPU configuration, **When** a user installs the distribution, **Then** the corresponding `xlseek-cli` executable is included and can be launched directly from the installation directory.
2. **Given** a desktop application containing the CLI is installed, **When** a user launches the installed CLI directly and requests help, **Then** the CLI displays its available options and exits successfully.
3. **Given** the installed CLI and a search target are available, **When** a user launches the CLI directly to search and export results according to the existing specification, **Then** the CLI returns the same search results, output, and exit status as the existing CLI.

### Edge Cases

- If an `xlseek-cli` executable is not produced for a target OS or CPU configuration, the distribution for that configuration must not be treated as complete while missing the CLI.
- If the CLI cannot launch or exits abnormally, the user must be able to recognize the failure and must not mistake it for success.
- Existing GUI functionality must remain available through the current development startup flow even when the CLI is absent.
- The application and CLI in one distribution must not come from different releases.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Desktop application distributions MUST include an `xlseek-cli` built for every supported OS and CPU configuration.
- **FR-002**: An installed distribution MUST include an `xlseek-cli` executable that users can launch directly. Launching the CLI from the Tauri application is out of scope.
- **FR-003**: If files required to launch the CLI are missing or the CLI cannot execute, the distribution process or execution result MUST clearly report the failure and MUST NOT treat it as success.
- **FR-004**: The desktop application and `xlseek-cli` in one distribution MUST belong to the same release.
- **FR-005**: This feature MUST NOT change existing `xlseek-cli` options, search results, output formats, or exit statuses.
- **FR-006**: Bundling the CLI MUST NOT prevent users from using existing desktop GUI features or the development startup flow.
- **FR-007**: Search and result export MUST remain local according to the existing CLI specification; search targets and queries MUST NOT be sent externally.

### Key Entities

- **Desktop Distribution**: A release unit containing the application users install and a bundled CLI that can also be launched on its own.
- **Bundled CLI**: `xlseek-cli` from the same release as the desktop application, launchable directly from the installation directory.
- **CLI Execution Result**: The CLI launch state, search results, output destination, and process exit status.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of desktop distributions for all supported OS and CPU configurations contain a launchable `xlseek-cli`.
- **SC-002**: For every supported configuration, users can display CLI help and complete a search after installation without an additional download or separate CLI installation.
- **SC-003**: 100% of existing acceptance scenarios for bundled CLI search results, output formats, and exit statuses continue to pass.
- **SC-004**: Users can continue completing the existing primary GUI operations with the CLI bundled.

## Assumptions

- “Include as a Sidecar” means bundling the CLI in the corresponding desktop distribution as a separately launchable executable. Launching it from the Tauri application is out of scope.
- Supported OS and CPU configurations are the same configurations supported for desktop application distribution by this project.
- New GUI operations, search options, and output features for the CLI are out of scope; existing CLI specifications 014/015 remain in effect.
- The CLI may be absent when starting the GUI during development, but it is required in release distributions.

## Out of Scope

- Adding GUI screens or the ability to launch `xlseek-cli` from the Tauri application.
- Changing `xlseek-cli` search/export functionality or command-line interface.
- Adding support for OS or CPU configurations that are not supported by the project.

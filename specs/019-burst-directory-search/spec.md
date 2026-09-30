<!--
Purpose: Defines the user journeys, acceptance criteria, requirements, and assumptions for bounded Burst subfolder discovery.
Inputs: The user's feature request and accepted clarification answers (text); output: a Markdown specification for planning.
Errors: This document has no runtime behavior; unclear or inconsistent requirements require review and clarification before implementation.
-->
# Feature Specification: Burst Directory Search

**Feature Branch**: `019-burst-directory-search`

**Created**: 2026-09-30

**Status**: Draft

**Input**: User description: "Add a Burst mode that searches subfolders concurrently during directory search. Provide a setting to switch between the existing sequential directory search and concurrent directory search." Additional requirement: "Allow the Burst parallel count to be specified in the settings screen and with a CLI option."

## Clarifications

### Session 2026-09-30

- Q: Should Burst mode start a search for every subfolder at once, or limit the number of concurrent subfolder searches? → A: Limit the number of concurrent subfolder searches.
- Q: Should Burst mode search only immediate child folders concurrently, or include subfolders at every depth? → A: Include subfolders at every depth.
- Q: Should the search mode switch be available only in the desktop app, or also in the standalone command-line search? → A: Provide it in both the desktop app and the command-line search.
- Q: Which mode should the command-line search use when no mode is specified? → A: Use sequential search.
- Q: What reduction in elapsed search time should Burst mode be required to achieve on the same directory tree? → A: Do not set a fixed reduction target.
- Q: What range should users be allowed to specify for the Burst parallel count? → A: Integers from 2 through 32.
- Q: How should the CLI behave when a Burst parallel count is supplied without an explicit search mode? → A: Activate Burst automatically; reject a count combined with explicit Sequential mode with a usage error.

**Clarified default**: The earlier sequential-default answer applies when both the mode and parallel count are omitted.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Choose a directory search mode in the desktop app (Priority: P1)

A user selects either the existing sequential mode or Burst mode in the settings screen. For Burst mode, the user can retain the automatic parallel count or save a custom count. Burst mode can visit subfolders at multiple depths concurrently to find eligible files, up to the selected count. Sequential mode visits subfolders one at a time. Workbook content search retains its existing behavior in both modes.

**Why this priority**: This is the primary way users choose the new behavior while retaining the existing search mode.

**Independent Test**: Prepare a directory tree containing searchable files in both immediate and deeper subfolders. Select each mode in the settings screen and search the same tree. Confirm that the chosen mode is used and that both searches find the same matches.

**Acceptance Scenarios**:

1. **Given** the settings screen is open, **when** the user saves sequential mode and searches a directory with multiple subfolders, **then** the app visits subfolders to find eligible files one at a time and displays results.
2. **Given** the settings screen is open, **when** the user saves Burst mode and searches a directory with subfolders at multiple depths, **then** immediate and deeper subfolders are eligible for concurrent file discovery and results are displayed.
3. **Given** a search has completed in either mode, **when** the user repeats it with the other mode using the same directory and search criteria, **then** the match set is identical, with no missing or duplicate matches.
4. **Given** the user has saved a search mode, **when** the desktop app restarts, **then** the saved mode appears in settings and applies to subsequent searches.
5. **Given** Burst mode is selected, **when** the user saves a custom parallel count from 2 through 32 and restarts the app, **then** that count remains selected and limits simultaneous subfolder visits in subsequent searches.
6. **Given** the user switches from Burst to Sequential and back, **when** the settings screen is reopened, **then** the saved custom Burst count is retained but has no effect while Sequential is active.

### User Story 2 - Choose a directory search mode from the command line (Priority: P2)

A command-line user chooses sequential or Burst mode and can provide a parallel count for Burst mode while retaining the existing search criteria and output formats. When neither a mode nor a parallel count is specified, the search uses sequential mode regardless of the desktop app's saved setting.

**Why this priority**: Standalone command-line users need the same mode choice without changing existing commands by default.

**Independent Test**: Search a directory tree with nested subfolders from the command line in both modes. Compare results and verify the mode used by a command that omits the mode selection.

**Acceptance Scenarios**:

1. **Given** searchable files exist at multiple subfolder depths, **when** the user specifies either mode in a command-line search, **then** the specified mode is used and both modes return the same matches.
2. **Given** neither a search mode nor a parallel count is specified in a command-line search, **when** the user searches a directory, **then** sequential mode is used, regardless of the desktop app's saved setting.
3. **Given** a valid parallel count from 2 through 32 is specified on the command line, **when** the user searches a directory, **then** Burst mode is used with that limit, even if the mode option is omitted.
4. **Given** Sequential mode is explicitly selected, **when** the user also specifies a Burst parallel count, **then** the command is rejected with a usage error instead of silently ignoring the count.

### Edge Cases

- A directory without subfolders completes successfully in either mode.
- If a subfolder cannot be accessed, the search continues in accessible locations and identifies the inaccessible location to the user.
- If the user cancels a search, unfinished work stops and already completed results follow the existing cancellation behavior.
- If a saved search mode is missing or invalid, the desktop app safely uses sequential mode.
- A different subfolder completion order does not produce missing or duplicate results.
- A missing or invalid saved custom count falls back to Automatic without resetting other valid search settings.
- A non-integer or out-of-range parallel count is rejected in the settings screen and on the command line.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide sequential mode and Burst mode for directory searches.
- **FR-002**: A desktop app user MUST be able to select and save the directory search mode in the settings screen.
- **FR-003**: The desktop app MUST restore the saved mode after restart and apply it to subsequent directory searches.
- **FR-004**: Sequential mode MUST visit subfolders one at a time while finding eligible files.
- **FR-005**: Burst mode MUST visit subfolders at every depth while finding eligible files, limit simultaneous subfolder visits to the effective parallel count, and start waiting visits as running visits finish.
- **FR-006**: For the same directory and criteria, both modes MUST return the same matches and MUST NOT skip files, search a file more than once, or duplicate matches.
- **FR-007**: Changing the mode MUST NOT change supported file types or the meaning of existing search criteria.
- **FR-008**: If a subfolder cannot be accessed or searched, the system MUST continue searching other accessible locations and identify the failed location to the user.
- **FR-009**: If the saved desktop app mode is missing or invalid, the system MUST use sequential mode.
- **FR-010**: Cancellation MUST work in either mode and preserve the existing handling of results found before cancellation.
- **FR-011**: The standalone command-line search MUST offer a way to select either mode and apply the selected mode to directory searches.
- **FR-012**: If a command-line search specifies neither a mode nor a Burst parallel count, it MUST use sequential mode independently of the desktop app's saved setting.
- **FR-013**: The desktop settings screen MUST let users choose Automatic or a custom Burst parallel count from 2 through 32, save that choice, restore it after restart, and use it for subsequent Burst searches.
- **FR-014**: The command-line search MUST accept a Burst parallel count from 2 through 32 for one invocation. Supplying the count without a mode MUST select Burst mode; combining it with explicit Sequential mode MUST produce a usage error.
- **FR-015**: Invalid saved custom counts MUST fall back to Automatic without discarding other valid desktop settings. The settings screen and CLI MUST reject non-integer or out-of-range user input.

### Key Entities

- **Directory Search Mode**: The sequential or Burst choice. The desktop app stores it as a user setting; a command-line search may select it for one invocation.
- **Burst Parallel Count**: Automatic or a user-selected integer from 2 through 32. It limits concurrent subfolder visits only in Burst mode; the desktop app persists the choice, while the CLI applies its value to one invocation.
- **Subfolder Discovery**: Work that visits one subfolder at any depth to find eligible files, with a success or failure state and discovered file paths.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can select and save either mode in the settings screen, and the selected mode is still shown after an app restart.
- **SC-002**: Searching the same directory with the same criteria in both modes produces zero differences in matches, zero missing matches, and zero duplicate matches.
- **SC-003**: Sequential mode never overlaps visits to separate subfolders. Burst mode overlaps visits to at least two eligible subfolders when work is available, while never exceeding the effective parallel count.
- **SC-004**: With one inaccessible subfolder, searches of accessible locations complete without an app crash, and the failed location is identified to the user.
- **SC-005**: A directory without subfolders completes successfully in both modes.
- **SC-006**: Command-line searches of the same directory and criteria in both modes produce zero differences in matches and zero duplicate matches.
- **SC-007**: In all checks where both the mode and parallel count are omitted, command-line searches use sequential mode regardless of the desktop app's saved setting.
- **SC-008**: A saved custom count from 2 through 32 is restored after an app restart and is never exceeded during a Burst search with enough subfolders to exercise the limit.
- **SC-009**: A CLI count from 2 through 32 is used for that invocation only; invalid counts and a count combined with explicit Sequential mode return a usage error.

## Assumptions

- The desktop app's mode is a persistent setting. Sequential mode is the default to preserve existing behavior.
- Burst mode applies to file discovery in subfolders at every depth. Files directly under the selected directory, workbook content processing, supported file types, and the meaning of search criteria follow existing behavior.
- Automatic uses a count from 2 through 8 chosen from available system resources. A custom count from 2 through 32 overrides Automatic for Burst mode. The count has no effect in Sequential mode, but a saved custom choice is retained for later Burst searches.
- The command-line search offers a mode choice while retaining existing search criteria and output formats. Omission of both the mode and parallel count selects sequential search.
- No fixed elapsed-time reduction is an acceptance target because it varies with the directory tree and storage performance.
- Search remains local; file contents and search terms are not sent to an external service.

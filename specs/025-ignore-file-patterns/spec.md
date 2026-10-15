# Feature Specification: Ignore File Patterns with Gitignore-Style Matching and History

**Feature Branch**: `025-ignore-file-patterns`

**Created**: 2026-10-15

**Status**: Draft

**Input**: User description: "gitignorパターン指定と同形式でカンマ区切りで無視ファイルリストをサポートする。検索ディレクトリ近くにパターン指定を配置。パターン履歴を保存し選択可能とする。"

## Clarifications

### Session 2026-10-15

- Q: How should the ignore pattern input field be positioned relative to the search directory within the search bar interface? → A: Dedicated row directly beneath the search directory input (above extension toggles).
- Q: Should an initial default ignore pattern (such as `~$*` for temporary Office lock files) be pre-populated out of the box, or should the ignore patterns field start completely empty? → A: Pre-populate `~$*` as the initial default pattern (editable/clearable by user).
- Q: Should default ignore patterns be configurable in the Settings dialog (under Saved Search Defaults) alongside default file extensions and search options? → A: Add "Default Ignore Patterns" to Settings dialog (Saved Defaults page) to customize baseline session defaults.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Specify Comma-Separated Gitignore-Style Ignore Patterns (Priority: P1)

As a user searching a large directory containing temporary files, office locks, build artifacts, or backup files,
I want to provide a comma-separated list of ignore patterns using standard `.gitignore` format in an input field placed near the search directory,
So that the search engine automatically skips all matching files and directories during traversal and focuses only on relevant documents.

**Why this priority**:
Core functionality requested by the user. Users currently have to scan through unwanted files (e.g., Excel lock files `~$*.xlsx`, temporary files `*.tmp`, or test/backup folders), slowing down searches and cluttering results.

**Independent Test**:
In a target directory containing both regular workbooks and temporary lock files (e.g., `~$Report.xlsx`) or backup folders (`backup/`), enter ignore pattern `~$*, backup/` in the ignore patterns field and execute a search. Verify that matching files and folders are excluded from scan progress and search results.

**Acceptance Scenarios**:

1. **Given** a directory containing Excel lock files starting with `~$` and regular Excel files,
   **When** the user enters `~$*` in the ignore patterns input and executes a search,
   **Then** all files matching `~$*` are skipped during scanning and do not appear in the search results.
2. **Given** multiple comma-separated ignore patterns (e.g., `~$*, *.tmp, backup/*`),
   **When** the user executes a search,
   **Then** each pattern token is parsed, whitespace is trimmed, and files or directories matching any of the specified patterns are excluded.
3. **Given** an empty ignore pattern field (or whitespace-only),
   **When** the search executes,
   **Then** no ignore filtering is applied and all matching documents in the target directory are scanned as usual.
4. **Given** ignore patterns targeting directories (e.g. `node_modules/` or `archive/`),
   **When** the directory scanner encounters matching folders,
   **Then** the entire folder and its descendants are skipped immediately without traversing into them.

---

### User Story 2 - UI Placement Adjacent to Search Directory (Priority: P2)

As a user configuring a search target,
I want the ignore patterns input field to be placed near the search directory input area in the search interface,
So that I can naturally review and configure folder scope and folder exclusion rules in one cohesive location.

**Why this priority**:
User explicitly specified: "検索ディレクトリ近くにパターン指定を配置" (Place the pattern specification near the search directory). Placing the ignore control alongside the directory selector provides ergonomic proximity and visual clarity.

**Independent Test**:
Inspect the search interface. The ignore patterns input field must be visibly positioned adjacent to or integrated within the search directory control area, with clear labeling, placeholder text, and tooltips.

**Acceptance Scenarios**:

1. **Given** the main search interface,
   **When** the user views the search controls,
   **Then** a dedicated ignore patterns row is visible directly beneath the search directory input row (and above the file extension toggles), labeled with an intuitive localized title, icon, and placeholder.
2. **Given** the user resizes the application window or changes themes,
   **When** viewing the search controls,
   **Then** the ignore patterns input neatly adapts to the layout without overlapping adjacent buttons or truncating input controls.
3. **Given** the user presses Enter or clicks Search while focusing the ignore patterns input,
   **When** the search is triggered,
   **Then** the search starts immediately using the entered ignore patterns.

---

### User Story 3 - Ignore Pattern History and Selection (Priority: P3)

As a frequent user who repeatedly uses common ignore patterns (such as ignoring lock files, archive directories, or draft folders),
I want the application to save previous ignore pattern strings in a persistent history and allow me to select them from a dropdown or history list,
So that I can quickly reapply frequently used exclusion rules without typing them manually each time.

**Why this priority**:
User explicitly requested: "パターン履歴を保存し選択可能とする" (Save pattern history and make it selectable). Storing and recalling history reduces typing friction and prevents syntax mistakes.

**Independent Test**:
Enter an ignore pattern (e.g. `~$*, *.bak`), run a search, clear the field, open the ignore pattern history dropdown, select the saved entry, and confirm that the field is populated with the selected pattern string.

**Acceptance Scenarios**:

1. **Given** an entered ignore pattern string used in a search,
   **When** the search starts successfully,
   **Then** the ignore pattern string is recorded in persistent storage (excluding empty strings and duplicates, moving recent items to the top).
2. **Given** saved ignore pattern history items,
   **When** the user clicks the history button or triggers history display on the ignore input,
   **Then** a dropdown menu displays previously used pattern combinations.
3. **Given** the history dropdown is open,
   **When** the user selects an item (by click or keyboard navigation),
   **Then** the input field is filled with the selected pattern string and the dropdown closes.
4. **Given** the application is closed and reopened,
   **When** the user accesses the ignore pattern history,
   **Then** previously saved patterns remain available across application sessions.

---

### User Story 4 - Configure Default Ignore Patterns in Settings (Priority: P3)

As a user with persistent preferences across projects,
I want to configure my preferred default ignore patterns in the Settings dialog (under Saved Search Defaults),
So that new search sessions automatically load my customized baseline ignore list instead of needing to re-enter it.

**Why this priority**:
Integrates with the existing default search options architecture (features 013 and 020), providing persistent customization of baseline session defaults alongside default file extensions and search options.

**Independent Test**:
In Settings > Saved Search Defaults, change the default ignore pattern from `~$*` to `~$*, *.bak`, save the settings, restart or reset the search bar, and verify that the search bar ignore pattern input initializes with `~$*, *.bak`.

**Acceptance Scenarios**:

1. **Given** the Settings dialog open to the "Saved Defaults" tab,
   **When** the user views the search defaults,
   **Then** a "Default Ignore Patterns" input field is displayed with current saved defaults.
2. **Given** the user modifies the default ignore patterns in Settings and saves,
   **When** a new search session is initiated or settings are applied,
   **Then** the search bar ignore patterns field updates to reflect the newly saved default patterns.
3. **Given** a user cancels or closes the Settings dialog without saving,
   **When** returning to the search screen,
   **Then** any unconfirmed draft changes to default ignore patterns are discarded.

---

### Edge Cases

- **Leading, Trailing, and Consecutive Commas**: Inputs such as `, ~$*, , *.tmp, ` must cleanly parse into `["~$*", "*.tmp"]` without producing empty pattern tokens or errors.
- **Quoted Patterns with Commas or Spaces**: Patterns enclosed in quotes (e.g. `"archive, old/*", "backup folder/*"`) must preserve internal spaces or commas within the pattern token.
- **Invalid Pattern Syntax**: Malformed glob expressions or invalid characters must fail gracefully with a friendly user notification rather than aborting traversal or crashing the application.
- **Relative vs Absolute Ignore Paths**: Patterns specified relative to the search root (e.g. `/build` or `build/`) must match relative to the active target directory.
- **Case-Insensitive File Systems**: On Windows and macOS, pattern matching must respect standard case-insensitive file system behavior (e.g., `*.TMP` matches `.tmp` files).
- **History Overflow**: When the number of saved history entries exceeds the configured maximum limit (e.g., 20 entries), the oldest least-recently-used entries are pruned.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept an ignore pattern string consisting of one or more comma-separated `.gitignore`-style wildcard expressions (e.g., `*`, `?`, `**`, `dir/`, `!negation`).
- **FR-002**: System MUST parse comma-separated pattern strings into individual ignore rules, trimming surrounding whitespace and discarding empty segments.
- **FR-003**: System MUST support double-quoted pattern tokens to allow patterns containing literal commas or whitespace.
- **FR-004**: System MUST evaluate each discovered file path against the active ignore pattern set during directory traversal and exclude any matching files before file parsing or content search.
- **FR-005**: System MUST evaluate directory paths against ignore rules and prune matching directories early, preventing traversal into excluded folder subtrees.
- **FR-006**: System MUST place the ignore pattern input control on a dedicated row directly beneath the search directory input row and above the file extension toggles in the search interface.
- **FR-007**: System MUST provide localized UI labels, tooltips, and placeholder text for the ignore pattern input across supported languages (Japanese and English).
- **FR-008**: System MUST record successfully submitted non-empty ignore pattern strings into persistent local history across application restarts.
- **FR-009**: System MUST deduplicate history entries and maintain recent ordering (moving newly used patterns to the top).
- **FR-010**: System MUST provide a dropdown or selection menu displaying saved ignore pattern history entries accessible via mouse click and keyboard navigation.
- **FR-011**: System MUST populate the ignore pattern input field immediately upon selecting an item from the history menu.
- **FR-012**: System MUST support clearing the ignore pattern input and executing searches with no exclusion rules.
- **FR-013**: System MUST support passing ignore patterns in CLI commands for consistency with GUI capabilities.
- **FR-014**: System MUST pre-populate `~$*` as the default ignore pattern upon fresh installation, while allowing users to edit, clear, or customize it.
- **FR-015**: System MUST allow users to view and update their preferred default ignore patterns in the Settings dialog (Saved Defaults page), persisting the preference in local storage and initializing search sessions with the configured default patterns.

### Key Entities *(include if feature involves data)*

- **Ignore Pattern String**: Comma-delimited text entered by the user containing one or more `.gitignore`-style expressions.
- **Ignore Rule**: A compiled glob/pattern representation used by the directory discovery pipeline to test candidate file and folder paths.
- **Ignore Pattern History**: An ordered collection of unique, previously executed ignore pattern strings persisted in local application storage.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can specify ignore patterns and see matching files and directories completely excluded from search results in 100% of test cases.
- **SC-002**: Excluding unneeded large directories (e.g., `node_modules` or `backup/`) reduces discovery and search scan time proportionally to the number of skipped files.
- **SC-003**: 100% of successfully executed non-empty ignore pattern queries are saved and immediately recallable from the history dropdown.
- **SC-004**: Users can select an ignore pattern from history in 2 interactions or fewer (open dropdown, select item).
- **SC-005**: All UI labels and placeholders are fully localized in English and Japanese with zero garbled text or missing translation keys.

## Assumptions

- Standard `.gitignore` glob semantics (e.g., wildcards `*`, `?`, directory indicator `/`, recursive wildcard `**`) are sufficient for user needs without requiring complex regular expressions in the ignore field.
- The default ignore pattern string is pre-populated with `~$*` upon fresh installation to automatically protect users from temporary Office lock files, while allowing full user editing, replacement, or clearing.
- Pattern matching against file and folder paths respects the case-sensitivity rules of the host operating system (case-insensitive on Windows and macOS, case-sensitive on Linux).
- Maximum history entries for ignore patterns follows standard history limits (e.g., default 20 entries) consistent with keyword and path history.

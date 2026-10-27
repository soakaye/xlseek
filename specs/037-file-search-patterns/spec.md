# Feature Specification: File Search Patterns with Extension Complement and Sequential Filtering

**Feature Branch**: `037-file-search-patterns`

**Created**: 2026-10-27

**Status**: Draft

**Input**: User description: "ファイル検索パターン指定を追加してください。入力パターン→無視パターンの順に適用。デフォルトファイル検索パターンはオプションで指定されている拡張子。入力パターンに拡張子が指定されていない場合はオプションで指定された拡張子で補完。入力パターンは複数指定可能。"

## Clarifications

### Session 2026-10-27

- Q: When a user enters a search pattern without wildcards or extensions (such as `budget`), should it match only the exact file name or match as a substring anywhere in the file name? → A: Standard glob matching: exact base name unless wildcards are typed (e.g., `budget` complements to `budget.xlsx` and matches only `budget.xlsx`; `*budget*` must be explicitly typed when substring matching is desired).
- Q: If a user specifies an explicit extension in a search pattern (such as `report.xls` or `*.xls`) while that extension's toggle button in the search options is turned off, should the search include or exclude that file? → A: Explicit pattern takes precedence: explicitly specified extensions in search patterns override unchecked extension toggle buttons for supported formats.
- Q: Where should the File Search Patterns input field be positioned within the search interface relative to the search directory and ignore patterns rows? → A: Dedicated row placed directly between the Search Directory row and the Ignore Patterns row (Directory → File Search Patterns → Ignore Patterns → Search Query / Options), mirroring the sequential filtering order.
- Q: Should the Settings dialog include a configurable "Default File Search Patterns" preference under Saved Defaults, or should the field always initialize empty out of the box? → A: Configurable in Settings: add "Default File Search Patterns" to Settings (Saved Defaults tab), initialized to empty `""` (which defaults to matching all enabled extensions).
- Q: How should file search patterns be supported in the Model Context Protocol (MCP) server? → A: The MCP `search` tool schema must include an optional `file_patterns` parameter (string, comma-separated), applying the same sequential filtering (file patterns then ignore patterns) and extension complement logic as the GUI and CLI.
- Q: Which short command-line flag should represent the new `--file-patterns` option in `xlseek-cli`? → A: Use `-g` (glob): `xlseek-cli` supports `-g` alongside `--file-patterns` for specifying inclusion glob patterns.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Specify Target File Search Patterns with Multiple Inclusion Matching (Priority: P1)

As a user searching for spreadsheets across directories containing diverse files and workbooks,
I want to specify one or more file search patterns (separated by commas) to target specific file names or path segments,
So that the search engine only scans relevant files (such as `*quarterly*, financial_*`) and avoids scanning unrelated files.

**Why this priority**:
Core capability requested by the user. Users currently have to scan all files matching the enabled extensions in a directory. Allowing targeted file search patterns significantly reduces scan duration and isolates relevant documents.

**Independent Test**:
In a test directory containing files such as `quarterly_report.xlsx`, `annual_budget.xlsx`, and `inventory_data.xlsx`, enter `*quarterly*, annual_*` in the file search patterns field and execute a search. Verify that only `quarterly_report.xlsx` and `annual_budget.xlsx` are scanned and searched, while `inventory_data.xlsx` is skipped.

**Acceptance Scenarios**:

1. **Given** a directory with various Excel files,
   **When** the user enters a single pattern `*budget*` and executes a search,
   **Then** only files whose names match `*budget*` are scanned.
2. **Given** multiple comma-separated patterns (e.g., `*summary*, *kpi*, plan_*`),
   **When** the user executes a search,
   **Then** any file matching at least one of the specified patterns is included for content scanning (logical OR matching across include patterns).
3. **Given** leading, trailing, or redundant whitespace around comma separators (e.g., `  sales* ,  *audit*  `),
   **When** the search is executed,
   **Then** each pattern token is cleanly trimmed of whitespace before matching.
4. **Given** an empty file search pattern field (or whitespace only),
   **When** the search is executed,
   **Then** the search automatically defaults to matching all files with the file extensions currently enabled in search options.

---

### User Story 2 - Sequential Filtering: Apply Search Patterns First, Then Ignore Patterns (Priority: P1)

As a user who relies on both inclusion search patterns and exclusion rules (such as skipping temporary files or backup directories),
I want the search engine to apply input search patterns first and then apply ignore patterns,
So that a file is only scanned if it matches the target search pattern AND is not excluded by any ignore pattern.

**Why this priority**:
User explicitly specified: "入力パターン→無視パターンの順に適用" (Apply input pattern, then ignore pattern). Ensuring strict evaluation order prevents unwanted or temporary files that match the include pattern from leaking into search operations.

**Independent Test**:
In a test directory containing `financial_report.xlsx`, temporary lock file `~$financial_report.xlsx`, and an archive folder `archive/financial_report.xlsx`, set search patterns to `financial_*` and ignore patterns to `~$*, archive/*`. Verify that only `financial_report.xlsx` is scanned, while `~$financial_report.xlsx` and `archive/financial_report.xlsx` are excluded.

**Acceptance Scenarios**:

1. **Given** a candidate file that matches an input search pattern and does NOT match any ignore pattern,
   **When** directory traversal evaluates the file,
   **Then** the file is accepted and its contents are searched.
2. **Given** a candidate file that matches an input search pattern BUT ALSO matches an active ignore pattern (e.g., `~$*`),
   **When** directory traversal evaluates the file,
   **Then** the file is excluded by the ignore filter and skipped without reading.
3. **Given** a candidate file that does NOT match any input search pattern,
   **When** directory traversal evaluates the file,
   **Then** the file is immediately skipped without requiring ignore pattern evaluation.

---

### User Story 3 - Automatic Extension Complement for Patterns Lacking Extensions (Priority: P2)

As a user entering search patterns without typing redundant file extension suffixes,
I want the system to automatically complement input patterns that lack an extension with all currently selected file extensions,
So that entering `budget*` automatically matches `budget*.xlsx`, `budget*.xlsm`, and any other format currently enabled in search options.

**Why this priority**:
User explicitly specified: "入力パターンに拡張子が指定されていない場合はオプションで指定された拡張子で補完" (If no extension is specified in the input pattern, complement with the extensions specified in options). This saves users from having to repeatedly type `.xlsx`, `.xlsm`, etc., while honoring the active extension toggle buttons.

**Independent Test**:
With `.xlsx` and `.xlsm` enabled in the extension options, enter pattern `sales_data*` (no extension specified). Verify that both `sales_data_2024.xlsx` and `sales_data_archive.xlsm` are matched and scanned, but `sales_data.csv` is not scanned if `.csv` is disabled.

**Acceptance Scenarios**:

1. **Given** search option extension toggles with `.xlsx` and `.xlsm` active,
   **When** the user enters an extensionless pattern `q1_report`,
   **Then** the system complements the pattern to match `q1_report.xlsx` and `q1_report.xlsm`.
2. **Given** an input pattern that explicitly specifies an extension (e.g., `custom_data.xls` or `*.xls`), even if that extension toggle is unchecked in the search options,
   **When** the pattern is processed,
   **Then** the system uses the specified extension as-is, does NOT append duplicate extension suffixes, and searches matching files for supported formats.
3. **Given** multiple comma-separated patterns where some have extensions and some do not (e.g., `budget*, summary.xlsx, *notes*`),
   **When** the search executes,
   **Then** extension complement is applied only to the tokens lacking extensions, while explicit extension tokens remain unchanged.
4. **Given** the user changes the active file extension toggles in the options,
   **When** an extensionless pattern is evaluated,
   **Then** the complemented extensions immediately reflect the newly updated options.

---

### User Story 4 - Search Interface Integration and History Selection (Priority: P2)

As a user searching from the graphical interface or command-line interface,
I want a dedicated input field for file search patterns positioned near the search controls, along with a persistent history dropdown,
So that I can easily configure, review, and recall frequent file search patterns across sessions.

**Why this priority**:
Consistent with the existing search directory and ignore pattern user experience. Storing pattern history eliminates repetitive typing and prevents input errors.

**Independent Test**:
Enter a file search pattern `*report*, *audit*`, execute a search, clear the input, open the pattern history dropdown, and select the saved entry. Verify that the input field is repopulated with the selected pattern string.

**Acceptance Scenarios**:

1. **Given** the search controls in the main user interface,
   **When** the user views the search configuration area,
   **Then** a dedicated input field for File Search Patterns is visible positioned directly between the Search Directory row and the Ignore Patterns row (above the search query row and extension toggles), equipped with an intuitive placeholder, icon, and pattern history button.
2. **Given** an entered file search pattern string used in an executed search,
   **When** the search starts,
   **Then** the pattern string is saved to persistent search pattern history (ignoring empty strings, trimming whitespace, and deduplicating recent entries).
3. **Given** saved history entries,
   **When** the user opens the pattern history dropdown or presses keyboard navigation keys,
   **Then** past pattern entries are displayed and selectable.
4. **Given** the user selects a pattern entry from the history dropdown,
   **When** selection is confirmed,
   **Then** the input field is filled with the selected pattern and the dropdown closes.
5. **Given** the CLI,
   **When** a user supplies file search pattern arguments (e.g. `--file-patterns` / `-g`),
   **Then** the search executes with identical sequential filtering and extension complement logic.
6. **Given** an MCP client invoking the `search` tool,
   **When** providing an optional `file_patterns` parameter (e.g., `*budget*, quarterly_*`),
   **Then** the MCP tool filters candidate documents by the specified file patterns and enabled extensions before applying `ignore_patterns`.

---

### User Story 5 - Configure Default File Search Patterns in Settings (Priority: P3)

As a user with consistent workflow patterns across projects,
I want to configure my preferred default file search patterns in the Settings dialog (under Saved Search Defaults),
So that new search sessions automatically load my customized baseline pattern instead of needing to re-enter it each time.

**Why this priority**:
Integrates with the existing default search options architecture (features 013, 020, and 025), providing persistent customization of baseline session defaults alongside default file extensions and default ignore patterns.

**Independent Test**:
In Settings > Saved Search Defaults, enter default file search pattern `*report*`, save the settings, reset the search bar or restart the application, and verify that the file search pattern input field initializes with `*report*`.

**Acceptance Scenarios**:

1. **Given** the Settings dialog open to the "Saved Defaults" tab,
   **When** the user views the search defaults,
   **Then** a "Default File Search Patterns" input field is displayed with current saved defaults (defaulting to empty `""`).
2. **Given** the user modifies the default file search patterns in Settings and saves,
   **When** a new search session is initiated or settings are applied,
   **Then** the search bar file search patterns field updates to reflect the newly saved default patterns.
3. **Given** a user cancels or closes the Settings dialog without saving,
   **When** returning to the search screen,
   **Then** any unconfirmed draft changes to default file search patterns are discarded.

---

### Edge Cases

- **Trailing, Leading, and Consecutive Commas**: Inputs such as `, *budget*, , *report*, ` must cleanly parse into `["*budget*", "*report*"]` without generating blank pattern tokens or error states.
- **Patterns Containing Internal Dots**: File patterns with dots that do not represent the final file extension (e.g., `release.v1.0*` or `dept.finance.*`) must correctly recognize whether a recognized file extension is present before applying extension complement.
- **Quoted Patterns with Commas**: Patterns wrapped in quotes (e.g., `"2024, Annual Report*", *summary*`) must preserve commas inside the quoted token as part of the pattern string.
- **Case-Insensitive File Matching**: Pattern matching must be case-insensitive across all platforms, ensuring `*REPORT*` matches `report.xlsx` and `REPORT.XLSX`.
- **Special Glob Characters**: Patterns containing special wildcard characters (such as `*`, `?`, `[`, `]`) must be safely evaluated without causing regular expression panics or glob compilation errors.
- **No Matching Files in Directory**: If no files in the target directory match the input search patterns, the search must conclude quickly with a clear message indicating 0 files matched the criteria, without error.
- **No Extensions Selected**: If a user disables all extension options and provides an extensionless pattern, the system must warn the user that no file extensions are enabled.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST allow users to specify one or more file search patterns (inclusion patterns) to filter which files are scanned during a search.
- **FR-002**: The system MUST support multiple file search patterns separated by commas, automatically trimming leading and trailing whitespace from each token.
- **FR-003**: The system MUST evaluate multiple file search patterns using logical OR semantics: a candidate file is selected if it matches any one of the specified patterns.
- **FR-004**: If the file search pattern input is empty or consists solely of whitespace, the system MUST default the file search pattern to match all files having any of the currently enabled file extensions.
- **FR-005**: If an input file search pattern does not specify a file extension, the system MUST automatically complement that pattern by generating variants for each file extension currently enabled in the search options.
- **FR-006**: If an input file search pattern explicitly includes a file extension, the system MUST use the pattern as provided and MUST NOT append additional extension suffixes. An explicitly specified extension in a search pattern takes precedence over unchecked extension toggle buttons for supported formats.
- **FR-007**: The system MUST evaluate file filter criteria in strict sequence: candidate files MUST first match the active file search patterns (or default extension patterns), and only candidate files that match MUST subsequently be evaluated against active ignore patterns.
- **FR-008**: Any candidate file that matches an active ignore pattern MUST be excluded from content scanning, regardless of whether it matched an input file search pattern.
- **FR-009**: The graphical user interface MUST provide a dedicated input field for file search patterns, positioned as a full-width row directly between the search directory row and the ignore patterns row (`Directory` → `File Search Patterns` → `Ignore Patterns`).
- **FR-010**: The system MUST persist non-empty file search patterns into a dedicated pattern history and allow users to view and select past patterns from a dropdown or keyboard navigation.
- **FR-011**: The command-line interface (CLI) MUST accept file search pattern arguments (e.g. `--file-patterns` / `-g`) and execute file filtering with identical sequential evaluation and extension complement rules.
- **FR-012**: The system MUST evaluate file search patterns using standard glob semantics (exact matching against file names unless explicit wildcards such as `*` or `?` are present), without implicit substring expansion (e.g., `budget` complements to `budget.xlsx` and matches only `budget.xlsx`, whereas `*budget*` matches any file containing `budget`).
- **FR-013**: The system MUST provide a "Default File Search Patterns" configuration option in the Settings dialog (under Saved Defaults), defaulting to empty (`""`), which initializes the search interface upon startup or reset.
- **FR-014**: The MCP server's `search` tool MUST include an optional `file_patterns` parameter in its tool schema, parsing comma-separated patterns, expanding extensionless tokens with active extensions, and evaluating include patterns prior to ignore patterns.

### Key Entities

- **File Search Pattern**: A user-specified glob or text pattern defining candidate file names or paths to include in the search.
- **Extension Complement Rule**: The transformation logic that expands an extensionless pattern into a set of patterns ending with each currently enabled file extension.
- **Sequential File Filter**: The two-stage filtering pipeline that applies include patterns first and ignore patterns second to determine whether a file should be scanned.
- **File Pattern History**: The persistent list of recently used file search pattern strings.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of candidate files matching an active file search pattern and an enabled extension (and not matching any ignore pattern) are correctly selected for content scanning.
- **SC-002**: 100% of candidate files matching an active ignore pattern are excluded from content scanning, even when they match an active file search pattern.
- **SC-003**: In target directories with mixed file names, applying a specific file search pattern reduces scanned file count and accelerates overall search completion by at least 50% compared to an unfiltered full-directory scan.
- **SC-004**: Users can input, complement, and execute a targeted multi-pattern search in under 10 seconds.
- **SC-005**: File search pattern evaluation adds less than 5% overhead to total directory traversal time across 10,000 files.

## Assumptions

- Standard glob syntax (such as `*` for zero or more characters and `?` for a single character) is supported for file search patterns, consistent with gitignore and standard file matching conventions.
- Comma is used as the default pattern separator, matching the convention established in ignore patterns.
- Quoted strings allow commas within individual pattern tokens if needed.
- Supported file extensions remain configurable via the existing file extension toggle options (`.xlsx`, `.xlsm`, `.xls`, `.xlsb`, etc.).
- When an extensionless pattern is complemented, the resulting match applies to any enabled extension in the search options.

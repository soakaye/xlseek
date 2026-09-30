<!--
Description: Defines user journeys, acceptance criteria, and scope for separating immediate preferences from settings requiring an explicit save.
Arguments & Returns: Inputs are the user's request, design feedback, and existing settings specifications; output is a planning-ready Markdown specification.
Errors: No runtime behavior. Ambiguous or inconsistent requirements must be resolved before implementation.
-->
# Feature Specification: Separate Settings Pages and Unify the Save Action

**Feature Branch**: `020-organize-settings-pages`

**Created**: 2026-10-01

**Status**: Draft

**Input**: User description: "Organize the settings screen by separating settings that apply immediately from settings that require saving, and consolidate saving into a single button."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Change the Display Language with Clear Application Timing (Priority: P1)

The user selects a display language on the "Display Language · Apply Immediately" page of the settings screen. They understand that no save action is required and can switch languages without confusing the change with unsaved edits on the other page.

**Why this priority**: Make it clear which actions apply settings and prevent mistaken save or cancel actions.

**Independent Test**: Open settings, change the display language, and close settings. Verify that the display changes without using a save button and that the selection remains when settings are reopened.

**Acceptance Scenarios**:

1. **Given** the settings screen is closed, **When** the user opens settings, **Then** the immediate-apply page is displayed and navigation clearly offers two pages: settings that apply immediately and settings that require saving.
2. **Given** the immediate-apply page is displayed, **When** the user selects Default, Japanese, or English, **Then** the display language changes without a save action, and the selection is also persisted for the next launch, as before.
3. **Given** the display language has been changed, **When** the user cancels on the save page or closes settings, **Then** the language change that has already taken effect is not reverted, and search input, search results, and selection state are preserved.
4. **Given** the immediate-apply page is displayed, **When** the user checks the action buttons, **Then** "Close" is shown, while a save button and the save page's cancel action are not shown.

---

### User Story 2 - Apply Search Settings with a Single Save Action (Priority: P1)

The user edits the history retention limit and default search conditions together on the "Search Settings · Save to Apply" page and commits them with a single "Save Settings" button. Edits do not affect the current settings or history before they are saved.

**Why this priority**: Remove ambiguity about the scope of multiple save buttons and let users complete settings changes with a single confirmation action.

**Independent Test**: Edit the history limit and search options together. Verify that existing settings remain unchanged before saving, that both changes apply after one save action, and that they persist after restarting.

**Acceptance Scenarios**:

1. **Given** the save page is open, **When** the user checks the settings and action buttons, **Then** the history limit, default search options, directory search mode, Burst concurrency, and target file extensions are displayed, and "Save Settings" is the only save button.
2. **Given** saved settings exist, **When** the user edits the history limit and multiple search settings, **Then** the edits are retained as unsaved changes, and the current search conditions and history remain unchanged.
3. **Given** the edits are valid, **When** the user activates "Save Settings" once, **Then** all settings on the save page are persisted and applied to the current search conditions and history limit, a completion notification appears, and settings close.
4. **Given** saving succeeded, **When** the user reopens settings or restarts the application, **Then** the saved values are displayed and the same values apply to the initial search state.
5. **Given** the save page has unsaved changes, **When** the user switches to the immediate-apply page and returns, **Then** the edits are preserved without being automatically saved or discarded.
6. **Given** a manual Burst concurrency value has been saved, **When** the user saves sequential search and later switches back to Burst, **Then** the manual concurrency value is preserved and is not used during sequential search.

---

### User Story 3 - Safely Discard Unsaved Changes (Priority: P1)

The user can close settings and discard unsaved changes through "Cancel" on the save page, the settings close action, the Esc key, or a click outside the settings screen. History is not deleted and search conditions are not updated until saving succeeds.

**Why this priority**: Prevent irreversible changes, such as reducing retained history, from being committed while the user is still editing.

**Independent Test**: Set the history limit to 0, change search conditions and file extensions, and close settings through each exit action. Verify that history and saved settings are preserved and that reopening settings restores the values from before editing.

**Acceptance Scenarios**:

1. **Given** the save page has unsaved changes, **When** the user closes settings through "Cancel", the header close button, the Esc key, or a click outside settings, **Then** all unsaved values are discarded, and saved settings, current search conditions, and history remain unchanged.
2. **Given** the user has edited the save page and moved to the immediate-apply page, **When** the user exits with "Close", **Then** unsaved changes on the save page are discarded and the language setting that has already taken effect is preserved.
3. **Given** settings were closed after discarding unsaved changes, **When** the user reopens settings, **Then** the last saved values are displayed, and neither previous draft values nor input errors remain.
4. **Given** the user is editing default search settings, **When** the user activates "Restore Defaults", **Then** only the draft search settings return to the existing initial values, the history limit and language setting remain unchanged, and the user can commit or discard the reset through saving or canceling.

---

### User Story 4 - Operate Settings Reliably on Small Screens and with a Keyboard (Priority: P2)

The user can always find Close, Save, and Cancel at supported window sizes and can scroll through the content to configure settings. They can also complete the entire flow, from switching pages to exiting, with a keyboard.

**Why this priority**: Action buttons can fall outside the visible area in the existing tall settings screen. Eliminate situations where window size or input method prevents users from completing an action.

**Independent Test**: Check the screen in Japanese and English at 1087×940 and the minimum supported size of 960×640. Verify that each page's action buttons remain visible and that editing, saving, and canceling can be completed using only the keyboard.

**Acceptance Scenarios**:

1. **Given** settings are open at the minimum supported window size, **When** either page is displayed, **Then** the header and exit or confirmation actions remain visible, and all settings can be reached by scrolling the content area when necessary.
2. **Given** the user opened settings with the keyboard, **When** the user switches pages, edits values, saves, or exits, **Then** the selected page and focus position are identifiable, focus remains within settings during interaction, and closing settings returns focus to the control that opened them.
3. **Given** the save page contains edits, **When** the user changes the display language on the immediate-apply page and returns to the save page, **Then** page names, instructions, buttons, and input errors appear in the selected language, and the edits are preserved.

### Edge Cases

- If the history limit is empty, negative, fractional, or greater than 50, saving is blocked and a message near the field explains that it must be an integer from 0 to 50. Both 0 and 50 are valid.
- If manual Burst concurrency is empty, fractional, less than 2, or greater than 32, saving is blocked only while manual Burst concurrency is active. In sequential search or automatic concurrency mode, invalid manual input is ignored when determining whether saving is allowed and must not overwrite the saved valid manual concurrency value.
- Saving with all target file extensions deselected is blocked, and a message explains that at least one extension is required.
- When the history limit is reduced, older history entries are deleted only after saving succeeds. Saving 0 clears both histories and stops further recording. Canceling does not delete history.
- If saving fails, no success notification or automatic close occurs. Edits are retained so the user can retry or cancel. Settings and history from before saving are preserved without partial application.
- Repeated save actions during saving do not duplicate the commit, and page switching, editing, or canceling before completion cannot change the settings being saved.
- If automatic persistence of the display language fails, follow the existing language settings specification: use the selected language for the current session and notify the user of the persistence failure. Edits on the save page are preserved.
- Closing or saving without changes does not alter existing setting values. Saving during a search does not affect that search's conditions or results; changes apply starting with the next search.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The settings screen MUST separate settings that apply immediately from those requiring an explicit save into two switchable pages. Page names and descriptions MUST explain when settings take effect. The immediate-apply page MUST be shown initially.
- **FR-002**: The immediate-apply page MUST provide the existing display language selector and current display language, and MUST NOT display a save button. Language selection MUST take effect and be persisted automatically without requiring a restart or confirmation on the save page.
- **FR-003**: The save page MUST provide the history retention limit together with the existing default search settings. Search settings MUST include case sensitivity, regular expressions, formulas, text in shapes, comments/notes, hidden sheets, target file extensions, sequential/Burst search mode, and automatic/manual Burst concurrency.
- **FR-004**: All saving within the settings screen MUST be consolidated into a single "Save Settings" action on the save page. Separate save buttons for individual settings MUST NOT be provided.
- **FR-005**: Edits on the save page MUST start from saved settings and be retained when switching pages. Editing alone MUST NOT update search conditions or history.
- **FR-006**: When saving succeeds, all settings on the save page MUST be persisted locally and applied to the history limit and current search screen. The user MUST receive a completion notification and settings MUST close. Settings MUST persist after restarting.
- **FR-007**: The save page MUST provide "Cancel", the immediate-apply page MUST provide "Close", and the header close button MUST be available on both pages. The Esc key and a click outside settings MUST also allow the user to exit.
- **FR-008**: All exit actions before saving begins MUST discard unsaved changes and preserve the last saved settings and history. Language changes that have already taken effect MUST NOT be reverted.
- **FR-009**: Validation MUST require an integer from 0 to 50 for the history limit, an integer from 2 to 32 for manual Burst concurrency, and at least one target file extension from the existing supported formats. Invalid fields MUST be identified and saving blocked. Manual concurrency MUST be validated only when Burst and manual concurrency are selected, and the saved manual value MUST be preserved when switching to sequential search or automatic concurrency.
- **FR-010**: Existing history MUST be deleted due to a reduced or zero history limit only after saving succeeds. Editing, switching pages, and canceling MUST NOT delete history.
- **FR-011**: "Restore Defaults" MUST reset only the draft default search settings to the existing initial values and MUST NOT persist or apply them before saving. The history limit and display language MUST remain unchanged.
- **FR-012**: If saving fails, the failure MUST be reported in the current display language, and the settings screen and edits MUST be retained. Settings and history from before saving MUST be preserved, and partial commits or success notifications MUST NOT occur.
- **FR-013**: During saving, duplicate commits and changes to or discarding of the settings being saved MUST be prevented. Appropriate actions MUST become available again after success or failure.
- **FR-014**: In Japanese and English, at supported window sizes of 960×640 or larger, the header and each page's exit or confirmation actions MUST remain visible. If the content does not fit, all settings MUST be reachable by scrolling.
- **FR-015**: Page switching, editing, saving, and exiting MUST be accessible through the keyboard, and the current page and focus MUST be identifiable. Focus MUST remain within settings while open and return to the originating control on exit.
- **FR-016**: Page names, descriptions, buttons, input errors, and action labels MUST follow the current display language. Changing the language MUST NOT discard edits on the save page.
- **FR-017**: Page switching, language changes, and canceling MUST preserve search input, search results, and selection state. Saved settings MUST apply to the next search and the initial state at the next launch without changing an ongoing search.

### Key Entities

- **Language Settings That Apply Immediately**: The selection of Default, Japanese, or English and the current display language. Selection takes effect and is persisted automatically, independently of canceling on the save page.
- **Settings Requiring Saving**: The history retention limit and default search conditions grouped into settings that the user explicitly commits.
- **Draft Settings**: Unsaved values initialized from the last saved settings. They are retained across page switches, committed when saving succeeds, and discarded on exit or cancellation.
- **History**: Existing search text and search directory histories. Limit changes and deletions apply only after the retention limit is committed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every setting belongs to exactly one of the two pages: immediate-apply or save-required. The immediate-apply page has 0 save buttons, and the save page has 1.
- **SC-002**: Settings screen text updates within 1 second of language selection without requiring a save action or restart.
- **SC-003**: In 100% of acceptance cases that change the history limit and multiple search settings, a single save action applies all changes, and the same values can be verified after restarting.
- **SC-004**: Settings and history from before saving are preserved across every cancellation path and all acceptance cases involving invalid-input rejection or save failure. Language changes remain in effect after cancellation.
- **SC-005**: In every screen check in Japanese and English at 1087×940 and 960×640, exit and confirmation actions remain visible and all settings are reachable.
- **SC-006**: The three main actions—changing the display language, saving search settings, and canceling unsaved changes—can each be completed from start to finish using only the keyboard.

## Assumptions

- "Separate pages" means two pages that can be switched within the same settings screen. The classification of display language as immediate-apply and the history limit and default search settings as save-required follows the preceding design feedback and mockup structure.
- The scope is the desktop application's settings experience; the existing mockup alone does not constitute implementation of this feature. No new settings, search features, additional languages, or separate settings window are introduced.
- Default language selection, automatic persistence, and handling of persistence failures follow [008-add-language-settings](../008-add-language-settings/spec.md). History limits, initial values, and deletion order follow [010-search-history-path-completion](../010-search-history-path-completion/spec.md).
- Default search option values and their application to the search screen after saving follow [013-default-options-config](../013-default-options-config/spec.md). Search modes and the range and retention of Burst concurrency follow [019-burst-directory-search](../019-burst-directory-search/spec.md). This specification reorganizes those settings actions and does not change CLI behavior.
- Closing before saving discards unsaved changes without adding a confirmation dialog. The save page's description explains this behavior in advance.
- Settings remain stored locally, as before, and changing settings does not require external communication.

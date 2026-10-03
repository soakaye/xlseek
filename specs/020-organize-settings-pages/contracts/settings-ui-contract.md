<!--
Description: Defines observable settings actions and the unified save interface.
Arguments & Returns: Input is the approved feature specification and repository research; output is a Markdown planning artifact.
Errors: No runtime execution; unresolved design requirements must be addressed before implementation.
-->
# Settings UI and Persistence Contract

## Page and Action Contract

| State | Visible content | Footer actions | Effect |
|---|---|---|---|
| Open / immediate | Existing language selector and immediate-apply explanation | Close | Language applies and persists independently; no save button |
| Open / saved | History limit and all existing search defaults | Restore Defaults, Cancel, Save Settings | Edits affect only a draft; one save button |
| Saving | Current page and candidate settings | Actions disabled | No page switch, edit, duplicate save, or discard |
| Save rejected / failed | Draft and localized field/error notification | Actions available | No persisted or committed changes; no success or close |
| Save succeeded | Completion notification, then dialog closes | None | Whole saved page applied; next opening/launch restores values |

The header close control exists on both pages. Header close, page Close/Cancel, Escape, and backdrop click all discard the draft before saving starts. No confirmation dialog is added. Reset affects only draft search defaults, including draft worker memory, and leaves draft history limit and language unchanged.

## Application Interface

The existing settings dialog receives committed preference values and a unified save callback. It submits only validated preferences: history limit, `DefaultSearchOptions`, and remembered manual worker count. It does not submit histories. The callback returns success/failure; failure reporting uses the existing localized notification path. The dialog closes only after success.

The history hook derives the complete candidate from its latest current histories, validates it, serializes it, and replaces the existing history storage key once. Only after that succeeds does it update its state/ref. App then applies search defaults and shows success. Read/serialization/storage failures are handled without escaping as unhandled exceptions. The old independent save callbacks are removed.

Accepted-search history recording keeps its existing memory-only fallback on write failure. Every such write retains the committed preference fields. Locale remains outside this record and callback. No Rust command or search query schema changes are required.

## Draft and Validation Contract

- Initialize from committed values on each opening, initially on the immediate page. Preserve edits across page changes, language changes, and ordinary history updates.
- History must be an integer from 0 to 50; empty input is invalid.
- At least one existing supported file extension must be selected.
- Validate manual worker input only in Burst/custom mode, as an integer from 2 to 32. Invalid inactive input cannot overwrite the last valid count. Automatic selection stores null for selected workers and preserves remembered count.
- Reducing the limit trims oldest entries only after persistence succeeds. Zero clears both histories and prevents subsequent recording.
- See [data-model.md](../data-model.md) for compatibility and conversion rules.

## Accessibility and Layout Contract

Use semantic tab/page navigation with an identifiable selected state, accessible panel relationships, and keyboard navigation. Give all controls localized accessible names and associate errors with their fields. Focus stays inside the modal while open and returns to the launcher after closing. Initial focus must be visible. Escape follows the same discard behavior as Close.

Constrain the dialog to the viewport; keep header/navigation and page actions visible, and scroll the content area. Verify Japanese and English at 960×640 and 1087×940. Translation updates must preserve raw drafts, including invalid input, and update all labels/errors within one second.

## Requirement Coverage

| Requirements | Contract / verification |
|---|---|
| FR-001–FR-004 | Page/action table; initial page; language persistence; button counts; complete existing option set |
| FR-005–FR-006 | Draft initialization/preservation and unified successful commit/restart |
| FR-007–FR-008 | Every exit discards drafts and retains language/history |
| FR-009–FR-011 | Boundary/active-only validation, successful-only trimming, draft-only reset |
| FR-012–FR-013 | Rejected write retains draft/state; synchronous save guard |
| FR-014–FR-016 | Viewport checks, keyboard focus, live localized labels/errors |
| FR-017 | Preserve input/results/selection; in-flight query unchanged; next search/launch applies defaults |

The standalone mock must expose the same page/action, draft, validation, persistence failure, and keyboard behavior using its existing browser-only implementation.

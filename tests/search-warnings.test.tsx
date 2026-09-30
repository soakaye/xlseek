/**
 * Verifies localized rendering and de-duplication of directory discovery warnings.
 * Arguments: Vitest and React Testing Library environment; returns no value.
 * Errors: assertion failures identify missing or duplicated user-facing warnings.
 */
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { SearchWarnings } from "../src/components/common/SearchWarnings";

/** Describes the visible behavior of search warning lists. */
describe("SearchWarnings", () => {
  /** Shows each inaccessible folder once even when overlapping roots report it repeatedly. */
  it("deduplicates issues by path and code", () => {
    const issue = { path: "C:/books/private", stage: "discovery" as const, code: "permission_denied" as const };
    render(<SearchWarnings issues={[issue, issue]} language="en" />);

    expect(screen.getByText("Folder access warnings")).toBeTruthy();
    expect(screen.getAllByText("Could not search C:/books/private")).toHaveLength(1);
  });

  /** Omits the panel when discovery did not encounter a nonfatal issue. */
  it("renders nothing for an empty issue list", () => {
    const { container } = render(<SearchWarnings issues={[]} language="en" />);
    expect(container.firstChild).toBeNull();
  });
});

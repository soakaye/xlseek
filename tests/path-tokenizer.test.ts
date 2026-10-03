/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for pathTokenizer utility (tests/path-tokenizer.test.ts)
 */

import { describe, expect, it } from "vitest";
import {
  splitSearchPaths,
  appendSearchPath,
  getActivePathSegment,
} from "../src/utils/pathTokenizer";

describe("splitSearchPaths", () => {
  it("splits unquoted comma-separated paths and trims whitespace", () => {
    const input = "  /dir/a , /dir/b  ,  ";
    expect(splitSearchPaths(input)).toEqual(["/dir/a", "/dir/b"]);
  });

  it("preserves commas and whitespace within double quotes", () => {
    const input = `"/path/with, comma", "/path with space", /normal/path`;
    expect(splitSearchPaths(input)).toEqual([
      "/path/with, comma",
      "/path with space",
      "/normal/path",
    ]);
  });

  it("handles escaped quotes properly", () => {
    const input = `"/path/with""quote"`;
    expect(splitSearchPaths(input)).toEqual(['/path/with"quote']);
  });

  it("returns empty array for empty or whitespace-only input", () => {
    expect(splitSearchPaths("   ")).toEqual([]);
    expect(splitSearchPaths(",,,")).toEqual([]);
  });
});

describe("appendSearchPath", () => {
  it("returns formatted path if input is empty", () => {
    expect(appendSearchPath("", "/dir/a")).toBe("/dir/a");
    expect(appendSearchPath("   ", "/dir with space")).toBe('"/dir with space"');
  });

  it("appends to existing path separated by comma and space", () => {
    expect(appendSearchPath("/dir/a", "/dir/b")).toBe("/dir/a, /dir/b");
  });

  it("quotes new path if it contains spaces or commas", () => {
    expect(appendSearchPath("/dir/a", "/dir with, space")).toBe(
      '/dir/a, "/dir with, space"'
    );
  });

  it("handles trailing commas in current input gracefully", () => {
    expect(appendSearchPath("/dir/a,", "/dir/b")).toBe("/dir/a, /dir/b");
  });
});

describe("getActivePathSegment", () => {
  it("returns segment under cursor for single path", () => {
    const input = "/my/path/folder";
    const res = getActivePathSegment(input, 5);
    expect(res.segment).toBe("/my/path/folder");
  });

  it("extracts correct active segment in multi-path string", () => {
    const input = "/first/path, /second/path, /third/path";
    // Cursor in /second/path (around index 18)
    const res = getActivePathSegment(input, 18);
    expect(res.segment).toBe("/second/path");
  });

  it("respects quoted segments with commas", () => {
    const input = `"/first, path", /second/path`;
    const res = getActivePathSegment(input, 5);
    expect(res.segment).toBe('"/first, path"');
  });
});

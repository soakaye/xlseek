/**
 * Description: Exercises the standalone settings prototype's page separation and draft lifecycle.
 * Arguments & Returns: Vitest loads the real HTML and runs user interactions; no public return value.
 * Errors: Assertions fail when closing commits drafts, validation permits invalid data, or persistence fails silently.
 */
import { JSDOM } from "jsdom";
import { afterEach, beforeEach, expect, it } from "vitest";
import prototype from "../design/mainui/index.html?raw";

let dom: JSDOM;

/**
 * Description: Finds a prototype control for interaction without replacing its implementation.
 * Arguments & Returns: id (string) is the element ID; returns the matching HTMLElement.
 * Errors: Throws if the expected control is missing.
 */
const control = <T extends HTMLElement>(id: string): T => {
  const element = dom.window.document.getElementById(id);
  if (!element) throw new Error(`Missing control: ${id}`);
  return element as T;
};

/**
 * Description: Loads the real standalone document and initializes its controls without CDN requests.
 * Arguments & Returns: No arguments; resolves void after the DOM and event handlers are ready.
 * Errors: Script evaluation failures surface directly to Vitest.
 */
beforeEach(async () => {
  dom = new JSDOM(prototype, { url: "http://localhost", runScripts: "outside-only" });
  await new Promise<void>((resolve) => dom.window.document.addEventListener("DOMContentLoaded", () => resolve(), { once: true }));
  // The CDN icon renderer is the only external dependency; settings logic runs unchanged.
  dom.window.lucide = { createIcons: () => undefined };
  const scripts = dom.window.document.querySelectorAll("script:not([src])");
  dom.window.eval(scripts[scripts.length - 1].textContent ?? "");
  dom.window.document.dispatchEvent(new dom.window.Event("DOMContentLoaded"));
  control("settingsDialogBtn").click();
});

/**
 * Description: Releases each document and pending toast timers.
 * Arguments & Returns: No arguments; returns void.
 * Errors: No persistence operations are performed during cleanup.
 */
afterEach(() => dom.window.close());

it("separates immediate language changes from the page with one save action", () => {
  expect(control("settingsImmediatePage").hidden).toBe(false);
  expect(control("settingsSavedPage").hidden).toBe(true);
  expect(control("settingsSavedActions").hidden).toBe(true);
  control<HTMLInputElement>("mockLanguageEnglish").click();
  expect(dom.window.localStorage.getItem("xlseek.language")).toBe("en");
  control("settingsSavedTab").click();
  expect(control("settingsImmediatePage").hidden).toBe(true);
  expect(control("settingsSavedPage").hidden).toBe(false);
  expect(dom.window.document.querySelectorAll('button[type="submit"]')).toHaveLength(1);
  control<HTMLInputElement>("historyLimit").value = "7";
  control("settingsCancelBtn").click();
  control("settingsDialogBtn").click();
  expect(control<HTMLInputElement>("historyLimit").value).toBe("20");
  expect(control<HTMLInputElement>("mockLanguageEnglish").checked).toBe(true);
  expect(control("settingsTitle").textContent).toBe("Settings");
});

it("remembers a valid manual worker count when sequential search is saved", () => {
  control("settingsSavedTab").click();
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockDirectoryMode"][value="burst"]')!.click();
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockWorkerChoice"][value="custom"]')!.click();
  control<HTMLInputElement>("mockBurstWorkers").value = "8";
  control("mockBurstWorkers").dispatchEvent(new dom.window.Event("input", { bubbles: true }));
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockDirectoryMode"][value="sequential"]')!.click();
  control("saveDefaultOptionsBtn").click();
  const saved = JSON.parse(dom.window.localStorage.getItem("xlseek.searchHistory") ?? "{}");
  expect(saved.defaultOptions.directory_mode).toBe("sequential");
  expect(saved.defaultOptions.burst_workers).toBe(8);
  expect(saved.rememberedBurstWorkers).toBe(8);
});

it("saves history and search defaults together and discards subsequent drafts on every close path", () => {
  control("settingsSavedTab").click();
  control<HTMLInputElement>("historyLimit").value = "7";
  control<HTMLInputElement>("mockOptRegex").click();
  dom.window.document.querySelector<HTMLButtonElement>('.mock-ext-btn[data-ext=".xls"]')!.click();
  control("saveDefaultOptionsBtn").click();
  const saved = JSON.parse(dom.window.localStorage.getItem("xlseek.searchHistory") ?? "{}");
  expect(saved.maxEntries).toBe(7);
  expect(saved.defaultOptions.use_regex).toBe(true);
  expect(dom.window.localStorage.getItem("xlseek.default_search_options")).toBeNull();
  expect(control<HTMLInputElement>("optRegex").checked).toBe(true);
  for (const close of ["settingsCloseBtn", "settingsDoneBtn", "settingsCancelBtn", "escape", "backdrop"]) {
    control("settingsDialogBtn").click();
    control("settingsSavedTab").click();
    control<HTMLInputElement>("historyLimit").value = "12";
    control<HTMLInputElement>("mockOptRegex").click();
    control("resetDefaultOptionsBtn").click();
    if (close === "escape") control("settingsModal").dispatchEvent(new dom.window.KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    else if (close === "backdrop") control("settingsModal").click();
    else control(close).click();
    expect(control("settingsModal").classList.contains("hidden")).toBe(true);
    control("settingsDialogBtn").click();
    expect(control<HTMLInputElement>("historyLimit").value).toBe("7");
    expect(control<HTMLInputElement>("mockOptRegex").checked).toBe(true);
    expect(control<HTMLInputElement>("optRegex").checked).toBe(true);
    expect(dom.window.document.querySelector('.mock-ext-btn[data-ext=".xls"]')!.getAttribute("aria-pressed")).toBe("false");
    control("settingsCloseBtn").click();
  }
});

it("blocks invalid history counts and only validates worker counts while Burst is active", () => {
  control("settingsSavedTab").click();
  const count = control<HTMLInputElement>("historyLimit");
  const save = control<HTMLButtonElement>("saveDefaultOptionsBtn");
  for (const value of ["", "-1", "51", "2.5"]) {
    count.value = value;
    count.dispatchEvent(new dom.window.Event("input", { bubbles: true }));
    expect(save.disabled).toBe(true);
  }
  count.value = "0";
  count.dispatchEvent(new dom.window.Event("input", { bubbles: true }));
  expect(save.disabled).toBe(false);
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockDirectoryMode"][value="burst"]')!.click();
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockWorkerChoice"][value="custom"]')!.click();
  control<HTMLInputElement>("mockBurstWorkers").value = "33";
  control("mockBurstWorkers").dispatchEvent(new dom.window.Event("input", { bubbles: true }));
  expect(save.disabled).toBe(true);
  dom.window.document.querySelector<HTMLInputElement>('input[name="mockDirectoryMode"][value="sequential"]')!.click();
  expect(save.disabled).toBe(false);
  save.click();
  expect(JSON.parse(dom.window.localStorage.getItem("xlseek.searchHistory") ?? "{}").maxEntries).toBe(0);
});

it("keeps the draft open without applying changes when storage rejects a save", () => {
  control("settingsSavedTab").click();
  control<HTMLInputElement>("historyLimit").value = "7";
  control<HTMLInputElement>("mockOptRegex").click();
  dom.window.Storage.prototype.setItem = () => { throw new Error("Storage quota exceeded"); };
  control("saveDefaultOptionsBtn").click();
  expect(control("settingsModal").classList.contains("hidden")).toBe(false);
  expect(control<HTMLInputElement>("historyLimit").value).toBe("7");
  expect(control<HTMLInputElement>("optRegex").checked).toBe(false);
  expect(control("toastMsg").textContent).toContain("Could not save settings");
});

import { expect } from "chai";

describe("Tauri app smoke test", () => {
  it("should launch and show a window", async () => {
    // Basic sanity: WDIO session exists
    const title = await browser.getTitle();

    // Many Tauri apps return empty title unless you set one; adjust to your app.
    // This assertion is intentionally weak to start with.
    expect(title).to.be.a("string");
  });
});

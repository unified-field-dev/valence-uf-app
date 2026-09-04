import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-auth", () => {
  test("pw-valence-auth-gate-sad-anonymous", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("valence-dashboard-page")).toHaveCount(0);
  });

  test("pw-valence-auth-gate-happy-admin", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-dashboard-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("valence-app-root")).toBeVisible();
  });
});

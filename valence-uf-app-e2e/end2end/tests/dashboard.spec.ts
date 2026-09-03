import { test, expect, seedAuth, waitForHydrated, expandShellNav } from "./fixtures";

test.describe("pw-valence-dashboard", () => {
  test("pw-dashboard-happy-load", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-dashboard-page")).toBeVisible({
      timeout: 60_000,
    });
    await expandShellNav(page);
    await expect(page.getByTestId("nav-schemas")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("nav-traits")).toBeVisible();
    await expect(page.getByTestId("nav-iters")).toBeVisible();
    await expect(page.getByTestId("nav-deletions")).toBeVisible();
  });

  test("pw-dashboard-nav-schemas-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expandShellNav(page);
    await page.getByTestId("nav-schemas").click();
    await expect(page.getByTestId("schema-index-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});

import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-traits", () => {
  test("pw-trait-list-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence/traits", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("trait-index-page")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-trait-detail-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/valence/traits/${encodeURIComponent(seeded.fixtures.trait_name)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-trait-detail-page")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-trait-detail-sad-unknown", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence/traits/__valence_e2e_no_such_trait__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-trait-detail-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});

import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-schemas", () => {
  test("pw-schema-list-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/valence/schema", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("schema-index-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText(seeded.fixtures.schema_name).first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-schema-detail-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(`/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-schema-detail-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText(seeded.fixtures.schema_name).first()).toBeVisible();
  });

  test("pw-schema-detail-sad-unknown", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence/schema/__valence_e2e_no_such_schema__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-schema-detail-page")).toBeVisible({
      timeout: 60_000,
    });
    // Detail page shows empty/error state rather than crashing.
    await expect(page.getByTestId("valence-app-root")).toBeVisible();
  });
});

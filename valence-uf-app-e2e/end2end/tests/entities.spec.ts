import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-entities", () => {
  test("pw-entity-view-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/id/${encodeURIComponent(seeded.fixtures.entity_id)}`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-entity-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("valence-entity-fields-card")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-entity-view-sad-unknown", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/id/__valence_e2e_missing_entity__`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-entity-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("valence-app-root")).toBeVisible();
  });

  test("pw-entity-privacy-outsider-loads-sad", async ({ page }) => {
    const seeded = await seedAuth(page, "outsider");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/id/${encodeURIComponent(seeded.fixtures.entity_id)}`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    // Outsider may see redacted fields but must not crash the page.
    await expect(page.getByTestId("valence-entity-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});

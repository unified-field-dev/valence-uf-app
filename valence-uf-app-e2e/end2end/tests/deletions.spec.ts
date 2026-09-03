import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-deletions", () => {
  test("pw-deletion-index-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence/deletions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-deletion-index-page")).toBeVisible({
      timeout: 60_000,
    });
    // Index may list status/table columns without full UUID text; durable detail
    // coverage is in pw-deletion-run-detail-happy (DeletionService elevates to System).
  });

  test("pw-deletion-run-detail-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/deletion/${encodeURIComponent(seeded.fixtures.deletion_run_id)}`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-deletion-run-page")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-deletion-run-detail-sad-unknown", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/deletion/__valence_e2e_missing_deletion__`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-deletion-run-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});

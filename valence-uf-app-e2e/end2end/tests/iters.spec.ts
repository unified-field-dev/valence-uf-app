import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-valence-iters", () => {
  test("pw-iter-index-happy", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence/iters", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-iter-index-page")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-iter-run-detail-happy", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/iter/${encodeURIComponent(seeded.fixtures.iter_run_id)}`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-iter-run-page")).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-iter-run-detail-sad-unknown", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    const path = `/valence/schema/${encodeURIComponent(seeded.fixtures.schema_name)}/iter/__valence_e2e_missing_iter__`;
    await page.goto(path, { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-iter-run-page")).toBeVisible({
      timeout: 60_000,
    });
  });
});

import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Page } from "@playwright/test";

async function completeVisibleTour(page: Page) {
  const footer = page.locator('[data-testid="spotlight-footer"]:visible');
  const next = footer.getByTestId("spotlight-tour-next");
  await expect(footer).toBeVisible({ timeout: 60_000 });
  for (let i = 0; i < 32; i++) {
    if ((await footer.count()) === 0) {
      break;
    }
    // Spotlight panels can sit partially off-screen; DOM click avoids Playwright
    // viewport hit-testing failures that still occur with { force: true }.
    await next.evaluate((el: HTMLElement) => el.click());
    try {
      await expect(footer).toHaveCount(0, { timeout: 2_000 });
      break;
    } catch {
      /* more steps */
    }
  }
  await expect(footer).toHaveCount(0, { timeout: 30_000 });
}

test.describe("help-spotlight", () => {
  test("help-spotlight-skips-when-seeded", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("valence-dashboard-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("help-step-valence-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-skips-auth-gate", async ({ page }) => {
    await seedAuth(page, "anonymous", { help_tour: true });
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("help-step-valence-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-dashboard-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
    await expect(page.getByTestId("help-step-valence-intro")).toHaveCount(0);

    await page.reload({ waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-intro")).toHaveCount(0);
  });

  test("help-spotlight-schema-index-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/schema", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-schema-index-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-schema-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const schema = seeded.fixtures.schema_name;
    await page.goto(`/valence/schema/${encodeURIComponent(schema)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-schema-detail-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-entity-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const { schema_name, entity_id } = seeded.fixtures;
    await page.goto(
      `/valence/schema/${encodeURIComponent(schema_name)}/id/${encodeURIComponent(entity_id)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-entity-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-iter-run-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const { schema_name, iter_run_id } = seeded.fixtures;
    await page.goto(
      `/valence/schema/${encodeURIComponent(schema_name)}/iter/${encodeURIComponent(iter_run_id)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-iter-run-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-deletion-run-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const { schema_name, deletion_run_id } = seeded.fixtures;
    await page.goto(
      `/valence/schema/${encodeURIComponent(schema_name)}/deletion/${encodeURIComponent(deletion_run_id)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-deletion-run-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-traits-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/traits", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-traits-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-trait-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const trait = seeded.fixtures.trait_name;
    await page.goto(`/valence/traits/${encodeURIComponent(trait)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-trait-detail-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-iters-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/iters", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-iters-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-deletions-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/deletions", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-deletions-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });
});

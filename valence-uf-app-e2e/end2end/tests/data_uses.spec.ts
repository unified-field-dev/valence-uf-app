import { test, expect, seedAuth, waitForHydrated, expandShellNav } from "./fixtures";
import type { Page } from "@playwright/test";

/** Distinctive purposes from `src/data_use_catalog_fixtures.rs` (scan snapshot). */
const PURPOSE = {
  schemaUser: "E2E catalog: load user for Valence schema Data uses.",
  traitPrincipal: "E2E catalog: list PermissionPrincipal rows for trait Data uses.",
  unscoped: "E2E catalog: QueryCore walk for Unscoped uses.",
  testTwin: "E2E_TEST_ONLY_PURPOSE — must stay out of UI snapshot",
} as const;

const TRAIT_PERMISSION_PRINCIPAL = "PermissionPrincipal";
const IMPLEMENTOR_SCHEMAS = [
  "permission_user_principal",
  "permission_group_principal",
] as const;
/** Registered schema with no catalog fixtures → empty Data uses. */
const EMPTY_SCHEMA = "account";

async function openSchemaDataUses(page: Page, schemaName: string) {
  await page.goto(`/valence/schema/${encodeURIComponent(schemaName)}`, {
    waitUntil: "domcontentloaded",
  });
  await waitForHydrated(page);
  await expect(page.getByTestId("valence-schema-detail-page")).toBeVisible({
    timeout: 60_000,
  });
  await expect(page.getByTestId("valence-schema-data-uses")).toBeVisible({
    timeout: 60_000,
  });
}

async function openTraitDataUses(page: Page, traitName: string) {
  await page.goto(`/valence/traits/${encodeURIComponent(traitName)}`, {
    waitUntil: "domcontentloaded",
  });
  await waitForHydrated(page);
  await expect(page.getByTestId("valence-trait-detail-page")).toBeVisible({
    timeout: 60_000,
  });
  await expect(page.getByTestId("valence-trait-data-uses")).toBeVisible({
    timeout: 60_000,
  });
}

async function openUnscopedUses(page: Page) {
  await page.goto("/valence/unscoped-uses", { waitUntil: "domcontentloaded" });
  await waitForHydrated(page);
  await expect(page.getByTestId("valence-unscoped-uses-page")).toBeVisible({
    timeout: 60_000,
  });
  await expect(page.getByTestId("valence-unscoped-data-uses")).toBeVisible({
    timeout: 60_000,
  });
}

/** Advance Help tour until the visible spotlight header title matches `title`. */
async function advanceTourUntilTitle(page: Page, title: string) {
  const footer = page.locator('[data-testid="spotlight-footer"]:visible');
  const next = footer.getByTestId("spotlight-tour-next");
  const header = page.locator('[data-testid="spotlight-header"]:visible');
  await expect(footer).toBeVisible({ timeout: 60_000 });
  for (let i = 0; i < 32; i++) {
    if ((await header.count()) > 0) {
      const text = (await header.innerText()).trim();
      if (text === title) {
        return;
      }
    }
    if ((await footer.count()) === 0) {
      break;
    }
    await next.evaluate((el: HTMLElement) => el.click());
    try {
      await expect(header).toHaveText(title, { timeout: 1_500 });
      return;
    } catch {
      /* keep advancing */
    }
  }
  await expect(header).toHaveText(title, { timeout: 5_000 });
}

test.describe("pw-valence-data-uses", () => {
  test("TM-UI-S-1 schema Data uses purpose + View source", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");

    const panel = page.getByTestId("valence-schema-data-uses");
    await expect(panel.getByText(/Reads \(/)).toBeVisible();
    await expect(panel.getByText(PURPOSE.schemaUser)).toBeVisible({
      timeout: 60_000,
    });

    const source = panel.getByRole("link", { name: "View source" }).first();
    await expect(source).toBeVisible();
    await expect(source).toHaveAttribute("href", /\/blob\/main\//);
  });

  test("TM-UI-S-3 schema trait fan-out via-trait badge", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, IMPLEMENTOR_SCHEMAS[0]);

    const panel = page.getByTestId("valence-schema-data-uses");
    await expect(panel.getByText(PURPOSE.traitPrincipal)).toBeVisible({
      timeout: 60_000,
    });
    await expect(panel.getByText(`via ${TRAIT_PERMISSION_PRINCIPAL}`)).toBeVisible();
  });

  test("TM-UI-S-6 schema with no uses empty state", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, EMPTY_SCHEMA);

    const panel = page.getByTestId("valence-schema-data-uses");
    await expect(panel.getByText("No declared uses")).toBeVisible({
      timeout: 60_000,
    });
    await expect(panel.getByText(PURPOSE.schemaUser)).toHaveCount(0);
    await expect(panel.getByText(PURPOSE.traitPrincipal)).toHaveCount(0);
  });

  test("TM-UI-S-7 unauthenticated schema purposes denied", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/valence/schema/user", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);

    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByText(PURPOSE.schemaUser)).toHaveCount(0);
    await expect(page.getByTestId("valence-schema-data-uses")).toHaveCount(0);
  });

  test("TM-UI-T-1 trait page trait rows only", async ({ page }) => {
    await seedAuth(page, "admin");
    await openTraitDataUses(page, TRAIT_PERMISSION_PRINCIPAL);

    const panel = page.getByTestId("valence-trait-data-uses");
    await expect(panel.getByText(PURPOSE.traitPrincipal)).toBeVisible({
      timeout: 60_000,
    });
    await expect(panel.getByText(PURPOSE.schemaUser)).toHaveCount(0);
    await expect(panel.getByText(PURPOSE.unscoped)).toHaveCount(0);
  });

  test("TM-UI-T-5 unauthenticated trait purposes denied", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto(
      `/valence/traits/${encodeURIComponent(TRAIT_PERMISSION_PRINCIPAL)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);

    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByText(PURPOSE.traitPrincipal)).toHaveCount(0);
  });

  test("TM-UI-U-1 nav to Unscoped uses", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/valence", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expandShellNav(page);

    await page.getByTestId("nav-unscoped-uses").click();
    await expect(page).toHaveURL(/\/valence\/unscoped-uses/);
    await expect(page.getByTestId("valence-unscoped-uses-page")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText("Unscoped uses").first()).toBeVisible();
  });

  test("TM-UI-U-2 Unscoped rows exclude schema and trait purposes", async ({
    page,
  }) => {
    await seedAuth(page, "admin");
    await openUnscopedUses(page);

    const panel = page.getByTestId("valence-unscoped-data-uses");
    await expect(panel.getByText(PURPOSE.unscoped)).toBeVisible({
      timeout: 60_000,
    });
    await expect(panel.getByText(PURPOSE.schemaUser)).toHaveCount(0);
    await expect(panel.getByText(PURPOSE.traitPrincipal)).toHaveCount(0);

    const source = panel.getByRole("link", { name: "View source" }).first();
    await expect(source).toBeVisible();
    await expect(source).toHaveAttribute(
      "href",
      /github\.com\/unified-field-dev\/valence-uf-app\/blob\/main\//,
    );
  });

  test("TM-UI-U-5 unauthenticated Unscoped denied", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/valence/unscoped-uses", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);

    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByText(PURPOSE.unscoped)).toHaveCount(0);
  });

  test("TM-UI-X-1 trait fan-out on implementors not Unscoped", async ({
    page,
  }) => {
    await seedAuth(page, "admin");

    await openTraitDataUses(page, TRAIT_PERMISSION_PRINCIPAL);
    await expect(
      page.getByTestId("valence-trait-data-uses").getByText(PURPOSE.traitPrincipal),
    ).toBeVisible({ timeout: 60_000 });

    for (const schema of IMPLEMENTOR_SCHEMAS) {
      await openSchemaDataUses(page, schema);
      const panel = page.getByTestId("valence-schema-data-uses");
      await expect(panel.getByText(PURPOSE.traitPrincipal)).toBeVisible({
        timeout: 60_000,
      });
      await expect(panel.getByText(`via ${TRAIT_PERMISSION_PRINCIPAL}`)).toBeVisible();
    }

    await openUnscopedUses(page);
    await expect(
      page.getByTestId("valence-unscoped-data-uses").getByText(PURPOSE.traitPrincipal),
    ).toHaveCount(0);
  });

  test("TM-UI-X-2 tests twin purpose absent on all surfaces", async ({
    page,
  }) => {
    await seedAuth(page, "admin");

    await openSchemaDataUses(page, "user");
    await expect(page.getByText(PURPOSE.testTwin)).toHaveCount(0);
    await expect(page.getByText(PURPOSE.schemaUser)).toBeVisible({
      timeout: 60_000,
    });

    await openTraitDataUses(page, TRAIT_PERMISSION_PRINCIPAL);
    await expect(page.getByText(PURPOSE.testTwin)).toHaveCount(0);

    await openUnscopedUses(page);
    await expect(page.getByText(PURPOSE.testTwin)).toHaveCount(0);
    await expect(
      page.getByTestId("valence-unscoped-data-uses").getByText(PURPOSE.unscoped),
    ).toBeVisible({ timeout: 60_000 });
  });

  test("TM-UI-HELP-1 schema Data uses spotlight step", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/schema/user", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-schema-detail-intro")).toBeVisible({
      timeout: 60_000,
    });
    await advanceTourUntilTitle(page, "Data uses");
    await expect(page.getByTestId("valence-schema-data-uses")).toBeAttached();
    await expect(page.getByTestId("help-step-valence-schema-data-uses")).toBeAttached();
  });

  test("TM-UI-HELP-2 trait Data uses spotlight step", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto(
      `/valence/traits/${encodeURIComponent(TRAIT_PERMISSION_PRINCIPAL)}`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-trait-detail-intro")).toBeVisible({
      timeout: 60_000,
    });
    await advanceTourUntilTitle(page, "Data uses");
    await expect(page.getByTestId("valence-trait-data-uses")).toBeAttached();
    await expect(page.getByTestId("help-step-valence-trait-data-uses")).toBeAttached();
  });

  test("TM-UI-HELP-3 Unscoped uses spotlight intro", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/unscoped-uses", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-valence-unscoped-uses-intro")).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByTestId("valence-unscoped-uses-page")).toBeVisible();
  });
});

import { test, expect, seedAuth, waitForHydrated, expandShellNav } from "./fixtures";
import type { Page } from "@playwright/test";

/** Distinctive purpose substrings from `src/data_use_catalog_fixtures.rs` (scan snapshot). */
const PURPOSE = {
  // Must not match file-path captions (`…/data_use_catalog_fixtures.rs`) or e2e_valence.
  schemaUser: "User access in data_use_catalog_fixtures so the suite",
  traitPrincipal: "Permission Principal Query All",
  unscoped: "Fixture this data access",
  testTwin: "E2E_TEST_ONLY_PURPOSE — must stay out of UI snapshot",
  hopOwner: "Todo owner hop",
  hopReverse: "Todo reverse owner",
  hopProjectOwner: "Project owner hop",
  hopTasks: "Project tasks hop",
  hopTagsGet: "Todo tags hop",
  hopRelate: "Todo relate tag",
  hopUnrelate: "Todo unrelate tag",
  // `valence-app/src/server/entities.rs`: the console's own use, reached through the host graph.
  appEntityExists: "check that the record id exists",
  // `probe/`: crates outside the e2e host that the host links or declares as inventory deps.
  probeProductRead: "a crate outside the workspace reaches the catalog",
  probeWorkerUpdate: "an out-of-process binary reaches the catalog",
  probeUnwired: "PROBE_UNWIRED_MUST_NOT_APPEAR",
} as const;

/** `data-use-probe-product::PROBE_TABLE`. */
const PROBE_SCHEMA = "data_use_probe_widget";

function dataUseRow(panel: ReturnType<Page["getByTestId"]>, purpose: string) {
  return panel.getByTestId("valence-data-use-row").filter({ hasText: purpose });
}

const TRAIT_PERMISSION_PRINCIPAL = "PermissionPrincipal";
const IMPLEMENTOR_SCHEMAS = [
  "permission_user_principal",
  "permission_group_principal",
] as const;
/** `data-use-probe-product::UNUSED_TABLE`: registered, with no declared uses anywhere in the host graph. */
const EMPTY_SCHEMA = "data_use_probe_unused";

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

/** Primary op tabs use `/^Reads (/` so they do not match `Referenced Reads (`. */
async function selectDataUsesTab(page: Page, panelTestId: string, label: RegExp) {
  const panel = page.getByTestId(panelTestId);
  await panel.getByRole("tab", { name: label }).click();
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
    await expect(panel.getByRole("tab", { name: /^Reads \(/ })).toBeVisible();
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
    await expect(panel.getByText(PURPOSE.hopOwner)).toHaveCount(0);
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

    const source = dataUseRow(panel, PURPOSE.unscoped).getByRole("link", {
      name: "View source",
    });
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

  test("TM-UI-R-1 HasOne inbound Referenced Read on User", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toBeVisible({ timeout: 60_000 });
    await expect(panel.getByRole("link", { name: "todo" })).toBeVisible();
  });

  test("TM-UI-R-2 primary hop stays on Todo Reads", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "todo");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toBeVisible({ timeout: 60_000 });
    await expect(panel.getByText("Source", { exact: true })).toHaveCount(0);
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toHaveCount(0);
  });

  test("TM-UI-R-3 plain User get not under Referenced", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    await expect(panel.getByText(PURPOSE.schemaUser)).toBeVisible({ timeout: 60_000 });
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.schemaUser)).toHaveCount(0);
  });

  test("TM-UI-R-4 reverse get_from not on User Referenced", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "todo");
    const todoPanel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    await expect(todoPanel.getByText(PURPOSE.hopReverse)).toBeVisible({
      timeout: 60_000,
    });

    await openSchemaDataUses(page, "user");
    const userPanel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(userPanel.getByText(PURPOSE.hopReverse)).toHaveCount(0);
  });

  test("TM-UI-R-5 HasMany peer Referenced Read on Task", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "task");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopTasks)).toBeVisible({ timeout: 60_000 });
    await expect(panel.getByRole("link", { name: "project" })).toBeVisible();

    await openSchemaDataUses(page, "project");
    const projectPanel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(projectPanel.getByText(PURPOSE.hopTasks)).toHaveCount(0);
  });

  test("TM-UI-R-6 M2M get Referenced Read on Tag", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "tag");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopTagsGet)).toBeVisible({ timeout: 60_000 });
  });

  test("TM-UI-R-7 M2M relate under Referenced Updates", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "tag");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Updates \(/);
    await expect(panel.getByText(PURPOSE.hopRelate)).toBeVisible({ timeout: 60_000 });
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopRelate)).toHaveCount(0);
  });

  test("TM-UI-R-8 M2M unrelate under Referenced Updates", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "tag");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Updates \(/);
    await expect(panel.getByText(PURPOSE.hopUnrelate)).toBeVisible({ timeout: 60_000 });
  });

  test("TM-UI-R-9 multi-source onto User Referenced Reads", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toBeVisible({ timeout: 60_000 });
    await expect(panel.getByText(PURPOSE.hopProjectOwner)).toBeVisible();
    await expect(panel.getByRole("link", { name: "todo" })).toBeVisible();
    await expect(panel.getByRole("link", { name: "project" })).toBeVisible();
  });

  test("TM-UI-R-10 tab isolation Reads vs Referenced", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    await expect(panel.getByText(PURPOSE.schemaUser)).toBeVisible({ timeout: 60_000 });
    await expect(panel.getByText(PURPOSE.hopOwner)).toHaveCount(0);
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toBeVisible();
    await expect(panel.getByText(PURPOSE.schemaUser)).toHaveCount(0);
  });

  test("TM-UI-R-11 View source on referenced row", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, "user");
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /Referenced Reads \(/);
    await expect(panel.getByText(PURPOSE.hopOwner)).toBeVisible({ timeout: 60_000 });
    const source = panel.getByRole("link", { name: "View source" }).first();
    await expect(source).toBeVisible();
    await expect(source).toHaveAttribute("href", /\/blob\/main\//);
  });

  test("TM-UI-R-12 unauth deny with referenced fixtures", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/valence/schema/user", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByText(PURPOSE.hopOwner)).toHaveCount(0);
    await expect(page.getByTestId("valence-schema-data-uses")).toHaveCount(0);
  });

  test("TM-UI-R-13 trait page has no Referenced tabs", async ({ page }) => {
    await seedAuth(page, "admin");
    await openTraitDataUses(page, TRAIT_PERMISSION_PRINCIPAL);
    const panel = page.getByTestId("valence-trait-data-uses");
    await expect(panel.getByRole("tab", { name: /Referenced Reads/ })).toHaveCount(0);
    await expect(panel.getByText(PURPOSE.hopOwner)).toHaveCount(0);
  });

  test("TM-UI-R-14 Unscoped page has no Referenced tabs", async ({ page }) => {
    await seedAuth(page, "admin");
    await openUnscopedUses(page);
    const panel = page.getByTestId("valence-unscoped-data-uses");
    await expect(panel.getByRole("tab", { name: /Referenced Reads/ })).toHaveCount(0);
    await expect(panel.getByText(PURPOSE.hopOwner)).toHaveCount(0);
  });

  test("TM-UI-R-HELP-1 schema help mentions Referenced", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/valence/schema/user", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await advanceTourUntilTitle(page, "Data uses");
    await expect(page.getByTestId("help-step-valence-schema-data-uses")).toBeAttached();
    await expect(
      page
        .getByTestId("help-step-valence-schema-data-uses")
        .getByText("Referenced rows name the Source schema"),
    ).toBeVisible();
  });

  test("TM-UI-M-1 valence-app own uses appear on Unscoped", async ({ page }) => {
    await seedAuth(page, "admin");
    await openUnscopedUses(page);
    const panel = page.getByTestId("valence-unscoped-data-uses");
    const row = dataUseRow(panel, PURPOSE.appEntityExists);
    await expect(row).toBeVisible({ timeout: 60_000 });
    await expect(row.getByText(/^valence-app · /)).toBeVisible();
    await expect(row.getByText(/^valence-app\/src\/server\/entities\.rs:\d+$/)).toBeVisible();
  });

  test("TM-UI-D-1 linked product crate outside the workspace on Reads", async ({
    page,
  }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, PROBE_SCHEMA);
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    const row = dataUseRow(panel, PURPOSE.probeProductRead);
    await expect(row).toBeVisible({ timeout: 60_000 });
    await expect(row.getByText(/^data-use-probe-product · /)).toBeVisible();
  });

  test("TM-UI-D-2 inventory-only worker binary on Updates", async ({ page }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, PROBE_SCHEMA);
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Updates \(/);
    const row = dataUseRow(panel, PURPOSE.probeWorkerUpdate);
    await expect(row).toBeVisible({ timeout: 60_000 });
    await expect(row.getByText(/^data-use-probe-worker · /)).toBeVisible();
  });

  test("TM-UI-D-3 undeclared binary absent from widget and Unscoped", async ({
    page,
  }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, PROBE_SCHEMA);
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    await expect(dataUseRow(panel, PURPOSE.probeProductRead)).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.getByText(PURPOSE.probeUnwired)).toHaveCount(0);

    await openUnscopedUses(page);
    await expect(
      page.getByTestId("valence-unscoped-data-uses").getByText(PURPOSE.appEntityExists),
    ).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(PURPOSE.probeUnwired)).toHaveCount(0);
  });

  test("TM-UI-D-4 View source is repo-relative for the probe product", async ({
    page,
  }) => {
    await seedAuth(page, "admin");
    await openSchemaDataUses(page, PROBE_SCHEMA);
    const panel = page.getByTestId("valence-schema-data-uses");
    await selectDataUsesTab(page, "valence-schema-data-uses", /^Reads \(/);
    const row = dataUseRow(panel, PURPOSE.probeProductRead);
    await expect(row).toBeVisible({ timeout: 60_000 });
    await expect(row.getByRole("link", { name: "View source" })).toHaveAttribute(
      "href",
      /^https:\/\/github\.com\/unified-field-dev\/valence-uf-app\/blob\/main\/probe\/data-use-probe-product\/src\/lib\.rs#L\d+$/,
    );
  });
});

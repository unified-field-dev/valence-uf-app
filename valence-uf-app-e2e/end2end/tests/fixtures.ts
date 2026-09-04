import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "admin" | "outsider" | "unverified";

export type SeedFixtures = {
  schema_name: string;
  entity_id: string;
  trait_name: string;
  iter_run_id: string;
  deletion_run_id: string;
  iter_name: string;
};

/** All Valence Help inventory keys — seed as seen so non-tour specs stay quiet. */
const VALENCE_HELP_STEPS_SEEN = [
  { route: "/valence", feature_highlight: "valence-intro", spotlight: null, replay: false },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-search",
    spotlight: "valence-dashboard-search",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-my-data",
    spotlight: "valence-dashboard-my-data",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-top-schemas",
    spotlight: "valence-dashboard-top-schemas",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-active-deletions",
    spotlight: "valence-dashboard-active-deletions",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-platform-toolbar",
    spotlight: "valence-dashboard-platform-toolbar",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-headline",
    spotlight: "valence-dashboard-headline",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-throughput",
    spotlight: "valence-dashboard-throughput",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-writes",
    spotlight: "valence-dashboard-writes",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-reads",
    spotlight: "valence-dashboard-reads",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-dashboard-errors",
    spotlight: "valence-dashboard-errors",
    replay: false,
  },
  {
    route: "/valence",
    feature_highlight: "valence-nav",
    spotlight: "valence-nav",
    replay: false,
  },
  {
    route: "/valence/deletions",
    feature_highlight: "valence-deletions-intro",
    spotlight: "valence-deletions-page",
    replay: false,
  },
  {
    route: "/valence/deletions",
    feature_highlight: "valence-deletions-list",
    spotlight: "valence-deletions-list",
    replay: false,
  },
  {
    route: "/valence/deletions",
    feature_highlight: "valence-deletions-open",
    spotlight: "valence-deletions-list",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight: "valence-deletion-run-intro",
    spotlight: "valence-deletion-run-header",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight: "valence-deletion-run-progress",
    spotlight: "valence-deletion-run-progress",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight: "valence-deletion-run-cancel",
    spotlight: "valence-deletion-run-cancel",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight: "valence-deletion-run-steps",
    spotlight: "valence-deletion-run-steps",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight: "valence-deletion-run-back",
    spotlight: "valence-deletion-run-back",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-intro",
    spotlight: "valence-entity-top-bar",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-fields",
    spotlight: "valence-entity-fields",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-connections",
    spotlight: "valence-entity-connections",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-owner",
    spotlight: "valence-entity-owner",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-privacy",
    spotlight: "valence-entity-privacy",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-iter-run",
    spotlight: "valence-entity-iter-run",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-deletions",
    spotlight: "valence-entity-deletions",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight: "valence-entity-export",
    spotlight: "valence-entity-export",
    replay: false,
  },
  {
    route: "/valence/iters",
    feature_highlight: "valence-iters-intro",
    spotlight: "valence-iters-page",
    replay: false,
  },
  {
    route: "/valence/iters",
    feature_highlight: "valence-iters-list",
    spotlight: "valence-iters-list",
    replay: false,
  },
  {
    route: "/valence/iters",
    feature_highlight: "valence-iters-open",
    spotlight: "valence-iters-list",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-intro",
    spotlight: "valence-iter-run-header",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-stats",
    spotlight: "valence-iter-run-stats",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-progress",
    spotlight: "valence-iter-run-progress",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-cancel",
    spotlight: "valence-iter-run-cancel",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-errors",
    spotlight: "valence-iter-run-errors",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight: "valence-iter-run-batches",
    spotlight: "valence-iter-run-batches",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-detail-intro",
    spotlight: "valence-schema-top-bar",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-overview",
    spotlight: "valence-schema-overview",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-samples",
    spotlight: "valence-schema-samples",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-open-latest",
    spotlight: "valence-schema-open-latest",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-export",
    spotlight: "valence-schema-export",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-privacy",
    spotlight: "valence-schema-privacy",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-fields",
    spotlight: "valence-schema-fields",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-connections",
    spotlight: "valence-schema-connections",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-traits",
    spotlight: "valence-schema-traits",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-iter-start",
    spotlight: "valence-schema-iter-start",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-iter-runs",
    spotlight: "valence-schema-iter-runs",
    replay: false,
  },
  {
    route: "/valence/schema/:schema_name",
    feature_highlight: "valence-schema-deletions",
    spotlight: "valence-schema-deletions",
    replay: false,
  },
  {
    route: "/valence/schema",
    feature_highlight: "valence-schema-index-intro",
    spotlight: "valence-schema-index-page",
    replay: false,
  },
  {
    route: "/valence/schema",
    feature_highlight: "valence-schema-index-search",
    spotlight: "valence-schema-index-search",
    replay: false,
  },
  {
    route: "/valence/schema",
    feature_highlight: "valence-schema-index-table",
    spotlight: "valence-schemas-list",
    replay: false,
  },
  {
    route: "/valence/schema",
    feature_highlight: "valence-schema-index-open",
    spotlight: "valence-schemas-list",
    replay: false,
  },
  {
    route: "/valence/traits/:trait_name",
    feature_highlight: "valence-trait-detail-intro",
    spotlight: "valence-trait-top-bar",
    replay: false,
  },
  {
    route: "/valence/traits/:trait_name",
    feature_highlight: "valence-trait-overview",
    spotlight: "valence-trait-overview",
    replay: false,
  },
  {
    route: "/valence/traits/:trait_name",
    feature_highlight: "valence-trait-fields",
    spotlight: "valence-trait-fields",
    replay: false,
  },
  {
    route: "/valence/traits/:trait_name",
    feature_highlight: "valence-trait-connections",
    spotlight: "valence-trait-connections",
    replay: false,
  },
  {
    route: "/valence/traits/:trait_name",
    feature_highlight: "valence-trait-used-by",
    spotlight: "valence-trait-used-by",
    replay: false,
  },
  {
    route: "/valence/traits",
    feature_highlight: "valence-traits-intro",
    spotlight: "valence-traits-page",
    replay: false,
  },
  {
    route: "/valence/traits",
    feature_highlight: "valence-traits-table",
    spotlight: "valence-traits-list",
    replay: false,
  },
  {
    route: "/valence/traits",
    feature_highlight: "valence-traits-open",
    spotlight: "valence-traits-list",
    replay: false,
  },
] as const;

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { help_tour?: boolean },
) {
  const helpTour = opts?.help_tour ?? false;
  await page.addInitScript(
    ([enableTour, seenSteps]) => {
      try {
        if (enableTour) {
          if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
            localStorage.removeItem("uf.help.tour_steps");
            sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
          }
          return;
        }
        localStorage.setItem("uf.help.tour_steps", JSON.stringify(seenSteps));
      } catch {
        /* ignore */
      }
    },
    [helpTour, VALENCE_HELP_STEPS_SEEN] as const,
  );

  const res = await page.request.post("/api/test/seed-data", {
    data: { auth },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  }>;
}

async function bootState(page: Page): Promise<"ready" | "error" | "loading"> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return "ready";
    }
    if (html.getAttribute("data-orbital-boot-state") === "error") {
      return "error";
    }
    return "loading";
  });
}

/**
 * Orbital can mark boot `error` from a non-WASM `unhandledrejection` matching
 * bare `fetch`, then refuse dismiss. Same recovery as chronon/gauge/spectra:
 * when wasm is complete and `main` is present, clear the error bit and dismiss.
 */
async function clearFalsePositiveBootError(page: Page): Promise<boolean> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return true;
    }
    if (html.getAttribute("data-orbital-boot-state") !== "error") {
      return false;
    }
    const progress = (
      window as unknown as {
        __orbitalBootProgress?: { steps?: { wasm?: string } };
        __orbitalBootDismissOverlay?: () => void;
      }
    );
    const wasmComplete =
      progress.__orbitalBootProgress?.steps?.wasm === "complete" ||
      document.querySelectorAll(".orbital-boot-step--complete").length >= 4;
    const shellReady = !!document.querySelector("main");
    if (!wasmComplete || !shellReady) {
      return false;
    }
    html.removeAttribute("data-orbital-boot-state");
    if (typeof progress.__orbitalBootDismissOverlay === "function") {
      progress.__orbitalBootDismissOverlay();
    }
    if (html.getAttribute("data-orbital-hydrated") !== "true") {
      html.setAttribute("data-orbital-hydrated", "true");
      document.getElementById("orbital-boot-overlay")?.remove();
    }
    return true;
  });
}

/**
 * Wait for Orbital hydrate. On terminal boot `error`, wait for wasm under a
 * false-positive error before reloading. Never reload while `loading`.
 */
export async function waitForHydrated(page: Page, timeoutMs = 180_000) {
  const deadline = Date.now() + timeoutMs;
  let refreshes = 0;
  const maxRefreshes = 3;

  while (Date.now() < deadline) {
    const state = await bootState(page);
    if (state === "ready") {
      break;
    }
    if (state === "error") {
      if (await clearFalsePositiveBootError(page)) {
        break;
      }
      const waitUntil = Math.min(Date.now() + 30_000, deadline);
      let recovered = false;
      while (Date.now() < waitUntil) {
        await page.waitForTimeout(500);
        if ((await bootState(page)) === "ready") {
          recovered = true;
          break;
        }
        if (await clearFalsePositiveBootError(page)) {
          recovered = true;
          break;
        }
      }
      if (recovered) {
        break;
      }
      if (refreshes >= maxRefreshes) {
        break;
      }
      refreshes += 1;
      await page.waitForTimeout(1_500);
      await page.reload({ waitUntil: "load" });
      continue;
    }
    await page.waitForTimeout(500);
  }

  if ((await bootState(page)) === "error") {
    await clearFalsePositiveBootError(page);
  }

  await expect
    .poll(async () => bootState(page), { timeout: 10_000 })
    .toBe("ready");
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Expand collapsed shell left-nav so nav-* testids become visible. */
export async function expandShellNav(page: Page) {
  const expand = page.getByRole("button", { name: "Expand navigation" });
  if (await expand.isVisible().catch(() => false)) {
    await expand.click();
  }
}

export const test = base;
export { expect };

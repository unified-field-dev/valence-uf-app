# valence-uf-app-e2e

Leptos lab host that mounts eager `ValenceRoutes` pages for Playwright. Lab-only:
insecure session cookies, `POST /api/test/seed-data`, harness auth (no lepton sign-in).

## Acceptance Test Map (slot K lock)

| Behavior | Unit | Integration (`runtime_contract`) | E2E Playwright | AWS | Bench |
|----------|------|----------------------------------|----------------|-----|-------|
| Schema list/detail | valence-backend helpers | SchemaRegistry lists `user` | `pw-schema-list-happy`, `pw-schema-detail-happy` / `*-sad-unknown` | N/A | N/A |
| Trait list/detail | valence-backend helpers | (registry) | `pw-trait-*` | N/A | N/A |
| Entity view / privacy | ID helpers | seeded user get / missing | `pw-entity-view-happy`, `*-sad-unknown`, `pw-entity-privacy-outsider-loads-sad` | N/A | N/A |
| Dashboard / nav | — | — | `pw-dashboard-happy-load`, `pw-dashboard-nav-schemas-happy` | N/A | N/A |
| Iter runs | lookup helpers | seeded pending run; partial-commit policy | `pw-iter-*` | N/A | N/A |
| Deletion runs | JSON map | seeded queued + cancel merge | `pw-deletion-*` | N/A | N/A |
| Authz | source smoke | ValenceAdmin allow/deny | `pw-valence-auth-gate-*` | N/A | N/A |
| Help spotlight tours | — | — | `pw-valence-help-spotlight-*` (skip + per-route green) | N/A | N/A |

`product_surface` source-scan tests remain **smoke** (composition guards), not primary coverage.

## Run

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cd /home/seanorourke/unified-field/L4-composers/valence-uf-app
cd valence-uf-app-e2e/end2end && npm ci && npx playwright install chromium && cd ../..
cargo leptos end-to-end --project valence-uf-app-e2e
```

Host listens on `127.0.0.1:3130`. Do not Ctrl-C; the run exits when Playwright finishes.

Runtime integration (no browser):

```bash
cargo test -p valence-uf-app-e2e --features ssr --test runtime_contract
```

## Seed

`POST /api/test/seed-data` with JSON `{ "auth": "admin" | "outsider" | "unverified" | "anonymous" }`.

Returns fixture ids: `schema_name`, `entity_id`, `trait_name`, `iter_run_id`, `deletion_run_id`, `iter_name`.

## Scenario catalog

Auth: `pw-valence-auth-gate-sad-anonymous`, `pw-valence-auth-gate-happy-admin`

Dashboard: `pw-dashboard-happy-load`, `pw-dashboard-nav-schemas-happy`

Schemas: `pw-schema-list-happy`, `pw-schema-detail-happy`, `pw-schema-detail-sad-unknown`

Entities: `pw-entity-view-happy`, `pw-entity-view-sad-unknown`, `pw-entity-privacy-outsider-loads-sad`

Traits: `pw-trait-list-happy`, `pw-trait-detail-happy`, `pw-trait-detail-sad-unknown`

Iters: `pw-iter-index-happy`, `pw-iter-run-detail-happy`, `pw-iter-run-detail-sad-unknown`

Deletions: `pw-deletion-index-happy`, `pw-deletion-run-detail-happy`, `pw-deletion-run-detail-sad-unknown`

Help spotlight: `help-spotlight-skips-when-seeded`, `help-spotlight-skips-auth-gate`, `help-spotlight-dashboard-green`, `help-spotlight-schema-index-green`, `help-spotlight-schema-detail-green`, `help-spotlight-entity-green`, `help-spotlight-iter-run-green`, `help-spotlight-deletion-run-green`, `help-spotlight-traits-green`, `help-spotlight-trait-detail-green`, `help-spotlight-iters-green`, `help-spotlight-deletions-green`

Default `seedAuth` marks all Valence Help steps seen (`replay: false`) so other specs stay quiet. Pass `{ help_tour: true }` only in the dedicated green-path suite.

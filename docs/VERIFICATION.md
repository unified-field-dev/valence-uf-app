# valence-uf-app verification

Re-run after code or doc changes. This workspace is the Valence operations app
(`valence-app` Leptos UI + `valence-backend` pure server contracts +
`valence-uf-app-e2e` lab host). Layer 1 covers schema/iter/deletion helpers and
sibling-source UI surface **smokes**. Layer 2 is the primary operator-UI gate
(Playwright + runtime contracts).

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
```

## Teaching host

Axum oneshot under [`examples/protected-valence-host`](../examples/protected-valence-host/).
Copy table + product mount sketches live in that host README.

```bash
cargo check -p protected-valence-host
cargo run -p protected-valence-host
```

Success line: `protected_valence_host: OK — /valence deny/allow + schema index`.
Hydrate/browser is out of gate for the oneshot (`cargo-leptos` + `wasm32` +
Orbital / `uf-product` belong to the Layer 2 lab host).

## Layer 1 — Unit + integration (CI)

GitHub Actions (`.github/workflows/ci.yml`) covers this Layer 1 subset plus the
teaching host and valence-backend rustdoc gate below.

Sibling-source UI **smokes** (no Orbital / `valence-app` compile):

```bash
cargo test -p valence-backend --test workspace_members --test product_surface
```

Backend contracts (preferred path; no UI graph):

```bash
cargo fmt -p valence-backend -p valence-app -p valence-uf-app-e2e -p protected-valence-host -- --check
cargo clippy -p valence-backend --all-targets -- -D warnings
cargo clippy -p protected-valence-host --all-targets -- -D warnings
cargo clippy -p valence-app --features ssr --all-targets -- \
  -D warnings -A clippy::pedantic -A clippy::nursery
cargo test -p valence-backend
```

`cargo fmt --all` can fail when a sibling checkout sits outside this workspace;
package-scoped fmt is the honest local gate.

Host-aligned SSR unit tests (when UI graph compiles):

```bash
cargo test -p valence-app --features ssr
```

Full workspace (includes `valence-app` UI). May fail when the sibling
`uf-product` / `uf-integrations` UI graph does not compile — that is a
host-product UI issue, not a Valence backend contract gap.
Surface needles for routes, nav testids, `RequireAuthenticated`, and
`ValenceAdmin` live in `product_surface` (**smoke** secondary; Layer 2 is primary).

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### leptos-lints (CI job `leptos-lints`)

Needs `cargo-dylint` / `dylint-link` 6.0.1 and toolchain `nightly-2025-05-14`
(see `leptos-lints@v0.1.2`). Workspace `[workspace.metadata.dylint]` pins the
library; rustc deny names are declared under `[workspace.lints.rust]`.
GitHub Actions runs the same command.

```bash
# cargo install cargo-dylint --locked --version 6.0.1
# cargo install dylint-link --locked --version 6.0.1
# rustup toolchain install nightly-2025-05-14 --component rustc-dev,llvm-tools-preview

export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
export CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback
export RUSTFLAGS="-D warnings -Zcrate-attr=feature(stdarch_x86_avx512)"

cargo dylint --all -p valence-app --no-deps -- --features hydrate
```

Hard CI job deferred: `valence-app` hydrate still depends on the Orbital / host
graph (same pin risk as UI compile in Layer 1). Run locally when that graph is
green.

## Layer 2 — E2E (lab host + Playwright, CI)

Primary operator-UI gate. Runs on pull requests and pushes to `main`/`master`
via the `e2e` job in `.github/workflows/ci.yml`. Dedicated lab host mounts eager
`ValenceRoutes` pages (same components as production Lazy routes), mem Valence,
Higgs session injection, and Gauge `ValenceAdmin`. Port `127.0.0.1:3130`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo check -p valence-uf-app-e2e --features ssr
cargo test -p valence-uf-app-e2e --features ssr --test runtime_contract
# From the valence-uf-app workspace root. Builds SSR + hydrate, then Playwright.
cd valence-uf-app-e2e/end2end && npm ci && npx playwright install chromium && cd ../..
cargo leptos end-to-end --project valence-uf-app-e2e
```

Do not interrupt the end-to-end run. It stops when Playwright finishes.

Scenario IDs (validating happy + sad): see
[`valence-uf-app-e2e/README.md`](../valence-uf-app-e2e/README.md).
Includes Help spotlight skip-by-default and per-route green paths
(`help_spotlight.spec.ts`).

## Layer 3 — Cloud + performance

**Waived.** This application workspace; no cloud resources or Criterion benches.
Correctness is in-process against Valence UF app DTO/mapping contracts and the
lab e2e host. L0 Valence / valence-platform campaigns remain separate for
worker orchestration.

## L5 host Playwright

**Deferred.** Live embedded/fleet `/valence` Playwright with full product host
wiring is out of scope for this workspace gate. Product ops-UI correctness is
covered by `valence-uf-app-e2e` (Layer 2).

## Rustdoc policy

Preferred deny gate (no UI graph):

```bash
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p valence-backend --no-deps
```

Workspace `rustdoc::broken_intra_doc_links` is `allow` in `Cargo.toml` because
sibling/cfg-gated links often fail under `--no-deps`. Prefer the
`RUSTDOCFLAGS` deny form above for the backend contract crate. `valence-app`
rustdoc with deny flags is pin-dependent on Orbital / host graphs.

## Notes

- Prefer `cargo test -p valence-backend` for backend contract CI when the UI
  dependency graph (`uf-product` via `uf-integrations` / `lepton-shell`) fails to
  compile — report that separately from Valence contract results.
- Tests may `unwrap`/`expect`; production server fns map failures to `ServerFnError`
  (no ordinary-path unwrap).
- Sad-path assertions check message content or `None` / empty — (stronger than `is_err()` alone).
- Happy-path tests are named `*_happy_path` so audits detect them.
- `product_surface` is labeled **smoke**; do not treat it as primary coverage.

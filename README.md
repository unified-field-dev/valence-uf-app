# Valence UF App

[![CI](https://github.com/unified-field-dev/valence-uf-app/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/valence-uf-app/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/valence-uf-app) · `cargo doc -p valence-backend --open`

## About

Valence UF App is the Unified Field **operations UI** for exploring Valence
schemas, entities, traits, iters, and deletions under `/valence`. Valence itself
has no built-in UI; hosts mount this crate so operators can discover schemas,
inspect privacy-aware entity views, and follow iter/deletion runs.

- **UI (`valence-app`)** — pages, Higgs `#[server]` wrappers, `ValenceRoutes`,
  `uf_app!` registration
- **Backend (`valence-backend`)** — pure schema/iter/deletion helpers (no Leptos);
  primary CI surface

Hosts supply a Valence runtime (router + actor), product schemas, and auth guard
context. Enable `ssr` / hydrate to match your host. Crate-root rustdoc owns
Concern → route → server fn tables; prefer `cargo doc -p valence-backend --open`
for the mapping contract. UI rustdoc is pin-dependent on Orbital / host graphs.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
valence-app = { git = "https://github.com/unified-field-dev/valence-uf-app", package = "valence-app", rev = "REPLACE_WITH_PIN", default-features = false }
valence-backend = { git = "https://github.com/unified-field-dev/valence-uf-app", package = "valence-backend", rev = "REPLACE_WITH_PIN" }
```

```rust,ignore
use valence_app::ValenceRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <ValenceRoutes />
    </Routes>
}
```

Wire Valence runtime + schemas + session extractors in host bootstrap, then mount
the routes above. Full Leptos SSR hosts live outside this repository; use the
local teaching host for the auth + schema index contract.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo test -p valence-backend
```

## Workspace

| Crate | Role |
|-------|------|
| [`valence-app`](valence-app/) | Leptos ops UI + `ValenceRoutes` + app registration |
| [`valence-backend`](valence-backend/) | Pure schema / iter / deletion helpers |
| [`protected-valence-host`](examples/protected-valence-host/) | Teaching host: deny/allow + schema index |

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-valence-host`](examples/protected-valence-host/) | Auth + `/valence` schema index | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-valence-uf-app cargo run -p protected-valence-host` | Deny/allow + schema JSON | Mount `ValenceRoutes` |

Copy table + product mount `Cargo.toml`:
[`examples/protected-valence-host/README.md`](examples/protected-valence-host/README.md).
More examples: [`examples/README.md`](examples/README.md).

## Security

Auth-gated `/valence` routes (layout gate plus `ValenceAdmin` for mutating
server fns) and private vulnerability reporting: [`SECURITY.md`](SECURITY.md).
Report vulnerabilities privately — do not open a public issue for
security-sensitive reports.

## Verify

GitHub Actions (`.github/workflows/ci.yml`) runs the CI subset from
[`docs/VERIFICATION.md`](docs/VERIFICATION.md): fmt, clippy `-D warnings` on
`valence-backend`, `valence-app` (SSR), and the teaching host; contract tests;
`protected-valence-host` check/run; valence-backend rustdoc with broken-intra-doc-link
deny; and the Layer 2 **e2e** job (`runtime_contract`, SSR units, Playwright).

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo fmt -p valence-backend -p valence-app -p protected-valence-host -- --check
cargo clippy -p valence-backend --all-targets -- -D warnings
cargo clippy -p valence-app --features ssr --all-targets -- \
  -D warnings -A clippy::pedantic -A clippy::nursery
cargo clippy -p protected-valence-host --all-targets -- -D warnings
cargo test -p valence-backend --test workspace_members --test product_surface
cargo test -p valence-backend
cargo check -p protected-valence-host
cargo run -p protected-valence-host
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p valence-backend --no-deps
```

Layer 2 (lab host + Playwright): see [`docs/VERIFICATION.md`](docs/VERIFICATION.md#layer-2--e2e-lab-host--playwright-ci).

Teaching host success line:
`protected_valence_host: OK — /valence deny/allow + schema index`.
Full command block: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is this a standalone Valence server?** No. `valence-app` mounts under a host
`<Routes>` tree. Persistence and worker orchestration live in Valence core /
valence-platform; hosts supply the runtime and schemas.

**Why is there a separate `valence-backend` crate?** So schema, iter, and
deletion helpers stay unit-testable without the Leptos/UI dependency graph.
`valence-app` `#[server]` fns are thin wrappers over those helpers.

**What can operators do from the UI?** Browse schemas and traits, inspect
privacy-aware entity views, and follow iter/deletion runs (including cancel and
delete-queue mutations when granted `ValenceAdmin`).

**Where does Valence core fit?** Schema registration, storage, and iter/deletion
workers live in Valence / valence-platform. This repo maps admin list/get/run
APIs into UF ops pages.

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md),
[SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

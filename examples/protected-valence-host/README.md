# protected-valence-host

Axum oneshot host under **`/valence`**: deny without session, allow with
`X-Demo-User`, return a seeded schema index using `valence-backend`
validate/lookup helpers.

Production Leptos hosts mount `ValenceRoutes` at **`/valence`** and gate
mutating ops with `ValenceAdmin`. This example proves the same path + auth +
schema-index contract without the SSR/WASM / Orbital graph. The oneshot path
`/valence` matches the Orbital app id/path (`valence` / `/valence`).

| | |
|---|---|
| **When to use** | First smoke of Valence UF app host wiring (auth gate + schema index API) |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-valence-uf-app cargo run -p protected-valence-host` |
| **Success** | Stdout: `protected_valence_host: OK — /valence deny/allow + schema index` |
| **Look next** | Mount [`ValenceRoutes`](../../valence-app/) ; wire Valence runtime + schemas |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `valence-backend` (schema index smoke) |
| Product mount `Cargo.toml` (below) | `valence-app` + `valence-backend` with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Session gate on `/valence`, schema JSON, inventory contract names |
| Leptos sketch (below) | `<ValenceRoutes />` under `/valence` |

### Product mount dependencies

```toml
[dependencies]
valence-app = { git = "https://github.com/unified-field-dev/valence-uf-app", package = "valence-app", rev = "REPLACE_WITH_PIN", default-features = false }
valence-backend = { git = "https://github.com/unified-field-dev/valence-uf-app", package = "valence-backend", rev = "REPLACE_WITH_PIN" }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "valence-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "valence-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use valence_app::ValenceRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <ValenceRoutes />
    </Routes>
}
```

Schema helpers (Leptos-free):

```rust,ignore
use valence_backend::{find_schema_by_name, sort_schemas_by_name, validate_schema_name};

validate_schema_name("demo_account")?;
sort_schemas_by_name(&mut schemas);
let found = find_schema_by_name(&schemas, "demo_account");
```

Inventory names match `valence` / `/valence`. Layout uses `RequireAuthenticated`;
mutating `#[server]` fns carry `ValenceAdmin` (manifest
`permissions::ValencePermission`). Wire a Valence runtime + product schemas +
session extractors in host bootstrap before mounting the routes.

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/unified-field-dev/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then mount `ValenceRoutes`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo check -p protected-valence-host
cargo run -p protected-valence-host
```

**Success:** stdout prints `protected_valence_host: OK — /valence deny/allow + schema index`.

## Hydrate / browser

Out of gate for this host. Full ops UI needs a product binary with
`cargo-leptos`, `wasm32`, session chrome, Valence runtime + schemas, and a
working Orbital / `uf-product` graph. Prefer the oneshot above.

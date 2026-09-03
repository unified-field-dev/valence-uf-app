# Examples

Runnable teaching hosts for this UF app. Each card: when to use · command ·
success · look next.

## Canonical path

### `protected-valence-host` — auth + `/valence` schema index

**Teaches:** session auth gate on `/valence` and seeded schema-index helpers
from `valence-backend`. Inventory names: `valence` / `/valence` /
`RequireAuthenticated` / `ValenceAdmin`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo run -p protected-valence-host
```

**Success:** stdout prints `protected_valence_host: OK — /valence deny/allow + schema index`.

**Next step:** Mount `<ValenceRoutes />` in a product host with Valence runtime
+ schemas.

Copy table + product mount `Cargo.toml`:
[`protected-valence-host/README.md`](protected-valence-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-valence-host`](protected-valence-host/) | Auth + `/valence` schema index | `cargo run -p protected-valence-host` | Deny/allow + schema JSON | Product host with `ValenceRoutes` |

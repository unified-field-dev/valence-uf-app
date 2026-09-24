# valence-app

Leptos operations UI for Valence: schema discovery, privacy-aware entity
inspection, traits, declared data uses, and iter/deletion run surfaces under
`/valence`.

```toml
# Pin tag or rev — do not use branch = "main".
valence-app = { git = "https://github.com/unified-field-dev/valence-uf-app", package = "valence-app", rev = "REPLACE_WITH_PIN", default-features = false }
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

Crate-root rustdoc owns Features, mount/getting-started guides, the route table,
and the Examples. Mapping helpers live in `valence-backend`.

Compose into a host that supplies a Valence runtime (router + actor), product
schemas, and the auth/context extractors the app expects. Enable `ssr` /
`hydrate` to match your host. For Help spotlight tours, enable `uf-integrations`
`offering-help` (or `full`) and call `valence_app::ensure_help_steps_linked()`.

## Data uses surfaces

Authenticated users (same gate as the schema index) can browse declared
purpose-required Model / Query APIs with `use_!` purposes:

- Schema detail — **Data uses** card (Schema rows plus trait fan-out) via
  `get_schema_data_uses`
- Trait detail — **Data uses** card (trait-scoped rows only) via
  `get_trait_data_uses`
- `/valence/unscoped-uses` — QueryCore / Unscoped declarations via
  `get_unscoped_data_uses` (nav sibling to Schemas)

Wire `uf-valence-data-use-scan` from the host `build.rs` so SSR includes the
`OUT_DIR/data_uses.rs` snapshot. Schemas and traits must declare `repository:`
for View source links. Help spotlights cover schema Data uses, trait Data uses,
and Unscoped uses. Full teaching path: crate rustdoc
[Browse declared data uses](https://docs.rs/valence-app).

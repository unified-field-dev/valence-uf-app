# valence-app

Leptos operations UI for Valence: schema discovery, privacy-aware entity
inspection, traits, and iter/deletion run surfaces under `/valence`.

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

Crate-root rustdoc owns Organized-by-task, Owns / does not own, the route table,
and the Examples. Mapping helpers live in `valence-backend`.

Compose into a host that supplies a Valence runtime (router + actor), product
schemas, and the auth/context extractors the app expects. Enable `ssr` /
`hydrate` to match your host. For Help spotlight tours, enable `uf-integrations`
`offering-help` (or `full`) and call `valence_app::ensure_help_steps_linked()`.

#![recursion_limit = "256"]
//! Valence operations app — browse schemas, inspect entities, and follow trait, iter, and
//! deletion runs under `/valence`.
//!
//! Leptos UI mounted under `/valence` so operators can inspect schema metadata, entity
//! fields and privacy evaluation, and iter/deletion run progress without building custom
//! pages. Registers alongside other product apps via `uf_app!` and gates mutating server
//! functions behind `ValenceAdmin`.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Features
//!
//! - **Valence admin routes** — Provides the nested `/valence` route tree behind auth for
//!   dashboard, schema, entity, trait, iter, and deletion pages. Mount once when the host
//!   router starts. [Get started](#mount-valence-routes)
//! - **Operator dashboard** — Shows My Data row counts on [`ValenceDashboardPage`] via
//!   [`get_dashboard_my_data_stats`] plus schema-or-id search from [`search_schema_or_id`].
//!   [Get started](#operator-dashboard)
//! - **Schema catalog** — Lists registered schemas and opens detail pages via [`get_schemas`]
//!   and [`get_schema`]. [Get started](#browse-schemas)
//! - **Entity inspection** — Shows field rows, privacy evaluation, and ownership transfers
//!   on [`ValenceEntityPage`] via [`get_entity_view`]; admins queue deletes with
//!   [`delete_entity_queue`]. [Get started](#inspect-entity)
//! - **Trait catalog** — Lists reusable field bundles and opens detail pages via
//!   [`get_traits`] and [`get_trait`]. [Get started](#browse-traits)
//! - **Iter run visibility** — Shows batch progress across schemas via [`list_iter_runs`]
//!   and [`get_iter_run`]; admins stop in-flight work with [`cancel_iter_run`].
//!   [Get started](#follow-iter-runs)
//! - **Deletion run visibility** — Shows cascade progress via [`list_deletion_runs`] and
//!   [`get_deletion_run`]; admins stop in-flight work with [`cancel_deletion_run`].
//!   [Get started](#follow-deletion-runs)
//! - **Help spotlight tours** — Route-scoped Help steps for dashboard, schemas, entities,
//!   traits, iters, and deletions. Call [`ensure_help_steps_linked`] so inventory links
//!   into the host; enable `offering-help` on the product shell.
//!   [Get started](#help-spotlight-tours)
//! - **Server function wrappers** — Exposes [`mod@server`] Higgs `#[server]` fns and DTO
//!   re-exports backed by [`valence_backend`] pure helpers.
//!
//! ## Mount Valence routes
//!
//! [`ValenceRoutes`] nests the full `/valence` subtree inside a host Leptos `<Routes>` tree.
//! Operators get dashboard, schema catalog, entity inspection, and iter/deletion run pages.
//! Mount during host router setup at startup, alongside other `uf_app!` product routes — the
//! macro registers launcher metadata and the `/valence` inventory entry.
//!
//! **Prerequisites:** `ssr` on this crate; authenticated session; `ValenceAdmin` for mutators
//! ([`VALENCE_ADMIN_PERMISSION`]); host Valence registry and worker backends for live data.
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use valence_app::ValenceRoutes;
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <ValenceRoutes />
//!     </Routes>
//! }
//! ```
//!
//! On success `/valence` resolves to the dashboard, `/valence/schema` lists registered
//! schemas, and nested entity, trait, iter, and deletion routes load per-schema pages.
//! Unauthenticated sessions are rejected by server functions — see root `SECURITY.md`.
//!
//! ## Operator dashboard
//!
//! The dashboard answers how much of the signed-in operator's My Data footprint is live:
//! owned rows, tables with data, and pending deletion counts. [`ValenceDashboardPage`] calls
//! [`get_dashboard_my_data_stats`] on each SSR render and [`search_schema_or_id`] when the
//! operator submits the search box — use this landing page after mounting routes when
//! operators need a quick health snapshot or a jump to schema/entity detail.
//!
//! **Prerequisites:** [`ValenceRoutes`] mounted; `ssr` feature; authenticated session with
//! Valence viewer access.
//!
//! ```rust,ignore
//! use valence_app::{
//!     ValenceDashboardPage, get_dashboard_my_data_stats, search_schema_or_id,
//!     DashboardMyDataStats,
//! };
//!
//! // ValenceDashboardPage calls these on each SSR render / search submit:
//! let stats: DashboardMyDataStats = get_dashboard_my_data_stats().await?;
//! assert_eq!(stats.tables_with_data, 3);
//! assert_eq!(stats.owned_rows, 12);
//!
//! search_schema_or_id("counter".into()).await?;
//! ```
//!
//! On success `stats` carries `owned_rows`, `tables_with_data`, and `pending_deletion_rows`;
//! search redirects to `/valence/schema/{name}` or `/valence/schema/{name}/id/{id}` when the
//! token resolves. Blank or unknown tokens return without navigation.
//!
//! ## Browse schemas
//!
//! Schema pages list every registered Valence schema and show fields, privacy policies,
//! samples, and registered iters on detail. [`ValenceSchemaIndexPage`] loads [`get_schemas`]
//! for the index; [`ValenceSchemaPage`] calls [`get_schema`] for one name. Open these routes
//! when operators need field types, foreign keys, or quick links into entity and iter views.
//!
//! **Prerequisites:** [`ValenceRoutes`] mounted; `ssr` feature; schema names must pass
//! `valence_backend::validate_schema_name` before registry lookup.
//!
//! ```rust,ignore
//! use valence_app::{
//!     ValenceSchemaIndexPage, ValenceSchemaPage, get_schemas, get_schema, SchemaListItem,
//! };
//!
//! // ValenceSchemaIndexPage loads get_schemas for the index:
//! let schemas: Vec<SchemaListItem> = get_schemas().await?;
//! assert_eq!(schemas.first().map(|s| s.name.as_str()), Some("counter"));
//!
//! let detail = get_schema("counter".into()).await?;
//! assert_eq!(detail.as_ref().map(|d| d.name.as_str()), Some("counter"));
//! ```
//!
//! On success the index returns sorted [`SchemaListItem`] rows and detail resolves one schema
//! or returns `None` when the name is unknown. Blank or path-unsafe names fail validation
//! before registry lookup.
//!
//! ## Inspect entity
//!
//! Entity pages show field values, privacy policy evaluation, ownership transfers, and
//! admin actions for one record. [`ValenceEntityPage`] calls [`get_entity_view`] for the
//! main card stack; admins call [`delete_entity_queue`] to enqueue a deletion run. Open this
//! route when operators audit a single row or queue cascade deletion.
//!
//! **Prerequisites:** Routes mounted; `ssr` feature; entity ids must pass
//! `valence_backend::validate_entity_id`; [`delete_entity_queue`] requires `ValenceAdmin`.
//!
//! ```rust,ignore
//! use valence_app::{ValenceEntityPage, get_entity_view, delete_entity_queue, EntityView};
//!
//! // ValenceEntityPage calls get_entity_view for the card stack:
//! let view: Option<EntityView> = get_entity_view("counter".into(), "singleton".into()).await?;
//! assert_eq!(view.as_ref().map(|v| v.schema.name.as_str()), Some("counter"));
//!
//! delete_entity_queue("counter".into(), "singleton".into()).await?;
//! ```
//!
//! On success `view` carries field rows and connection metadata for the entity. Deletion
//! queue returns after the run is accepted; denied `ValenceAdmin` sessions surface as
//! `ServerFnError` before the queue call runs.
//!
//! ## Browse traits
//!
//! Trait pages list reusable field bundles and show which schemas implement each trait.
//! [`ValenceTraitIndexPage`] loads [`get_traits`] for the index; [`ValenceTraitDetailPage`]
//! calls [`get_trait`] for one name. Open these routes when operators confirm trait field
//! names before inspecting a schema that includes them.
//!
//! **Prerequisites:** [`ValenceRoutes`] mounted; `ssr` feature; trait names must pass
//! `valence_backend::validate_trait_name` on detail lookup.
//!
//! ```rust,ignore
//! use valence_app::{
//!     ValenceTraitIndexPage, ValenceTraitDetailPage, get_traits, get_trait, TraitListItem,
//! };
//!
//! // ValenceTraitIndexPage loads get_traits for the index:
//! let traits: Vec<TraitListItem> = get_traits().await?;
//! assert_eq!(traits.first().map(|t| t.name.as_str()), Some("HasOwner"));
//!
//! let detail = get_trait("HasOwner".into()).await?;
//! assert_eq!(detail.as_ref().map(|d| d.name.as_str()), Some("HasOwner"));
//! ```
//!
//! On success the index returns sorted trait rows and detail resolves one trait or returns
//! `None` when the name is unknown.
//!
//! ## Follow iter runs
//!
//! Iter run pages list batch progress across schemas and drill into per-run errors and
//! batches. [`ValenceIterIndexPage`] calls [`list_iter_runs`]; [`ValenceIterRunPage`] calls
//! [`get_iter_run`]; admins call [`cancel_iter_run`] to stop an in-flight run. Open these
//! routes when auditing a long-running iter or stopping a stuck batch.
//!
//! **Prerequisites:** Routes mounted; `ssr` feature; run ids must pass
//! `valence_backend::validate_run_id`; [`cancel_iter_run`] requires `ValenceAdmin`.
//!
//! ```rust,ignore
//! use valence_app::{
//!     ValenceIterIndexPage, list_iter_runs, get_iter_run, cancel_iter_run, IterRunSummary,
//! };
//!
//! // ValenceIterIndexPage loads list_iter_runs for the index:
//! let page = list_iter_runs(0, 20).await?;
//! let first: &IterRunSummary = page.items.first().expect("iter run");
//! assert_eq!(first.run_id, "run-1");
//!
//! let detail = get_iter_run("run-1".into()).await?;
//! assert_eq!(detail.as_ref().map(|d| d.run_id.as_str()), Some("run-1"));
//!
//! cancel_iter_run("run-1".into()).await?;
//! ```
//!
//! On success `page.items` carries recent [`IterRunSummary`] rows and detail resolves one
//! run view. Cancel returns `Ok(())` when the worker accepts the stop request.
//!
//! ## Follow deletion runs
//!
//! Deletion run pages list cascade progress across schemas and drill into per-step status.
//! [`ValenceDeletionIndexPage`] calls [`list_deletion_runs`]; [`ValenceDeletionRunPage`]
//! calls [`get_deletion_run`]; admins call [`cancel_deletion_run`] to stop an in-flight run.
//! Open these routes when auditing cascade deletes or stopping a stuck run.
//!
//! **Prerequisites:** Routes mounted; `ssr` feature; run ids must pass
//! `valence_backend::validate_run_id`; [`cancel_deletion_run`] requires `ValenceAdmin`.
//!
//! ```rust,ignore
//! use valence_app::{
//!     ValenceDeletionIndexPage, list_deletion_runs, get_deletion_run, cancel_deletion_run,
//!     DeletionRunView,
//! };
//!
//! // ValenceDeletionIndexPage loads list_deletion_runs for the index:
//! let runs: Vec<DeletionRunView> = list_deletion_runs(20).await?;
//! assert_eq!(runs.first().map(|r| r.run_id.as_str()), Some("del-1"));
//!
//! let detail: Option<DeletionRunView> = get_deletion_run("del-1".into()).await?;
//! assert_eq!(detail.as_ref().map(|d| d.run_id.as_str()), Some("del-1"));
//!
//! cancel_deletion_run("del-1".into()).await?;
//! ```
//!
//! On success `runs` carries recent deletion summaries and detail resolves one run view.
//! Cancel returns `Ok(())` when the worker accepts the stop request.
//!
//! ## Help spotlight tours
//!
//! Valence ships Help spotlight steps for each ops route (dashboard, schema catalog,
//! schema detail, entity, trait catalog, trait detail, iter index, iter run, deletion
//! index, deletion run). Hosts that enable `offering-help` (or `full`) mount
//! `HelpTourPlayer`; call [`ensure_help_steps_linked`] at route mount so `inventory`
//! submissions from [`mod@help_steps`] are retained.
//!
//! **Prerequisites:** `uf-help` hydrate/ssr features on this crate; product host with
//! Help player mounted; authenticated session for Valence visit tracking.
//!
//! ```rust,ignore
//! use valence_app::{ensure_help_steps_linked, ValenceRoutes};
//!
//! ensure_help_steps_linked();
//! // Mount ValenceRoutes inside the host <Routes> tree as usual.
//! ```
//!
//! On success, visiting `/valence` (and other Valence paths) can show pending spotlight
//! steps. Replay restarts the tour for the current route via the Help menu.
//!
//! ## Feature flags
//!
//! | Flag | Effect |
//! |------|--------|
//! | `ssr` | Server-side Leptos split; required for `#[server]` fns and Higgs/Valence IO. |
//! | `hydrate` | Client-side hydration for routed pages and Orbital shell components. |
//!
//! ## Routes
//!
//! Mounted under `/valence` by [`ValenceRoutes`]. Auth gating lives in [`ValenceLayout`].
//! Mutating server fns listed in `valence_backend::VALENCE_ADMIN_SERVER_FNS` require
//! `ValenceAdmin`.
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/valence` | [`ValenceDashboardPage`] | [`get_dashboard_my_data_stats`], [`search_schema_or_id`] |
//! | `/valence/schema` | [`ValenceSchemaIndexPage`] | [`get_schemas`], [`get_schemas_page`] |
//! | `/valence/schema/:schema_name` | [`ValenceSchemaPage`] | [`get_schema`], [`get_schema_privacy_policies`], [`get_schema_samples`], [`get_schema_iters`] |
//! | `/valence/schema/:schema_name/iter/:run_id` | [`ValenceIterRunPage`] | [`get_iter_run`], [`list_iter_run_errors`], [`list_iter_run_batches`], [`cancel_iter_run`] |
//! | `/valence/schema/:schema_name/deletion/:run_id` | [`ValenceDeletionRunPage`] | [`get_deletion_run`], [`list_deletion_run_steps`], [`cancel_deletion_run`] |
//! | `/valence/schema/:schema_name/id/:entity_id` | [`ValenceEntityPage`] | [`get_entity_view`], [`get_entity_privacy_evaluation`], [`get_entity_ownership_transfers`], [`delete_entity_queue`], [`run_iter_on_entity`] |
//! | `/valence/traits` | [`ValenceTraitIndexPage`] | [`get_traits`], [`get_traits_page`] |
//! | `/valence/traits/:trait_name` | [`ValenceTraitDetailPage`] | [`get_trait`] |
//! | `/valence/iters` | [`ValenceIterIndexPage`] | [`list_iter_runs`] |
//! | `/valence/deletions` | [`ValenceDeletionIndexPage`] | [`list_deletion_runs`] |
//! | `/valence/schemas`, `/valence/schemas/:schema_name`, `/valence/schemas/:schema_name/id/:entity_id` | — | Legacy redirects into the `/valence/schema/...` paths above |
//!
//! ## Examples
//!
//! Start with [Mount Valence routes](#mount-valence-routes). The `valence-backend` unit and integ
//! suites in `docs/VERIFICATION.md` cover server-fn contracts. Runnable host:
//! `examples/protected-valence-host` (deny/allow + schema index; inventory `valence` / `/valence`).
//!
//! ## Where to look next
//!
//! - [`mod@help_steps`] — Help spotlight tour inventory; call [`ensure_help_steps_linked`].
//! - [`ValenceLayout`] — shared app bar / nav shell wrapping every route.
//! - [`mod@server`] — server functions backing the UI, including dashboard metrics helpers.
//! - [`permissions::ValencePermission`] / [`VALENCE_ADMIN_PERMISSION`] — permission enum and admin gate name.
//! - `valence_backend` — id validation, lookup helpers, and admin server-fn name list.

#![allow(missing_docs)]
// SSR bodies / hydrate stubs leave imports and Leptos props unused under
// `-D warnings` (clippy SSR + leptos-lints hydrate).
#![cfg_attr(
    any(feature = "ssr", feature = "hydrate"),
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unknown_lints,
        // Leptos empty `view! {}` branches and page error formatting — UI clippy debt
        // tracked separately from backend `-D warnings` gates.
        clippy::unused_unit,
        clippy::unit_arg,
        clippy::to_string_in_format_args,
        clippy::bind_instead_of_map,
        clippy::let_unit_value,
        clippy::too_many_arguments,
        clippy::question_mark,
        clippy::needless_borrow
    )
)]
use leptos::prelude::*;
use leptos_router::{components::*, path, Lazy};
use uf_product_macros::uf_app;

mod components;
/// Help spotlight tour step inventory for Valence routes.
pub mod help_steps;
mod layout;
mod lazy_routes;
mod pages;
/// Permission manifest for Valence admin server functions.
pub mod permissions;
/// SSR server functions and DTOs backing the Valence UI.
pub mod server;

pub use help_steps::ensure_help_steps_linked;
pub use layout::ValenceLayout;
pub use lazy_routes::{
    prefetch_family, ValenceDashboardRoute, ValenceDeletionIndexRoute, ValenceDeletionRunRoute,
    ValenceEntityRoute, ValenceIterIndexRoute, ValenceIterRunRoute, ValenceSchemaIndexRoute,
    ValenceSchemaRoute, ValenceTraitDetailRoute, ValenceTraitIndexRoute,
};
pub use pages::{
    ValenceDashboardPage, ValenceDeletionIndexPage, ValenceDeletionRunPage, ValenceEntityPage,
    ValenceIterIndexPage, ValenceIterRunPage, ValenceSchemaIndexPage, ValenceSchemaPage,
    ValenceTraitDetailPage, ValenceTraitIndexPage,
};
pub use server::{
    cancel_deletion_run, cancel_iter_run, delete_entity_queue, get_dashboard_my_data_stats,
    get_deletion_run, get_entity_ownership_transfers, get_entity_privacy_evaluation,
    get_entity_view, get_iter_run, get_schema, get_schema_iters, get_schema_privacy_policies,
    get_schema_samples, get_schemas, get_schemas_page, get_trait, get_traits, get_traits_page,
    list_deletion_run_steps, list_deletion_runs, list_iter_run_batches, list_iter_run_errors,
    list_iter_runs, run_iter_on_entity, search_schema_or_id, DashboardMyDataStats, DeletionRunView,
    EntityView, ForeignKeyRef, IterRunSummary, Schema, SchemaEdge, SchemaField, SchemaListItem,
    SchemaMeta, SchemaPrivacy, TraitDetail, TraitFieldInfo, TraitListItem,
    VALENCE_ADMIN_PERMISSION,
};

// Define the Valence application metadata.
uf_app! {
    name: "Valence",
    id: "valence",
    description: "Valence ORM application",
    icon: "🔧",
    version: "0.1.0",
    routes: ValenceRoutes,
    route_path: "/valence",
    permission_manifest: permissions::ValencePermission,
}

/// Redirect component for /valence/schemas/:schema_name -> /valence/schema/:schema_name
#[component]
fn RedirectSchemasSchemaName() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let schema_name = params
        .get()
        .get("schema_name")
        .map(|s| s.to_string())
        .unwrap_or_default();
    view! {
        <div data-testid="valence-redirect-schema-name">
            <Redirect path=valence_backend::valence_schema_path(&schema_name) />
        </div>
    }
}

/// Redirect component for /valence/schemas/:schema_name/id/:entity_id -> /valence/schema/:schema_name/id/:entity_id
#[component]
fn RedirectSchemasEntity() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let schema_name = params
        .get()
        .get("schema_name")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let entity_id = params
        .get()
        .get("entity_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    view! {
        <div data-testid="valence-redirect-schema-entity">
            <Redirect path=valence_backend::valence_entity_path(&schema_name, &entity_id) />
        </div>
    }
}

/// Valence application routes. Auth gating lives inside [`ValenceLayout`] so the
/// app bar stays visible when access is denied.
///
/// Leaf pages are [`LazyRoute`](leptos_router::LazyRoute) views so
/// `cargo leptos --split` can emit a separate WASM chunk for this family.
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn ValenceRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    crate::help_steps::ensure_help_steps_linked();
    view! {
        <ParentRoute path=path!("valence") view=ValenceLayout>
            <Route path=path!("") view={Lazy::<ValenceDashboardRoute>::new()} />
            <Route path=path!("schemas") view=move || view! { <Redirect path=crate::paths::SCHEMA /> } />
            // Register `schema/:…` before static `schema` so matchers never prefer the index
            // for paths like `/valence/schema/counter`.
            <Route path=path!("schema/:schema_name") view={Lazy::<ValenceSchemaRoute>::new()} />
            <Route path=path!("schema/:schema_name/iter/:run_id") view={Lazy::<ValenceIterRunRoute>::new()} />
            <Route path=path!("schema/:schema_name/deletion/:run_id") view={Lazy::<ValenceDeletionRunRoute>::new()} />
            <Route path=path!("schemas/:schema_name") view=RedirectSchemasSchemaName />
            <Route path=path!("schema/:schema_name/id/:entity_id") view={Lazy::<ValenceEntityRoute>::new()} />
            <Route path=path!("schemas/:schema_name/id/:entity_id") view=RedirectSchemasEntity />
            <Route path=path!("schema") view={Lazy::<ValenceSchemaIndexRoute>::new()} />
            <Route path=path!("traits") view={Lazy::<ValenceTraitIndexRoute>::new()} />
            <Route path=path!("traits/:trait_name") view={Lazy::<ValenceTraitDetailRoute>::new()} />
            <Route path=path!("iters") view={Lazy::<ValenceIterIndexRoute>::new()} />
            <Route path=path!("deletions") view={Lazy::<ValenceDeletionIndexRoute>::new()} />
        </ParentRoute>
    }
    .into_inner()
}

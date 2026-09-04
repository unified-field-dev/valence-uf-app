//! Eager `/valence` routes for the Playwright host.
//!
//! Production [`valence_app::ValenceRoutes`] wraps leaf pages in `Lazy` for
//! wasm-split. Nested `Lazy` under `ParentRoute` still panics on
//! `hydrate_body` in this Leptos pin, so the lab host mounts the same page
//! components without `Lazy`.

use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use valence_app::{
    ValenceDashboardPage, ValenceDeletionIndexPage, ValenceDeletionRunPage, ValenceEntityPage,
    ValenceIterIndexPage, ValenceIterRunPage, ValenceLayout, ValenceSchemaIndexPage,
    ValenceSchemaPage, ValenceTraitDetailPage, ValenceTraitIndexPage,
};

/// Same paths as [`valence_app::ValenceRoutes`], without Lazy route views.
#[component(transparent)]
pub fn ValenceRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("valence") view=ValenceLayout>
            <Route path=path!("") view=ValenceDashboardPage />
            <Route path=path!("schema/:schema_name") view=ValenceSchemaPage />
            <Route path=path!("schema/:schema_name/iter/:run_id") view=ValenceIterRunPage />
            <Route path=path!("schema/:schema_name/deletion/:run_id") view=ValenceDeletionRunPage />
            <Route path=path!("schema/:schema_name/id/:entity_id") view=ValenceEntityPage />
            <Route path=path!("schema") view=ValenceSchemaIndexPage />
            <Route path=path!("traits") view=ValenceTraitIndexPage />
            <Route path=path!("traits/:trait_name") view=ValenceTraitDetailPage />
            <Route path=path!("iters") view=ValenceIterIndexPage />
            <Route path=path!("deletions") view=ValenceDeletionIndexPage />
        </ParentRoute>
    }
    .into_inner()
}

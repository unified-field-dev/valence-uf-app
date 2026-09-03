//! Lazy-loaded route views for WASM code-splitting (`cargo leptos --split`).

use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::{
    ValenceDashboardPage, ValenceDeletionIndexPage, ValenceDeletionRunPage, ValenceEntityPage,
    ValenceIterIndexPage, ValenceIterRunPage, ValenceSchemaIndexPage, ValenceSchemaPage,
    ValenceTraitDetailPage, ValenceTraitIndexPage,
};

/// Prefetch the valence family WASM chunk (leaf pages share split modules).
pub async fn prefetch_family() {
    ValenceDashboardRoute::preload().await;
}

/// Lazy `/valence` dashboard.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceDashboardRoute;

#[lazy_route]
impl LazyRoute for ValenceDashboardRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceDashboardPage /> }.into_any()
    }
}

/// Lazy `/valence/schema/:schema_name`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceSchemaRoute;

#[lazy_route]
impl LazyRoute for ValenceSchemaRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceSchemaPage /> }.into_any()
    }
}

/// Lazy `/valence/schema/:schema_name/iter/:run_id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceIterRunRoute;

#[lazy_route]
impl LazyRoute for ValenceIterRunRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceIterRunPage /> }.into_any()
    }
}

/// Lazy `/valence/schema/:schema_name/deletion/:run_id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceDeletionRunRoute;

#[lazy_route]
impl LazyRoute for ValenceDeletionRunRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceDeletionRunPage /> }.into_any()
    }
}

/// Lazy `/valence/schema/:schema_name/id/:entity_id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceEntityRoute;

#[lazy_route]
impl LazyRoute for ValenceEntityRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceEntityPage /> }.into_any()
    }
}

/// Lazy `/valence/schema` index.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceSchemaIndexRoute;

#[lazy_route]
impl LazyRoute for ValenceSchemaIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceSchemaIndexPage /> }.into_any()
    }
}

/// Lazy `/valence/traits`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceTraitIndexRoute;

#[lazy_route]
impl LazyRoute for ValenceTraitIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceTraitIndexPage /> }.into_any()
    }
}

/// Lazy `/valence/traits/:trait_name`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceTraitDetailRoute;

#[lazy_route]
impl LazyRoute for ValenceTraitDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceTraitDetailPage /> }.into_any()
    }
}

/// Lazy `/valence/iters`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceIterIndexRoute;

#[lazy_route]
impl LazyRoute for ValenceIterIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceIterIndexPage /> }.into_any()
    }
}

/// Lazy `/valence/deletions`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValenceDeletionIndexRoute;

#[lazy_route]
impl LazyRoute for ValenceDeletionIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ValenceDeletionIndexPage /> }.into_any()
    }
}

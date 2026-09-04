//! ValenceRoutes Playwright lab host.
#![allow(missing_docs)]

mod app;
#[cfg(feature = "ssr")]
mod e2e_valence;
mod gate_demos;
mod harness_auth_menu;
#[cfg(feature = "ssr")]
pub mod seed;
mod valence_routes_eager;

pub use app::{shell, wire_gauge_permissions_bridge, App};
#[cfg(feature = "ssr")]
pub use e2e_valence::{
    e2e_admin_valence, e2e_fixtures, e2e_higgs_config, e2e_outsider_valence, e2e_router,
    e2e_system_valence, init_e2e_valence, store_fixtures, FixtureIds, E2E_ENTITY_ID, E2E_ITER_NAME,
    E2E_SCHEMA_NAME, E2E_TRAIT_NAME,
};
#[cfg(feature = "ssr")]
pub use gate_demos::inject_e2e_session_snapshot;

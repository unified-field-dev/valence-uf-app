//! `ValenceAdmin` permission contract backing `valence-app` server macros.
//!
//! Browse/read endpoints (schema/trait registry, entity view, privacy eval,
//! iter/deletion run detail) stay open under viewer Valence (VU-01). Only
//! mutating admin ops listed here require `ValenceAdmin` + session.

/// Gauge permission name synced with `valence-app`'s `ValencePermission` manifest.
pub const VALENCE_ADMIN_PERMISSION: &str = "ValenceAdmin";

/// Server function names that must carry
/// `#[uf_product_macros::server(permission = "ValenceAdmin")]`.
pub const VALENCE_ADMIN_SERVER_FNS: &[&str] = &[
    "start_iter_run",
    "run_iter_on_entity",
    "cancel_iter_run",
    "delete_entity_queue",
    "cancel_deletion_run",
];

/// Returns whether `name` is a Valence mutating admin server function.
pub fn is_valence_admin_server_fn(name: &str) -> bool {
    VALENCE_ADMIN_SERVER_FNS.iter().any(|n| *n == name.trim())
}

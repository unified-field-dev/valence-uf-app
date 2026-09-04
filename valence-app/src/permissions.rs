//! Permission manifest for the Valence operations app.

use uf_product_macros::UfPermissionManifest;

/// Admin permission for Valence mutating server functions.
///
/// Synced into the `valence` domain; gated with
/// `#[uf_product_macros::server(permission = "ValenceAdmin")]`.
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "valence",
    domain_name = "Valence",
    domain_description = "Valence schema and operations administration"
)]
pub enum ValencePermission {
    /// Cancel iter/deletion runs, queue deletes, and dispatch iters
    /// (`start_iter_run`, `run_iter_on_entity`, `cancel_iter_run`,
    /// `delete_entity_queue`, `cancel_deletion_run`).
    #[permission(description = "Administer Valence iter, deletion, and entity mutations")]
    ValenceAdmin,
}

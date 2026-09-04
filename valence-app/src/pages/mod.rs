//! Route-level page components for the Valence ops UI.
//!
//! Each submodule maps to a `/valence/...` route (dashboard, schema catalog, entity
//! detail, traits, iters, deletions). Prefer importing pages from the crate root;
//! use [`crate::server`] for `#[server]` functions backing each view.

pub mod dashboard;
pub mod deletion_index;
pub mod deletion_run;
pub mod entity;
pub mod iter_index;
pub mod iter_run;
pub mod schema;
pub mod schema_index;
pub mod trait_detail;
pub mod trait_index;

pub use dashboard::ValenceDashboardPage;
pub use deletion_index::ValenceDeletionIndexPage;
pub use deletion_run::ValenceDeletionRunPage;
pub use entity::ValenceEntityPage;
pub use iter_index::ValenceIterIndexPage;
pub use iter_run::ValenceIterRunPage;
pub use schema::ValenceSchemaPage;
pub use schema_index::ValenceSchemaIndexPage;
pub use trait_detail::ValenceTraitDetailPage;
pub use trait_index::ValenceTraitIndexPage;

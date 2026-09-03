//! Valence app server functions and DTOs.
//!
//! Host integrators should import server functions and DTOs from the [`valence_app`]
//! crate root. This module exposes the same **curated Ship surface** plus
//! `pub(crate)` helpers used by in-crate pages (dashboard platform cards, iter
//! dispatch, schema cards).
//!
//! This module backs the `/valence` UI with APIs for:
//! - schema and trait discovery from runtime registries,
//! - schema/entity detail payload assembly for pages,
//! - privacy card/evaluation payload generation,
//! - schema-or-id search and route redirection helpers,
//! - dashboard My Data and platform metric cards.
//!
//! ## Security map
//!
//! - Every `#[server]` path requires an authenticated session before registry or
//!   Valence IO (viewer Valence for entity/field paths).
//! - Entity get/search/samples and owner contact use **viewer Valence** so schema
//!   field privacy applies (pages stay loadable; fields redact).
//! - Privacy evaluation binds to the **request actor only** (no client-supplied
//!   `viewer_id`).
//! - Mutating admin ops ([`valence_backend::VALENCE_ADMIN_SERVER_FNS`]) require
//!   session + `ValenceAdmin` via `#[uf_product_macros::server(permission = "...")]`.
//! - Detail hrefs use `valence_backend::valence_*_path` (percent-encoded segments);
//!   `validate_*` rejects blank, oversized, and path-unsafe ids.
//!
//! Schema list / iter / deletion pure contracts live in [`valence_backend`];
//! this module keeps Leptos `#[server]` wrappers and SSR-only registry IO.
//!
//! ## Errors
//!
//! Fallible ops return [`ServerFnError`](leptos::prelude::ServerFnError) (Leptos
//! boundary). Messages use stable class prefixes so operators can triage without
//! typed errors crossing the server-fn wire:
//!
//! - `auth:` — missing session or viewer Valence build failure
//! - `validation:` — blank, oversized, or path-unsafe ids ([`valence_backend::ValenceIdError`])
//! - `not_found:` — unknown schema, run, or entity after validation
//! - `permission:` — missing `ValenceAdmin` or Spectra query permission
//! - `io:` — Valence or registry IO failures

mod dashboard;
mod deletions;
mod entities;
mod iters;
mod schemas;
mod traits;
mod types;

#[cfg(feature = "ssr")]
mod conversions;
#[cfg(feature = "ssr")]
mod helpers;
#[cfg(feature = "ssr")]
mod privacy;
#[cfg(feature = "ssr")]
mod registry;

// --- Ship surface (matches crate-root re-exports in `lib.rs`) -----------------

pub use dashboard::{get_dashboard_my_data_stats, DashboardMyDataStats};
pub use deletions::{
    cancel_deletion_run, get_deletion_run, list_deletion_run_steps, list_deletion_runs,
};
pub use entities::{
    delete_entity_queue, get_entity_ownership_transfers, get_entity_privacy_evaluation,
    get_entity_view,
};
pub use iters::{
    cancel_iter_run, get_iter_run, get_schema_iters, list_iter_run_batches, list_iter_run_errors,
    list_iter_runs, run_iter_on_entity,
};
pub use schemas::{
    get_schema, get_schema_privacy_policies, get_schema_samples, get_schemas, get_schemas_page,
    search_schema_or_id,
};
pub use traits::{get_trait, get_traits, get_traits_page};
pub use types::{
    EntityView, ForeignKeyRef, Schema, SchemaEdge, SchemaField, SchemaMeta, SchemaPrivacy,
    TraitDetail, TraitFieldInfo,
};
pub use valence_backend::{
    DeletionRunView, IterRunSummary, SchemaListItem, TraitListItem, VALENCE_ADMIN_PERMISSION,
};

// --- In-crate UI wiring (not part of the host integrator contract) ------------

pub(crate) use dashboard::{
    get_dashboard_active_deletions, get_dashboard_my_data_top_schemas,
    get_dashboard_platform_error_offenders, get_dashboard_platform_headline,
    get_dashboard_platform_reads_breakdown, get_dashboard_platform_throughput,
    get_dashboard_platform_writes_breakdown, DashboardChartPoint, DashboardChartSeries,
    DashboardErrorSlice, DashboardPlatformHeadline, DashboardSchemaRowCount, DashboardStatCard,
};
pub(crate) use deletions::list_deletion_runs_for_schema;
pub(crate) use iters::{
    evaluate_iter_for_entity, list_recent_iter_runs_for_schema, start_iter_run,
};
pub(crate) use schemas::SearchSchemaOrId;
pub(crate) use types::{
    DeletionRequest, EntityPrivacyEvalCardData, EntityRecord, InverseConnectionData,
    InverseSchemaConnection, Owner, OwnershipTransferRow, SampleRecord, SchemaConnection,
    SchemaPrivacyCardData,
};
pub(crate) use valence_backend::{
    DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo, IterRowErrorView, IterRunView,
    ITERS_PAGE_SIZE, SCHEMAS_PAGE_SIZE, TRAITS_PAGE_SIZE,
};

#[cfg(all(test, feature = "ssr"))]
mod ownership_row_parse_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::registry::transfer_row_from_value;
    use super::types::OwnershipTransferRow;

    #[test]
    fn transfer_row_parses_string_fields() {
        let t = transfer_row_from_value(serde_json::json!({
            "id": "t1",
            "from_owner_id": "a",
            "from_owner_type": "user",
            "to_owner_id": "b",
            "to_owner_type": "user",
            "transferred_at": "2024-01-02T00:00:00Z",
            "transferred_by": "alice",
            "reason": "handoff",
        }))
        .expect("parsed");
        assert_eq!(t.id, "t1");
        assert_eq!(t.reason.as_deref(), Some("handoff"));
    }

    #[test]
    fn transfer_row_non_string_timestamp_stringifies() {
        let t = transfer_row_from_value(serde_json::json!({
            "id": "t2",
            "from_owner_id": "a",
            "from_owner_type": "user",
            "to_owner_id": "b",
            "to_owner_type": "account",
            "transferred_at": 42,
            "transferred_by": "sys",
        }))
        .unwrap();
        assert_eq!(t.transferred_at, "42");
    }

    #[test]
    fn transfer_row_missing_id_returns_none() {
        assert!(transfer_row_from_value(serde_json::json!({ "from_owner_id": "x" })).is_none());
    }

    #[test]
    fn ownership_transfer_row_serde_defaults() {
        let json = r#"{"id":"x","from_owner_id":"a","from_owner_type":"user","to_owner_id":"b","to_owner_type":"user","transferred_at":"t","transferred_by":"u"}"#;
        let row: OwnershipTransferRow = serde_json::from_str(json).unwrap();
        assert!(row.reason.is_none());
    }
}

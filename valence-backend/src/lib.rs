//! Pure backend contracts for the Valence ops UI server surface.
//!
//! Validation, registry lookup, paging adapters, and admin permission name lists that
//! `valence-app` `#[server]` functions call after resolving Higgs request context. Keeps
//! schema/trait/iter/deletion shapes unit-testable without a Leptos host or UI graph.
//!
//! ## Features
//!
//! - **Path and id validation** — Validates schema, entity, run, iter, and trait names so
//!   blank, oversized, or path-unsafe values fail closed before registry IO or ops path
//!   encoding. [Get started](#validate-path-and-ids)
//! - **Registry lookup and paging** — Provides case-insensitive schema/trait lookup,
//!   iter/deletion run resolution, and `DataTable` page adapters for schema and trait
//!   indexes. [Get started](#lookup-and-page-query)
//! - **Admin permission gate list** — Exposes the canonical `ValenceAdmin` permission name
//!   and the mutating server-fn name list checked by host manifests and macros.
//!   [Get started](#admin-permission-names)
//! - **Ops path encoding** — Builds percent-encoded path segments for `/valence` hrefs via
//!   [`encode_ops_path_segment`], [`valence_schema_path`], [`valence_entity_path`],
//!   [`valence_iter_run_path`], [`valence_deletion_run_path`], and [`valence_trait_path`].
//! - **Owner contact redaction** — Builds operator-safe contact display from user rows via
//!   [`owner_contact_from_user_record`].
//!
//! ## Validate path and ids
//!
//! Schema, entity, run, iter, and trait parameters are validated before registry lookups
//! or href encoding so path segments cannot smuggle slashes or control characters into
//! routing. [`validate_schema_name`], [`validate_entity_id`], and related validators run
//! synchronously in `valence-app` server functions — call them in custom wrappers when you
//! add new read paths that accept Valence id parameters.
//!
//! **Prerequisites:** None beyond importing this crate; validators return
//! [`ValenceIdError`] on failure.
//!
//! ```rust,ignore
//! use valence_backend::{
//!     validate_schema_name, validate_entity_id, encode_ops_path_segment, valence_schema_path,
//!     ValenceIdError, MAX_VALENCE_ID_CHARS,
//! };
//!
//! validate_schema_name("counter").expect("valid schema");
//! assert_eq!(
//!     validate_schema_name("").unwrap_err(),
//!     ValenceIdError::EmptySchemaName
//! );
//! assert_eq!(valence_schema_path("a/b"), "/valence/schema/a%2Fb");
//! assert_eq!(MAX_VALENCE_ID_CHARS, 256);
//! ```
//!
//! On success validators return `Ok(())` and encoded paths are safe for `/valence` hrefs.
//! Blank, oversized, control-character, slash, backslash, or `.` / `..` names map to typed
//! [`ValenceIdError`] variants with operator-facing messages.
//!
//! ## Lookup and page query
//!
//! Registry lookup and paging provides in-memory schema/trait resolution and `DataTable`
//! page adapters so server functions can answer detail and index queries without another
//! network hop. [`find_schema_by_name`] and [`find_trait_by_name`] back detail pages;
//! [`apply_schema_page_query`] and [`apply_trait_page_query`] back paginated index server
//! functions after the server layer has already loaded the rows.
//!
//! **Prerequisites:** Callers supply owned `Vec` rows — helpers mutate or filter in place
//! and do not perform network IO.
//!
//! ```rust,ignore
//! use valence_backend::{
//!     find_schema_by_name, apply_schema_page_query, find_iter_run_by_id, SchemaListItem,
//!     IterRunView,
//! };
//! use orbital_paging::PageRequest;
//!
//! let schemas = vec![SchemaListItem {
//!     name: "counter".into(),
//!     databases: vec!["mem".into()],
//!     version: "1".into(),
//!     description: None,
//! }];
//! let found = find_schema_by_name(&schemas, "counter").expect("listed");
//! assert_eq!(found.name, "counter");
//!
//! let mut items = schemas.clone();
//! apply_schema_page_query(&mut items, &PageRequest::default());
//! assert_eq!(items.len(), 1);
//!
//! let runs = vec![IterRunView {
//!     run_id: "r1".into(),
//!     iter_name: "CleanupIter".into(),
//!     target_table: "counter".into(),
//!     status: "pending".into(),
//!     total_rows: 10,
//!     scanned_rows: 0,
//!     processed_rows: 0,
//!     skipped_rows: 0,
//!     failed_rows: 0,
//!     created_at: "2026-01-01T00:00:00Z".into(),
//!     error_message: None,
//!     target_row_id: None,
//! }];
//! let run = find_iter_run_by_id(&runs, "r1").expect("listed");
//! assert_eq!(run.iter_name, "CleanupIter");
//! ```
//!
//! On success lookup helpers return a reference into the supplied slice or `None` when the
//! name or id is unknown; page adapters retain rows matching quick search and filter rules.
//!
//! ## Admin permission names
//!
//! Mutating Valence ops (start/cancel iter runs, queue entity deletion, cancel deletion
//! runs) require the `ValenceAdmin` Gauge permission. [`VALENCE_ADMIN_PERMISSION`] is the
//! canonical permission string; [`VALENCE_ADMIN_SERVER_FNS`] lists server function names that
//! must carry `#[uf_product_macros::server(permission = "ValenceAdmin")]`.
//! [`is_valence_admin_server_fn`] answers manifest and codegen checks for a given name.
//!
//! **Prerequisites:** Host manifests must declare [`VALENCE_ADMIN_PERMISSION`] alongside
//! viewer Valence permissions; browse/read server functions stay outside this list.
//!
//! ```rust,ignore
//! use valence_backend::{
//!     is_valence_admin_server_fn, VALENCE_ADMIN_PERMISSION, VALENCE_ADMIN_SERVER_FNS,
//! };
//!
//! assert_eq!(VALENCE_ADMIN_PERMISSION, "ValenceAdmin");
//! assert!(is_valence_admin_server_fn("delete_entity_queue"));
//! assert!(!is_valence_admin_server_fn("get_schemas"));
//! assert_eq!(VALENCE_ADMIN_SERVER_FNS.len(), 5);
//! ```
//!
//! On success `is_valence_admin_server_fn` returns `true` only for the five mutating admin
//! server functions; all other names are treated as viewer-accessible reads.
//!
//! ## Examples
//!
//! Start with [Validate path and ids](#validate-path-and-ids). This crate's unit and integ suites
//! are listed in `docs/VERIFICATION.md`. Runnable host: `examples/protected-valence-host`
//! (inventory `valence` / `/valence`).

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod admin_permission;
mod entity_id;
mod lookup;
mod map;
mod owner_contact;
mod page_query;
mod types;
mod validate;

pub use admin_permission::{
    is_valence_admin_server_fn, VALENCE_ADMIN_PERMISSION, VALENCE_ADMIN_SERVER_FNS,
};
pub use entity_id::{extract_id_from_record_display, normalize_entity_id_for_lookup};
pub use lookup::{
    find_deletion_run_by_id, find_iter_by_name, find_iter_run_by_id, find_iter_run_summary_by_id,
    find_schema_by_name, find_trait_by_name, resolve_schema_name_from_list, schema_has_iter,
    sort_schemas_by_name, sort_traits_by_name,
};
pub use map::{clamp_deletion_list_limit, deletion_run_view_from_value};
pub use owner_contact::owner_contact_from_user_record;
pub use page_query::{apply_schema_page_query, apply_trait_page_query};
pub use types::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem, ITERS_PAGE_SIZE,
    SCHEMAS_PAGE_SIZE, TRAITS_PAGE_SIZE,
};
pub use validate::{
    encode_ops_path_segment, valence_deletion_run_path, valence_entity_path, valence_iter_run_path,
    valence_schema_path, valence_trait_path, validate_entity_id, validate_iter_name,
    validate_run_id, validate_schema_name, validate_trait_name, ValenceIdError,
    MAX_VALENCE_ID_CHARS,
};

#[cfg(test)]
mod tests {
    use orbital_data::DataValue;
    use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

    use super::*;

    fn sample_schema(name: &str, description: Option<&str>) -> SchemaListItem {
        SchemaListItem {
            name: name.into(),
            databases: vec!["mem".into()],
            version: "1".into(),
            description: description.map(str::to_string),
        }
    }

    fn sample_trait(name: &str) -> TraitListItem {
        TraitListItem {
            name: name.into(),
            version: String::new(),
            description: None,
        }
    }

    fn sample_iter_run(id: &str, iter_name: &str) -> IterRunSummary {
        IterRunSummary {
            run_id: id.into(),
            iter_name: iter_name.into(),
            target_table: "counter".into(),
            status: "pending".into(),
            created_at: "2026-01-01T00:00:00Z".into(),
            total_rows: 10,
            processed_rows: 0,
        }
    }

    fn sample_deletion(id: &str, table: &str) -> DeletionRunView {
        DeletionRunView {
            run_id: id.into(),
            root_table: table.into(),
            root_record_id: "r1".into(),
            status: "running".into(),
            total_steps: 3,
            completed_steps: 1,
            failed_steps: 0,
            requested_at: "2026-01-01T00:00:00Z".into(),
        }
    }

    #[test]
    fn validate_schema_name_accepts_name_happy_path() {
        validate_schema_name("counter").expect("name");
    }

    #[test]
    fn validate_schema_name_rejects_blank_sad() {
        assert_eq!(
            validate_schema_name("  ").expect_err("blank"),
            ValenceIdError::EmptySchemaName
        );
        assert!(ValenceIdError::EmptySchemaName
            .to_string()
            .contains("required"));
    }

    #[test]
    fn validate_run_id_accepts_id_happy_path() {
        validate_run_id("run-1").expect("id");
    }

    #[test]
    fn validate_run_id_rejects_blank_sad() {
        assert_eq!(
            validate_run_id("").expect_err("blank"),
            ValenceIdError::EmptyRunId
        );
    }

    #[test]
    fn validate_entity_id_accepts_id_happy_path() {
        validate_entity_id("singleton").expect("id");
    }

    #[test]
    fn validate_entity_id_rejects_blank_sad() {
        assert_eq!(
            validate_entity_id(" ").expect_err("whitespace"),
            ValenceIdError::EmptyEntityId
        );
    }

    #[test]
    fn validate_iter_name_accepts_name_happy_path() {
        validate_iter_name("MyIter").expect("name");
    }

    #[test]
    fn validate_iter_name_rejects_blank_sad() {
        assert_eq!(
            validate_iter_name("").expect_err("blank"),
            ValenceIdError::EmptyIterName
        );
    }

    #[test]
    fn validate_trait_name_rejects_blank_sad() {
        assert_eq!(
            validate_trait_name("  ").expect_err("blank"),
            ValenceIdError::EmptyTraitName
        );
    }

    #[test]
    fn validate_schema_name_rejects_slash_control_dotdot_sad() {
        assert_eq!(
            validate_schema_name("a/b").expect_err("slash"),
            ValenceIdError::UnsafeSchemaName
        );
        assert_eq!(
            validate_schema_name("a\\b").expect_err("backslash"),
            ValenceIdError::UnsafeSchemaName
        );
        assert_eq!(
            validate_schema_name("..\x00").expect_err("control"),
            ValenceIdError::UnsafeSchemaName
        );
        assert_eq!(
            validate_schema_name("..").expect_err("dotdot"),
            ValenceIdError::UnsafeSchemaName
        );
        assert_eq!(
            validate_schema_name(".").expect_err("dot"),
            ValenceIdError::UnsafeSchemaName
        );
    }

    #[test]
    fn validate_entity_id_rejects_slash_and_oversized_sad() {
        assert_eq!(
            validate_entity_id("a/b").expect_err("slash"),
            ValenceIdError::UnsafeEntityId
        );
        let oversized: String = "e".repeat(MAX_VALENCE_ID_CHARS + 1);
        assert_eq!(
            validate_entity_id(&oversized).expect_err("too long"),
            ValenceIdError::EntityIdTooLong
        );
    }

    #[test]
    fn validate_run_id_rejects_oversized_sad() {
        let oversized: String = "r".repeat(MAX_VALENCE_ID_CHARS + 1);
        assert_eq!(
            validate_run_id(&oversized).expect_err("too long"),
            ValenceIdError::RunIdTooLong
        );
    }

    #[test]
    fn encode_ops_path_segment_encodes_slash_and_space_happy_path() {
        assert_eq!(encode_ops_path_segment("orders"), "orders");
        assert_eq!(encode_ops_path_segment("a/b"), "a%2Fb");
        assert_eq!(encode_ops_path_segment("a b"), "a%20b");
        assert_eq!(encode_ops_path_segment("a\\b"), "a%5Cb");
    }

    #[test]
    fn valence_ops_paths_encode_segments_happy_path() {
        assert_eq!(valence_schema_path("a/b"), "/valence/schema/a%2Fb");
        assert_eq!(
            valence_entity_path("a/b", "c d"),
            "/valence/schema/a%2Fb/id/c%20d"
        );
        assert_eq!(
            valence_iter_run_path("counter", "r/1"),
            "/valence/schema/counter/iter/r%2F1"
        );
        assert_eq!(
            valence_deletion_run_path("user", "d\\1"),
            "/valence/schema/user/deletion/d%5C1"
        );
        assert_eq!(
            valence_trait_path("Has/Owner"),
            "/valence/traits/Has%2FOwner"
        );
    }

    #[test]
    fn find_schema_by_name_resolves_case_insensitive_happy_path() {
        let schemas = vec![sample_schema("Counter", None), sample_schema("user", None)];
        let found = find_schema_by_name(&schemas, "counter").expect("listed");
        assert_eq!(found.name, "Counter");
    }

    #[test]
    fn find_schema_by_name_unknown_is_none_sad() {
        let schemas = vec![sample_schema("counter", None)];
        assert!(find_schema_by_name(&schemas, "__no_such_schema__").is_none());
    }

    #[test]
    fn resolve_schema_name_from_list_prefers_registry_casing_happy_path() {
        let names = vec!["counter".into(), "user".into()];
        assert_eq!(
            resolve_schema_name_from_list(&names, "Counter").as_deref(),
            Some("counter")
        );
    }

    #[test]
    fn resolve_schema_name_from_list_unknown_is_none_sad() {
        let names = vec!["counter".into()];
        assert!(resolve_schema_name_from_list(&names, "__missing__").is_none());
    }

    #[test]
    fn schema_has_iter_registered_happy_path() {
        let iters = vec!["CleanupIter".into(), "ArchiveIter".into()];
        assert!(schema_has_iter(&iters, "CleanupIter"));
    }

    #[test]
    fn schema_has_iter_unknown_sad() {
        let iters = vec!["CleanupIter".into()];
        assert!(!schema_has_iter(&iters, "MissingIter"));
    }

    #[test]
    fn find_iter_run_summary_by_id_resolves_exact_happy_path() {
        let runs = vec![
            sample_iter_run("r1", "CleanupIter"),
            sample_iter_run("r2", "ArchiveIter"),
        ];
        let found = find_iter_run_summary_by_id(&runs, "r2").expect("listed");
        assert_eq!(found.iter_name, "ArchiveIter");
    }

    #[test]
    fn find_iter_run_summary_by_id_unknown_is_none_sad() {
        let runs = vec![sample_iter_run("r1", "CleanupIter")];
        assert!(find_iter_run_summary_by_id(&runs, "__missing__").is_none());
    }

    #[test]
    fn find_deletion_run_by_id_resolves_exact_happy_path() {
        let runs = vec![
            sample_deletion("d1", "counter"),
            sample_deletion("d2", "user"),
        ];
        let found = find_deletion_run_by_id(&runs, "d2").expect("listed");
        assert_eq!(found.root_table, "user");
    }

    #[test]
    fn find_deletion_run_by_id_unknown_is_none_sad() {
        let runs = vec![sample_deletion("d1", "counter")];
        assert!(find_deletion_run_by_id(&runs, "__missing__").is_none());
    }

    #[test]
    fn sort_schemas_by_name_orders_lexicographically_happy_path() {
        let mut schemas = vec![sample_schema("zeta", None), sample_schema("alpha", None)];
        sort_schemas_by_name(&mut schemas);
        assert_eq!(schemas[0].name, "alpha");
        assert_eq!(schemas[1].name, "zeta");
    }

    #[test]
    fn sort_traits_by_name_orders_lexicographically_happy_path() {
        let mut traits = vec![sample_trait("Named"), sample_trait("HasOwner")];
        sort_traits_by_name(&mut traits);
        assert_eq!(traits[0].name, "HasOwner");
        assert_eq!(traits[1].name, "Named");
    }

    #[test]
    fn normalize_entity_id_strips_table_prefix_happy_path() {
        assert_eq!(normalize_entity_id_for_lookup("user:⟨abc⟩".into()), "abc");
    }

    #[test]
    fn normalize_entity_id_bare_key_unchanged_happy_path() {
        assert_eq!(
            normalize_entity_id_for_lookup("singleton".into()),
            "singleton"
        );
    }

    #[test]
    fn extract_id_from_record_display_empty_after_colon_sad() {
        let err = extract_id_from_record_display("user:").expect_err("empty id");
        assert_eq!(err, ValenceIdError::InvalidRecordIdDisplay);
    }

    #[test]
    fn deletion_run_view_from_value_maps_fields_happy_path() {
        let row = serde_json::json!({
            "id": "del-1",
            "root_table": "counter",
            "root_record_id": "c1",
            "status": "completed",
            "total_steps": 2,
            "completed_steps": 2,
            "failed_steps": 0,
            "requested_at": "2026-01-01T00:00:00Z",
        });
        let view = deletion_run_view_from_value(&row).expect("mapped");
        assert_eq!(view.run_id, "del-1");
        assert_eq!(view.root_table, "counter");
        assert_eq!(view.status, "completed");
        assert_eq!(view.total_steps, 2);
    }

    #[test]
    fn deletion_run_view_from_value_missing_id_none_sad() {
        let row = serde_json::json!({
            "root_table": "counter",
        });
        assert!(deletion_run_view_from_value(&row).is_none());
    }

    #[test]
    fn clamp_deletion_list_limit_bounds_happy_path() {
        assert_eq!(clamp_deletion_list_limit(0), 1);
        assert_eq!(clamp_deletion_list_limit(50), 50);
        assert_eq!(clamp_deletion_list_limit(999), 200);
    }

    #[test]
    fn apply_schema_page_query_filters_quick_search_happy_path() {
        let mut items = vec![
            sample_schema("counter", Some("counts things")),
            sample_schema("user", Some("people")),
        ];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("count".into()),
            filter: None,
            sort: None,
        };
        apply_schema_page_query(&mut items, &request);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "counter");
    }

    #[test]
    fn apply_schema_page_query_unknown_name_filter_empty_sad() {
        let mut items = vec![sample_schema("counter", None)];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: None,
            filter: Some(FilterQuery {
                logic: FilterLogicWire::And,
                items: vec![FilterRuleParam {
                    field: "name".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("__no_such__".into()),
                }],
            }),
            sort: None,
        };
        apply_schema_page_query(&mut items, &request);
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn apply_trait_page_query_filters_by_name_happy_path() {
        let mut items = vec![sample_trait("Named"), sample_trait("HasOwner")];
        let request = PageRequest {
            offset: 0,
            limit: 20,
            quick_search: Some("owner".into()),
            filter: None,
            sort: None,
        };
        apply_trait_page_query(&mut items, &request);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "HasOwner");
    }

    #[test]
    fn find_iter_by_name_resolves_exact_happy_path() {
        let iters = vec![
            IterInfo {
                name: "CleanupIter".into(),
                description: String::new(),
                table_name: "counter".into(),
            },
            IterInfo {
                name: "ArchiveIter".into(),
                description: String::new(),
                table_name: "counter".into(),
            },
        ];
        let found = find_iter_by_name(&iters, "ArchiveIter").expect("listed");
        assert_eq!(found.table_name, "counter");
    }

    #[test]
    fn find_iter_by_name_unknown_is_none_sad() {
        let iters = vec![IterInfo {
            name: "CleanupIter".into(),
            description: String::new(),
            table_name: "counter".into(),
        }];
        assert!(find_iter_by_name(&iters, "Missing").is_none());
    }
}

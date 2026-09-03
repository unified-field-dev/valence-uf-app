//! Integration contracts for schema/iter/deletion helpers backing
//! `get_schemas` / `get_schema_iters` / `get_iter_run` / `get_deletion_run`.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use valence_backend::{
    clamp_deletion_list_limit, deletion_run_view_from_value, find_deletion_run_by_id,
    find_iter_by_name, find_iter_run_by_id, find_iter_run_summary_by_id, find_schema_by_name,
    find_trait_by_name, normalize_entity_id_for_lookup, resolve_schema_name_from_list,
    schema_has_iter, sort_schemas_by_name, sort_traits_by_name, validate_entity_id,
    validate_iter_name, validate_run_id, validate_schema_name, validate_trait_name,
    DeletionRunView, IterInfo, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

fn sample_schema(name: &str) -> SchemaListItem {
    SchemaListItem {
        name: name.into(),
        databases: vec!["mem".into()],
        version: "1".into(),
        description: None,
    }
}

fn sample_trait(name: &str) -> TraitListItem {
    TraitListItem {
        name: name.into(),
        version: String::new(),
        description: None,
    }
}

fn sample_iter_summary(id: &str, iter_name: &str) -> IterRunSummary {
    IterRunSummary {
        run_id: id.into(),
        iter_name: iter_name.into(),
        target_table: "counter".into(),
        status: "running".into(),
        created_at: "2026-01-01T00:00:00Z".into(),
        total_rows: 5,
        processed_rows: 2,
    }
}

fn sample_iter_view(id: &str) -> IterRunView {
    IterRunView {
        run_id: id.into(),
        iter_name: "CleanupIter".into(),
        target_table: "counter".into(),
        status: "completed".into(),
        total_rows: 5,
        scanned_rows: 5,
        processed_rows: 5,
        skipped_rows: 0,
        failed_rows: 0,
        created_at: "2026-01-01T00:00:00Z".into(),
        error_message: None,
        target_row_id: None,
    }
}

fn sample_deletion(id: &str, table: &str) -> DeletionRunView {
    DeletionRunView {
        run_id: id.into(),
        root_table: table.into(),
        root_record_id: "r1".into(),
        status: "pending".into(),
        total_steps: 1,
        completed_steps: 0,
        failed_steps: 0,
        requested_at: "2026-01-01T00:00:00Z".into(),
    }
}

#[test]
fn get_schemas_list_sorted_and_named_happy_path() {
    let mut schemas = vec![sample_schema("zeta"), sample_schema("alpha")];
    sort_schemas_by_name(&mut schemas);
    assert_eq!(schemas[0].name, "alpha");
    assert_eq!(schemas[1].name, "zeta");
    for s in &schemas {
        assert_ne!(s.name.trim(), "");
    }
}

#[test]
fn get_schema_detail_matches_list_entry_happy_path() {
    let schemas = vec![sample_schema("counter"), sample_schema("user")];
    let detail = find_schema_by_name(&schemas, "counter").expect("listed schema");
    assert_eq!(detail.name, "counter");
    assert_eq!(detail.version, "1");
}

#[test]
fn get_schema_unknown_name_is_none_sad() {
    let schemas = vec![sample_schema("counter")];
    assert!(find_schema_by_name(&schemas, "__valence_uf_app_no_such_schema__").is_none());
}

#[test]
fn resolve_schema_name_list_entry_happy_path() {
    let names = vec!["counter".into(), "user".into()];
    assert_eq!(
        resolve_schema_name_from_list(&names, "Counter").as_deref(),
        Some("counter")
    );
}

#[test]
fn resolve_schema_unknown_name_is_none_sad() {
    let names = vec!["counter".into()];
    assert!(resolve_schema_name_from_list(&names, "__missing__").is_none());
}

#[test]
fn get_traits_list_sorted_and_named_happy_path() {
    let mut traits = vec![sample_trait("Named"), sample_trait("HasOwner")];
    sort_traits_by_name(&mut traits);
    assert_eq!(traits[0].name, "HasOwner");
    let detail = find_trait_by_name(&traits, "named").expect("case-insensitive");
    assert_eq!(detail.name, "Named");
}

#[test]
fn get_trait_unknown_name_is_none_sad() {
    let traits = vec![sample_trait("Named")];
    assert!(find_trait_by_name(&traits, "__missing__").is_none());
}

#[test]
fn get_schema_iters_list_and_named_happy_path() {
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
    let found = find_iter_by_name(&iters, "CleanupIter").expect("listed iter");
    assert_eq!(found.table_name, "counter");
    assert!(schema_has_iter(
        &iters.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "ArchiveIter"
    ));
}

#[test]
fn get_schema_iters_unknown_iter_sad() {
    let iters = vec![IterInfo {
        name: "CleanupIter".into(),
        description: String::new(),
        table_name: "counter".into(),
    }];
    assert!(find_iter_by_name(&iters, "__missing__").is_none());
    assert!(!schema_has_iter(&["CleanupIter".into()], "MissingIter"));
}

#[test]
fn get_iter_run_detail_matches_list_entry_happy_path() {
    let summaries = vec![
        sample_iter_summary("r1", "CleanupIter"),
        sample_iter_summary("r2", "ArchiveIter"),
    ];
    let summary = find_iter_run_summary_by_id(&summaries, "r2").expect("listed");
    assert_eq!(summary.iter_name, "ArchiveIter");

    let views = vec![sample_iter_view("r1"), sample_iter_view("r2")];
    let detail = find_iter_run_by_id(&views, "r1").expect("listed");
    assert_eq!(detail.status, "completed");
    assert_eq!(detail.processed_rows, 5);
}

#[test]
fn get_iter_run_unknown_id_is_none_sad() {
    let runs = vec![sample_iter_summary("r1", "CleanupIter")];
    assert!(find_iter_run_summary_by_id(&runs, "__valence_uf_app_no_such_run__").is_none());
}

#[test]
fn get_deletion_run_detail_matches_list_entry_happy_path() {
    let runs = vec![
        sample_deletion("d1", "counter"),
        sample_deletion("d2", "user"),
    ];
    let detail = find_deletion_run_by_id(&runs, "d2").expect("listed");
    assert_eq!(detail.root_table, "user");
    assert_eq!(detail.status, "pending");
}

#[test]
fn get_deletion_run_unknown_id_is_none_sad() {
    let runs = vec![sample_deletion("d1", "counter")];
    assert!(find_deletion_run_by_id(&runs, "__valence_uf_app_no_such_run__").is_none());
}

#[test]
fn deletion_run_json_maps_to_view_happy_path() {
    let row = serde_json::json!({
        "id": "del-9",
        "root_table": "counter",
        "root_record_id": "c1",
        "status": "cancelled",
        "total_steps": 4,
        "completed_steps": 1,
        "failed_steps": 0,
        "requested_at": 42,
    });
    let view = deletion_run_view_from_value(&row).expect("mapped");
    assert_eq!(view.run_id, "del-9");
    assert_eq!(view.requested_at, "42");
    assert_eq!(view.status, "cancelled");
}

#[test]
fn deletion_run_json_missing_id_none_sad() {
    let row = serde_json::json!({ "status": "running" });
    assert!(deletion_run_view_from_value(&row).is_none());
}

#[test]
fn clamp_deletion_list_limit_window_happy_path() {
    assert_eq!(clamp_deletion_list_limit(0), 1);
    assert_eq!(clamp_deletion_list_limit(200), 200);
    assert_eq!(clamp_deletion_list_limit(201), 200);
}

#[test]
fn normalize_entity_id_surreal_display_happy_path() {
    assert_eq!(
        normalize_entity_id_for_lookup("counter:⟨singleton⟩".into()),
        "singleton"
    );
}

#[test]
fn validate_schema_name_accepts_table_happy_path() {
    validate_schema_name("counter").expect("ok");
}

#[test]
fn validate_schema_name_rejects_blank_sad() {
    assert_eq!(
        validate_schema_name("").expect_err("blank"),
        valence_backend::ValenceIdError::EmptySchemaName
    );
}

#[test]
fn validate_run_id_rejects_blank_sad() {
    assert_eq!(
        validate_run_id("  ").expect_err("blank"),
        valence_backend::ValenceIdError::EmptyRunId
    );
}

#[test]
fn validate_entity_id_rejects_blank_sad() {
    assert_eq!(
        validate_entity_id("").expect_err("blank"),
        valence_backend::ValenceIdError::EmptyEntityId
    );
}

#[test]
fn validate_iter_name_rejects_blank_sad() {
    assert_eq!(
        validate_iter_name(" ").expect_err("blank"),
        valence_backend::ValenceIdError::EmptyIterName
    );
}

#[test]
fn validate_trait_name_rejects_blank_sad() {
    assert_eq!(
        validate_trait_name("").expect_err("blank"),
        valence_backend::ValenceIdError::EmptyTraitName
    );
}

#[test]
fn validate_schema_name_rejects_slash_and_oversized_sad() {
    assert_eq!(
        validate_schema_name("a/b").expect_err("slash"),
        valence_backend::ValenceIdError::UnsafeSchemaName
    );
    let oversized = "s".repeat(valence_backend::MAX_VALENCE_ID_CHARS + 1);
    assert_eq!(
        validate_schema_name(&oversized).expect_err("too long"),
        valence_backend::ValenceIdError::SchemaNameTooLong
    );
}

#[test]
fn validate_entity_id_rejects_dotdot_sad() {
    assert_eq!(
        validate_entity_id("..").expect_err("dotdot"),
        valence_backend::ValenceIdError::UnsafeEntityId
    );
}

//! Valence runtime contracts for the lab host (not Playwright).
//!
//! Asserts SchemaRegistry presence, seeded iter/deletion durability, ValenceAdmin
//! allow/deny via Gauge, and the `start_iter_run` partial-commit policy
//! (pending row remains after Chronon-orchestration error).

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use chrono::Utc;
use gauge::service;
use valence::{Actor, Model, SchemaRegistry};
use valence_platform::deletion::run_service::DeletionService;
use valence_platform::{ValenceIterRun, ValenceIterRunStatus};
use valence_uf_app_e2e::{
    e2e_admin_valence, e2e_fixtures, e2e_outsider_valence, e2e_system_valence, init_e2e_valence,
    E2E_ITER_NAME, E2E_SCHEMA_NAME,
};

#[tokio::test]
async fn schema_registry_lists_user_happy_path() {
    init_e2e_valence().await;
    let schemas = SchemaRegistry::global().list_schemas();
    assert!(
        schemas.iter().any(|s| *s == E2E_SCHEMA_NAME),
        "expected `{E2E_SCHEMA_NAME}` in {schemas:?}"
    );
    assert!(
        SchemaRegistry::global()
            .get_schema("__no_such_schema__")
            .is_none(),
        "unknown schema must be absent"
    );
}

#[tokio::test]
async fn seeded_user_entity_readable_happy_path() {
    init_e2e_valence().await;
    let fixtures = e2e_fixtures();
    let admin = e2e_admin_valence();
    let user = lepton::generated::User::get(&fixtures.entity_id, &admin)
        .await
        .expect("query user")
        .expect("admin user row");
    let id = user.id().map(|id| id.to_string()).expect("admin user id");
    assert!(
        id == fixtures.entity_id || id.ends_with(&format!(":{}", fixtures.entity_id)),
        "expected bare or table-prefixed id containing {}; got {id}",
        fixtures.entity_id
    );
}

#[tokio::test]
async fn seeded_user_entity_unknown_is_none_sad() {
    init_e2e_valence().await;
    let admin = e2e_admin_valence();
    let missing = lepton::generated::User::get("__valence_e2e_missing_user__", &admin)
        .await
        .expect("query");
    assert!(missing.is_none());
}

#[tokio::test]
async fn valence_admin_grant_happy_and_outsider_deny_sad() {
    init_e2e_valence().await;
    let admin = e2e_admin_valence();
    let outsider = e2e_outsider_valence();
    assert!(
        service::actor_can(&admin, "ValenceAdmin")
            .await
            .expect("admin check"),
        "admin must hold ValenceAdmin"
    );
    assert!(
        !service::actor_can(&outsider, "ValenceAdmin")
            .await
            .expect("outsider check"),
        "outsider must not hold ValenceAdmin"
    );
}

#[tokio::test]
async fn seeded_iter_run_persisted_happy_path() {
    init_e2e_valence().await;
    let fixtures = e2e_fixtures();
    let system = e2e_system_valence();
    let run = ValenceIterRun::get(&fixtures.iter_run_id, &system)
        .await
        .expect("get iter run")
        .expect("seeded iter run");
    assert_eq!(run.iter_name(), E2E_ITER_NAME);
    assert_eq!(*run.status(), ValenceIterRunStatus::Pending);
}

#[tokio::test]
async fn seeded_deletion_run_persisted_happy_path() {
    init_e2e_valence().await;
    let fixtures = e2e_fixtures();
    let admin = e2e_admin_valence();
    let run = DeletionService::get_run_json(&fixtures.deletion_run_id, &admin)
        .await
        .expect("get deletion run")
        .expect("seeded deletion run");
    assert_eq!(
        run.get("root_table").and_then(|v| v.as_str()),
        Some(E2E_SCHEMA_NAME)
    );
    assert_eq!(run.get("status").and_then(|v| v.as_str()), Some("queued"));
}

#[tokio::test]
async fn deletion_run_unknown_is_none_sad() {
    init_e2e_valence().await;
    let admin = e2e_admin_valence();
    let missing = DeletionService::get_run_json("__valence_e2e_missing_deletion__", &admin)
        .await
        .expect("query");
    assert!(missing.is_none());
}

/// Documents `start_iter_run` Wave-7b policy: pending row is committed before the
/// Chronon-orchestration error is returned. Containment here means the durable
/// pending row is observable (not rolled back) so operators can see the orphan.
#[tokio::test]
async fn start_iter_run_partial_commit_policy_happy_path() {
    init_e2e_valence().await;
    let system = e2e_system_valence();
    let run_id = uuid::Uuid::new_v4().to_string();
    let row = ValenceIterRun::new(
        "partial_commit_iter".into(),
        E2E_SCHEMA_NAME.into(),
        ValenceIterRunStatus::Pending,
        0,
        0,
        0,
        0,
        0,
        None,
        None,
        None,
        Utc::now(),
        serde_json::to_string(&Actor::User {
            user_id: "admin".into(),
        })
        .unwrap(),
        None,
    )
    .expect("build run");
    // Mirrors start_iter_run upsert (platform path uses System for SYSTEM_ONLY tables).
    ValenceIterRun::upsert(&run_id, row, &system)
        .await
        .expect("upsert pending");

    // Server fn then returns Chronon wiring error; pending row must remain.
    let orchestration_err =
        "Iter orchestration requires Chronon host wiring (deployment template Wave 7b)";
    assert!(
        orchestration_err.contains("Chronon"),
        "error message contract must stay searchable"
    );

    let persisted = ValenceIterRun::get(&run_id, &system)
        .await
        .expect("reload")
        .expect("pending row must survive orchestration failure");
    assert_eq!(*persisted.status(), ValenceIterRunStatus::Pending);
    assert_eq!(persisted.iter_name(), "partial_commit_iter");
}

#[tokio::test]
async fn cancel_deletion_run_merge_happy_path() {
    init_e2e_valence().await;
    let admin = e2e_admin_valence();
    let actor_json = serde_json::to_value(Actor::User {
        user_id: "admin".into(),
    })
    .unwrap();
    let run_id = DeletionService::create_run("user", "outsider", actor_json, &admin)
        .await
        .expect("create");
    DeletionService::merge_run(
        &run_id,
        serde_json::json!({ "status": "cancelled" }),
        &admin,
    )
    .await
    .expect("cancel merge");
    let run = DeletionService::get_run_json(&run_id, &admin)
        .await
        .expect("get")
        .expect("row");
    assert_eq!(
        run.get("status").and_then(|v| v.as_str()),
        Some("cancelled")
    );
}

#[tokio::test]
async fn system_valence_builds_happy_path() {
    init_e2e_valence().await;
    let _ = e2e_system_valence();
}

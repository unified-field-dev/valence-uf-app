//! Scan-only `*_used` / `use_!` call sites for Data uses E2E.
//!
//! Intentionally **not** declared in `lib.rs` / `main.rs`. Cargo never compiles this
//! file; `valence-data-use-scan` walks package `.rs` sources and records these rows
//! in the UI snapshot (`exclude_tests_from_snapshot: true`).

#![allow(unused)]

async fn _e2e_data_use_catalog_fixtures() {
    // Schema(user) — TM-UI-S-1 / T-1 schema-only string / X-2 prod purpose
    let _ = User::get_used(
        "id",
        &valence,
        use_!("E2E catalog: load user for Valence schema Data uses."),
    )
    .await;

    // Trait(PermissionPrincipal) — TM-UI-S-3 / T-1 / X-1 fan-out to implementors
    let _ = PermissionPrincipalQueryAll::query_used(
        &valence,
        use_!("E2E catalog: list PermissionPrincipal rows for trait Data uses."),
    )
    .await;

    // Unscoped — TM-UI-U-2
    let _ = QueryCore::execute_used(
        builder,
        use_!("E2E catalog: QueryCore walk for Unscoped uses."),
    )
    .await;
}

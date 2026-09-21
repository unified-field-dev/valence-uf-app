//! Scan-only `*_used` / `use_!` call sites for Data uses E2E.
//!
//! Intentionally **not** declared in `lib.rs` / `main.rs`. Cargo never compiles this
//! file; `valence-data-use-scan` walks package `.rs` sources and records these rows
//! in the UI snapshot (`exclude_tests_from_snapshot: true`).

#![allow(unused)]

async fn _e2e_data_use_catalog_fixtures() {
    // Schema(user) — TM-UI-S-1 / plain read (not Referenced)
    let _ = User::get_used(
        "id",
        &valence,
        valence::use_!(r#"**Test:** Fixture **User** access in `data_use_catalog_fixtures` so the suite can arrange and assert persistence. CI and developers running the suite only."#),
    )
    .await;

    // Trait(PermissionPrincipal) — TM-UI-S-3 / T-1 / X-1 fan-out to implementors
    let _ = PermissionPrincipalQueryAll::query_used(
        &valence,
        valence::use_!(r#"**Test:** Fixture **Permission Principal Query All** access in `data_use_catalog_fixtures` so the suite can arrange and assert persistence. CI and developers running the suite only."#),
    )
    .await;

    // Unscoped — TM-UI-U-2
    let _ = QueryCore::execute_used(
        builder,
        valence::use_!(r#"**Test:** Fixture **this data** access in `data_use_catalog_fixtures` so the suite can arrange and assert persistence. CI and developers running the suite only."#),
    )
    .await;

    // HasOne forward — Referenced Read on user (Source=todo)
    let _ = Todo::get_owner_used(
        &valence,
        valence::use_!(r#"**Test:** Fixture **Todo owner hop** load for Referenced Reads on User."#),
    )
    .await;

    // Reverse — primary on todo only; must NOT appear under User Referenced
    let _ = Todo::get_from_owner_id_used(
        "owner-id",
        &valence,
        valence::use_!(r#"**Test:** Fixture **Todo reverse owner** lookup; stays on Todo Reads only."#),
    )
    .await;

    // Second source → same peer (multi-source)
    let _ = Project::get_owner_used(
        &valence,
        valence::use_!(r#"**Test:** Fixture **Project owner hop** second source onto User Referenced Reads."#),
    )
    .await;

    // HasMany — Referenced Read on task (Source=project)
    let _ = Project::get_tasks_used(
        &valence,
        valence::use_!(r#"**Test:** Fixture **Project tasks hop** for Referenced Reads on Task."#),
    )
    .await;

    // M2M get — Referenced Read on tag
    let _ = Todo::get_tags_used(
        &valence,
        valence::use_!(r#"**Test:** Fixture **Todo tags hop** for Referenced Reads on Tag."#),
    )
    .await;

    // M2M relate — Referenced Update on tag
    let _ = Todo::relate_to_tag_used(
        &tag,
        &valence,
        valence::use_!(r#"**Test:** Fixture **Todo relate tag** for Referenced Updates on Tag."#),
    )
    .await;

    // M2M unrelate — Referenced Update on tag
    let _ = Todo::unrelate_from_tag_used(
        &tag,
        &valence,
        valence::use_!(r#"**Test:** Fixture **Todo unrelate tag** for Referenced Updates on Tag."#),
    )
    .await;
}

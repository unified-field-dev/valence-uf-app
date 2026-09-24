//! Test twin of the catalog fixtures — must be omitted from the UI snapshot.
//!
//! Lives under `tests/` so `exclude_tests_from_snapshot` drops it. Not a Cargo
//! integration test target (subdirectory); syn-parsed only when exclusion is off.

#![allow(unused)]

async fn _e2e_data_use_catalog_twin() {
    let _ = User::get(
        "id",
        &valence,
        valence::use_!(r#"**Test:** Fixture **User** access in `data_use_catalog_twin` so the suite can arrange and assert persistence. CI and developers running the suite only."#),
    )
    .await;
}

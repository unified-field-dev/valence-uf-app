//! Test twin of the catalog fixtures — must be omitted from the UI snapshot.
//!
//! Lives under `tests/` so `exclude_tests_from_snapshot` drops it. Not a Cargo
//! integration test target (subdirectory); syn-parsed only when exclusion is off.

#![allow(unused)]

async fn _e2e_data_use_catalog_twin() {
    let _ = User::get_used(
        "id",
        &valence,
        use_!("E2E_TEST_ONLY_PURPOSE — must stay out of UI snapshot"),
    )
    .await;
}

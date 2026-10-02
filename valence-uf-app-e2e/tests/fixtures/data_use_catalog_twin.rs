//! Test twin of the catalog fixtures — must be omitted from the UI snapshot.
//!
//! Lives under `tests/`, which the data-use scan never reads (it walks `src/` only).
//! Not a Cargo integration test target (subdirectory).

#![allow(unused)]

async fn _e2e_data_use_catalog_twin() {
    let _ = User::get(
        "id",
        &valence,
        valence::use_!(r#"**Test:** Fixture **User** access in `data_use_catalog_twin` so the suite can arrange and assert persistence. CI and developers running the suite only."#),
    )
    .await;
}

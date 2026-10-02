//! Data-use catalog coverage for the e2e deployment (TM-PROBE-1).
//!
//! The catalog must hold the server crate, the out-of-workspace product it links,
//! and the worker it declares as an inventory dependency, but never the workspace
//! binary it does not declare.

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use valence::data_use::{DataOp, DataUseTarget};
use valence_uf_app_e2e::data_use_catalog;

#[test]
fn probe_catalog_rows() {
    let catalog = data_use_catalog();
    let crates = catalog.crate_names();
    for expected in [
        "valence-uf-app-e2e",
        "valence-app",
        "data-use-probe-product",
        "data-use-probe-worker",
    ] {
        assert!(
            crates.contains(expected),
            "{expected} missing from {crates:?}"
        );
    }
    assert!(
        !crates.contains("data-use-probe-unwired"),
        "undeclared binary leaked into {crates:?}"
    );
    assert!(catalog
        .entries()
        .iter()
        .all(|e| !e.purpose.contains("PROBE_UNWIRED_MUST_NOT_APPEAR")));
}

#[test]
fn probe_worker_update_targets_widget_with_repo_relative_path() {
    let catalog = data_use_catalog();
    let worker = catalog
        .entries()
        .iter()
        .find(|e| e.crate_name == "data-use-probe-worker")
        .expect("worker row");
    assert_eq!(worker.op, DataOp::Update);
    assert_eq!(
        worker.target,
        DataUseTarget::Schema("data_use_probe_widget".into())
    );
    assert_eq!(worker.file, "probe/data-use-probe-worker/src/main.rs");
    assert_eq!(
        worker.repository,
        "https://github.com/unified-field-dev/valence-uf-app"
    );

    for entry in catalog.entries() {
        assert!(
            !entry.file.starts_with('/'),
            "absolute path in catalog: {}",
            entry.file
        );
    }
}

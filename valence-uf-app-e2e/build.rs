//! Build-time data-use catalog for the Playwright host workspace scan.

use std::env;
use std::path::PathBuf;

use valence_data_use_scan::{generate, Config, ConnectionEdge};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    // valence-uf-app-e2e sits under the valence-uf-app workspace root.
    let workspace_root = manifest_dir
        .parent()
        .expect("e2e crate parent is workspace root")
        .to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    // Ensure fixture sources re-trigger the scan.
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir
            .join("src/data_use_catalog_fixtures.rs")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir
            .join("tests/fixtures/data_use_catalog_twin.rs")
            .display()
    );

    if let Err(err) = generate(&Config {
        workspace_root,
        out_dir,
        exclude_tests_from_snapshot: true,
        connection_edges: vec![
            ConnectionEdge {
                from_table: "todo".into(),
                from_field: "owner".into(),
                to_table: "user".into(),
            },
            ConnectionEdge {
                from_table: "project".into(),
                from_field: "owner".into(),
                to_table: "user".into(),
            },
            ConnectionEdge {
                from_table: "project".into(),
                from_field: "tasks".into(),
                to_table: "task".into(),
            },
            ConnectionEdge {
                from_table: "todo".into(),
                from_field: "tags".into(),
                to_table: "tag".into(),
            },
        ],
    }) {
        panic!("valence data-use scan failed: {err}");
    }
}

//! Build-time data-use catalog for the e2e server: this package, everything it
//! links, and the inventory components in its `cfg(any())` table.

use std::env;
use std::path::PathBuf;

use valence_data_use_scan::{generate, Config, ConnectionEdge, HostPackage};

fn main() {
    // Only the SSR server installs the catalog; the hydrate (wasm) build never includes it.
    if env::var_os("CARGO_FEATURE_SSR").is_none() {
        return;
    }
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .expect("e2e crate parent is workspace root")
        .to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    if let Err(err) = generate(&Config {
        workspace_root,
        out_dir,
        host: HostPackage::FromBuildScript,
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

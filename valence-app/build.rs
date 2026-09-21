//! Build-time data-use catalog for the Valence ops UI.

use std::env;
use std::path::PathBuf;

use valence_data_use_scan::{generate, Config};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    // valence-app sits one level under the valence-uf-app workspace root.
    let workspace_root = manifest_dir
        .parent()
        .expect("valence-app parent is workspace root")
        .to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    if let Err(err) = generate(&Config {
        workspace_root,
        out_dir,
        exclude_tests_from_snapshot: true,
        connection_edges: vec![],
    }) {
        // Loud failure — empty catalogs must not silently pretend there are no uses.
        panic!("valence data-use scan failed: {err}");
    }
}

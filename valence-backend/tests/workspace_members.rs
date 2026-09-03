//! Gate: valence-app / valence-backend / protected host are members of this workspace.
//!
//! Featureless sibling-source contract (photon / boson / chronon / spectra / gauge /
//! lepton-shell pattern).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn valence_product_workspace_members_happy_path() {
    let root =
        fs::read_to_string(workspace_root().join("Cargo.toml")).expect("workspace Cargo.toml");
    for member in [
        "valence-app",
        "valence-backend",
        "valence-uf-app-e2e",
        "examples/protected-valence-host",
    ] {
        assert!(
            root.contains(&format!("\"{member}\"")),
            "workspace must list {member}"
        );
        assert!(
            workspace_root().join(member).join("Cargo.toml").is_file(),
            "missing crate dir {member}"
        );
    }
}

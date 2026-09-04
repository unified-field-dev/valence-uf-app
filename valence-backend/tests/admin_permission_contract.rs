//! Contract for ValenceAdmin-gated mutating server functions (VU-08).
//!
//! Browse/read server fns stay ungated so entity pages load without admin;
//! every mutating admin op must appear here and use the shared permission name.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use valence_backend::{
    is_valence_admin_server_fn, VALENCE_ADMIN_PERMISSION, VALENCE_ADMIN_SERVER_FNS,
};

fn valence_app_server_rs() -> String {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = manifest.join("../valence-app/src/server");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rs"))
        .collect();
    files.sort();
    let mut out = String::new();
    for path in files {
        out.push_str(
            &std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
        );
        out.push('\n');
    }
    out
}

#[test]
fn valence_admin_permission_name_happy_path() {
    assert_eq!(VALENCE_ADMIN_PERMISSION, "ValenceAdmin");
}

#[test]
fn valence_admin_server_fns_lists_all_mutators_happy_path() {
    assert_eq!(VALENCE_ADMIN_SERVER_FNS.len(), 5);
    for name in [
        "start_iter_run",
        "run_iter_on_entity",
        "cancel_iter_run",
        "delete_entity_queue",
        "cancel_deletion_run",
    ] {
        assert!(
            is_valence_admin_server_fn(name),
            "expected {name} in VALENCE_ADMIN_SERVER_FNS"
        );
    }
}

#[test]
fn server_rs_mutators_carry_valence_admin_macro_happy_path() {
    let src = valence_app_server_rs();
    for name in VALENCE_ADMIN_SERVER_FNS {
        let needle = format!("fn {name}(");
        let idx = src
            .find(&needle)
            .unwrap_or_else(|| panic!("missing server fn {name}"));
        let prefix = &src[..idx];
        let macro_line = prefix
            .lines()
            .rev()
            .find(|line| line.contains("#[uf_product_macros::server"))
            .unwrap_or_else(|| panic!("no server macro above {name}"));
        assert!(
            macro_line.contains(r#"permission = "ValenceAdmin""#),
            "{name} must use ValenceAdmin macro; got: {macro_line}"
        );
    }
}

#[test]
fn server_rs_entity_view_stays_ungated_happy_path() {
    let src = valence_app_server_rs();
    let idx = src
        .find("pub async fn get_entity_view")
        .expect("get_entity_view");
    let prefix = &src[..idx];
    let macro_line = prefix
        .lines()
        .rev()
        .find(|line| line.contains("#[uf_product_macros::server"))
        .expect("server macro");
    assert!(
        !macro_line.contains("permission"),
        "entity view must stay loadable without ValenceAdmin: {macro_line}"
    );
}

#[test]
fn is_valence_admin_server_fn_rejects_browse_reads_sad() {
    for name in [
        "get_entity_view",
        "get_schema",
        "get_schemas",
        "search_schema_or_id",
        "evaluate_iter_for_entity",
        "get_iter_run",
        "list_deletion_runs",
    ] {
        assert!(
            !is_valence_admin_server_fn(name),
            "{name} must stay browse/read without ValenceAdmin"
        );
    }
}

#[test]
fn is_valence_admin_server_fn_rejects_blank_sad() {
    assert!(!is_valence_admin_server_fn(""));
    assert!(!is_valence_admin_server_fn("   "));
}

//! Product surface **smoke** contracts for valence-app (sibling crate).
//!
//! These are composition / source-needle guards (routes, testids, `ValenceAdmin`
//! string presence). They are **not** validating happy/sad behavior coverage —
//! see `valence-uf-app-e2e` Layer 2 Playwright + `runtime_contract` for that.
//!
//! Lives under `valence-backend` so CI can gate route/testid/auth/ValenceAdmin needles
//! without compiling Orbital/turf UI when host pins churn. Pattern matches
//! photon-uf-app / boson-uf-app / chronon-uf-app / spectra-uf-app
//! `*-backend/tests/product_surface.rs`, gauge `gauge/tests/product_surface.rs`,
//! and lepton-uf-app `lepton-shell/tests/product_surface.rs`.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_app(rel: &str) -> String {
    let path = workspace_root().join("valence-app").join("src").join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn read_server_sources() -> String {
    let dir = workspace_root()
        .join("valence-app")
        .join("src")
        .join("server");
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
            &fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
        );
        out.push('\n');
    }
    out
}

#[test]
fn valence_routes_mount_happy_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("valence")"#,
        r#"path!("")"#,
        r#"path!("schema")"#,
        r#"path!("schema/:schema_name")"#,
        r#"path!("schema/:schema_name/id/:entity_id")"#,
        r#"path!("schema/:schema_name/iter/:run_id")"#,
        r#"path!("schema/:schema_name/deletion/:run_id")"#,
        r#"path!("traits")"#,
        r#"path!("traits/:trait_name")"#,
        r#"path!("iters")"#,
        r#"path!("deletions")"#,
        "ValenceLayout",
        "id: \"valence\"",
        "route_path: \"/valence\"",
        "permission_manifest: permissions::ValencePermission",
    ] {
        assert!(
            lib.contains(needle),
            "ValenceRoutes / uf_app missing `{needle}`"
        );
    }
}

#[test]
fn valence_routes_drop_leaf_sad_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("schema/:schema_name")"#,
        r#"path!("schema/:schema_name/id/:entity_id")"#,
        r#"path!("schema/:schema_name/iter/:run_id")"#,
        r#"path!("schema/:schema_name/deletion/:run_id")"#,
        r#"path!("traits/:trait_name")"#,
        r#"path!("iters")"#,
        r#"path!("deletions")"#,
    ] {
        assert!(
            lib.contains(needle),
            "removing `{needle}` drops a Valence ops funnel entry"
        );
    }
    assert!(
        !lib.contains("unimplemented!"),
        "ValenceRoutes must not ship unimplemented placeholders"
    );
}

#[test]
fn uf_app_wrong_id_sad_path() {
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"valence\""),
        "wrong uf_app id breaks Orbital host registration"
    );
    assert!(
        !lib.contains("id: \"valence-app\""),
        "uf_app id must stay `valence` (product route id), not crate name valence-app"
    );
}

#[test]
fn layout_auth_gate_and_nav_happy_path() {
    let layout = read_app("layout.rs");
    for needle in [
        "valence-app-root",
        "RequireAuthenticated",
        "Outlet",
        "nav-dashboard",
        "nav-schemas",
        "nav-traits",
        "nav-iters",
        "nav-deletions",
        "AppBarUserMenu",
        "UnifiedFieldShellLayout",
    ] {
        assert!(
            layout.contains(needle),
            "ValenceLayout missing contract `{needle}`"
        );
    }
}

#[test]
fn layout_drop_auth_guard_sad_path() {
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated") && layout.contains("<Outlet />"),
        "removing RequireAuthenticated opens /valence pages to anonymous sessions"
    );
}

#[test]
fn layout_missing_nav_sad_path() {
    let layout = read_app("layout.rs");
    for id in [
        "nav-dashboard",
        "nav-schemas",
        "nav-traits",
        "nav-iters",
        "nav-deletions",
    ] {
        assert!(
            layout.contains(id),
            "dropping `{id}` breaks operator left-nav contract"
        );
    }
}

#[test]
fn admin_mutators_require_valence_admin_happy_path() {
    let server = read_server_sources();
    for fn_name in [
        "start_iter_run",
        "run_iter_on_entity",
        "cancel_iter_run",
        "delete_entity_queue",
        "cancel_deletion_run",
    ] {
        assert!(server.contains(fn_name), "server missing `{fn_name}`");
    }
    let admin_attr = r#"permission = "ValenceAdmin""#;
    assert!(
        server.matches(admin_attr).count() >= 5,
        "mutating admin server fns must carry ValenceAdmin permission attribute"
    );
}

#[test]
fn admin_mutators_drop_valence_admin_sad_path() {
    let server = read_server_sources();
    let admin_attr = r#"permission = "ValenceAdmin""#;
    assert!(
        server.matches(admin_attr).count() >= 5,
        "dropping ValenceAdmin from any mutator opens Valence mutations without the admin gate"
    );
    assert!(
        !server.contains(r#"permission = "GaugeAdmin""#)
            && !server.contains(r#"permission = "PhotonAdmin""#)
            && !server.contains(r#"permission = "BosonAdmin""#)
            && !server.contains(r#"permission = "ChrononAdmin""#),
        "Valence mutators must not gate on GaugeAdmin / PhotonAdmin / BosonAdmin / ChrononAdmin"
    );
}

#[test]
fn browse_reads_stay_ungated_happy_path() {
    let server = read_server_sources();
    let idx = server
        .find("pub async fn get_entity_view")
        .expect("get_entity_view");
    let prefix = &server[..idx];
    let macro_line = prefix
        .lines()
        .rev()
        .find(|line| line.contains("#[uf_product_macros::server") || line.contains("#[server"))
        .expect("server macro above get_entity_view");
    assert!(
        !macro_line.contains("permission"),
        "entity view must stay loadable without ValenceAdmin: {macro_line}"
    );
}

#[test]
fn server_require_session_happy_path() {
    let server = read_server_sources();
    assert!(
        server.contains("fn require_session")
            && server.contains("auth: Authentication required")
            && server.contains("require_authenticated_session")
            && server.contains("session_user_id()"),
        "server must fail closed without a session for admin mutators and registry browse"
    );

    for call_site in [
        "start_iter_run",
        "run_iter_on_entity",
        "cancel_iter_run",
        "delete_entity_queue",
        "cancel_deletion_run",
    ] {
        assert!(server.contains(call_site), "server missing `{call_site}`");
    }
}

#[test]
fn server_drop_require_session_on_mutators_sad_path() {
    let server = read_server_sources();

    let start = server.find("fn start_iter_run(").expect("start_iter_run");
    let body = &server[start..start + 520.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "start_iter_run must call require_session before Valence IO"
    );

    let start = server
        .find("pub async fn cancel_deletion_run")
        .expect("cancel_deletion_run");
    let body = &server[start..start + 350.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "cancel_deletion_run must call require_session before Valence IO"
    );

    let start = server
        .find("pub async fn delete_entity_queue")
        .expect("delete_entity_queue");
    let body = &server[start..start + 480.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "delete_entity_queue must call require_session before Valence IO"
    );
}

#[test]
fn index_pages_testid_and_list_bindings_happy_path() {
    let dashboard = read_app("pages/dashboard/mod.rs");
    assert!(
        dashboard.contains("valence-dashboard-page"),
        "ValenceDashboardPage missing valence-dashboard-page testid"
    );
    let search = read_app("pages/dashboard/search_header.rs");
    assert!(
        search.contains("SearchSchemaOrId"),
        "DashboardSearchHeader must bind SearchSchemaOrId"
    );

    let schema_index = read_app("pages/schema_index/mod.rs");
    assert!(
        schema_index.contains("schema-index-page"),
        "ValenceSchemaIndexPage missing schema-index-page testid"
    );
    let schema_fetcher = read_app("pages/schema_index/components/schema_table/fetcher.rs");
    assert!(
        schema_fetcher.contains("get_schemas_page"),
        "schema table fetcher must bind get_schemas_page"
    );

    let trait_index = read_app("pages/trait_index/mod.rs");
    assert!(
        trait_index.contains("trait-index-page"),
        "ValenceTraitIndexPage missing trait-index-page testid"
    );
    let trait_fetcher = read_app("pages/trait_index/components/trait_table/fetcher.rs");
    assert!(
        trait_fetcher.contains("get_traits_page"),
        "trait table fetcher must bind get_traits_page"
    );

    let iter_index = read_app("pages/iter_index/mod.rs");
    assert!(
        iter_index.contains("valence-iter-index-page"),
        "ValenceIterIndexPage missing valence-iter-index-page testid"
    );
    let iter_list = read_app("pages/iter_index/components/iter_runs_list_section.rs");
    assert!(
        iter_list.contains("list_iter_runs"),
        "iter index must bind list_iter_runs"
    );

    let deletion_index = read_app("pages/deletion_index/mod.rs");
    assert!(
        deletion_index.contains("valence-deletion-index-page"),
        "ValenceDeletionIndexPage missing valence-deletion-index-page testid"
    );
    assert!(
        deletion_index.contains("list_deletion_runs"),
        "deletion index must bind list_deletion_runs"
    );
}

#[test]
fn index_drop_dashboard_testid_sad_path() {
    let dashboard = read_app("pages/dashboard/mod.rs");
    assert!(
        dashboard.contains("data_testid=\"valence-dashboard-page\""),
        "dropping valence-dashboard-page breaks host / future Playwright parity"
    );
    let schema_index = read_app("pages/schema_index/mod.rs");
    assert!(
        schema_index.contains("data_testid=\"schema-index-page\""),
        "dropping schema-index-page breaks host / future Playwright parity"
    );
    let deletion_index = read_app("pages/deletion_index/mod.rs");
    assert!(
        deletion_index.contains("data_testid=\"valence-deletion-index-page\""),
        "dropping valence-deletion-index-page breaks host / future Playwright parity"
    );
}

#[test]
fn detail_pages_testid_and_bindings_happy_path() {
    let schema = read_app("pages/schema/mod.rs");
    assert!(
        schema.contains("valence-schema-detail-page") && schema.contains("get_schema"),
        "schema detail must expose testid and bind get_schema"
    );
    let iters_card = read_app("pages/schema/components/iters_card.rs");
    assert!(
        iters_card.contains("start_iter_run"),
        "schema iters card must bind start_iter_run"
    );

    let entity = read_app("pages/entity/mod.rs");
    assert!(
        entity.contains("valence-entity-page")
            && entity.contains("get_entity_view")
            && entity.contains("get_entity_privacy_evaluation"),
        "entity page must expose testid and bind entity view / privacy eval"
    );
    let entity_iters = read_app("pages/entity/components/iters_card.rs");
    assert!(
        entity_iters.contains("run_iter_on_entity"),
        "entity iters card must bind run_iter_on_entity"
    );

    let trait_detail = read_app("pages/trait_detail/mod.rs");
    assert!(
        trait_detail.contains("valence-trait-detail-page") && trait_detail.contains("get_trait"),
        "trait detail must expose testid and bind get_trait"
    );

    let iter_run = read_app("pages/iter_run/mod.rs");
    assert!(
        iter_run.contains("valence-iter-run-page")
            && iter_run.contains("get_iter_run")
            && iter_run.contains("cancel_iter_run"),
        "iter run page must expose testid and bind get/cancel"
    );

    let deletion_run = read_app("pages/deletion_run/mod.rs");
    assert!(
        deletion_run.contains("valence-deletion-run-page")
            && deletion_run.contains("get_deletion_run")
            && deletion_run.contains("cancel_deletion_run"),
        "deletion run page must expose testid and bind get/cancel"
    );
}

#[test]
fn detail_pages_missing_bindings_sad_path() {
    let entity = read_app("pages/entity/mod.rs");
    assert!(
        entity.contains("get_entity_view"),
        "entity page must bind get_entity_view"
    );
    assert!(
        !entity.contains("unimplemented!"),
        "entity page must not ship unimplemented placeholders"
    );
    let iter_run = read_app("pages/iter_run/mod.rs");
    assert!(
        iter_run.contains("cancel_iter_run"),
        "iter run page must keep cancel_iter_run binding"
    );
}

#[test]
fn permission_manifest_valence_admin_happy_path() {
    let perms = read_app("permissions.rs");
    for needle in [
        "domain_key = \"valence\"",
        "ValenceAdmin",
        "UfPermissionManifest",
    ] {
        assert!(
            perms.contains(needle),
            "ValencePermission manifest missing `{needle}`"
        );
    }
}

#[test]
fn protected_valence_host_matches_uf_app_happy_path() {
    let host =
        fs::read_to_string(workspace_root().join("examples/protected-valence-host/src/main.rs"))
            .expect("protected-valence-host main.rs");
    for needle in [
        "\"app_id\": \"valence\"",
        "\"route_path\": \"/valence\"",
        "\"auth_gate\": \"RequireAuthenticated\"",
        "\"admin_permission\": \"ValenceAdmin\"",
        "find_schema_by_name",
        "validate_schema_name",
    ] {
        assert!(
            host.contains(needle),
            "protected-valence-host missing contract `{needle}`"
        );
    }
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"valence\"") && lib.contains("route_path: \"/valence\""),
        "host inventory must stay aligned with uf_app!"
    );
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated"),
        "host auth_gate must stay aligned with ValenceLayout guard"
    );
    let perms = read_app("permissions.rs");
    assert!(
        perms.contains("ValenceAdmin"),
        "host admin_permission must stay aligned with ValencePermission"
    );
}

#[test]
fn lazy_routes_wire_pages_happy_path() {
    let lazy = read_app("lazy_routes.rs");
    for needle in [
        "ValenceDashboardPage",
        "ValenceSchemaIndexPage",
        "ValenceSchemaPage",
        "ValenceEntityPage",
        "ValenceTraitIndexPage",
        "ValenceTraitDetailPage",
        "ValenceIterIndexPage",
        "ValenceIterRunPage",
        "ValenceDeletionIndexPage",
        "ValenceDeletionRunPage",
    ] {
        assert!(
            lazy.contains(needle),
            "lazy_routes missing page wire `{needle}`"
        );
    }
}

#[test]
fn ops_path_helpers_encode_segments_happy_path() {
    let schema_table = read_app("pages/schema_index/components/schema_table/schema_data_table.rs");
    let trait_table = read_app("pages/trait_index/components/trait_table/trait_data_table.rs");
    let samples = read_app("pages/schema/components/samples_card.rs");
    let deletion_row = read_app("pages/schema/components/deletion_run_row.rs");
    let deletion_index = read_app("pages/deletion_index/mod.rs");
    let iter_row = read_app("pages/iter_index/components/iter_run_row.rs");
    let connection_row = read_app("pages/entity/components/connection_row.rs");
    for (label, src) in [
        ("schema_data_table", schema_table.as_str()),
        ("trait_data_table", trait_table.as_str()),
        ("samples_card", samples.as_str()),
        ("deletion_run_row", deletion_row.as_str()),
        ("deletion_index", deletion_index.as_str()),
        ("iter_run_row", iter_row.as_str()),
        ("connection_row", connection_row.as_str()),
    ] {
        assert!(
            src.contains("valence_backend::valence_")
                || src.contains("valence_schema_path")
                || src.contains("valence_entity_path")
                || src.contains("valence_iter_run_path")
                || src.contains("valence_deletion_run_path")
                || src.contains("valence_trait_path"),
            "{label} must build detail hrefs via valence_backend path helpers"
        );
        assert!(
            !src.contains("crate::paths::schema(")
                && !src.contains("crate::paths::schema_id(")
                && !src.contains("crate::paths::schema_iter(")
                && !src.contains("crate::paths::r#trait(")
                && !src.contains("format!(\"/valence/schema/{}/deletion/{}"),
            "{label} must not interpolate raw ids into orbital paths::* or format! hrefs"
        );
    }
}

#[test]
fn ops_path_helpers_drop_encoding_sad_path() {
    let schema_table = read_app("pages/schema_index/components/schema_table/schema_data_table.rs");
    assert!(
        schema_table.contains("valence_backend::valence_schema_path"),
        "dropping valence_schema_path reopens path-segment smuggling via schema names"
    );
    let deletion_row = read_app("pages/schema/components/deletion_run_row.rs");
    assert!(
        deletion_row.contains("valence_backend::valence_deletion_run_path"),
        "dropping valence_deletion_run_path reopens path-segment smuggling via deletion run ids"
    );
    let samples = read_app("pages/schema/components/samples_card.rs");
    assert!(
        samples.contains("valence_backend::valence_entity_path"),
        "dropping valence_entity_path reopens path-segment smuggling via entity ids"
    );
}

#[test]
fn entity_privacy_binds_request_actor_happy_path() {
    let entities = read_app("server/entities.rs");
    assert!(
        entities.contains("get_entity_privacy_evaluation")
            && entities.contains("viewer_valence.actor()")
            && !entities.contains("viewer_id"),
        "privacy evaluation must bind the request actor only (no client-supplied viewer_id)"
    );
    let mod_rs = read_app("server/mod.rs");
    assert!(
        mod_rs.contains("request actor only") && mod_rs.contains("viewer Valence"),
        "server security map must document viewer Valence + request-actor privacy eval"
    );
}

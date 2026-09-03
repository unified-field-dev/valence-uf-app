//! Protected `/valence` host: session auth gate + in-memory schema-index happy path.
//!
//! Copy surfaces for product hosts: this package's `Cargo.toml` + `main.rs`,
//! plus the product-mount dependency / Leptos sketches in the host README.
//! Oneshot path `/valence` matches Orbital app id/path `valence` / `/valence`
//! (see JSON `inventory`).
//!
//! Mirrors what a real host does before mounting [`valence_app::ValenceRoutes`]:
//! deny anonymous traffic under `/valence`, then serve a seeded schema list the UI
//! schema index builds via `valence-backend` validate/lookup helpers.
//!
//! ## When to use
//! Smoke the `/valence` auth + schema-index contract without a full Leptos SSR graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-valence-uf-app
//! cargo run -p protected-valence-host
//! ```
//!
//! ## Success
//! Stdout prints `protected_valence_host: OK — /valence deny/allow + schema index`.
//!
//! ## Look next
//! Mount `<ValenceRoutes />` in a product host; wire Valence runtime + product schemas.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;
use valence_backend::{
    find_schema_by_name, sort_schemas_by_name, validate_schema_name, SchemaListItem,
};

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

fn seeded_schemas() -> Vec<SchemaListItem> {
    let mut schemas = vec![
        SchemaListItem {
            name: "demo_widget".into(),
            databases: vec!["mem".into()],
            version: "1".into(),
            description: Some("lab schema".into()),
        },
        SchemaListItem {
            name: "demo_account".into(),
            databases: vec!["mem".into()],
            version: "1".into(),
            description: None,
        },
    ];
    sort_schemas_by_name(&mut schemas);
    schemas
}

async fn valence_schemas(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    let schemas = seeded_schemas();
    validate_schema_name("demo_account").expect("seeded name");
    let found = find_schema_by_name(&schemas, "demo_account").expect("sorted lookup");
    Json(serde_json::json!({
        "path": "/valence",
        "user": session.user_id,
        "schema_count": schemas.len(),
        "first": found.name,
        "schemas": schemas,
        "inventory": {
            "app_id": "valence",
            "route_path": "/valence",
            "auth_gate": "RequireAuthenticated",
            "admin_permission": "ValenceAdmin",
        },
    }))
}

fn app() -> Router {
    Router::new()
        .route("/valence", get(valence_schemas))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let denied = status_for("/valence", None).await;
    assert_eq!(denied, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/valence")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/valence");
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["schema_count"], 2);
    assert_eq!(body["first"], "demo_account");
    assert_eq!(body["inventory"]["app_id"], "valence");
    assert_eq!(body["inventory"]["route_path"], "/valence");
    assert_eq!(body["inventory"]["auth_gate"], "RequireAuthenticated");
    assert_eq!(body["inventory"]["admin_permission"], "ValenceAdmin");

    println!("protected_valence_host: OK — /valence deny/allow + schema index");
}

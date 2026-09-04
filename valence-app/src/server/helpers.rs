//! Session and actor helpers for SSR server functions.

use leptos::prelude::ServerFnError;
use valence_backend::ValenceIdError;

/// Stable `auth:` prefix for missing session or viewer Valence build failures.
pub(crate) fn auth_error(detail: impl Into<String>) -> ServerFnError {
    ServerFnError::new(format!("auth: {}", detail.into()))
}

/// Stable `validation:` prefix for [`ValenceIdError`] and related input checks.
pub(crate) fn validation_error(err: ValenceIdError) -> ServerFnError {
    ServerFnError::new(format!("validation: {err}"))
}

/// Stable `validation:` prefix for ad hoc validation messages.
pub(crate) fn validation_message(detail: impl Into<String>) -> ServerFnError {
    ServerFnError::new(format!("validation: {}", detail.into()))
}

/// Stable `not_found:` prefix for missing registry or Valence rows after validation.
pub(crate) fn not_found_error(detail: impl Into<String>) -> ServerFnError {
    ServerFnError::new(format!("not_found: {}", detail.into()))
}

/// Stable `permission:` prefix for Gauge / Spectra permission denials surfaced here.
pub(crate) fn permission_error(detail: impl Into<String>) -> ServerFnError {
    ServerFnError::new(format!("permission: {}", detail.into()))
}

/// Stable `io:` prefix for Valence/registry IO failures.
pub(crate) fn io_error(detail: impl Into<String>) -> ServerFnError {
    ServerFnError::new(format!("io: {}", detail.into()))
}

pub(crate) fn require_session(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(auth_error("Authentication required"))
    }
}

/// Request-scoped Valence for the active Higgs actor (same pattern as other uf-apps).
pub(crate) async fn viewer_valence() -> Result<valence::Valence, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    ctx.valence()
        .map_err(|e| auth_error(format!("Failed to build Valence: {e}")))
}

/// Ensures the request has an authenticated session before registry-only reads.
pub(crate) async fn require_authenticated_session() -> Result<(), ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)
}

pub(crate) fn actor_label(actor: &valence::Actor) -> String {
    match actor {
        valence::Actor::User { user_id } => format!("user:{user_id}"),
        valence::Actor::ServiceUser { service_name } => format!("service:{service_name}"),
        valence::Actor::System { operation } => format!("system:{operation}"),
        valence::Actor::Anonymous => "anonymous".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_prefixes_are_stable() {
        assert!(auth_error("Authentication required")
            .to_string()
            .contains("auth: Authentication required"));
        assert!(validation_error(ValenceIdError::EmptySchemaName)
            .to_string()
            .contains("validation: Schema name is required"));
        assert!(not_found_error("Schema not found: counter")
            .to_string()
            .contains("not_found: Schema not found: counter"));
        assert!(permission_error("ValenceAdmin required")
            .to_string()
            .contains("permission: ValenceAdmin required"));
        assert!(io_error("registry unavailable")
            .to_string()
            .contains("io: registry unavailable"));
    }
}

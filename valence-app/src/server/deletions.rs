//! Deletion run list/detail/cancel server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};
#[cfg(feature = "ssr")]
use valence::SchemaRegistry;
#[cfg(feature = "ssr")]
use valence::TraitRegistry;
use valence_backend::{apply_schema_page_query, apply_trait_page_query};
#[cfg(feature = "ssr")]
use valence_backend::{
    clamp_deletion_list_limit, deletion_run_view_from_value, normalize_entity_id_for_lookup,
    schema_has_iter, validate_entity_id, validate_iter_name, validate_run_id, validate_schema_name,
    validate_trait_name,
};

#[cfg(feature = "ssr")]
use super::helpers::require_session;
#[cfg(feature = "ssr")]
use super::registry::resolve_schema_table_name;
use super::types::*;
use valence_backend::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

#[uf_product_macros::server]
pub(crate) async fn list_deletion_runs_for_schema(
    schema_name: String,
) -> Result<Vec<DeletionRunView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::deletion::DeletionService;

        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        let Some(table) = resolve_schema_table_name(&schema_name) else {
            return Ok(Vec::new());
        };
        let v = super::helpers::viewer_valence().await?;
        let rows = DeletionService::list_runs_for_schema(table.as_str(), &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| deletion_run_view_from_value(&row))
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = schema_name;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn list_deletion_runs(limit: u32) -> Result<Vec<DeletionRunView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::deletion::DeletionService;

        let v = super::helpers::viewer_valence().await?;
        let rows = DeletionService::list_runs_recent(clamp_deletion_list_limit(limit), &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(rows
            .into_iter()
            .filter_map(|row| deletion_run_view_from_value(&row))
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = limit;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn get_deletion_run(run_id: String) -> Result<Option<DeletionRunView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::deletion::DeletionService;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let row = DeletionService::get_run_json(&run_id, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(row.as_ref().and_then(deletion_run_view_from_value))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = run_id;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn list_deletion_run_steps(
    run_id: String,
) -> Result<Vec<DeletionStepView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::query::{QueryCore, SortDirection, StringPredicate};
        use valence_platform::ValenceDeletionStep;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let mut rows: Vec<ValenceDeletionStep> =
            QueryCore::new("valence_deletion_step".to_string())
                .where_string(
                    "run_id".to_string(),
                    StringPredicate::Equals(run_id.clone()),
                )
                .order_by("depth".to_string(), SortDirection::Desc)
                .limit(500)
                .execute(&v)
                .await
                .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        rows.sort_by(|a, b| {
            let da = *a.depth();
            let db = *b.depth();
            db.cmp(&da).then_with(|| a.record_id().cmp(b.record_id()))
        });
        Ok(rows
            .into_iter()
            .filter_map(|s| {
                let step_id = s.id()?.id().to_string();
                Some(DeletionStepView {
                    step_id,
                    record_table: s.record_table().clone(),
                    record_id: s.record_id().clone(),
                    action: s.action().to_string(),
                    depth: *s.depth(),
                    status: s.status().to_string(),
                })
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = run_id;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server(permission = "ValenceAdmin")]
pub async fn cancel_deletion_run(run_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use higgs::Higgs;
        use valence::deletion::DeletionService;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let ctx = Higgs::from_request().await?;
        require_session(&ctx)?;
        let v = ctx
            .valence()
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        DeletionService::merge_run(&run_id, serde_json::json!({ "status": "cancelled" }), &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = run_id;
        unreachable!("Server functions require SSR feature")
    }
}

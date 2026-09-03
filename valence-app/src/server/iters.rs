//! Iter registration, run, and cancel server functions.

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
use super::registry::{get_schema_metadata_by_name, resolve_table_for_schema};
use super::types::*;
use valence_backend::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

#[uf_product_macros::server]
pub async fn get_schema_iters(schema_name: String) -> Result<Vec<IterInfo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::iter_descriptors_for_table;
        super::helpers::require_authenticated_session().await?;
        let table = resolve_table_for_schema(&schema_name)?;
        let descs = iter_descriptors_for_table(&table);
        Ok(descs
            .into_iter()
            .map(|d| IterInfo {
                name: d.iter_type_name.to_string(),
                description: String::new(),
                table_name: d.table_name.to_string(),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = schema_name;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub(crate) async fn list_recent_iter_runs_for_schema(
    schema_name: String,
    limit: u32,
) -> Result<Vec<IterRunSummary>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::SortDirection;
        use valence::StringPredicate;
        use valence_platform::ValenceIterRun;

        let table = resolve_table_for_schema(&schema_name)?;
        let v = super::helpers::viewer_valence().await?;
        let rows = ValenceIterRun::query(&v)
            .where_target_table(StringPredicate::Equals(table))
            .order_by_created_at(SortDirection::Desc)
            .limit(limit)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(rows
            .into_iter()
            .map(|run| IterRunSummary {
                run_id: run.id().map(|r| r.to_string()).unwrap_or_default(),
                iter_name: run.iter_name().clone(),
                target_table: run.target_table().clone(),
                status: run.status().to_string(),
                created_at: run.created_at().to_rfc3339(),
                total_rows: *run.total_rows(),
                processed_rows: *run.processed_rows(),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, limit);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn list_iter_runs(
    offset: u32,
    limit: u32,
) -> Result<Page<IterRunSummary>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::SortDirection;
        use valence_platform::ValenceIterRun;

        let v = super::helpers::viewer_valence().await?;
        let mut rows = ValenceIterRun::query(&v)
            .order_by_created_at(SortDirection::Desc)
            .offset(offset)
            .limit(limit + 1)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        let page = Page::from_oversized(
            rows.drain(..)
                .map(|run| IterRunSummary {
                    run_id: run.id().map(|r| r.to_string()).unwrap_or_default(),
                    iter_name: run.iter_name().clone(),
                    target_table: run.target_table().clone(),
                    status: run.status().to_string(),
                    created_at: run.created_at().to_rfc3339(),
                    total_rows: *run.total_rows(),
                    processed_rows: *run.processed_rows(),
                })
                .collect(),
            limit,
            if offset == 0 { Some(0) } else { None },
        );
        Ok(page)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (offset, limit);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server(permission = "ValenceAdmin")]
pub(crate) async fn start_iter_run(
    schema_name: String,
    iter_name: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use chrono::Utc;
        use higgs::Higgs;
        use valence::Model;
        use valence_platform::{ValenceIterRun, ValenceIterRunStatus};

        validate_iter_name(&iter_name).map_err(super::helpers::validation_error)?;
        let ctx = Higgs::from_request().await?;
        require_session(&ctx)?;
        let v = ctx
            .valence()
            .map_err(|e| super::helpers::auth_error(e.to_string()))?;
        let table = resolve_table_for_schema(&schema_name)?;

        let schema = get_schema_metadata_by_name(&schema_name).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;
        if !schema_has_iter(&schema.iters, &iter_name) {
            return Err(super::helpers::validation_message(format!(
                "Iter {iter_name} not registered on schema {schema_name}"
            )));
        }

        let viewer = ctx
            .valence()
            .map_err(|e| super::helpers::auth_error(format!("viewer valence: {e}")))?;
        let initiated_by =
            serde_json::to_string(viewer.actor()).unwrap_or_else(|_| "\"unknown\"".to_string());

        let run_id = uuid::Uuid::new_v4().to_string();
        let row = ValenceIterRun::new(
            iter_name,
            table,
            ValenceIterRunStatus::Pending,
            0,
            0,
            0,
            0,
            0,
            None,
            None,
            None,
            Utc::now(),
            initiated_by,
            None,
        )
        .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        ValenceIterRun::upsert(&run_id, row, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        // Chronon orchestration is composed by deployment shells (Wave 7b), not standalone uf-app.
        let _ = ctx;
        return Err(super::helpers::validation_message(
            "Iter orchestration requires Chronon host wiring (deployment template Wave 7b)",
        ));
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, iter_name);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub(crate) async fn evaluate_iter_for_entity(
    schema_name: String,
    entity_id: String,
) -> Result<Vec<IterEntityEvaluation>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::find_iter_descriptor;
        use valence::query::QueryCore;

        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let table = resolve_table_for_schema(&schema_name)?;
        let entity_id = normalize_entity_id_for_lookup(entity_id);
        let row = QueryCore::get_record_json(table.clone(), &entity_id, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?
            .ok_or_else(|| super::helpers::not_found_error("Entity not found"))?;

        let schema = get_schema_metadata_by_name(&schema_name).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;

        let mut out = Vec::new();
        for iter_name in &schema.iters {
            let Some(desc) = find_iter_descriptor(&table, iter_name) else {
                continue;
            };
            match (desc.should_run)(v.clone(), row.clone()).await {
                Ok(ev) => out.push(IterEntityEvaluation {
                    iter_name: iter_name.clone(),
                    should_run: ev.should_run,
                    reason: ev.reason,
                }),
                Err(e) => out.push(IterEntityEvaluation {
                    iter_name: iter_name.clone(),
                    should_run: false,
                    reason: format!("should_run error: {}", e),
                }),
            }
        }
        Ok(out)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server(permission = "ValenceAdmin")]
pub async fn run_iter_on_entity(
    schema_name: String,
    entity_id: String,
    iter_name: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use chrono::Utc;
        use higgs::Higgs;
        use valence::Model;
        use valence_platform::iter::row_worker::ValenceIterRowWorker;
        use valence_platform::iter::row_worker::ValenceIterRowWorkerParams;
        use valence_platform::{ValenceIterRun, ValenceIterRunStatus};

        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;
        validate_iter_name(&iter_name).map_err(super::helpers::validation_error)?;
        let ctx = Higgs::from_request().await?;
        require_session(&ctx)?;
        let v_sys = ctx
            .valence()
            .map_err(|e| super::helpers::auth_error(e.to_string()))?;
        let table = resolve_table_for_schema(&schema_name)?;
        let entity_id = normalize_entity_id_for_lookup(entity_id);

        let schema = get_schema_metadata_by_name(&schema_name).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;
        if !schema_has_iter(&schema.iters, &iter_name) {
            return Err(super::helpers::validation_message(
                "Iter not registered on schema",
            ));
        }

        let viewer = ctx
            .valence()
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        let initiated_by =
            serde_json::to_string(viewer.actor()).unwrap_or_else(|_| "\"unknown\"".to_string());

        let run_id = uuid::Uuid::new_v4().to_string();
        let row = ValenceIterRun::new(
            iter_name.clone(),
            table.clone(),
            ValenceIterRunStatus::Pending,
            1,
            0,
            0,
            0,
            0,
            None,
            None,
            None,
            Utc::now(),
            initiated_by,
            Some(entity_id.clone()),
        )
        .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        ValenceIterRun::upsert(&run_id, row, &v_sys)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        let batch_id = uuid::Uuid::new_v4().to_string();
        let actor_json = serde_json::to_value(viewer.actor())
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        ValenceIterRowWorker::send_with(
            actor_json,
            ValenceIterRowWorkerParams {
                run_id: run_id.clone(),
                batch_id,
                row_id: entity_id,
                iter_name,
                table_name: table,
            },
        )
        .await
        .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        Ok(run_id)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id, iter_name);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn get_iter_run(run_id: String) -> Result<Option<IterRunView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::Model;
        use valence_platform::ValenceIterRun;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let r = ValenceIterRun::get(&run_id, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok(r.map(|run| IterRunView {
            run_id: run_id.clone(),
            iter_name: run.iter_name().clone(),
            target_table: run.target_table().clone(),
            status: run.status().to_string(),
            total_rows: *run.total_rows(),
            scanned_rows: *run.scanned_rows(),
            processed_rows: *run.processed_rows(),
            skipped_rows: *run.skipped_rows(),
            failed_rows: *run.failed_rows(),
            created_at: run.created_at().to_rfc3339(),
            error_message: run.error_message().cloned(),
            target_row_id: run.target_row_id().cloned(),
        }))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = run_id;
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn list_iter_run_errors(
    run_id: String,
    offset: u32,
    limit: u32,
) -> Result<Page<IterRowErrorView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::SortDirection;
        use valence::StringPredicate;
        use valence_platform::ValenceIterRowError;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let mut rows = ValenceIterRowError::query(&v)
            .where_run_id(StringPredicate::Equals(run_id))
            .order_by_created_at(SortDirection::Desc)
            .offset(offset)
            .limit(limit + 1)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        let page = Page::from_oversized(
            rows.drain(..)
                .map(|e| IterRowErrorView {
                    id: e.id().map(|id| id.to_string()).unwrap_or_default(),
                    row_id: e.row_id().clone(),
                    error_kind: e.error_kind().to_string(),
                    error_message: e.error_message().clone(),
                    created_at: e.created_at().to_rfc3339(),
                })
                .collect(),
            limit,
            if offset == 0 { Some(0) } else { None },
        );
        Ok(page)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (run_id, offset, limit);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server]
pub async fn list_iter_run_batches(
    run_id: String,
    offset: u32,
    limit: u32,
) -> Result<Page<IterBatchView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::SortDirection;
        use valence::StringPredicate;
        use valence_platform::ValenceIterBatch;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let v = super::helpers::viewer_valence().await?;
        let mut rows = ValenceIterBatch::query(&v)
            .where_run_id(StringPredicate::Equals(run_id))
            .order_by_batch_index(SortDirection::Asc)
            .offset(offset)
            .limit(limit + 1)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;

        let page = Page::from_oversized(
            rows.drain(..)
                .map(|b| IterBatchView {
                    id: b.id().map(|id| id.to_string()).unwrap_or_default(),
                    batch_index: *b.batch_index(),
                    status: b.status().to_string(),
                    row_count: *b.row_count(),
                    processed: *b.processed(),
                    skipped: *b.skipped(),
                    failed: *b.failed(),
                })
                .collect(),
            limit,
            if offset == 0 { Some(0) } else { None },
        );
        Ok(page)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (run_id, offset, limit);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server(permission = "ValenceAdmin")]
pub async fn cancel_iter_run(run_id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use higgs::Higgs;
        use valence::Model;
        use valence_platform::ValenceIterRun;

        validate_run_id(&run_id).map_err(super::helpers::validation_error)?;
        let ctx = Higgs::from_request().await?;
        require_session(&ctx)?;
        let v = ctx
            .valence()
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        ValenceIterRun::merge(&run_id, serde_json::json!({ "status": "cancelled" }), &v)
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

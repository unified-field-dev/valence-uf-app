//! Entity view, privacy evaluation, ownership, and delete-queue server functions.

use leptos::prelude::*;
use orbital_paging::{Page, PageRequest};
#[cfg(feature = "ssr")]
use std::collections::BTreeMap;
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
use super::privacy::build_entity_privacy_eval_card_data;
#[cfg(feature = "ssr")]
use super::registry::{
    build_owner_from_ownership, get_schema_metadata_by_name, resolve_schema_table_name,
    transfer_row_from_value,
};
use super::types::*;
use valence_backend::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

#[uf_product_macros::server]
pub async fn get_entity_privacy_evaluation(
    schema_name: String,
    entity_id: String,
) -> Result<EntityPrivacyEvalCardData, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::query::QueryCore;

        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        let entity_id = normalize_entity_id_for_lookup(entity_id);
        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;

        let registry = SchemaRegistry::global();
        let key = resolve_schema_table_name(&schema_name).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;
        let schema = registry.get_full_schema(key.as_str()).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;

        let viewer_valence = super::helpers::viewer_valence().await?;

        let entity = QueryCore::get_entity(schema.name.clone(), &entity_id, &viewer_valence)
            .await
            .map_err(|e| super::helpers::io_error(format!("Failed to query record: {e}")))?
            .ok_or_else(|| {
                super::helpers::not_found_error(format!(
                    "Record not found: {schema_name}/{entity_id}"
                ))
            })?;

        let record = serde_json::Value::Object(entity.data.into_iter().collect());
        let viewer_actor = viewer_valence.actor().clone();

        Ok(build_entity_privacy_eval_card_data(
            schema,
            &record,
            &viewer_actor,
        ))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id);
        unreachable!("Server functions require SSR feature")
    }
}

/// Get an entity view (schema + record + related panels) by schema name + id.
///
/// Loads under **viewer Valence** with an authenticated session so schema field
/// privacy applies; denied fields are redacted rather than failing the page.
#[uf_product_macros::server]
pub async fn get_entity_view(
    schema_name: String,
    entity_id: String,
) -> Result<Option<EntityView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::privacy::PrivacyEvaluator;
        use valence::query::QueryCore;

        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        let entity_id = normalize_entity_id_for_lookup(entity_id);
        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;

        let schema = match get_schema_metadata_by_name(&schema_name) {
            Some(s) => s,
            None => return Ok(None),
        };

        let registry = SchemaRegistry::global();
        let table_key = schema.name.as_str();
        if registry.get_schema(table_key).is_none() {
            return Err(super::helpers::not_found_error(format!(
                "Schema metadata not found: {schema_name}"
            )));
        }

        let viewer_v = super::helpers::viewer_valence().await?;

        // Existence check (id-only) then privacy-aware get. On entity-level deny,
        // keep the page loadable with primary-key fields only.
        let exists = QueryCore::get_id_only(table_key, &entity_id, &viewer_v)
            .await
            .map_err(|e| super::helpers::io_error(format!("Failed to query record: {e}")))?;
        if exists.is_none() {
            return Ok(None);
        }

        let (filtered_data, hidden_fields) =
            match QueryCore::get_entity(table_key, &entity_id, &viewer_v).await {
                Ok(Some(entity)) => (entity.data, entity.hidden_fields),
                Ok(None) => return Ok(None),
                Err(_) => {
                    let mut filtered = BTreeMap::new();
                    let mut hidden = Vec::new();
                    for field in &schema.fields {
                        if field.primary {
                            filtered.insert(
                                field.name.clone(),
                                serde_json::Value::String(entity_id.clone()),
                            );
                        } else {
                            hidden.push(field.name.clone());
                        }
                    }
                    (filtered, hidden)
                }
            };

        let record_values: BTreeMap<String, String> = filtered_data
            .iter()
            .map(|(k, v)| {
                let string_value = match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                    serde_json::Value::Array(arr) => arr
                        .iter()
                        .map(|item| item.as_str().unwrap_or(&item.to_string()).to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    serde_json::Value::Object(_) => serde_json::to_string(&v).unwrap_or_default(),
                };
                (k.clone(), string_value)
            })
            .collect();

        let record = EntityRecord {
            id: entity_id.clone(),
            values: record_values,
        };

        let owner = {
            use valence::ownership::OwnershipService;

            let rid = valence::ownership::normalize_record_id_for_ownership(&entity_id);
            let ownership_json = OwnershipService::get_ownership_json(table_key, &rid, &viewer_v)
                .await
                .ok()
                .flatten();

            if let Some(ref own) = ownership_json {
                let transfers_raw =
                    OwnershipService::transfer_history(table_key, &rid, &viewer_v, 25)
                        .await
                        .unwrap_or_default();
                let transfers = transfers_raw
                    .into_iter()
                    .filter_map(transfer_row_from_value)
                    .collect::<Vec<_>>();
                build_owner_from_ownership(own, transfers, &viewer_v).await
            } else {
                super::types::Owner {
                    id: "unknown".to_string(),
                    name: "Unknown".to_string(),
                    role: "—".to_string(),
                    email: "—".to_string(),
                    handle: "—".to_string(),
                    owner_kind: "unknown".to_string(),
                    ownership_status: "—".to_string(),
                    owner_entity_path: None,
                    transfers: Vec::new(),
                }
            }
        };

        let deletions: Vec<DeletionRequest> = {
            use valence::deletion::DeletionService;

            let rid = valence::ownership::normalize_record_id_for_ownership(&entity_id);
            let rows = DeletionService::list_runs_for_record(table_key, &rid, &viewer_v)
                .await
                .unwrap_or_default();
            rows.into_iter()
                .filter_map(|row| {
                    let id = row.get("id").and_then(|x| x.as_str())?.to_string();
                    let status = row
                        .get("status")
                        .and_then(|x| x.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let requested_at = row
                        .get("requested_at")
                        .map(|x| x.to_string())
                        .unwrap_or_default();
                    Some(DeletionRequest {
                        id,
                        status,
                        requested_at,
                    })
                })
                .collect()
        };

        let inverse_connections = {
            use valence::query::{RecordPredicate, SortDirection};

            let target_rid = valence::RecordId::new(table_key, entity_id.as_str());
            let mut results = Vec::new();

            for inv in &schema.inverse_connections {
                let query = QueryCore::new(inv.from_table.clone())
                    .select(vec!["id".to_string()])
                    .where_record(
                        inv.from_field.clone(),
                        RecordPredicate::Equals(target_rid.clone()),
                    )
                    .order_by("id".to_string(), SortDirection::Desc)
                    .limit(10);

                let ids: Vec<valence::IdOnlyRecord> =
                    query.execute(&viewer_v).await.unwrap_or_default();

                let privacy_restricted =
                    if let Some(ref_meta) = registry.get_schema(&inv.from_table) {
                        PrivacyEvaluator::check_entity_read(
                            ref_meta,
                            &serde_json::Value::Object(serde_json::Map::new()),
                            &viewer_v,
                        )
                        .await
                        .is_err()
                    } else {
                        false
                    };

                results.push(InverseConnectionData {
                    from_table: inv.from_table.clone(),
                    from_field: inv.from_field.clone(),
                    label: inv.label.clone(),
                    referencing_ids: ids.into_iter().map(|r| r.id).collect(),
                    privacy_restricted,
                });
            }

            results
        };

        Ok(Some(EntityView {
            schema,
            record,
            owner,
            deletions,
            hidden_fields,
            inverse_connections,
        }))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id);
        unreachable!("Server functions require SSR feature")
    }
}

/// Recent ownership transfers for a row (same source as the entity page transfer list).
#[uf_product_macros::server]
pub async fn get_entity_ownership_transfers(
    schema_name: String,
    entity_id: String,
) -> Result<Vec<OwnershipTransferRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::ownership::OwnershipService;

        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        let entity_id = normalize_entity_id_for_lookup(entity_id);
        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;
        let Some(key) = resolve_schema_table_name(&schema_name) else {
            return Ok(Vec::new());
        };
        let viewer_v = super::helpers::viewer_valence().await?;
        let rid = valence::ownership::normalize_record_id_for_ownership(&entity_id);
        let rows = OwnershipService::transfer_history(key.as_str(), &rid, &viewer_v, 50)
            .await
            .map_err(|e| super::helpers::io_error(format!("Failed to load transfers: {e}")))?;
        Ok(rows
            .into_iter()
            .filter_map(transfer_row_from_value)
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id);
        unreachable!("Server functions require SSR feature")
    }
}

#[uf_product_macros::server(permission = "ValenceAdmin")]
pub async fn delete_entity_queue(
    schema_name: String,
    entity_id: String,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use higgs::Higgs;

        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        validate_entity_id(&entity_id).map_err(super::helpers::validation_error)?;
        let ctx = Higgs::from_request().await?;
        require_session(&ctx)?;
        let v = ctx
            .valence()
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        let Some(table) = resolve_schema_table_name(&schema_name) else {
            return Err(super::helpers::not_found_error("Unknown schema"));
        };
        let eid = normalize_entity_id_for_lookup(entity_id);
        valence::admin_entity_delete::queue_delete_entity(table.as_str(), &eid, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("{e}")))?;
        Ok("Deletion queued".into())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (schema_name, entity_id);
        unreachable!("Server functions require SSR feature")
    }
}

//! Schema discovery, samples, and schema-or-id search server functions.

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
use super::privacy::build_schema_privacy_card_data;
#[cfg(feature = "ssr")]
use super::registry::{get_schema_metadata_by_name, resolve_schema_table_name};
use super::types::*;
use valence_backend::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

/// Get all schemas (returns simplified list items for the index page)
/// Uses lightweight metadata only - no full schema parsing required
#[uf_product_macros::server]
pub async fn get_schemas() -> Result<Vec<SchemaListItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        let registry = SchemaRegistry::global();
        let schema_names = registry.list_schemas();

        let mut schemas = Vec::new();
        for name in schema_names {
            if let Some(metadata) = registry.get_schema(name) {
                schemas.push(SchemaListItem {
                    name: metadata.table_name.to_string(),
                    databases: metadata.databases.to_vec(),
                    version: metadata.version.to_string(),
                    description: metadata.description.map(str::to_string),
                });
            }
        }
        Ok(schemas)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server functions require SSR feature")
    }
}

/// Paginated schemas endpoint.
///
/// Returns a [`Page<SchemaListItem>`] using the standard `orbital-paging`
/// over-fetch pattern. Schemas come from the in-memory registry so we
/// fetch all, sort, then slice. Quick search and structured filters from
/// [`PageRequest`] are applied server-side before pagination.
#[uf_product_macros::server]
pub async fn get_schemas_page(request: PageRequest) -> Result<Page<SchemaListItem>, ServerFnError> {
    let mut all = get_schemas().await?;
    all.sort_by(|a, b| a.name.cmp(&b.name));
    apply_schema_page_query(&mut all, &request);

    let total_count: Option<u64> = if request.is_first_page() {
        Some(all.len() as u64)
    } else {
        None
    };

    let sliced: Vec<SchemaListItem> = all
        .into_iter()
        .skip(request.offset as usize)
        .take((request.limit + 1) as usize)
        .collect();

    Ok(Page::from_oversized(sliced, request.limit, total_count))
}

/// Get a single schema by name
#[uf_product_macros::server]
pub async fn get_schema(schema_name: String) -> Result<Option<Schema>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        Ok(get_schema_metadata_by_name(&schema_name))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = schema_name;
        unreachable!("Server functions require SSR feature")
    }
}

/// Get schema privacy policies (mock data for now)
#[uf_product_macros::server]
pub async fn get_schema_privacy_policies(
    schema_name: String,
) -> Result<SchemaPrivacyCardData, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        let registry = SchemaRegistry::global();
        let key = resolve_schema_table_name(&schema_name).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;
        let schema = registry.get_full_schema(key.as_str()).ok_or_else(|| {
            super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
        })?;

        Ok(build_schema_privacy_card_data(schema))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = schema_name;
        unreachable!("Server functions require SSR feature")
    }
}

/// Get entity privacy evaluation for the **request actor only**.
///
/// Record load and evaluation both use viewer Valence; clients cannot supply a

#[uf_product_macros::server]
pub async fn get_schema_samples(
    schema_name: String,
    limit: u32,
) -> Result<Vec<SampleRecord>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use valence::query::QueryCore;

        super::helpers::require_authenticated_session().await?;
        let registry = SchemaRegistry::global();
        let Some(key) = resolve_schema_table_name(&schema_name) else {
            return Ok(Vec::new());
        };
        let table_name = if let Some(metadata) = registry.get_schema(key.as_str()) {
            metadata.table_name
        } else {
            return Ok(Vec::new());
        };

        let v = super::helpers::viewer_valence().await?;

        if table_name == "counter" {
            let mut samples: Vec<SampleRecord> = QueryCore::latest_ids("counter", limit, &v)
                .await
                .map_err(|e| {
                    super::helpers::io_error(format!("Failed to query counter samples: {e}"))
                })?
                .into_iter()
                .map(|r| SampleRecord { id: r.id })
                .collect();
            if samples.is_empty()
                && QueryCore::get_entity("counter", "singleton", &v)
                    .await
                    .map_err(|e| super::helpers::io_error(format!("Counter sample fallback: {e}")))?
                    .is_some()
            {
                samples.push(SampleRecord {
                    id: "singleton".into(),
                });
            }
            return Ok(samples);
        }

        let ids = QueryCore::latest_ids(table_name, limit, &v)
            .await
            .map_err(|e| super::helpers::io_error(format!("Failed to query samples: {e}")))?;

        let samples: Vec<SampleRecord> = ids
            .into_iter()
            .map(|id_record| SampleRecord { id: id_record.id })
            .collect();

        Ok(samples)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Ok(Vec::new())
    }
}

/// Paginated traits endpoint.
///
/// Returns a [`Page<TraitListItem>`] using the standard `orbital-paging`
/// over-fetch pattern. Quick search and structured filters from

#[uf_product_macros::server]
#[server(SearchSchemaOrId)]
pub async fn search_schema_or_id(query: String) -> Result<(), ServerFnError> {
    use leptos_axum::redirect;
    use urlencoding::encode;
    use valence::query::QueryCore;

    let query = query.trim();

    if query.is_empty() {
        redirect(crate::paths::SCHEMA);
        return Ok(());
    }

    super::helpers::require_authenticated_session().await?;

    let registry = SchemaRegistry::global();

    // Check if it's an exact schema name match (case-insensitive)
    let schema_names = registry.list_schemas();
    for schema_name in &schema_names {
        if schema_name.eq_ignore_ascii_case(query) {
            redirect(&valence_backend::valence_schema_path(schema_name));
            return Ok(());
        }
    }

    // Check if input contains ':' (schema:id format)
    if let Some(colon_pos) = query.find(':') {
        let schema_input = &query[..colon_pos];
        let entity_id = &query[colon_pos + 1..];

        if let Some(ui_schema) = get_schema_metadata_by_name(schema_input) {
            let table_key = ui_schema.name.as_str();
            let v = super::helpers::viewer_valence().await?;

            if let Ok(Some(_record)) = QueryCore::get_entity(table_key, entity_id, &v).await {
                redirect(&valence_backend::valence_entity_path(table_key, entity_id));
                return Ok(());
            }
        }
    } else {
        // No colon - search across all schemas for the ID (viewer-scoped).
        let v = super::helpers::viewer_valence().await?;

        for schema_name in schema_names.iter().copied() {
            if let Ok(Some(_record)) = QueryCore::get_entity(schema_name, query, &v).await {
                redirect(&valence_backend::valence_entity_path(schema_name, query));
                return Ok(());
            }
        }
    }

    // No matches found - redirect to schema index with query prefilled
    redirect(&format!("{}?q={}", crate::paths::SCHEMA, encode(query)));
    Ok(())
}

// --- Valence Iter (Milestone 12) -------------------------------------------------

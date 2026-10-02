//! Server functions for declared data-use catalog rows.

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use valence::data_use::DataUseCatalog;
#[cfg(feature = "ssr")]
use valence::SchemaRegistry;
#[cfg(feature = "ssr")]
use valence::TraitRegistry;
#[cfg(feature = "ssr")]
use valence_backend::{validate_schema_name, validate_trait_name};

#[cfg(feature = "ssr")]
use super::data_use_rows::{installed_entries, schema_rows, trait_rows, unscoped_rows};
use super::types::DataUseRow;

/// Error code prefix returned by the data-use server functions when the host never
/// installed a [`DataUseCatalog`](valence::data_use::DataUseCatalog) at boot.
///
/// The Data uses panels match on it to show a wiring error instead of an empty list.
pub const DATA_USE_CATALOG_NOT_INSTALLED: &str = "data_use_catalog_not_installed";

/// Declared uses for a schema: Schema(S) ∪ Trait(T) where S implements T,
/// plus Referenced Reads/Updates whose peer resolves to S.
#[uf_product_macros::server]
pub async fn get_schema_data_uses(schema_name: String) -> Result<Vec<DataUseRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;
        let entries = installed_entries(DataUseCatalog::global(), "schema")?;

        let traits_for_schema: Vec<String> = {
            let reg = TraitRegistry::global();
            reg.list_traits()
                .into_iter()
                .filter(|trait_name| {
                    reg.tables_for_trait(trait_name)
                        .iter()
                        .any(|t| *t == schema_name)
                })
                .map(str::to_string)
                .collect()
        };
        let schema_repo = super::registry::get_schema_metadata_by_name(&schema_name)
            .map(|s| s.meta.repository)
            .unwrap_or_default();

        let rows = schema_rows(
            entries,
            &schema_name,
            &schema_repo,
            &traits_for_schema,
            |trait_name| {
                TraitRegistry::global()
                    .get_definition(trait_name)
                    .map(|d| d.repository.to_string())
            },
            resolve_peer_from_registry,
        );

        tracing::debug!(
            target = "schema",
            row_count = rows.len(),
            "get_schema_data_uses"
        );
        Ok(rows)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = schema_name;
        unreachable!("Server functions require SSR feature")
    }
}

/// Declared uses for a trait (Trait(T) only — no schema-only rows).
#[uf_product_macros::server]
pub async fn get_trait_data_uses(trait_name: String) -> Result<Vec<DataUseRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        validate_trait_name(&trait_name).map_err(super::helpers::validation_error)?;
        let entries = installed_entries(DataUseCatalog::global(), "trait")?;

        let repo = TraitRegistry::global()
            .get_definition(&trait_name)
            .map(|d| d.repository.to_string())
            .unwrap_or_default();
        let rows = trait_rows(entries, &trait_name, &repo);

        tracing::debug!(
            target = "trait",
            row_count = rows.len(),
            "get_trait_data_uses"
        );
        Ok(rows)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = trait_name;
        unreachable!("Server functions require SSR feature")
    }
}

/// Declared Unscoped uses (QueryCore / raw paths).
#[uf_product_macros::server]
pub async fn get_unscoped_data_uses() -> Result<Vec<DataUseRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        let entries = installed_entries(DataUseCatalog::global(), "unscoped")?;
        let rows = unscoped_rows(entries);

        tracing::debug!(
            target = "unscoped",
            row_count = rows.len(),
            "get_unscoped_data_uses"
        );
        Ok(rows)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server functions require SSR feature")
    }
}

#[cfg(feature = "ssr")]
fn resolve_peer_from_registry(from_table: &str, hop_field: &str) -> Option<String> {
    let registry = SchemaRegistry::global();
    let meta = registry.get_schema(from_table)?;
    let schema = meta.schema;
    if !schema.connections.is_empty() {
        for conn in &schema.connections {
            if hop_field_matches(hop_field, &conn.from_field) {
                return Some(conn.to_table.clone());
            }
        }
    } else {
        for edge in &schema.edges {
            if hop_field_matches(hop_field, &edge.from_field) {
                return Some(edge.to_table.clone());
            }
        }
    }
    None
}

#[cfg(feature = "ssr")]
fn hop_field_matches(hop_field: &str, from_field: &str) -> bool {
    if hop_field == from_field {
        return true;
    }
    let singular = from_field.strip_suffix('s').unwrap_or(from_field);
    hop_field == singular
}

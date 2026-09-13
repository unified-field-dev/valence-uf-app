//! Server functions for declared data-use catalog rows.

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use valence::TraitRegistry;
#[cfg(feature = "ssr")]
use valence_backend::{validate_schema_name, validate_trait_name};

use super::types::DataUseRow;

#[cfg(feature = "ssr")]
mod snapshot {
    include!(concat!(env!("OUT_DIR"), "/data_uses.rs"));
}

/// Declared uses for a schema: Schema(S) ∪ Trait(T) where S implements T.
#[uf_product_macros::server]
pub async fn get_schema_data_uses(schema_name: String) -> Result<Vec<DataUseRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        validate_schema_name(&schema_name).map_err(super::helpers::validation_error)?;

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

        let mut rows = Vec::new();
        for entry in snapshot::DATA_USES {
            match &entry.target {
                snapshot::DataUseTarget::Schema(name) if *name == schema_name => {
                    rows.push(to_row(entry, None, &schema_repo));
                }
                snapshot::DataUseTarget::Trait(trait_name)
                    if traits_for_schema.iter().any(|t| t == trait_name) =>
                {
                    let trait_repo = TraitRegistry::global()
                        .get_definition(trait_name)
                        .map(|d| d.repository.to_string())
                        .unwrap_or_else(|| schema_repo.clone());
                    rows.push(to_row(entry, Some((*trait_name).to_string()), &trait_repo));
                }
                _ => {}
            }
        }

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

        let repo = TraitRegistry::global()
            .get_definition(&trait_name)
            .map(|d| d.repository.to_string())
            .unwrap_or_default();
        let mut rows = Vec::new();
        for entry in snapshot::DATA_USES {
            if let snapshot::DataUseTarget::Trait(name) = &entry.target {
                if *name == trait_name {
                    rows.push(to_row(entry, None, &repo));
                }
            }
        }

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

        let mut rows = Vec::new();
        for entry in snapshot::DATA_USES {
            if matches!(entry.target, snapshot::DataUseTarget::Unscoped) {
                // Unscoped has no schema/trait owner — View source uses the
                // declaring package's Cargo.toml repository from the scan.
                rows.push(to_row(entry, None, entry.repository));
            }
        }

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
fn github_blob_url(repository: &str, path: &str, line: u32) -> String {
    if repository.trim().is_empty() {
        return String::new();
    }
    let repo = repository.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{repo}/blob/main/{path}#L{line}")
}

#[cfg(feature = "ssr")]
fn to_row(
    entry: &snapshot::DataUseEntry,
    via_trait: Option<String>,
    repository: &str,
) -> DataUseRow {
    let op = match entry.op {
        snapshot::DataOp::Read => "read",
        snapshot::DataOp::Create => "create",
        snapshot::DataOp::Update => "update",
        snapshot::DataOp::Delete => "delete",
    };
    DataUseRow {
        purpose: entry.purpose.to_string(),
        file: entry.file.to_string(),
        line: entry.line,
        crate_name: entry.crate_name.to_string(),
        op: op.to_string(),
        method: entry.method.to_string(),
        via_trait,
        source_url: github_blob_url(repository, entry.file, entry.line),
    }
}

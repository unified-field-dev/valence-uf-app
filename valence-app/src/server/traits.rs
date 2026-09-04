//! Trait list and detail server functions.

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
use super::registry::get_schema_metadata_by_name;
use super::types::*;
use valence_backend::{
    DeletionRunView, DeletionStepView, IterBatchView, IterEntityEvaluation, IterInfo,
    IterRowErrorView, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

#[uf_product_macros::server]
pub async fn get_traits_page(request: PageRequest) -> Result<Page<TraitListItem>, ServerFnError> {
    let mut all = get_traits().await?;
    all.sort_by(|a, b| a.name.cmp(&b.name));
    apply_trait_page_query(&mut all, &request);

    let total_count: Option<u64> = if request.is_first_page() {
        Some(all.len() as u64)
    } else {
        None
    };

    let sliced: Vec<TraitListItem> = all
        .into_iter()
        .skip(request.offset as usize)
        .take((request.limit + 1) as usize)
        .collect();

    Ok(Page::from_oversized(sliced, request.limit, total_count))
}

/// List all registered Valence traits (summary items for the index page).
#[uf_product_macros::server]
pub async fn get_traits() -> Result<Vec<TraitListItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        let reg = TraitRegistry::global();
        let mut items: Vec<TraitListItem> = reg
            .iter()
            .map(|def| TraitListItem {
                name: def.name.to_string(),
                version: String::new(),
                description: None,
            })
            .collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(items)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server functions require SSR feature")
    }
}

/// Get full detail for a single Valence trait by name.
#[uf_product_macros::server]
pub async fn get_trait(trait_name: String) -> Result<Option<TraitDetail>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        super::helpers::require_authenticated_session().await?;
        validate_trait_name(&trait_name).map_err(super::helpers::validation_error)?;
        let reg = TraitRegistry::global();
        let def = match reg.get_definition(&trait_name) {
            Some(d) => d,
            None => return Ok(None),
        };

        let fields: Vec<TraitFieldInfo> = def
            .fields
            .iter()
            .map(|f| TraitFieldInfo {
                name: f.name.to_string(),
                field_type: f.field_type.to_string(),
                required: f.required,
            })
            .collect();

        let implementors: Vec<String> = reg
            .tables_for_trait(&trait_name)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Derive connection details from the first implementing schema.
        let connections = if !implementors.is_empty() && !def.connection_names.is_empty() {
            let conn_name_set: std::collections::HashSet<&str> =
                def.connection_names.iter().copied().collect();
            get_schema_metadata_by_name(&implementors[0])
                .map(|s| {
                    s.connections
                        .into_iter()
                        .filter(|c| conn_name_set.contains(c.name.as_str()))
                        .collect()
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let policies = def.policies.and_then(|p| {
            p.read.map(|read| {
                let mut rows = Vec::new();
                for rule in read.always_block {
                    rows.push(PolicyDisplayRow {
                        action: "read".to_string(),
                        bucket: "always_block".to_string(),
                        policy: PolicyDisplay {
                            name: rule.name.clone(),
                            description: rule.description.clone(),
                            trait_source: None,
                        },
                    });
                }
                for rule in read.always_allow {
                    rows.push(PolicyDisplayRow {
                        action: "read".to_string(),
                        bucket: "always_allow".to_string(),
                        policy: PolicyDisplay {
                            name: rule.name.clone(),
                            description: rule.description.clone(),
                            trait_source: None,
                        },
                    });
                }
                for rule in read.block {
                    rows.push(PolicyDisplayRow {
                        action: "read".to_string(),
                        bucket: "block".to_string(),
                        policy: PolicyDisplay {
                            name: rule.name.clone(),
                            description: rule.description.clone(),
                            trait_source: None,
                        },
                    });
                }
                for rule in read.allow {
                    rows.push(PolicyDisplayRow {
                        action: "read".to_string(),
                        bucket: "allow".to_string(),
                        policy: PolicyDisplay {
                            name: rule.name.clone(),
                            description: rule.description.clone(),
                            trait_source: None,
                        },
                    });
                }
                SchemaPrivacyCardData { rows }
            })
        });

        Ok(Some(TraitDetail {
            name: trait_name,
            fields,
            connections,
            implementors,
            policies,
        }))
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = trait_name;
        unreachable!("Server functions require SSR feature")
    }
}

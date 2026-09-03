//! Schema registry / ownership helpers used by entity and schema server fns (SSR).

use super::privacy::compute_inverse_connections;
use super::types::*;
use leptos::prelude::ServerFnError;
use valence::SchemaRegistry;
use valence::TraitRegistry;
use valence_backend::validate_schema_name;

pub(crate) fn annotate_trait_sources(schema: &mut Schema) {
    use std::collections::HashSet;

    let trait_reg = TraitRegistry::global();
    for trait_name in schema.traits.clone() {
        if let Some(def) = trait_reg.get_definition(&trait_name) {
            let existing_field_names: HashSet<String> =
                schema.fields.iter().map(|f| f.name.clone()).collect();
            let existing_conn_names: HashSet<String> =
                schema.connections.iter().map(|c| c.name.clone()).collect();

            for field in &mut schema.fields {
                if def.fields.iter().any(|tf| tf.name == field.name) && field.trait_source.is_none()
                {
                    field.trait_source = Some(trait_name.clone());
                }
            }

            for tf in def.fields {
                if existing_field_names.contains(tf.name) {
                    continue;
                }
                let fk = extract_record_table(tf.field_type).map(|ref_table| ForeignKeyRef {
                    ref_table: ref_table.to_string(),
                    field: "id".to_string(),
                });
                schema.fields.push(SchemaField {
                    name: tf.name.to_string(),
                    field_type: tf.field_type.to_string(),
                    primary: false,
                    nullable: !tf.required,
                    indexed: false,
                    unique: false,
                    default: None,
                    fk,
                    trait_source: Some(trait_name.clone()),
                });
            }

            for conn in &mut schema.connections {
                if def.connection_names.contains(&conn.name.as_str()) && conn.trait_source.is_none()
                {
                    conn.trait_source = Some(trait_name.clone());
                }
            }

            for &conn_name in def.connection_names {
                if existing_conn_names.contains(conn_name) {
                    continue;
                }
                if let Some(tf) = def.fields.iter().find(|f| f.name == conn_name) {
                    if let Some(ref_table) = extract_record_table(tf.field_type) {
                        let label = conn_name
                            .split('_')
                            .map(|w| {
                                let mut c = w.chars();
                                match c.next() {
                                    Some(first) => {
                                        first.to_uppercase().collect::<String>() + c.as_str()
                                    }
                                    None => String::new(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        schema.connections.push(SchemaConnection {
                            name: conn_name.to_string(),
                            from_table: schema.name.clone(),
                            from_field: conn_name.to_string(),
                            to_table: ref_table.to_string(),
                            cardinality: "HasOne".to_string(),
                            required: tf.required,
                            on_delete: "Cascade".to_string(),
                            label,
                            target_trait: None,
                            trait_source: Some(trait_name.clone()),
                        });
                    }
                }
            }
        }
    }
}

pub(crate) fn extract_record_table(field_type: &str) -> Option<&str> {
    if field_type.starts_with("record<") && field_type.ends_with('>') {
        return field_type
            .strip_prefix("record<")
            .and_then(|s| s.strip_suffix('>'));
    }
    if field_type.starts_with("record(") && field_type.ends_with(')') {
        return field_type
            .strip_prefix("record(")
            .and_then(|s| s.strip_suffix(')'));
    }
    None
}

pub(crate) fn json_scalar_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        _ => v.to_string(),
    }
}

pub(crate) fn transfer_row_from_value(v: serde_json::Value) -> Option<OwnershipTransferRow> {
    Some(OwnershipTransferRow {
        id: json_scalar_string(v.get("id")?),
        from_owner_id: v
            .get("from_owner_id")
            .map(json_scalar_string)
            .unwrap_or_default(),
        from_owner_type: v
            .get("from_owner_type")
            .map(json_scalar_string)
            .unwrap_or_default(),
        to_owner_id: v
            .get("to_owner_id")
            .map(json_scalar_string)
            .unwrap_or_default(),
        to_owner_type: v
            .get("to_owner_type")
            .map(json_scalar_string)
            .unwrap_or_default(),
        transferred_at: v
            .get("transferred_at")
            .map(json_scalar_string)
            .unwrap_or_default(),
        transferred_by: v
            .get("transferred_by")
            .map(json_scalar_string)
            .unwrap_or_default(),
        reason: v
            .get("reason")
            .and_then(|r| match r {
                serde_json::Value::Null => None,
                serde_json::Value::String(s) => Some(s.clone()),
                _ => Some(r.to_string()),
            })
            .filter(|s| !s.is_empty()),
    })
}

pub(crate) fn owner_entity_path_for_kind(
    registry: &SchemaRegistry,
    owner_kind: &str,
    owner_id: &str,
) -> Option<String> {
    let table = match owner_kind {
        "user" => "user",
        "application" => "application",
        "account" => "account",
        _ => return None,
    };
    if registry.get_schema(table).is_none() {
        return None;
    }
    Some(format!(
        "/valence/schema/{}/id/{}",
        table,
        urlencoding::encode(owner_id)
    ))
}

pub(crate) async fn build_owner_from_ownership(
    ownership: &serde_json::Value,
    transfers: Vec<OwnershipTransferRow>,
    viewer_v: &valence::Valence,
) -> Owner {
    use valence::query::QueryCore;
    use valence_backend::owner_contact_from_user_record;

    let owner_kind = ownership
        .get("owner_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let ownership_status = ownership
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let owner_id = ownership
        .get("owner_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let registry = SchemaRegistry::global();
    let owner_entity_path = owner_entity_path_for_kind(&registry, &owner_kind, &owner_id);

    let (name, email, handle) = if owner_kind == "user" {
        // Resolve under viewer Valence so user-field privacy (email) applies.
        let user = match QueryCore::get_entity("user", &owner_id, viewer_v).await {
            Ok(Some(entity)) => Some(serde_json::Value::Object(entity.data.into_iter().collect())),
            _ => None,
        };
        owner_contact_from_user_record(user.as_ref(), &owner_id)
    } else {
        let name = format!("{owner_kind}: {owner_id}");
        let email = "—".to_string();
        let handle = owner_id.clone();
        (name, email, handle)
    };

    let role = format!("{} · {}", owner_kind, ownership_status);

    Owner {
        id: owner_id.clone(),
        name,
        role,
        email,
        handle,
        owner_kind,
        ownership_status,
        owner_entity_path,
        transfers,
    }
}

/// Map URL or display input (e.g. `Counter`) to the registry's canonical table key (e.g. `counter`).
pub(crate) fn resolve_schema_table_name(name: &str) -> Option<String> {
    let registry = SchemaRegistry::global();
    let trimmed = name.trim();
    for n in registry.list_schemas() {
        if n.eq_ignore_ascii_case(trimmed) {
            return Some(n.to_string());
        }
    }
    None
}

/// Helper function to get full schema by name from registry
/// Returns structured schema data from the registry
/// Converts from valence::Schema to valence-app::server::Schema
pub(crate) fn get_schema_metadata_by_name(raw_name: &str) -> Option<Schema> {
    let registry = SchemaRegistry::global();

    let Some(name) = resolve_schema_table_name(raw_name) else {
        return None;
    };

    // Check if metadata exists first
    let metadata_exists = registry.get_schema(name.as_str()).is_some();
    if !metadata_exists {
        return None;
    }

    // get_full_schema() handles lazy parsing and caching automatically
    match registry.get_full_schema(name.as_str()) {
        Some(s) => {
            let mut schema: Schema = s.into();
            schema.inverse_connections = compute_inverse_connections(&schema.name);
            annotate_trait_sources(&mut schema);
            Some(schema)
        }
        None => None,
    }
}

pub(crate) fn resolve_table_for_schema(schema_name: &str) -> Result<String, ServerFnError> {
    validate_schema_name(schema_name).map_err(super::helpers::validation_error)?;
    let key = resolve_schema_table_name(schema_name).ok_or_else(|| {
        super::helpers::not_found_error(format!("Schema not found: {schema_name}"))
    })?;
    Ok(key)
}

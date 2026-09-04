//! Privacy policy display and evaluation helpers (SSR).

use super::helpers::actor_label;
use super::types::*;
use valence::SchemaRegistry;
use valence::TraitRegistry;

pub(crate) fn policy_display_from_rule(rule: &valence::SchemaPolicyRule) -> PolicyDisplay {
    PolicyDisplay {
        name: rule.name.clone(),
        description: rule.description.clone(),
        trait_source: None,
    }
}

pub(crate) fn build_schema_privacy_card_data(schema: &valence::Schema) -> SchemaPrivacyCardData {
    let mut data = SchemaPrivacyCardData { rows: Vec::new() };

    if let Some(policies) = schema.policies.as_ref() {
        append_schema_operation_display_rows(&mut data, "read", policies.read.as_ref());
        append_schema_operation_display_rows(&mut data, "create", policies.create.as_ref());
        append_schema_operation_display_rows(&mut data, "update", policies.update.as_ref());
        append_schema_operation_display_rows(&mut data, "delete", policies.delete.as_ref());
    }

    union_trait_policies_into_display(&mut data, &schema.traits);
    data
}

pub(crate) fn append_schema_operation_display_rows(
    data: &mut SchemaPrivacyCardData,
    action: &str,
    rules: Option<&valence::SchemaPolicyRules>,
) {
    let Some(rules) = rules else { return };
    append_display_rows(data, action, "always_block", &rules.always_block, None);
    append_display_rows(data, action, "always_allow", &rules.always_allow, None);
    append_display_rows(data, action, "block", &rules.block, None);
    append_display_rows(data, action, "allow", &rules.allow, None);
}

pub(crate) fn append_display_rows(
    data: &mut SchemaPrivacyCardData,
    action: &str,
    bucket: &str,
    rules: &[valence::SchemaPolicyRule],
    trait_source: Option<String>,
) {
    for rule in rules {
        let mut policy = policy_display_from_rule(rule);
        policy.trait_source = trait_source.clone();
        data.rows.push(PolicyDisplayRow {
            action: action.to_string(),
            bucket: bucket.to_string(),
            policy,
        });
    }
}

/// Append trait-inherited policy rules into a
/// `SchemaPrivacyCardData`, tagging each with `trait_source`.
pub(crate) fn union_trait_policies_into_display(
    data: &mut SchemaPrivacyCardData,
    traits: &[String],
) {
    let trait_reg = TraitRegistry::global();
    for trait_name in traits {
        let Some(def) = trait_reg.get_definition(trait_name) else {
            continue;
        };
        let Some(policies) = def.policies else {
            continue;
        };
        let source = Some(trait_name.clone());

        if let Some(read) = policies.read {
            append_display_rows(
                data,
                "read",
                "always_block",
                read.always_block,
                source.clone(),
            );
            append_display_rows(
                data,
                "read",
                "always_allow",
                read.always_allow,
                source.clone(),
            );
            append_display_rows(data, "read", "block", read.block, source.clone());
            append_display_rows(data, "read", "allow", read.allow, source.clone());
        }
        if let Some(create) = policies.create {
            append_display_rows(
                data,
                "create",
                "always_block",
                create.always_block,
                source.clone(),
            );
            append_display_rows(
                data,
                "create",
                "always_allow",
                create.always_allow,
                source.clone(),
            );
            append_display_rows(data, "create", "block", create.block, source.clone());
            append_display_rows(data, "create", "allow", create.allow, source.clone());
        }
        if let Some(update) = policies.update {
            append_display_rows(
                data,
                "update",
                "always_block",
                update.always_block,
                source.clone(),
            );
            append_display_rows(
                data,
                "update",
                "always_allow",
                update.always_allow,
                source.clone(),
            );
            append_display_rows(data, "update", "block", update.block, source.clone());
            append_display_rows(data, "update", "allow", update.allow, source.clone());
        }
        if let Some(delete) = policies.delete {
            append_display_rows(
                data,
                "delete",
                "always_block",
                delete.always_block,
                source.clone(),
            );
            append_display_rows(
                data,
                "delete",
                "always_allow",
                delete.always_allow,
                source.clone(),
            );
            append_display_rows(data, "delete", "block", delete.block, source.clone());
            append_display_rows(data, "delete", "allow", delete.allow, source.clone());
        }
    }
}

pub(crate) fn build_entity_privacy_eval_card_data(
    schema: &valence::Schema,
    record: &serde_json::Value,
    viewer: &valence::Actor,
) -> EntityPrivacyEvalCardData {
    let mut data = EntityPrivacyEvalCardData {
        rows: Vec::new(),
        viewer_label: actor_label(viewer),
    };

    if let Some(policies) = schema.policies.as_ref() {
        append_schema_operation_eval_rows(
            &mut data,
            "read",
            policies.read.as_ref(),
            record,
            viewer,
        );
        append_schema_operation_eval_rows(
            &mut data,
            "create",
            policies.create.as_ref(),
            record,
            viewer,
        );
        append_schema_operation_eval_rows(
            &mut data,
            "update",
            policies.update.as_ref(),
            record,
            viewer,
        );
        append_schema_operation_eval_rows(
            &mut data,
            "delete",
            policies.delete.as_ref(),
            record,
            viewer,
        );
    }

    union_trait_policies_into_eval(&mut data, &schema.traits, record, viewer);
    data
}

pub(crate) fn append_schema_operation_eval_rows(
    data: &mut EntityPrivacyEvalCardData,
    action: &str,
    rules: Option<&valence::SchemaPolicyRules>,
    record: &serde_json::Value,
    viewer: &valence::Actor,
) {
    let Some(rules) = rules else { return };
    append_eval_rows(
        data,
        action,
        "always_block",
        &rules.always_block,
        record,
        viewer,
        true,
        None,
    );
    append_eval_rows(
        data,
        action,
        "always_allow",
        &rules.always_allow,
        record,
        viewer,
        false,
        None,
    );
    append_eval_rows(
        data,
        action,
        "block",
        &rules.block,
        record,
        viewer,
        true,
        None,
    );
    append_eval_rows(
        data,
        action,
        "allow",
        &rules.allow,
        record,
        viewer,
        false,
        None,
    );
}

/// Append trait-inherited policy rules into an
/// `EntityPrivacyEvalCardData`, evaluating each and tagging with `trait_source`.
pub(crate) fn union_trait_policies_into_eval(
    data: &mut EntityPrivacyEvalCardData,
    traits: &[String],
    record: &serde_json::Value,
    viewer: &valence::Actor,
) {
    let trait_reg = TraitRegistry::global();
    for trait_name in traits {
        let Some(def) = trait_reg.get_definition(trait_name) else {
            continue;
        };
        let Some(policies) = def.policies else {
            continue;
        };
        let source = Some(trait_name.clone());

        if let Some(read) = policies.read {
            append_eval_rows(
                data,
                "read",
                "always_block",
                read.always_block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "read",
                "always_allow",
                read.always_allow,
                record,
                viewer,
                false,
                source.clone(),
            );
            append_eval_rows(
                data,
                "read",
                "block",
                read.block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "read",
                "allow",
                read.allow,
                record,
                viewer,
                false,
                source.clone(),
            );
        }
        if let Some(create) = policies.create {
            append_eval_rows(
                data,
                "create",
                "always_block",
                create.always_block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "create",
                "always_allow",
                create.always_allow,
                record,
                viewer,
                false,
                source.clone(),
            );
            append_eval_rows(
                data,
                "create",
                "block",
                create.block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "create",
                "allow",
                create.allow,
                record,
                viewer,
                false,
                source.clone(),
            );
        }
        if let Some(update) = policies.update {
            append_eval_rows(
                data,
                "update",
                "always_block",
                update.always_block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "update",
                "always_allow",
                update.always_allow,
                record,
                viewer,
                false,
                source.clone(),
            );
            append_eval_rows(
                data,
                "update",
                "block",
                update.block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "update",
                "allow",
                update.allow,
                record,
                viewer,
                false,
                source.clone(),
            );
        }
        if let Some(delete) = policies.delete {
            append_eval_rows(
                data,
                "delete",
                "always_block",
                delete.always_block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "delete",
                "always_allow",
                delete.always_allow,
                record,
                viewer,
                false,
                source.clone(),
            );
            append_eval_rows(
                data,
                "delete",
                "block",
                delete.block,
                record,
                viewer,
                true,
                source.clone(),
            );
            append_eval_rows(
                data,
                "delete",
                "allow",
                delete.allow,
                record,
                viewer,
                false,
                source.clone(),
            );
        }
    }
}

pub(crate) fn append_eval_rows(
    data: &mut EntityPrivacyEvalCardData,
    action: &str,
    bucket: &str,
    rules: &[valence::SchemaPolicyRule],
    record: &serde_json::Value,
    viewer: &valence::Actor,
    invert_match: bool,
    trait_source: Option<String>,
) {
    for rule in rules {
        let matches = evaluate_sync_rule(rule, record, viewer);
        let passes = match matches {
            Some(is_match) => {
                if invert_match {
                    !is_match
                } else {
                    is_match
                }
            }
            None => false,
        };

        data.rows.push(PolicyEvalRow {
            action: action.to_string(),
            bucket: bucket.to_string(),
            policy: PolicyEval {
                name: rule.name.clone(),
                passes,
                trait_source: trait_source.clone(),
            },
        });
    }
}

pub(crate) fn evaluate_sync_rule(
    rule: &valence::SchemaPolicyRule,
    record: &serde_json::Value,
    viewer: &valence::Actor,
) -> Option<bool> {
    let evaluator = rule.evaluator?;
    let sync_rule = evaluator.as_any().downcast_ref::<valence::PrivacyRule>()?;
    Some((sync_rule.check)(record, viewer))
}

/// Scan the quark SchemaRegistry for connections whose `to_table` matches
/// `target_table`, producing a list of inbound (inverse) connections.
pub(crate) fn compute_inverse_connections(target_table: &str) -> Vec<InverseSchemaConnection> {
    let registry = SchemaRegistry::global();
    let mut result = Vec::new();

    for meta in registry
        .list_schemas()
        .iter()
        .filter_map(|name| registry.get_schema(name))
    {
        let schema = meta.schema;
        if schema.name == target_table {
            continue;
        }

        // Prefer first-class connections; fall back to legacy edges
        if !schema.connections.is_empty() {
            for conn in &schema.connections {
                if conn.to_table == target_table {
                    result.push(InverseSchemaConnection {
                        from_table: schema.name.clone(),
                        from_field: conn.from_field.clone(),
                        cardinality: conn.cardinality.clone(),
                        label: conn.label.clone(),
                    });
                }
            }
        } else {
            for edge in &schema.edges {
                if edge.to_table == target_table {
                    result.push(InverseSchemaConnection {
                        from_table: schema.name.clone(),
                        from_field: edge.from_field.clone(),
                        cardinality: "HasOne".to_string(),
                        label: edge.label.clone(),
                    });
                }
            }
        }
    }

    result.sort_by(|a, b| a.from_table.cmp(&b.from_table));
    result
}

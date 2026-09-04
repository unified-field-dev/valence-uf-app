//! SSR conversions from Valence registry types into UI DTOs.

use super::types::*;
use valence::{
    ForeignKeyRef as ValenceForeignKeyRef, Schema as ValenceSchema,
    SchemaConnection as ValenceSchemaConnection, SchemaEdge as ValenceSchemaEdge,
    SchemaField as ValenceSchemaField, SchemaMeta as ValenceSchemaMeta,
    SchemaPrivacy as ValenceSchemaPrivacy, SchemaTtlPolicy as ValenceSchemaTtlPolicy,
};

impl From<ValenceSchemaPrivacy> for SchemaPrivacy {
    fn from(v: ValenceSchemaPrivacy) -> Self {
        SchemaPrivacy {
            read: v.read,
            write: v.write,
        }
    }
}

impl From<&ValenceSchemaPrivacy> for SchemaPrivacy {
    fn from(v: &ValenceSchemaPrivacy) -> Self {
        SchemaPrivacy {
            read: v.read.clone(),
            write: v.write.clone(),
        }
    }
}

impl From<ValenceForeignKeyRef> for ForeignKeyRef {
    fn from(v: ValenceForeignKeyRef) -> Self {
        ForeignKeyRef {
            ref_table: v.ref_table,
            field: v.field,
        }
    }
}

impl From<&ValenceForeignKeyRef> for ForeignKeyRef {
    fn from(v: &ValenceForeignKeyRef) -> Self {
        ForeignKeyRef {
            ref_table: v.ref_table.clone(),
            field: v.field.clone(),
        }
    }
}

impl From<ValenceSchemaField> for SchemaField {
    fn from(v: ValenceSchemaField) -> Self {
        SchemaField {
            name: v.name,
            field_type: v.field_type,
            primary: v.primary,
            nullable: v.nullable,
            indexed: v.indexed,
            unique: v.unique,
            default: v.default,
            fk: v.fk.map(Into::into),
            trait_source: None,
        }
    }
}

impl From<&ValenceSchemaField> for SchemaField {
    fn from(v: &ValenceSchemaField) -> Self {
        SchemaField {
            name: v.name.clone(),
            field_type: v.field_type.clone(),
            primary: v.primary,
            nullable: v.nullable,
            indexed: v.indexed,
            unique: v.unique,
            default: v.default.clone(),
            fk: v.fk.as_ref().map(Into::into),
            trait_source: None,
        }
    }
}

impl From<ValenceSchemaEdge> for SchemaEdge {
    fn from(v: ValenceSchemaEdge) -> Self {
        SchemaEdge {
            from_field: v.from_field,
            to_table: v.to_table,
            label: v.label,
        }
    }
}

impl From<&ValenceSchemaEdge> for SchemaEdge {
    fn from(v: &ValenceSchemaEdge) -> Self {
        SchemaEdge {
            from_field: v.from_field.clone(),
            to_table: v.to_table.clone(),
            label: v.label.clone(),
        }
    }
}

impl From<ValenceSchemaConnection> for SchemaConnection {
    fn from(v: ValenceSchemaConnection) -> Self {
        SchemaConnection {
            name: v.name,
            from_table: v.from_table,
            from_field: v.from_field,
            to_table: v.to_table,
            cardinality: v.cardinality,
            required: v.required,
            on_delete: v.on_delete,
            label: v.label,
            target_trait: v.target_trait,
            trait_source: None,
        }
    }
}

impl From<&ValenceSchemaConnection> for SchemaConnection {
    fn from(v: &ValenceSchemaConnection) -> Self {
        SchemaConnection {
            name: v.name.clone(),
            from_table: v.from_table.clone(),
            from_field: v.from_field.clone(),
            to_table: v.to_table.clone(),
            cardinality: v.cardinality.clone(),
            required: v.required,
            on_delete: v.on_delete.clone(),
            label: v.label.clone(),
            target_trait: v.target_trait.clone(),
            trait_source: None,
        }
    }
}

impl From<ValenceSchemaTtlPolicy> for SchemaTtlPolicy {
    fn from(v: ValenceSchemaTtlPolicy) -> Self {
        SchemaTtlPolicy {
            seconds: v.seconds,
            mode: v.mode,
        }
    }
}

impl From<&ValenceSchemaTtlPolicy> for SchemaTtlPolicy {
    fn from(v: &ValenceSchemaTtlPolicy) -> Self {
        SchemaTtlPolicy {
            seconds: v.seconds,
            mode: v.mode.clone(),
        }
    }
}

impl From<ValenceSchemaMeta> for SchemaMeta {
    fn from(v: ValenceSchemaMeta) -> Self {
        SchemaMeta {
            retention: v.retention,
            row_count: v.row_count,
            owner: v.owner,
            description: v.description,
        }
    }
}

impl From<&ValenceSchemaMeta> for SchemaMeta {
    fn from(v: &ValenceSchemaMeta) -> Self {
        SchemaMeta {
            retention: v.retention.clone(),
            row_count: v.row_count,
            owner: v.owner.clone(),
            description: v.description.clone(),
        }
    }
}

impl From<ValenceSchema> for Schema {
    fn from(v: ValenceSchema) -> Self {
        let connections: Vec<SchemaConnection> = if v.connections.is_empty() {
            v.edges
                .iter()
                .map(|e| SchemaConnection {
                    name: e.from_field.clone(),
                    from_table: v.name.clone(),
                    from_field: e.from_field.clone(),
                    to_table: e.to_table.clone(),
                    cardinality: "HasOne".to_string(),
                    required: true,
                    on_delete: "Cascade".to_string(),
                    label: e.label.clone(),
                    target_trait: None,
                    trait_source: None,
                })
                .collect()
        } else {
            v.connections.into_iter().map(Into::into).collect()
        };
        Schema {
            name: v.name,
            databases: v.databases,
            privacy: v.privacy.into(),
            fields: v.fields.into_iter().map(Into::into).collect(),
            edges: v.edges.into_iter().map(Into::into).collect(),
            connections,
            inverse_connections: Vec::new(),
            traits: v.traits,
            iters: v.iters,
            ttl: v.ttl.map(Into::into),
            meta: v.meta.into(),
        }
    }
}

impl From<&ValenceSchema> for Schema {
    fn from(v: &ValenceSchema) -> Self {
        let connections: Vec<SchemaConnection> = if v.connections.is_empty() {
            v.edges
                .iter()
                .map(|e| SchemaConnection {
                    name: e.from_field.clone(),
                    from_table: v.name.clone(),
                    from_field: e.from_field.clone(),
                    to_table: e.to_table.clone(),
                    cardinality: "HasOne".to_string(),
                    required: true,
                    on_delete: "Cascade".to_string(),
                    label: e.label.clone(),
                    target_trait: None,
                    trait_source: None,
                })
                .collect()
        } else {
            v.connections.iter().map(Into::into).collect()
        };
        Schema {
            name: v.name.clone(),
            databases: v.databases.clone(),
            privacy: (&v.privacy).into(),
            fields: v.fields.iter().map(Into::into).collect(),
            edges: v.edges.iter().map(Into::into).collect(),
            connections,
            inverse_connections: Vec::new(),
            traits: v.traits.clone(),
            iters: v.iters.clone(),
            ttl: v.ttl.as_ref().map(Into::into),
            meta: (&v.meta).into(),
        }
    }
}

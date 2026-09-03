//! DTOs shared by Valence UI pages and `#[server]` functions.
//!
//! These types only depend on `serde` so they compile for both SSR and hydrate.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaPrivacy {
    pub read: String,
    pub write: String,
}

/// Foreign key reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignKeyRef {
    pub ref_table: String,
    pub field: String,
}

/// Schema field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub primary: bool,
    pub nullable: bool,
    pub indexed: bool,
    pub unique: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fk: Option<ForeignKeyRef>,
    /// When this field was inherited from a Valence trait, the trait name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_source: Option<String>,
}

/// Edge/relationship definition (deprecated — use SchemaConnection)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaEdge {
    pub from_field: String,
    pub to_table: String,
    pub label: String,
}

/// Connection/relationship definition (replaces SchemaEdge)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaConnection {
    pub name: String,
    pub from_table: String,
    pub from_field: String,
    pub to_table: String,
    pub cardinality: String,
    pub required: bool,
    pub on_delete: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_trait: Option<String>,
    /// When this connection was inherited from a Valence trait, the trait name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_source: Option<String>,
}

/// Table-level TTL policy (if declared by schema).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaTtlPolicy {
    pub seconds: u64,
    pub mode: String,
}

/// Inbound connection discovered by scanning the quark SchemaRegistry.
/// Represents another schema's connection that points TO the current schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InverseSchemaConnection {
    pub from_table: String,
    pub from_field: String,
    pub cardinality: String,
    pub label: String,
}

/// Resolved inverse connection for the entity page, including the actual
/// referencing record IDs found via DB query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InverseConnectionData {
    pub from_table: String,
    pub from_field: String,
    pub label: String,
    pub referencing_ids: Vec<String>,
    pub privacy_restricted: bool,
}

/// Schema metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMeta {
    pub retention: String,
    pub row_count: u64,
    pub owner: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Complete schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub databases: Vec<String>,
    pub privacy: SchemaPrivacy,
    pub fields: Vec<SchemaField>,
    #[serde(default)]
    pub edges: Vec<SchemaEdge>,
    /// First-class connections (preferred over edges)
    #[serde(default)]
    pub connections: Vec<SchemaConnection>,
    /// Inbound connections from other schemas (computed via quark registry scan)
    #[serde(default)]
    pub inverse_connections: Vec<InverseSchemaConnection>,
    /// Valence traits included by this schema (e.g. `["Named", "HasOwner"]`).
    #[serde(default)]
    pub traits: Vec<String>,
    /// Registered [`valence::ValenceIter`] types (`iters: [...]` in schema DSL).
    #[serde(default)]
    pub iters: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<SchemaTtlPolicy>,
    pub meta: SchemaMeta,
}

/// UI display for a privacy policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDisplay {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When this policy was inherited from a Valence trait, the trait name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_source: Option<String>,
}

/// One row in the schema privacy table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDisplayRow {
    pub action: String,
    pub bucket: String,
    pub policy: PolicyDisplay,
}

/// Schema privacy policy card data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaPrivacyCardData {
    pub rows: Vec<PolicyDisplayRow>,
}

/// UI display for an evaluated privacy policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEval {
    pub name: String,
    pub passes: bool,
    /// When this policy was inherited from a Valence trait, the trait name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_source: Option<String>,
}

/// One row in the entity privacy evaluation table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvalRow {
    pub action: String,
    pub bucket: String,
    pub policy: PolicyEval,
}

/// Entity privacy evaluation card data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityPrivacyEvalCardData {
    pub rows: Vec<PolicyEvalRow>,
    /// Request actor label used for evaluation (not client-selectable).
    pub viewer_label: String,
}

// -- Trait types ----------------------------------------------------------

/// A single field defined by a Valence trait.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitFieldInfo {
    pub name: String,
    pub field_type: String,
    pub required: bool,
}

/// Full detail for a single Valence trait (used by the trait detail page).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDetail {
    pub name: String,
    pub fields: Vec<TraitFieldInfo>,
    pub connections: Vec<SchemaConnection>,
    pub implementors: Vec<String>,
    /// Entity-level privacy policies declared by this trait.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<SchemaPrivacyCardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRecord {
    pub id: String,
    pub values: BTreeMap<String, String>,
}

/// Sample record containing only the ID field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SampleRecord {
    pub id: String,
}

/// One row from `valence_ownership_transfer` (shown on the entity page).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnershipTransferRow {
    pub id: String,
    pub from_owner_id: String,
    pub from_owner_type: String,
    pub to_owner_id: String,
    pub to_owner_type: String,
    pub transferred_at: String,
    pub transferred_by: String,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Owner/user info shown on the entity page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub role: String,
    pub email: String,
    pub handle: String,
    /// Raw `owner_type` from `valence_data_ownership` (e.g. `user`, `system`).
    #[serde(default)]
    pub owner_kind: String,
    /// `status` from `valence_data_ownership` (e.g. `active`, `pending_deletion`).
    #[serde(default)]
    pub ownership_status: String,
    /// When the owner maps to a known Valence table, link target for the owner entity page.
    #[serde(default)]
    pub owner_entity_path: Option<String>,
    #[serde(default)]
    pub transfers: Vec<OwnershipTransferRow>,
}

/// Active deletion request info shown on the entity page
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletionRequest {
    pub id: String,
    pub requested_at: String,
    pub status: String,
}

/// Full payload needed by the entity page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityView {
    pub schema: Schema,
    pub record: EntityRecord,
    pub owner: Owner,
    pub deletions: Vec<DeletionRequest>,
    /// Fields hidden by privacy policies (viewer lacks permission)
    #[serde(default)]
    pub hidden_fields: Vec<String>,
    /// Resolved inverse connections with referencing record IDs
    #[serde(default)]
    pub inverse_connections: Vec<InverseConnectionData>,
}

//! DTOs and page-size constants for Valence UF app schema/iter/deletion surfaces.

use serde::{Deserialize, Serialize};

/// Page size used by the schema index table.
pub const SCHEMAS_PAGE_SIZE: u32 = 20;

/// Page size used by the trait index table.
pub const TRAITS_PAGE_SIZE: u32 = 20;

/// Page size used by the iter runs index infinite scroll.
pub const ITERS_PAGE_SIZE: u32 = 20;

/// Schema list item (simplified for the index page).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaListItem {
    /// Canonical registry table name.
    pub name: String,
    /// Databases the schema is registered against.
    pub databases: Vec<String>,
    /// Schema version string from registry metadata.
    pub version: String,
    /// Optional human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Summary item for the trait index page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitListItem {
    /// Trait name as registered.
    pub name: String,
    /// Trait version string (may be empty when registry omits it).
    pub version: String,
    /// Optional human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// One registered iter for a schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterInfo {
    /// Iter type name.
    pub name: String,
    /// Optional description (often empty from descriptor listing).
    pub description: String,
    /// Target table name.
    pub table_name: String,
}

/// `should_run` evaluation for one iter on an entity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterEntityEvaluation {
    /// Iter type name evaluated.
    pub iter_name: String,
    /// Whether the iter should run for the row.
    pub should_run: bool,
    /// Human-readable reason from `should_run`.
    pub reason: String,
}

/// Payload for iter run detail UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterRunView {
    /// Run identifier.
    pub run_id: String,
    /// Iter type name.
    pub iter_name: String,
    /// Target table name.
    pub target_table: String,
    /// Run status display string.
    pub status: String,
    /// Total rows planned.
    pub total_rows: i64,
    /// Rows scanned so far.
    pub scanned_rows: i64,
    /// Rows processed successfully.
    pub processed_rows: i64,
    /// Rows skipped.
    pub skipped_rows: i64,
    /// Rows that failed.
    pub failed_rows: i64,
    /// RFC3339 created-at timestamp.
    pub created_at: String,
    /// Optional top-level error message.
    pub error_message: Option<String>,
    /// Optional single-row target id (entity-scoped runs).
    pub target_row_id: Option<String>,
}

/// One row-level error from an iter run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterRowErrorView {
    /// Error row id.
    pub id: String,
    /// Target record id that failed.
    pub row_id: String,
    /// Error kind display string.
    pub error_kind: String,
    /// Error message.
    pub error_message: String,
    /// RFC3339 created-at timestamp.
    pub created_at: String,
}

/// One batch from an iter run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterBatchView {
    /// Batch id.
    pub id: String,
    /// Batch index within the run.
    pub batch_index: i64,
    /// Batch status display string.
    pub status: String,
    /// Rows in the batch.
    pub row_count: i64,
    /// Processed count.
    pub processed: i64,
    /// Skipped count.
    pub skipped: i64,
    /// Failed count.
    pub failed: i64,
}

/// Short row for schema “recent runs” and global iter index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterRunSummary {
    /// Run identifier.
    pub run_id: String,
    /// Iter type name.
    pub iter_name: String,
    /// Target table name.
    pub target_table: String,
    /// Run status display string.
    pub status: String,
    /// RFC3339 created-at timestamp.
    pub created_at: String,
    /// Total rows planned.
    pub total_rows: i64,
    /// Rows processed successfully.
    pub processed_rows: i64,
}

/// Deletion run summary for admin UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionRunView {
    /// Run identifier.
    pub run_id: String,
    /// Root table for the deletion.
    pub root_table: String,
    /// Root record id.
    pub root_record_id: String,
    /// Run status display string.
    pub status: String,
    /// Total planned steps.
    pub total_steps: i64,
    /// Completed steps.
    pub completed_steps: i64,
    /// Failed steps.
    pub failed_steps: i64,
    /// Requested-at display string (JSON scalar stringified).
    pub requested_at: String,
}

/// One step in a deletion run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionStepView {
    /// Step identifier.
    pub step_id: String,
    /// Table of the record being deleted.
    pub record_table: String,
    /// Record id.
    pub record_id: String,
    /// Action display string.
    pub action: String,
    /// Graph depth.
    pub depth: i64,
    /// Step status display string.
    pub status: String,
}

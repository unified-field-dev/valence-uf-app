//! JSON / DTO mapping helpers for deletion (and related) admin views.

use crate::types::DeletionRunView;

/// Clamp a deletion-runs list limit to the server-accepted window `[1, 200]`.
#[must_use]
pub fn clamp_deletion_list_limit(limit: u32) -> u32 {
    limit.clamp(1, 200)
}

fn json_scalar_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        _ => v.to_string(),
    }
}

/// Map a deletion-run JSON row (from `DeletionService`) into a UI view.
///
/// Returns `None` when the required `id` field is missing or non-string.
#[must_use]
pub fn deletion_run_view_from_value(row: &serde_json::Value) -> Option<DeletionRunView> {
    Some(DeletionRunView {
        run_id: row
            .get("id")
            .and_then(serde_json::Value::as_str)?
            .to_string(),
        root_table: row
            .get("root_table")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string(),
        root_record_id: row
            .get("root_record_id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string(),
        status: row
            .get("status")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
            .to_string(),
        total_steps: row
            .get("total_steps")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0),
        completed_steps: row
            .get("completed_steps")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0),
        failed_steps: row
            .get("failed_steps")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0),
        requested_at: row
            .get("requested_at")
            .map(json_scalar_string)
            .unwrap_or_default(),
    })
}

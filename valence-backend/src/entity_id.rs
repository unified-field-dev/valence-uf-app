//! Entity id normalization for Valence record lookups.

use crate::validate::ValenceIdError;

/// Normalize `entity_id` from a URL: accepts either a bare record key (e.g. `singleton`)
/// or a full Surreal thing string (e.g. `user:⟨…⟩`) so lookups match record JSON gets.
///
/// Mirrors `valence::connection::extract_id_from_record_display`, falling back to the
/// original string when extraction fails (empty id part).
#[must_use]
pub fn normalize_entity_id_for_lookup(entity_id: String) -> String {
    extract_id_from_record_display(&entity_id).unwrap_or(entity_id)
}

/// Extract the id portion from a `table:id` / bracketed Surreal display string.
///
/// # Errors
///
/// Returns [`ValenceIdError::InvalidRecordIdDisplay`] when the extracted id part is empty.
pub fn extract_id_from_record_display(s: &str) -> Result<String, ValenceIdError> {
    let id = s.split_once(':').map_or(s, |(_, id_part)| id_part).trim();
    let id = id
        .trim_start_matches(['⟨', '‹', '«'])
        .trim_end_matches(['⟩', '›', '»']);
    if id.is_empty() {
        return Err(ValenceIdError::InvalidRecordIdDisplay);
    }
    Ok(id.to_string())
}

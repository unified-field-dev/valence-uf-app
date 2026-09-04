pub mod connection_row;
pub mod connections_card;
pub mod deletions_card;
pub mod fields_card;
pub mod iters_card;
pub mod owner_card;
pub mod privacy_eval_card;
pub mod quick_actions_card;
pub mod top_bar;

pub use connections_card::EntityConnectionsCard;
pub use deletions_card::EntityDeletionsCard;
pub use fields_card::EntityFieldsCard;
pub use iters_card::EntityItersCard;
pub use owner_card::EntityOwnerCard;
pub use privacy_eval_card::EntityPrivacyEvalCard;
pub use quick_actions_card::EntityQuickActionsCard;
pub use top_bar::EntityTopBar;

/// Extract the bare record ID from a SurrealDB record identifier.
///
/// SurrealDB stores record IDs in `table:id` format (e.g. `user:abc123`).
/// This function strips the table prefix and returns only the ID portion.
/// If the value does not contain a colon, it is returned as-is — this
/// handles cases where the ID has already been normalized or comes from a
/// schema that stores plain IDs.
///
/// # Examples
/// ```ignore
/// assert_eq!(strip_record_id_prefix("user:abc123"), "abc123");
/// assert_eq!(strip_record_id_prefix("abc123"), "abc123");
/// ```
pub(crate) fn strip_record_id_prefix(value: &str) -> &str {
    match value.split_once(':') {
        Some((table, id)) if !table.is_empty() && !id.is_empty() => id,
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_table_prefix() {
        assert_eq!(
            strip_record_id_prefix("user:m4u9fm9hoo12vypama3r"),
            "m4u9fm9hoo12vypama3r"
        );
    }

    #[test]
    fn strips_different_table_names() {
        assert_eq!(strip_record_id_prefix("notification:abc123"), "abc123");
        assert_eq!(strip_record_id_prefix("org_member:xyz789"), "xyz789");
    }

    #[test]
    fn passes_through_plain_id() {
        assert_eq!(
            strip_record_id_prefix("m4u9fm9hoo12vypama3r"),
            "m4u9fm9hoo12vypama3r"
        );
    }

    #[test]
    fn passes_through_empty_string() {
        assert_eq!(strip_record_id_prefix(""), "");
    }

    #[test]
    fn passes_through_leading_colon() {
        // `:abc` has an empty table portion — not a valid record ID, return as-is
        assert_eq!(strip_record_id_prefix(":abc"), ":abc");
    }

    #[test]
    fn passes_through_trailing_colon() {
        // `user:` has an empty ID portion — not a valid record ID, return as-is
        assert_eq!(strip_record_id_prefix("user:"), "user:");
    }

    #[test]
    fn passes_through_bare_colon() {
        assert_eq!(strip_record_id_prefix(":"), ":");
    }

    #[test]
    fn handles_id_containing_colon() {
        // If the ID itself contains a colon (e.g. compound key), only the
        // first colon is treated as the table separator.
        assert_eq!(
            strip_record_id_prefix("table:some:compound:id"),
            "some:compound:id"
        );
    }
}

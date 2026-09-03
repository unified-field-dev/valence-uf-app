//! Plain-language help copy for Valence explorer UI.

/// Context for building on-delete explanations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnDeleteContext<'a> {
    pub schema_name: &'a str,
    pub from_field: &'a str,
    pub to_table: &'a str,
    pub label: &'a str,
    pub cardinality: &'a str,
    pub on_delete: &'a str,
}

/// Short visible summary for an on-delete rule.
pub fn on_delete_summary(ctx: &OnDeleteContext<'_>) -> String {
    let label = ctx.label;
    match normalize_on_delete(ctx.on_delete) {
        OnDeleteKind::Cascade => match normalize_cardinality(ctx.cardinality) {
            CardinalityKind::HasMany => format!("Also deletes linked {label}"),
            CardinalityKind::HasOne => "Deleting the linked record deletes this".to_string(),
            CardinalityKind::ManyToMany => format!("Also removes linked {label}"),
            CardinalityKind::Other => format!("Also deletes linked {label}"),
        },
        OnDeleteKind::Restrict => match normalize_cardinality(ctx.cardinality) {
            CardinalityKind::HasMany => "Blocks delete if links exist".to_string(),
            CardinalityKind::HasOne => "Blocks deleting the linked record".to_string(),
            CardinalityKind::ManyToMany => "Blocks delete while links exist".to_string(),
            CardinalityKind::Other => "Blocks delete if links exist".to_string(),
        },
        OnDeleteKind::SetNull => "Clears the link".to_string(),
        OnDeleteKind::Other => format!("On delete: {}", ctx.on_delete),
    }
}

/// Longer popover detail for an on-delete rule.
pub fn on_delete_detail(ctx: &OnDeleteContext<'_>) -> String {
    let schema = ctx.schema_name;
    let to_table = ctx.to_table;
    let label = ctx.label;
    let field = ctx.from_field;

    match normalize_on_delete(ctx.on_delete) {
        OnDeleteKind::Cascade => match normalize_cardinality(ctx.cardinality) {
            CardinalityKind::HasMany => format!(
                "When a {schema} is deleted, related {to_table} records ({label}) are deleted too."
            ),
            CardinalityKind::HasOne => format!(
                "When the linked {to_table} record is deleted, this {schema} record is deleted too."
            ),
            CardinalityKind::ManyToMany => format!(
                "When a {schema} is deleted, related {label} links in {to_table} are removed too."
            ),
            CardinalityKind::Other => format!(
                "When a {schema} is deleted, related {to_table} records ({label}) are deleted too."
            ),
        },
        OnDeleteKind::Restrict => match normalize_cardinality(ctx.cardinality) {
            CardinalityKind::HasMany => format!(
                "This {schema} cannot be deleted while it still has {label}."
            ),
            CardinalityKind::HasOne => format!(
                "The linked {to_table} record cannot be deleted while this {schema} still references it."
            ),
            CardinalityKind::ManyToMany => format!(
                "This {schema} cannot be deleted while {label} links still exist."
            ),
            CardinalityKind::Other => format!(
                "This {schema} cannot be deleted while related {label} still exist."
            ),
        },
        OnDeleteKind::SetNull => format!(
            "When the linked {to_table} record is deleted, {field} is cleared on this {schema}."
        ),
        OnDeleteKind::Other => format!(
            "When records are deleted, the {field} link follows the {label} rule ({on_delete}).",
            on_delete = ctx.on_delete
        ),
    }
}

/// Tooltip text for connection cardinality badges.
pub fn cardinality_badge_tooltip(cardinality: &str) -> &'static str {
    match normalize_cardinality(cardinality) {
        CardinalityKind::HasOne => "One linked record (single reference)",
        CardinalityKind::HasMany => "Many linked records (one-to-many)",
        CardinalityKind::ManyToMany => "Many-to-many link through a join table",
        CardinalityKind::Other => "Connection type",
    }
}

/// Tooltip for entity outgoing FK badge.
pub fn fk_badge_tooltip() -> &'static str {
    "Foreign key — points to another record"
}

/// Tooltip for incoming reference badge.
pub fn ref_badge_tooltip() -> &'static str {
    "Another table references this record"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OnDeleteKind {
    Cascade,
    Restrict,
    SetNull,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardinalityKind {
    HasOne,
    HasMany,
    ManyToMany,
    Other,
}

fn normalize_on_delete(value: &str) -> OnDeleteKind {
    match value.to_ascii_lowercase().as_str() {
        "cascade" => OnDeleteKind::Cascade,
        "restrict" => OnDeleteKind::Restrict,
        "setnull" | "set_null" | "set null" => OnDeleteKind::SetNull,
        _ => OnDeleteKind::Other,
    }
}

fn normalize_cardinality(value: &str) -> CardinalityKind {
    match value {
        "HasOne" => CardinalityKind::HasOne,
        "HasMany" => CardinalityKind::HasMany,
        "ManyToMany" => CardinalityKind::ManyToMany,
        _ => CardinalityKind::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx<'a>(
        schema: &'a str,
        field: &'a str,
        to: &'a str,
        label: &'a str,
        card: &'a str,
        on_delete: &'a str,
    ) -> OnDeleteContext<'a> {
        OnDeleteContext {
            schema_name: schema,
            from_field: field,
            to_table: to,
            label,
            cardinality: card,
            on_delete,
        }
    }

    #[test]
    fn cascade_has_many_summary_and_detail() {
        let c = ctx(
            "account",
            "memberships",
            "account_membership",
            "Memberships",
            "HasMany",
            "Cascade",
        );
        assert_eq!(on_delete_summary(&c), "Also deletes linked Memberships");
        assert!(on_delete_detail(&c).contains("account_membership"));
        assert!(on_delete_detail(&c).contains("account"));
    }

    #[test]
    fn cascade_has_one_summary() {
        let c = ctx("user_counter", "user", "user", "User", "HasOne", "Cascade");
        assert_eq!(
            on_delete_summary(&c),
            "Deleting the linked record deletes this"
        );
    }

    #[test]
    fn restrict_has_many_blocks_parent_delete() {
        let c = ctx(
            "post", "comments", "comment", "Comments", "HasMany", "Restrict",
        );
        assert_eq!(on_delete_summary(&c), "Blocks delete if links exist");
        assert!(on_delete_detail(&c).contains("cannot be deleted"));
    }

    #[test]
    fn set_null_clears_link() {
        let c = ctx("run", "job", "chronon_job", "Job", "HasOne", "SetNull");
        assert_eq!(on_delete_summary(&c), "Clears the link");
        assert!(on_delete_detail(&c).contains("cleared"));
    }
}

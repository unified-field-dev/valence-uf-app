//! In-memory list lookups mirroring Valence UF app server resolve helpers.

use crate::types::{
    DeletionRunView, IterInfo, IterRunSummary, IterRunView, SchemaListItem, TraitListItem,
};

/// Case-insensitive resolve of a schema name against a list of registry names.
#[must_use]
pub fn resolve_schema_name_from_list(names: &[String], raw_name: &str) -> Option<String> {
    let trimmed = raw_name.trim();
    names
        .iter()
        .find(|n| n.eq_ignore_ascii_case(trimmed))
        .cloned()
}

/// Find a schema list item by exact (case-insensitive) name.
#[must_use]
pub fn find_schema_by_name<'a>(
    schemas: &'a [SchemaListItem],
    name: &str,
) -> Option<&'a SchemaListItem> {
    let trimmed = name.trim();
    schemas
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case(trimmed))
}

/// Find a trait list item by exact (case-insensitive) name.
#[must_use]
pub fn find_trait_by_name<'a>(
    traits: &'a [TraitListItem],
    name: &str,
) -> Option<&'a TraitListItem> {
    let trimmed = name.trim();
    traits.iter().find(|t| t.name.eq_ignore_ascii_case(trimmed))
}

/// Find a registered iter by exact name on a schema's iter list.
#[must_use]
pub fn find_iter_by_name<'a>(iters: &'a [IterInfo], iter_name: &str) -> Option<&'a IterInfo> {
    let trimmed = iter_name.trim();
    iters.iter().find(|i| i.name == trimmed)
}

/// Whether `iter_name` appears in the schema's registered iter name list.
#[must_use]
pub fn schema_has_iter(iter_names: &[String], iter_name: &str) -> bool {
    let trimmed = iter_name.trim();
    iter_names.iter().any(|i| i == trimmed)
}

/// Find an iter run summary by exact run id.
#[must_use]
pub fn find_iter_run_summary_by_id<'a>(
    runs: &'a [IterRunSummary],
    run_id: &str,
) -> Option<&'a IterRunSummary> {
    let trimmed = run_id.trim();
    runs.iter().find(|r| r.run_id == trimmed)
}

/// Find an iter run detail by exact run id.
#[must_use]
pub fn find_iter_run_by_id<'a>(runs: &'a [IterRunView], run_id: &str) -> Option<&'a IterRunView> {
    let trimmed = run_id.trim();
    runs.iter().find(|r| r.run_id == trimmed)
}

/// Find a deletion run by exact run id.
#[must_use]
pub fn find_deletion_run_by_id<'a>(
    runs: &'a [DeletionRunView],
    run_id: &str,
) -> Option<&'a DeletionRunView> {
    let trimmed = run_id.trim();
    runs.iter().find(|r| r.run_id == trimmed)
}

/// Sort schema list items by name (lexicographic).
pub fn sort_schemas_by_name(schemas: &mut [SchemaListItem]) {
    schemas.sort_by(|a, b| a.name.cmp(&b.name));
}

/// Sort trait list items by name (lexicographic).
pub fn sort_traits_by_name(traits: &mut [TraitListItem]) {
    traits.sort_by(|a, b| a.name.cmp(&b.name));
}

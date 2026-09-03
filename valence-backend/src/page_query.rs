//! `DataTable` / page-request adapters for schema and trait index lists.

use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

use crate::types::{SchemaListItem, TraitListItem};

fn filter_rule_text(value: &DataValue) -> String {
    value.display_string()
}

fn text_contains(haystack: &str, needle: &str) -> bool {
    let needle = needle.trim();
    if needle.is_empty() {
        return true;
    }
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn text_equals(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right.trim())
}

fn schema_matches_filter_rule(item: &SchemaListItem, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&item.name, &value),
            "equals" | "is" => text_equals(&item.name, &value),
            "not_equals" | "is_not" => !text_equals(&item.name, &value),
            "starts_with" => item
                .name
                .to_lowercase()
                .starts_with(&value.trim().to_lowercase()),
            "ends_with" => item
                .name
                .to_lowercase()
                .ends_with(&value.trim().to_lowercase()),
            _ => true,
        },
        "databases" => {
            let matches = item
                .databases
                .iter()
                .any(|db| match rule.operator.as_str() {
                    "equals" | "is" => text_equals(db, &value),
                    "not_equals" | "is_not" => !text_equals(db, &value),
                    "starts_with" => db.to_lowercase().starts_with(&value.trim().to_lowercase()),
                    "ends_with" => db.to_lowercase().ends_with(&value.trim().to_lowercase()),
                    _ => text_contains(db, &value),
                });
            match rule.operator.as_str() {
                "not_contains" | "not_equals" | "is_not" => !matches,
                _ => matches,
            }
        }
        "version" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(&item.version, &value),
            "not_equals" | "is_not" => !text_equals(&item.version, &value),
            _ => text_contains(&item.version, &value),
        },
        "description" => {
            let description = item.description.as_deref().unwrap_or("");
            match rule.operator.as_str() {
                "equals" | "is" => text_equals(description, &value),
                "not_equals" | "is_not" => !text_equals(description, &value),
                _ => text_contains(description, &value),
            }
        }
        _ => true,
    }
}

fn apply_schema_filter_query(items: &mut Vec<SchemaListItem>, filter: &FilterQuery) {
    if filter.items.is_empty() {
        return;
    }
    items.retain(|item| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| schema_matches_filter_rule(item, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Apply quick-search and structured filters from a [`PageRequest`] to schema list items.
pub fn apply_schema_page_query(items: &mut Vec<SchemaListItem>, request: &PageRequest) {
    if let Some(ref quick) = request.quick_search {
        let q_lower = quick.trim().to_lowercase();
        if !q_lower.is_empty() {
            items.retain(|s| {
                s.name.to_lowercase().contains(&q_lower)
                    || s.description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&q_lower))
            });
        }
    }
    if let Some(ref filter) = request.filter {
        apply_schema_filter_query(items, filter);
    }
}

fn trait_matches_filter_rule(item: &TraitListItem, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&item.name, &value),
            "equals" | "is" => text_equals(&item.name, &value),
            "not_equals" | "is_not" => !text_equals(&item.name, &value),
            "starts_with" => item
                .name
                .to_lowercase()
                .starts_with(&value.trim().to_lowercase()),
            "ends_with" => item
                .name
                .to_lowercase()
                .ends_with(&value.trim().to_lowercase()),
            _ => true,
        },
        "version" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(&item.version, &value),
            "not_equals" | "is_not" => !text_equals(&item.version, &value),
            _ => text_contains(&item.version, &value),
        },
        "description" => {
            let description = item.description.as_deref().unwrap_or("");
            match rule.operator.as_str() {
                "equals" | "is" => text_equals(description, &value),
                "not_equals" | "is_not" => !text_equals(description, &value),
                _ => text_contains(description, &value),
            }
        }
        _ => true,
    }
}

fn apply_trait_filter_query(items: &mut Vec<TraitListItem>, filter: &FilterQuery) {
    if filter.items.is_empty() {
        return;
    }
    items.retain(|item| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| trait_matches_filter_rule(item, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Apply quick-search and structured filters from a [`PageRequest`] to trait list items.
pub fn apply_trait_page_query(items: &mut Vec<TraitListItem>, request: &PageRequest) {
    if let Some(ref quick) = request.quick_search {
        let q_lower = quick.trim().to_lowercase();
        if !q_lower.is_empty() {
            items.retain(|t| t.name.to_lowercase().contains(&q_lower));
        }
    }
    if let Some(ref filter) = request.filter {
        apply_trait_filter_query(items, filter);
    }
}

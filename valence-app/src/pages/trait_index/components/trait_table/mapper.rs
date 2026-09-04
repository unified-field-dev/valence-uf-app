use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::TraitListItem;

const DESCRIPTION_CARD_MAX_LEN: usize = 120;

fn truncate_description(text: &str) -> String {
    if text.chars().count() <= DESCRIPTION_CARD_MAX_LEN {
        return text.to_string();
    }
    let truncated: String = text.chars().take(DESCRIPTION_CARD_MAX_LEN).collect();
    format!("{truncated}…")
}

fn format_description(description: Option<String>) -> String {
    match description {
        Some(text) if !text.trim().is_empty() => truncate_description(text.trim()),
        _ => "—".to_string(),
    }
}

fn format_version(version: &str) -> String {
    if version.trim().is_empty() {
        "—".to_string()
    } else {
        version.to_string()
    }
}

pub fn trait_item_to_record(item: TraitListItem) -> DataRecord {
    let name = item.name.clone();
    let version = format_version(&item.version);
    let description = format_description(item.description);
    DataRecord::new(
        name.clone(),
        HashMap::from([
            ("name".into(), DataValue::Text(name)),
            ("version".into(), DataValue::Text(version)),
            ("description".into(), DataValue::Text(description)),
        ]),
    )
}

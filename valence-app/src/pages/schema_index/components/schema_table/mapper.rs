use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::SchemaListItem;

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

pub fn schema_item_to_record(item: SchemaListItem) -> DataRecord {
    let name = item.name.clone();
    let databases = item.databases.join(" \u{2022} ");
    let version = item.version;
    let description = format_description(item.description);
    DataRecord::new(
        name.clone(),
        HashMap::from([
            ("name".into(), DataValue::Text(name)),
            ("databases".into(), DataValue::Text(databases)),
            ("version".into(), DataValue::Text(version)),
            ("description".into(), DataValue::Text(description)),
        ]),
    )
}

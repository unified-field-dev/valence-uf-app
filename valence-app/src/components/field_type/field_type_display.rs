use leptos::prelude::*;

use super::ext_enum_display::ExtEnumDisplay;
use super::inline_enum_display::InlineEnumDisplay;
use super::parse::{parse_field_type, ParsedFieldType};
use super::plain_type_label::PlainTypeLabel;
use super::typography::FieldTypeTypography;

#[component]
pub fn FieldTypeDisplay(
    field_type: String,
    field_name: String,
    context_name: String,
    typography: FieldTypeTypography,
    #[prop(optional, into)] subtle_class: MaybeProp<String>,
) -> impl IntoView {
    let parsed = parse_field_type(&field_type, &context_name, &field_name);

    match parsed {
        ParsedFieldType::Plain(text) => view! {
            <PlainTypeLabel
                text=text
                typography=typography
                subtle_class=subtle_class
            />
        }
        .into_any(),
        ParsedFieldType::InlineEnum { name, variants } => view! {
            <InlineEnumDisplay
                enum_name=name
                variants=variants
                typography=typography
                subtle_class=subtle_class
            />
        }
        .into_any(),
        ParsedFieldType::ExternalEnum { name, path } => view! {
            <ExtEnumDisplay
                enum_name=name
                path=path
                typography=typography
                subtle_class=subtle_class
            />
        }
        .into_any(),
    }
}

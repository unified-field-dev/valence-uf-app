use leptos::prelude::*;
use orbital::primitives::Tooltip;

use super::typography::{FieldTypeText, FieldTypeTypography};

#[component]
pub fn ExtEnumDisplay(
    enum_name: String,
    path: String,
    typography: FieldTypeTypography,
    #[prop(optional, into)] subtle_class: MaybeProp<String>,
) -> impl IntoView {
    let label = format!("Enum: {enum_name}");

    view! {
        <span data-testid="valence-field-type-enum">
            <Tooltip content=path>
                <FieldTypeText
                    text=label
                    typography=typography
                    subtle_class=subtle_class
                />
            </Tooltip>
        </span>
    }
}

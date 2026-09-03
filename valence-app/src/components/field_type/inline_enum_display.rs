use leptos::prelude::*;
use orbital::primitives::{Popover, PopoverTrigger};

use super::enum_variants_panel::EnumVariantsPanel;
use super::typography::{FieldTypeText, FieldTypeTypography};

#[component]
pub fn InlineEnumDisplay(
    enum_name: String,
    variants: Vec<String>,
    typography: FieldTypeTypography,
    #[prop(optional, into)] subtle_class: MaybeProp<String>,
) -> impl IntoView {
    let label = format!("Enum: {enum_name}");

    view! {
        <Popover>
            <PopoverTrigger slot>
                <span data-testid="valence-field-type-enum">
                    <FieldTypeText
                        text=label
                        typography=typography
                        subtle_class=subtle_class
                    />
                </span>
            </PopoverTrigger>
            <EnumVariantsPanel variants=variants />
        </Popover>
    }
}

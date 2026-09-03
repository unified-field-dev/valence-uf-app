use leptos::prelude::*;
use orbital::components::{Stack, StackConfig, Tag, TagAppearance, TagGroup};
use orbital::primitives::FlexGap;

const PANEL_STACK: StackConfig = StackConfig {
    gap: FlexGap::Small,
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn EnumVariantsPanel(variants: Vec<String>) -> impl IntoView {
    let variants = StoredValue::new(variants);
    let appearance = Signal::from(TagAppearance::Outline);

    view! {
        <Stack config=PANEL_STACK>
            <TagGroup appearance=appearance>
                <For
                    each=move || variants.get_value()
                    key=|v| v.clone()
                    let:variant
                >
                    <Tag>{variant.clone()}</Tag>
                </For>
            </TagGroup>
        </Stack>
    }
}

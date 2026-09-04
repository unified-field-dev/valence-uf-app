use leptos::prelude::*;
use orbital::components::Stack;
use orbital::components::StackConfig;
use orbital::primitives::{Flex, FlexAlign, FlexGap, FlexWrap};

const ROW_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(4),
    horizontal: false,
    align: None,
    justify: None,
};

/// Two-line connection row: primary link row + optional indented meta row.
#[component]
pub fn ConnectionRowLayout(
    list_item_class: String,
    meta_row_class: String,
    badge: AnyView,
    primary: AnyView,
    #[prop(optional)] meta: Option<AnyView>,
) -> impl IntoView {
    view! {
        <Stack config=ROW_STACK>
            <Flex class=list_item_class align=FlexAlign::Center wrap=FlexWrap::Wrap>
                {badge}
                {primary}
            </Flex>
            {meta.map(|m| view! {
                <Flex class=meta_row_class align=FlexAlign::Center wrap=FlexWrap::Wrap>
                    {m}
                </Flex>
            })}
        </Stack>
    }
}

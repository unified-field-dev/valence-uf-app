use leptos::prelude::*;
use orbital::components::{SpacingSize, Subtitle2};
use orbital::primitives::Flex;

use super::trait_table::TraitDataTable;

/// Displays the paginated, filtered list of traits with a heading.
#[component]
pub fn TraitsListSection(initial_quick_search: Memo<String>) -> impl IntoView {
    view! {
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Subtitle2>"Traits"</Subtitle2>
            <TraitDataTable initial_quick_search=initial_quick_search />
        </Flex>
    }
}

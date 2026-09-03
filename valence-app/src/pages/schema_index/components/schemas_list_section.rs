use leptos::prelude::*;
use orbital::components::{SpacingSize, Subtitle2};
use orbital::primitives::Flex;

use super::schema_table::SchemaDataTable;

/// Displays the paginated, filtered list of schemas with a heading.
#[component]
pub fn SchemasListSection(initial_quick_search: Memo<String>) -> impl IntoView {
    view! {
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Subtitle2>"Schemas"</Subtitle2>
            <SchemaDataTable initial_quick_search=initial_quick_search />
        </Flex>
    }
}

mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use orbital::components::{ContentContainer, SpacingSize};
use orbital::primitives::*;
use urlencoding::decode;

use crate::pages::schema_index::components::*;

/// Extract a query parameter from the URL search string
fn extract_query_param(search: &str, param_name: &str) -> Option<String> {
    let trimmed = search.trim_start_matches('?');
    if trimmed.is_empty() {
        return None;
    }

    for pair in trimmed.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == param_name {
                return Some(value.to_string());
            }
        }
    }
    None
}

#[component]
pub fn ValenceSchemaIndexPage() -> impl IntoView {
    let location = use_location();
    let initial_quick_search = Memo::new(move |_| {
        extract_query_param(&location.search.get(), "q")
            .and_then(|q| decode(&q).ok().map(|s| s.to_string()))
            .unwrap_or_default()
    });

    view! {
        <div id="valence-schema-index-page">
        <ContentContainer data_testid="schema-index-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <div id="valence-schema-index-search">
                    <div id="valence-schemas-list">
                        <SchemasListSection initial_quick_search=initial_quick_search />
                    </div>
                </div>
            </Flex>
        </ContentContainer>
        </div>
    }
}

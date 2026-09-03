use leptos::prelude::*;
use orbital::components::{Icon, Input, InputAppearance, InputPrefix, Title3};
use turf::inline_style_sheet_values;

use crate::server::SearchSchemaOrId;

#[component]
pub fn DashboardSearchHeader() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let search_action = ServerAction::<SearchSchemaOrId>::new();

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Header {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }

        .SearchRow {
            width: min(560px, 100%);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div id="valence-dashboard-search" class=class_names.header data-testid="valence-dashboard-search-header">
            <Title3>"Valence"</Title3>
            <form
                on:submit=move |ev| {
                    ev.prevent_default();
                    let query_val = query.get();
                    if !query_val.trim().is_empty() {
                        search_action.dispatch(SearchSchemaOrId { query: query_val });
                    }
                }
            >
                <div data-testid="valence-dashboard-search" class=class_names.search_row>
                    <Input
                        bind=query
                        appearance=InputAppearance::with_placeholder("Search for schema or ID.")
                    >
                        <InputPrefix slot>
                            <Icon icon=icondata::AiSearchOutlined />
                        </InputPrefix>
                    </Input>
                </div>
            </form>
        </div>
    }
}

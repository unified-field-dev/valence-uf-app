use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{MessageBar, MessageBarIntent};
use orbital::primitives::*;

use super::columns::trait_table_columns;
use super::fetcher::{build_trait_fetcher, TRAIT_TABLE_PAGE_SIZE};

#[component]
pub fn TraitDataTable(initial_quick_search: Memo<String>) -> impl IntoView {
    let navigate = use_navigate();
    let handle = RwSignal::new(None::<DataTableHandle>);

    let on_row_click = Callback::new(move |(id,): (String,)| {
        let _ = navigate(
            &valence_backend::valence_trait_path(&id),
            Default::default(),
        );
    });

    Effect::new(move |_| {
        let q = initial_quick_search.get();
        if let Some(h) = handle.get() {
            h.set_quick_search.run((q,));
        }
    });

    let initial_q = initial_quick_search.get();
    let initial_state = DataTableInitialState {
        quick_search: if initial_q.is_empty() {
            None
        } else {
            Some(initial_q)
        },
        ..Default::default()
    };

    let data_source = DataTableSource::Server {
        fetcher: build_trait_fetcher(),
        page_size: TRAIT_TABLE_PAGE_SIZE,
    };

    view! {
        <div data-testid="valence-traits-list-section">
            <DataTable
                data_source=data_source
                paging=PagingMode::Paged
                features=DataTableFeatures::LIST_VIEW | DataTableFeatures::MULTI_FILTER
                list_view=ListViewConfig::new("name")
                    .with_secondary_fields(vec![
                        "version".into(),
                        "description".into(),
                    ])
                columns=trait_table_columns()
                sortable=false
                toolbar_config=DataTableToolbarConfig {
                    quick_search: true,
                    filter_panel: true,
                    column_picker: false,
                    pivot: false,
                    export_menu: false,
                }
                header_chrome=DataTableHeaderChromeConfig {
                    column_menu: false,
                    column_filter_button: false,
                    column_hide: false,
                }
                initial_state=initial_state
                on_handle=Callback::new(move |h: DataTableHandle| {
                    handle.set(Some(h.clone()));
                    let q = initial_quick_search.get();
                    if !q.is_empty() {
                        h.set_quick_search.run((q,));
                    }
                })
                events=DataTableEvents {
                    on_row_click: Some(on_row_click),
                    ..Default::default()
                }
            >
                <DataTableEmptyView slot>
                    <MessageBar intent=MessageBarIntent::Info>
                        "No traits registered."
                    </MessageBar>
                </DataTableEmptyView>
            </DataTable>
        </div>
    }
}

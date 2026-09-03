mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Body1, ContentContainer};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::pages::trait_detail::components::*;
use crate::server::get_trait;

#[component]
pub fn ValenceTraitDetailPage() -> impl IntoView {
    let params = use_params_map();
    let trait_name = Memo::new(move |_| {
        params
            .get()
            .get("trait_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let trait_res = Resource::new(
        move || trait_name.get(),
        |name| async move {
            if name.is_empty() {
                return Ok(None);
            }
            get_trait(name).await
        },
    );

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .GridTwo {
            align-items: start;
        }

        .Section {
            display: flex;
            flex-direction: column;
            gap: 24px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="valence-trait-detail-page">
            <Suspense fallback=move || view! { <Body1>"Loading trait..."</Body1> }>
                {move || match trait_res.get() {
                    Some(Ok(Some(detail))) => {
                        let detail = detail.clone();
                        let field_count = detail.fields.len();
                        let conn_count = detail.connections.len();
                        let impl_count = detail.implementors.len();
                        view! {
                            <TraitTopBar trait_name=detail.name.clone() />

                            <Grid config=GridConfig::with_gaps(2, 24, 0) class=class_names.grid_two>
                                // Left column
                                <GridItem><div class=class_names.section>
                                    <TraitOverviewCard
                                        trait_name=detail.name.clone()
                                        field_count=field_count
                                        connection_count=conn_count
                                        implementor_count=impl_count
                                    />
                                    <TraitFieldsTable
                                        trait_name=detail.name.clone()
                                        fields=detail.fields.clone()
                                    />
                                    <TraitConnectionsCard
                                        trait_name=detail.name.clone()
                                        connections=detail.connections.clone()
                                    />
                                </div></GridItem>

                                // Right column
                                <GridItem><div class=class_names.section>
                                    <UsedByCard implementors=detail.implementors.clone() />
                                </div></GridItem>
                            </Grid>
                        }.into_any()
                    }
                    Some(Ok(None)) => view! {
                        <MessageBar intent=MessageBarIntent::Warning>
                            "Trait not found"
                        </MessageBar>
                    }.into_any(),
                    Some(Err(err)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>
                            {format!("Failed to load trait: {}", err.to_string())}
                        </MessageBar>
                    }.into_any(),
                    None => view! { <Body1>"Loading..."</Body1> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
    }
}

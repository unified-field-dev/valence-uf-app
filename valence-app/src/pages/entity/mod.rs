mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Body1, ContentContainer, Stack, StackConfig};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::pages::entity::components::*;
use crate::server::{get_entity_privacy_evaluation, get_entity_view, EntityPrivacyEvalCardData};

const SECTION_GAP: FlexGap = FlexGap::Size(24);

#[component]
pub fn ValenceEntityPage() -> impl IntoView {
    let params = use_params_map();
    let schema_name = Memo::new(move |_| {
        params
            .get()
            .get("schema_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });
    let entity_id = Memo::new(move |_| {
        params
            .get()
            .get("entity_id")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let entity_res = Resource::new(
        move || (schema_name.get(), entity_id.get()),
        |(schema_name, entity_id)| async move {
            if schema_name.is_empty() || entity_id.is_empty() {
                return Ok(None);
            }
            get_entity_view(schema_name, entity_id).await
        },
    );

    let privacy_eval_res = Resource::new(
        move || (schema_name.get(), entity_id.get()),
        |(schema_name, entity_id)| async move {
            if schema_name.is_empty() || entity_id.is_empty() {
                return Ok(EntityPrivacyEvalCardData {
                    rows: vec![],
                    viewer_label: String::new(),
                });
            }
            get_entity_privacy_evaluation(schema_name, entity_id).await
        },
    );

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .PageGrid {
            width: 100%;
        }

        @media (width < 980px) {
            .PageGrid {
                grid-template-columns: 1fr !important;
            }
        }

        .MainColumn {
            min-width: 0;
        }

        .AsideColumn {
            min-width: 0;
        }

        @media (width >= 980px) {
            .MainColumn {
                grid-column: 1;
                grid-row: 1;
            }

            .AsideColumn {
                grid-column: 2;
                grid-row: 1;
            }
        }
    };

    let page_stack = StackConfig::vertical(SECTION_GAP);
    let column_stack = StackConfig::vertical(SECTION_GAP);

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="valence-entity-page">
            <Suspense fallback=move || view! { <Body1>"Loading record..."</Body1> }>
                {move || match entity_res.get() {
                    Some(Ok(Some(entity))) => {
                        let schema = entity.schema.clone();
                        let record = entity.record.clone();
                        let hidden_fields = entity.hidden_fields.clone();

                        let schema_name = schema.name.clone();
                        let record_id = record.id.clone();

                        let title = format!("/schemas/{}/{}", schema_name, record_id);
                        let back_href = valence_backend::valence_schema_path(&schema_name);
                        let back_label = "Back to Schema".to_string();

                        let aside_schema_name = schema_name.clone();
                        let main_schema_name = schema_name.clone();
                        let quick_record = record.clone();

                        view! {
                            <Stack config=page_stack>
                                <EntityTopBar
                                    title=title
                                    back_href=back_href
                                    back_label=back_label
                                />

                                <Grid
                                    config=GridConfig::with_gaps(2, 24, 24)
                                    class=class_names.page_grid
                                >
                                    <GridItem class=class_names.aside_column>
                                        <Stack config=column_stack>
                                            <EntityOwnerCard owner=entity.owner.clone() />
                                            <EntityQuickActionsCard
                                                schema_name=aside_schema_name.clone()
                                                record=quick_record
                                            />
                                            <EntityDeletionsCard deletions=entity.deletions.clone() />
                                        </Stack>
                                    </GridItem>

                                    <GridItem class=class_names.main_column>
                                        <Stack config=column_stack>
                                            <EntityFieldsCard
                                                schema_name=main_schema_name.clone()
                                                schema_fields=schema.fields.clone()
                                                record_values=record.values.clone()
                                                hidden_fields=hidden_fields.clone()
                                            />
                                            <EntityConnectionsCard
                                                schema_name=main_schema_name.clone()
                                                schema_connections=schema.connections.clone()
                                                record_values=record.values.clone()
                                                hidden_fields=hidden_fields.clone()
                                                inverse_connections=entity.inverse_connections.clone()
                                            />
                                            {move || match privacy_eval_res.get() {
                                                Some(Ok(data)) => view! {
                                                    <EntityPrivacyEvalCard data=data />
                                                }.into_any(),
                                                Some(Err(err)) => view! {
                                                    <MessageBar intent=MessageBarIntent::Error>
                                                        {format!("Failed to load privacy evaluation: {}", err.to_string())}
                                                    </MessageBar>
                                                }.into_any(),
                                                None => view! {
                                                    <Body1>"Loading privacy evaluation..."</Body1>
                                                }.into_any(),
                                            }}
                                            <EntityItersCard
                                                schema_name=main_schema_name.clone()
                                                entity_id=record_id.clone()
                                            />
                                        </Stack>
                                    </GridItem>
                                </Grid>
                            </Stack>
                        }.into_any()
                    }
                    Some(Ok(None)) => view! {
                        <MessageBar intent=MessageBarIntent::Warning>
                            "Record not found"
                        </MessageBar>
                    }.into_any(),
                    Some(Err(err)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>
                            {format!("Failed to load record: {}", err.to_string())}
                        </MessageBar>
                    }.into_any(),
                    None => view! { <Body1>"Loading..."</Body1> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
    }
}

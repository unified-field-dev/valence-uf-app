mod components;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Body1, ContentContainer, Stack, StackConfig};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::pages::schema::components::*;
use crate::server::{get_schema, get_schema_privacy_policies, SchemaPrivacyCardData};

const SECTION_GAP: FlexGap = FlexGap::Size(24);

#[component]
pub fn ValenceSchemaPage() -> impl IntoView {
    let params = use_params_map();
    let schema_name = Memo::new(move |_| {
        params
            .get()
            .get("schema_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let schema_res = Resource::new(
        move || schema_name.get(),
        |name| async move {
            if name.is_empty() {
                return Ok(None);
            }
            get_schema(name).await
        },
    );

    let privacy_res = Resource::new(
        move || schema_name.get(),
        |name| async move {
            if name.is_empty() {
                return Ok(SchemaPrivacyCardData { rows: vec![] });
            }
            get_schema_privacy_policies(name).await
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
                grid-row: 2;
            }

            .AsideColumn {
                grid-column: 2;
                grid-row: 2;
            }
        }
    };

    let page_stack = StackConfig::vertical(SECTION_GAP);
    let column_stack = StackConfig::vertical(SECTION_GAP);

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="valence-schema-detail-page">
            <Suspense fallback=move || view! { <Body1>"Loading schema..."</Body1> }>
                {move || match schema_res.get() {
                    Some(Ok(Some(schema))) => {
                        let schema = schema.clone();
                        let databases = schema.databases.clone();
                        let fields = schema.fields.clone();
                        let connections = schema.connections.clone();
                        let inverse_connections = schema.inverse_connections.clone();
                        let meta = schema.meta.clone();
                        let privacy = schema.privacy.clone();
                        let schema_traits = schema.traits.clone();
                        let schema_name = schema.name.clone();
                        let schema_for_actions = schema.clone();
                        let aside_schema_name = schema_name.clone();
                        let main_schema_name = schema_name.clone();
                        view! {
                            <Stack config=page_stack>
                                <SchemaTopBar schema_name=schema_name.clone() />

                                <Grid
                                    config=GridConfig::with_gaps(2, 24, 24)
                                    class=class_names.page_grid
                                >
                                    <GridItem config=GridItemConfig::span(2)>
                                        <OverviewCard
                                            schema_name=schema_name.clone()
                                            databases=databases
                                            privacy=privacy
                                            meta=meta
                                        />
                                    </GridItem>

                                    <GridItem class=class_names.aside_column>
                                        <Stack config=column_stack>
                                            <SamplesCard schema_name=aside_schema_name.clone() />
                                            <QuickActionsCard
                                                schema=schema_for_actions
                                                schema_name=aside_schema_name.clone()
                                            />
                                        </Stack>
                                    </GridItem>

                                    <GridItem class=class_names.main_column>
                                        <Stack config=column_stack>
                                            {move || match privacy_res.get() {
                                                Some(Ok(policy_data)) => {
                                                    view! {
                                                        <PrivacyPoliciesCard data=policy_data />
                                                    }.into_any()
                                                }
                                                Some(Err(err)) => view! {
                                                    <MessageBar intent=MessageBarIntent::Error>
                                                        {format!("Failed to load privacy policies: {}", err.to_string())}
                                                    </MessageBar>
                                                }.into_any(),
                                                None => view! {
                                                    <Body1>"Loading privacy policies..."</Body1>
                                                }.into_any(),
                                            }}
                                            <FieldsTable schema_name=main_schema_name.clone() fields=fields />
                                            <ConnectionsCard
                                                schema_name=main_schema_name.clone()
                                                connections=connections
                                                inverse_connections=inverse_connections
                                            />
                                            <TraitsCard traits=schema_traits />
                                            <ItersCard schema_name=main_schema_name.clone() />
                                            <SchemaDeletionsCard schema_name=main_schema_name.clone() />
                                        </Stack>
                                    </GridItem>
                                </Grid>
                            </Stack>
                        }.into_any()
                    }
                    Some(Ok(None)) => view! {
                        <MessageBar intent=MessageBarIntent::Warning>
                            "Schema not found"
                        </MessageBar>
                    }.into_any(),
                    Some(Err(err)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>
                            {format!("Failed to load schema: {}", err.to_string())}
                        </MessageBar>
                    }.into_any(),
                    None => view! { <Body1>"Loading..."</Body1> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
    }
}

use leptos::prelude::*;
use orbital::components::{
    Body1, Caption1, Card, CardContent, CardFooter, CardHeader, SpacingSize, Stack, StackConfig,
    Tag,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{ValenceHelpCardHeader, ValenceHelpColumnHeader};
use crate::server::{SchemaMeta, SchemaPrivacy};

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(24),
    horizontal: false,
    align: None,
    justify: None,
};

const META_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(8),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn OverviewCard(
    schema_name: String,
    databases: Vec<String>,
    privacy: SchemaPrivacy,
    meta: SchemaMeta,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .Subtle {
            color: var(--colorNeutralForeground3);
        }

        .TagWrap {
            flex-wrap: wrap;
        }

        @media (max-width: 599px) {
            .MetaGrid {
                grid-template-columns: 1fr;
            }
        }
    };

    let databases = StoredValue::new(databases);
    let description = meta.description.clone().unwrap_or_else(|| "—".to_string());

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-overview" data-testid="valence-schema-overview-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Overview"
                        description="Summary of this table and how it is stored."
                    />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <Grid config=GridConfig::with_gaps(2, 16, 16) class=class_names.meta_grid>
                            <GridItem>
                                <Stack config=META_STACK>
                                    <Caption1 class=class_names.subtle>"Table Name"</Caption1>
                                    <Body1>{schema_name.clone()}</Body1>
                                </Stack>
                            </GridItem>
                            <GridItem>
                                <Stack config=META_STACK>
                                    <ValenceHelpColumnHeader
                                        label="Databases in Use"
                                        info=view! {
                                            <Caption1>"Where this data is physically stored."</Caption1>
                                        }.into_any()
                                    />
                                    <Flex gap=SpacingSize::Size80.flex_gap() class=class_names.tag_wrap>
                                        <For each=move || databases.get_value() key=|db| db.clone() let:db>
                                            { view! { <Tag>{db}</Tag> } }
                                        </For>
                                    </Flex>
                                </Stack>
                            </GridItem>
                            <GridItem>
                                <Stack config=META_STACK>
                                    <ValenceHelpColumnHeader
                                        label="Privacy – Read"
                                        info=view! {
                                            <Caption1>"Who can view records in this table by default."</Caption1>
                                        }.into_any()
                                    />
                                    <Body1>{privacy.read.clone()}</Body1>
                                </Stack>
                            </GridItem>
                            <GridItem>
                                <Stack config=META_STACK>
                                    <ValenceHelpColumnHeader
                                        label="Privacy – Write"
                                        info=view! {
                                            <Caption1>"Who can create or change records in this table by default."</Caption1>
                                        }.into_any()
                                    />
                                    <Body1>{privacy.write.clone()}</Body1>
                                </Stack>
                            </GridItem>
                        </Grid>
                        <div data-testid="valence-schema-description">
                            <Stack config=META_STACK>
                                <Caption1 class=class_names.subtle>"Description"</Caption1>
                                <Body1>{description}</Body1>
                            </Stack>
                        </div>
                    </Stack>
                </CardContent>
                <CardFooter>
                    <ValenceHelpColumnHeader
                        label="Retention & stats"
                        info=view! {
                            <Caption1>"How long records are kept, approximate row count, and default owner label."</Caption1>
                        }.into_any()
                    />
                    <Caption1 class=class_names.subtle>
                        {format!("Retention: {} • Rows: {} • Owner: {}",
                            meta.retention.clone(),
                            meta.row_count,
                            meta.owner.clone())}
                    </Caption1>
                </CardFooter>
            </Card>
        </div>
    }
}

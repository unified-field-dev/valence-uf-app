use leptos::prelude::*;
use orbital::components::{Body1, Caption1, Card, CardHeader, Subtitle2};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

#[component]
pub fn TraitOverviewCard(
    trait_name: String,
    field_count: usize,
    connection_count: usize,
    implementor_count: usize,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .MetaGrid {
            padding: 16px;
        }

        .MetaItem {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div id="valence-trait-overview" data-testid="valence-trait-overview-card" style="width: 100%;">
            <Card class=class_names.card>
            <CardHeader>
                <Subtitle2>"Overview"</Subtitle2>
            </CardHeader>
            <Grid config=GridConfig::with_gaps(2, 16, 16) class=class_names.meta_grid>
                <GridItem><div class=class_names.meta_item>
                    <Caption1>"Trait Name"</Caption1>
                    <Body1>{trait_name}</Body1>
                </div></GridItem>
                <GridItem><div class=class_names.meta_item>
                    <Caption1>"Fields"</Caption1>
                    <Body1>{field_count.to_string()}</Body1>
                </div></GridItem>
                <GridItem><div class=class_names.meta_item>
                    <Caption1>"Connections"</Caption1>
                    <Body1>{connection_count.to_string()}</Body1>
                </div></GridItem>
                <GridItem><div class=class_names.meta_item>
                    <Caption1>"Implementing Schemas"</Caption1>
                    <Body1>{implementor_count.to_string()}</Body1>
                </div></GridItem>
            </Grid>
            </Card>
        </div>
    }
}

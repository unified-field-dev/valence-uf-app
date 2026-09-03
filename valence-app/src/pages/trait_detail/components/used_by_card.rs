use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{
    Body1, Caption1, Card, CardContent, CardHeader, CardSectionBorder, Subtitle2, Text, TextTag,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

#[component]
pub fn UsedByCard(implementors: Vec<String>) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .ListItem {
            gap: 8px;
            align-items: center;
        }
    };

    let implementors = StoredValue::new(implementors);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-trait-used-by" data-testid="valence-trait-used-by-card" style="width: 100%;">
            <Card class=class_names.card gap=FlexGap::Size(0)>
            <CardHeader>
                <Subtitle2>"Used By"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true>
                    {move || {
                        let imp = implementors.get_value();
                        if imp.is_empty() {
                            Some(view! {
                                <Caption1>"No schemas implement this trait."</Caption1>
                            })
                        } else {
                            None
                        }
                    }}
                    <For each=move || implementors.get_value() key=|s| s.clone() let:schema_name>
                        {
                            let href = valence_backend::valence_schema_path(&schema_name);
                            view! {
                                <>
                                    <Flex class=class_names.list_item align=FlexAlign::Center>
                                        <Badge>"Table"</Badge>
                                        <Body1>
                                            <A href=href>
                                                <Button appearance=ButtonAppearance::Subtle>
                                                    <Text tag=TextTag::Code font=TextFont::Monospace>{schema_name.clone()}</Text>
                                                </Button>
                                            </A>
                                        </Body1>
                                    </Flex>
                                    <CardSectionBorder />
                                </>
                            }
                        }
                    </For>
                </Flex>
            </CardContent>
            </Card>
        </div>
    }
}

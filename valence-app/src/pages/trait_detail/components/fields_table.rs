use leptos::prelude::*;
use orbital::components::{
    Body1, Caption1, Card, CardContent, CardHeader, CardSectionBorder, Subtitle2, Text, TextTag,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{FieldTypeDisplay, FieldTypeTypography};
use crate::server::TraitFieldInfo;

#[component]
pub fn TraitFieldsTable(trait_name: String, fields: Vec<TraitFieldInfo>) -> impl IntoView {
    let trait_name = StoredValue::new(trait_name);
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .Actions {
            gap: 8px;
        }
    };

    let fields = StoredValue::new(fields);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-trait-fields" data-testid="valence-trait-fields-table-card" style="width: 100%;">
            <Card class=class_names.card gap=FlexGap::Size(0)>
            <CardHeader>
                <Subtitle2>"Fields"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true>
                    {move || {
                        let f = fields.get_value();
                        if f.is_empty() {
                            Some(view! {
                                <Caption1>"No fields defined."</Caption1>
                            })
                        } else {
                            None
                        }
                    }}
                    <For each=move || fields.get_value() key=|f| f.name.clone() let:f>
                        {
                            let field_name = f.name.clone();
                            let type_field_name = f.name.clone();
                            let field_type = f.field_type.clone();
                            view! {
                                <>
                                    <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                                        <Body1>
                                            <Text tag=TextTag::Code font=TextFont::Monospace>{field_name.clone()}</Text>
                                        </Body1>
                                        <Flex class=class_names.actions align=FlexAlign::Center>
                                            <FieldTypeDisplay
                                                field_type=field_type
                                                field_name=type_field_name
                                                context_name=trait_name.get_value()
                                                typography=FieldTypeTypography::Caption
                                            />
                                            {if f.required {
                                                view! { <Badge>"required"</Badge> }.into_any()
                                            } else {
                                                view! { <Badge color=BadgeColor::Important>"optional"</Badge> }.into_any()
                                            }}
                                        </Flex>
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

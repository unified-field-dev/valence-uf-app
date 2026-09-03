use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, CardHeader};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::ValenceHelpCardHeader;

#[component]
pub fn TraitsCard(traits: Vec<String>) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .ListItem { gap: 8px; align-items: center; }
        .Code {
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
            font-size: 13px;
            background-color: var(--colorNeutralBackground3);
            padding: 2px 6px;
            border-radius: 4px;
        }
        .Subtle { color: var(--colorNeutralForeground3); }
    };

    let traits = StoredValue::new(traits);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-traits" data-testid="valence-schema-traits-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Traits"
                        description="Shared behavior this table inherits — fields, connections, or policies."
                    />
                <CardContent>
                    <Flex vertical=true>
                        {move || {
                            if traits.get_value().is_empty() {
                                Some(view! { <Caption1 class=class_names.subtle>"No traits implemented."</Caption1> })
                            } else { None }
                        }}
                        <For each=move || traits.get_value() key=|t| t.clone() let:t>
                            {
                                let href = valence_backend::valence_trait_path(&t);
                                view! {
                                    <Flex class=class_names.list_item align=FlexAlign::Center>
                                        <Badge>"Trait"</Badge>
                                        <Body1>
                                            <A href=href>
                                                <Button appearance=ButtonAppearance::Subtle>
                                                    <code class=class_names.code>{t.clone()}</code>
                                                </Button>
                                            </A>
                                        </Body1>
                                    </Flex>
                                }
                            }
                        </For>
                    </Flex>
                </CardContent>
            </Card>
        </div>
    }
}

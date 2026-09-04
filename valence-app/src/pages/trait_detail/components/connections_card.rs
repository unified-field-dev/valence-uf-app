use leptos::prelude::*;
use orbital::components::{Caption1, Card};
use orbital::primitives::*;

use crate::components::{SchemaConnectionRow, ValenceHelpCardHeader};
use crate::server::SchemaConnection;

#[component]
pub fn TraitConnectionsCard(
    trait_name: String,
    connections: Vec<SchemaConnection>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .List {
            gap: 12px;
            padding: 16px;
        }
    };

    let connections = StoredValue::new(connections);
    let trait_name_stored = StoredValue::new(trait_name);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-trait-connections" data-testid="valence-trait-connections-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Connections"
                    description="Connection definitions shared by every table that implements this trait."
                    info=view! {
                        <Caption1>
                            "These links are inherited by schemas that include this trait."
                        </Caption1>
                    }.into_any()
                />
                <Flex vertical=true class=class_names.list>
                    {move || {
                        let c = connections.get_value();
                        if c.is_empty() {
                            Some(view! {
                                <Caption1>"No connections defined."</Caption1>
                            })
                        } else {
                            None
                        }
                    }}
                    <For each=move || connections.get_value() key=|c| format!("{}-{}", c.from_field, c.to_table) let:c>
                        <SchemaConnectionRow
                            schema_name=trait_name_stored.get_value()
                            connection=c
                        />
                    </For>
                </Flex>
            </Card>
        </div>
    }
}

use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent};
use orbital::primitives::*;

use crate::components::{
    code_style_classes, InverseSchemaConnectionRow, SchemaConnectionRow, ValenceHelpCardHeader,
};
use crate::server::{InverseSchemaConnection, SchemaConnection};

#[component]
pub fn ConnectionsCard(
    schema_name: String,
    connections: Vec<SchemaConnection>,
    #[prop(optional)] inverse_connections: Vec<InverseSchemaConnection>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .SectionLabel {
            font-size: 12px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            color: var(--colorNeutralForeground3);
            padding-top: 4px;
        }
    };

    let styles = code_style_classes();
    let connections = StoredValue::new(connections);
    let inverse_connections = StoredValue::new(inverse_connections);
    let schema_name_stored = StoredValue::new(schema_name);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-connections" data-testid="valence-schema-connections-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Connections"
                    description="How this table links to other tables."
                    info=view! {
                        <Caption1>
                            "Outgoing links point from this table to others. Incoming links show which tables reference this one."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    <Flex vertical=true>
                        {move || {
                            let conns = connections.get_value();
                            if conns.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <div class=class_names.section_label>"Outgoing"</div>
                                    <Caption1 class=styles.subtle.clone()>
                                        "Links from this table to related records."
                                    </Caption1>
                                })
                            }
                        }}
                        <For each=move || connections.get_value() key=|c| format!("{}-{}", c.from_field, c.to_table) let:c>
                            <SchemaConnectionRow
                                schema_name=schema_name_stored.get_value()
                                connection=c
                            />
                        </For>

                        {move || {
                            let inv = inverse_connections.get_value();
                            if inv.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <div class=class_names.section_label>"Incoming"</div>
                                    <Caption1 class=styles.subtle.clone()>
                                        "Other tables that reference this one."
                                    </Caption1>
                                })
                            }
                        }}
                        <For
                            each=move || inverse_connections.get_value()
                            key=|c| format!("inv-{}-{}", c.from_table, c.from_field)
                            let:c
                        >
                            <InverseSchemaConnectionRow connection=c />
                        </For>
                    </Flex>
                </CardContent>
            </Card>
        </div>
    }
}

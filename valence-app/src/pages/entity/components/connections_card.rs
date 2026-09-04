use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, SectionTitle, Stack, StackConfig};
use orbital::primitives::*;

use super::connection_row::{ConnectionRow, InverseConnectionRow};
use crate::components::ValenceHelpCardHeader;
use crate::server::{InverseConnectionData, SchemaConnection};

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

/// Sentinel value used to indicate a connection value was hidden by privacy.
const REDACTED_VALUE: &str = "-----";

#[component]
pub fn EntityConnectionsCard(
    schema_name: String,
    schema_connections: Vec<SchemaConnection>,
    record_values: std::collections::BTreeMap<String, String>,
    #[prop(optional)] hidden_fields: Vec<String>,
    #[prop(optional)] inverse_connections: Vec<InverseConnectionData>,
) -> impl IntoView {
    let schema_name_stored = StoredValue::new(schema_name);
    let schema_connections = StoredValue::new(schema_connections);
    let record_values = StoredValue::new(record_values);
    let hidden_fields = StoredValue::new(hidden_fields);
    let inverse_connections = StoredValue::new(inverse_connections);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
    };

    view! {
        <style>{style_sheet}</style>
        <div id="valence-entity-connections" data-testid="valence-entity-connections-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Connections"
                    description="Links from this record to other records."
                    info=view! {
                        <Caption1>
                            "Outgoing links show where this record points. Referenced By lists other records that point here."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    <Stack config=BODY_STACK>
                        {move || {
                            let conns = schema_connections.get_value();
                            if conns.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <SectionTitle>"Outgoing"</SectionTitle>
                                })
                            }
                        }}
                        <For
                            each=move || schema_connections.get_value()
                            key=|c| format!("{}-{}", c.from_field, c.to_table)
                            let:c
                        >
                            {
                                let is_hidden = hidden_fields.with_value(|hf| hf.contains(&c.from_field));
                                let value = if is_hidden {
                                    REDACTED_VALUE.to_string()
                                } else {
                                    record_values.with_value(|rv| {
                                        rv.get(&c.from_field).cloned().unwrap_or_default()
                                    })
                                };
                                view! {
                                    <ConnectionRow
                                        schema_name=schema_name_stored.get_value()
                                        from_field=c.from_field.clone()
                                        to_table=c.to_table.clone()
                                        label=c.label.clone()
                                        value=value
                                        cardinality=c.cardinality.clone()
                                        on_delete=c.on_delete.clone()
                                        target_trait=c.target_trait.clone().unwrap_or_default()
                                        trait_source=c.trait_source.clone().unwrap_or_default()
                                    />
                                }
                            }
                        </For>

                        {move || {
                            let inv = inverse_connections.get_value();
                            if inv.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <SectionTitle>"Referenced By"</SectionTitle>
                                    <Caption1>"Other records pointing to this one."</Caption1>
                                })
                            }
                        }}
                        <For
                            each=move || inverse_connections.get_value()
                            key=|c| format!("inv-{}-{}", c.from_table, c.from_field)
                            let:c
                        >
                            <InverseConnectionRow inv=c />
                        </For>
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

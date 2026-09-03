use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, EmptyState};
use orbital::primitives::*;

use crate::components::bordered_table_styles;
use crate::components::{
    BadgeHint, FieldTypeDisplay, FieldTypeTypography, ValenceHelpCardHeader,
    ValenceHelpColumnHeader,
};
use crate::server::SchemaField;

#[component]
pub fn EntityFieldsCard(
    schema_name: String,
    schema_fields: Vec<SchemaField>,
    record_values: std::collections::BTreeMap<String, String>,
    #[prop(optional)] hidden_fields: Vec<String>,
) -> impl IntoView {
    let schema_name = StoredValue::new(schema_name);
    let schema_fields = StoredValue::new(schema_fields);
    let record_values = StoredValue::new(record_values);
    let hidden_fields = StoredValue::new(hidden_fields);
    let table_styles = bordered_table_styles();

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
    };

    view! {
        <style>{style_sheet}</style>
        <style>{table_styles.sheet.clone()}</style>
        <div id="valence-entity-fields" data-testid="valence-entity-fields-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Fields"
                    description="Values stored on this specific record."
                    info=view! {
                        <Caption1>
                            "A row of dashes means the value is hidden by privacy rules for your current view."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    {move || {
                        let fields = schema_fields.get_value();
                        if fields.is_empty() {
                            view! {
                                <EmptyState message="No fields defined" />
                            }.into_any()
                        } else {
                            view! {
                                <div class=table_styles.table_scroller.clone()>
                                    <div class=table_styles.table_wrap.clone()>
                                        <Table class=table_styles.compact_table.clone()>
                                            <TableHeader>
                                                <TableRow>
                                                    <TableHeaderCell>
                                                        <ValenceHelpColumnHeader label="Field" />
                                                    </TableHeaderCell>
                                                    <TableHeaderCell>
                                                        <ValenceHelpColumnHeader label="Type" />
                                                    </TableHeaderCell>
                                                    <TableHeaderCell>
                                                        <ValenceHelpColumnHeader label="Value" />
                                                    </TableHeaderCell>
                                                </TableRow>
                                            </TableHeader>
                                            <TableBody>
                                                <For
                                                    each=move || schema_fields.get_value()
                                                    key=|f| f.name.clone()
                                                    let:f
                                                >
                                                    {
                                                        let field_name = f.name.clone();
                                                        let is_hidden = hidden_fields.with_value(|hf| hf.contains(&field_name));
                                                        let value = record_values.with_value(|rv| {
                                                            rv.get(&field_name)
                                                                .cloned()
                                                                .unwrap_or_else(|| "—".to_string())
                                                        });
                                                        let fk = f.fk.clone();
                                                        let row_field_name = field_name.clone();
                                                        let type_field_name = field_name.clone();

                                                        let value_view = if is_hidden {
                                                            view! {
                                                                <Body1 class=table_styles.redacted.clone()>"-----"</Body1>
                                                            }.into_any()
                                                        } else if let Some(fk) = &fk {
                                                            if value != "—" {
                                                                let id = super::strip_record_id_prefix(&value);
                                                                let href = valence_backend::valence_entity_path(&fk.ref_table, id);
                                                                view! {
                                                                    <A href=href>
                                                                        <Button appearance=ButtonAppearance::Subtle>
                                                                            {value.clone()}
                                                                        </Button>
                                                                    </A>
                                                                }
                                                                .into_any()
                                                            } else {
                                                                view! {
                                                                    <Body1 class=table_styles.subtle.clone()>{value.clone()}</Body1>
                                                                }.into_any()
                                                            }
                                                        } else {
                                                            view! {
                                                                <Body1>{value.clone()}</Body1>
                                                            }.into_any()
                                                        };

                                                        view! {
                                                            <TableRow>
                                                                <TableCell>
                                                                    <code class=table_styles.code.clone()>{row_field_name.clone()}</code>
                                                                    {if f.primary {
                                                                        view! {
                                                                            <BadgeHint
                                                                                label="PK".to_string()
                                                                                tooltip="Primary key — uniquely identifies this record"
                                                                            />
                                                                        }.into_any()
                                                                    } else {
                                                                        view! {}.into_any()
                                                                    }}
                                                                </TableCell>
                                                                <TableCell>
                                                                    <FieldTypeDisplay
                                                                        field_type=f.field_type.clone()
                                                                        field_name=type_field_name
                                                                        context_name=schema_name.get_value()
                                                                        typography=FieldTypeTypography::Body
                                                                        subtle_class=table_styles.subtle.clone()
                                                                    />
                                                                </TableCell>
                                                                <TableCell>{value_view}</TableCell>
                                                            </TableRow>
                                                        }
                                                    }
                                                </For>
                                            </TableBody>
                                        </Table>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </CardContent>
            </Card>
        </div>
    }
}

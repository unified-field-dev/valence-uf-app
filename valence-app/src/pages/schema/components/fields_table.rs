use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{BadgeHint, FieldTypeDisplay, FieldTypeTypography, ValenceHelpCardHeader};
use crate::server::SchemaField;

#[component]
pub fn FieldsTable(schema_name: String, fields: Vec<SchemaField>) -> impl IntoView {
    let schema_name = StoredValue::new(schema_name);
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .Row {
            padding: 12px 0;
            border-bottom: 1px solid var(--colorNeutralStroke2);
        }

        .Row:last-child {
            border-bottom: 0;
        }

        .RowTop {
            gap: 12px;
            align-items: center;
        }

        .RowMeta {
            margin-top: 4px;
        }

        .Code {
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
            font-size: 13px;
            background-color: var(--colorNeutralBackground3);
            padding: 2px 6px;
            border-radius: 4px;
        }

        .Subtle {
            color: var(--colorNeutralForeground3);
        }

        .Actions {
            gap: 8px;
            align-items: center;
            justify-content: flex-end;
        }
    };

    let fields = StoredValue::new(fields);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-fields" data-testid="valence-schema-fields-table-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Fields"
                    description="Columns stored for each record in this table."
                    info=view! {
                        <Caption1>
                            "Optional means the field can be empty. Indexed fields are faster to search. Unique fields cannot repeat across records."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    <Flex vertical=true>
                        <For each=move || fields.get_value() key=|f| f.name.clone() let:f>
                            {
                                let fk_ref = f.fk.clone();
                                let trait_source = f.trait_source.clone();
                                let field_name = f.name.clone();
                                let type_field_name = f.name.clone();
                                let field_type = f.field_type.clone();
                                view! {
                                    <div class=class_names.row>
                                        <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                                            <div>
                                                <Flex class=class_names.row_top align=FlexAlign::Center>
                                                    <Body1>
                                                        <code class=class_names.code>{field_name.clone()}</code>
                                                    </Body1>
                                                    <FieldTypeDisplay
                                                        field_type=field_type
                                                        field_name=type_field_name
                                                        context_name=schema_name.get_value()
                                                        typography=FieldTypeTypography::Caption
                                                        subtle_class=class_names.subtle.to_string()
                                                    />
                                                </Flex>
                                                <div class=class_names.row_meta>
                                                    <Caption1 class=class_names.subtle>
                                                        {format!(
                                                            "optional: {} • searchable: {} • unique: {} • default: {}",
                                                            if f.nullable { "yes" } else { "no" },
                                                            if f.indexed { "yes" } else { "no" },
                                                            if f.unique { "yes" } else { "no" },
                                                            f.default.clone().unwrap_or_else(|| "—".to_string())
                                                        )}
                                                    </Caption1>
                                                </div>
                                            </div>
                                            <Flex class=class_names.actions align=FlexAlign::Center>
                                                {if f.primary {
                                                    view! {
                                                        <BadgeHint
                                                            label="PK".to_string()
                                                            tooltip="Primary key — uniquely identifies each record"
                                                        />
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }}
                                                {if let Some(fk) = &fk_ref {
                                                    let ref_table = fk.ref_table.clone();
                                                    view! {
                                                        <A href=valence_backend::valence_schema_path(&ref_table)>
                                                            <Button appearance=ButtonAppearance::Subtle>
                                                                {format!("FK → {}", ref_table)}
                                                            </Button>
                                                        </A>
                                                    }.into_any()
                                                } else {
                                                    view! {}.into_any()
                                                }}
                                                {match trait_source {
                                                    Some(trait_name) => {
                                                        let href = valence_backend::valence_trait_path(&trait_name);
                                                        view! {
                                                            <Caption1 class=class_names.subtle>
                                                                "Inherited from "
                                                                <A href=href>
                                                                    <Button appearance=ButtonAppearance::Subtle>
                                                                        {trait_name}
                                                                    </Button>
                                                                </A>
                                                            </Caption1>
                                                        }.into_any()
                                                    }
                                                    None => view! {}.into_any(),
                                                }}
                                            </Flex>
                                        </Flex>
                                    </div>
                                }
                            }
                        </For>
                    </Flex>
                </CardContent>
            </Card>
        </div>
    }
}

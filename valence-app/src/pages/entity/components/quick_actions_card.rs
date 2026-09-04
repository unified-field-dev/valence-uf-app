use leptos::prelude::*;
use orbital::components::{Card, CardContent, CardHeader};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{download_text_file, ValenceHelpCardHeader};
use crate::server::EntityRecord;

#[component]
pub fn EntityQuickActionsCard(schema_name: String, record: EntityRecord) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .Actions { display: flex; gap: 8px; flex-wrap: wrap; }
    };

    let schema_name_stored = StoredValue::new(schema_name);
    let record_stored = StoredValue::new(record);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-entity-quick-actions" data-testid="valence-entity-quick-actions-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Quick Actions"
                        description="Export downloads a JSON copy of this record."
                    />
                <CardContent>
                    <div class=class_names.actions>
                        <div id="valence-entity-export">
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(move |_| {
                                let schema_name = schema_name_stored.get_value();
                                let record = record_stored.get_value();
                                let export = serde_json::json!({
                                    "schema": schema_name,
                                    "id": record.id,
                                    "values": record.values,
                                });
                                let json = serde_json::to_string_pretty(&export).unwrap_or_default();
                                let filename = format!("{}.{}.json", schema_name, record.id);
                                download_text_file(&filename, &json, "application/json");
                            })
                        >
                            "Export"
                        </Button>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

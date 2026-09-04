use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Card, CardContent, CardHeader};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{download_text_file, ValenceHelpCardHeader};
use crate::server::{get_schema_samples, Schema};

#[component]
pub fn QuickActionsCard(schema: Schema, schema_name: String) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .Actions { display: flex; gap: 8px; flex-wrap: wrap; }
    };

    let schema_stored = StoredValue::new(schema);
    let schema_name_stored = StoredValue::new(schema_name.clone());

    let latest_res = Resource::new(
        move || schema_name.clone(),
        |name| async move { get_schema_samples(name, 1).await },
    );

    let latest_id = Memo::new(move |_| {
        latest_res
            .get()
            .and_then(|r| r.ok())
            .and_then(|samples| samples.first().map(|s| s.id.clone()))
    });

    let open_disabled = Memo::new(move |_| {
        latest_res.get().is_none()
            || latest_res.get().is_some_and(|r| r.is_err())
            || latest_id.get().is_none()
    });

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-quick-actions" data-testid="valence-schema-quick-actions-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Quick Actions"
                        description="Shortcuts to open a recent record or download this table definition."
                    />
                <CardContent>
                    <div class=class_names.actions>
                        {move || {
                            let id = latest_id.get();
                            let schema_name = schema_name_stored.get_value();
                            if let Some(id) = id {
                                let href = valence_backend::valence_entity_path(&schema_name, &id);
                                view! {
                                    <div id="valence-schema-open-latest">
                                    <A href=href>
                                        <Button appearance=ButtonAppearance::Primary>"Open latest record"</Button>
                                    </A>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div id="valence-schema-open-latest">
                                    <Button
                                        appearance=ButtonAppearance::Primary
                                        disabled=Signal::derive(move || open_disabled.get())
                                    >
                                        "Open latest record"
                                    </Button>
                                    </div>
                                }.into_any()
                            }
                        }}
                        <div id="valence-schema-export">
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(move |_| {
                                let schema = schema_stored.get_value();
                                let name = schema_name_stored.get_value();
                                let json = serde_json::to_string_pretty(&schema).unwrap_or_default();
                                download_text_file(&format!("{}.schema.json", name), &json, "application/json");
                            })
                        >
                            "Export Schema"
                        </Button>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

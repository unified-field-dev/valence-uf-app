use leptos::prelude::*;

use crate::components::DataUsesPanel;
use crate::server::{get_schema_data_uses, DataUseRow};
use orbital::components::{Body1, MessageBar, MessageBarIntent};

/// Schema detail card: declared uses for this table plus trait fan-out.
#[component]
pub fn SchemaDataUsesCard(schema_name: String) -> impl IntoView {
    let uses_res = Resource::new(
        move || schema_name.clone(),
        |name| async move {
            if name.is_empty() {
                return Ok(Vec::<DataUseRow>::new());
            }
            get_schema_data_uses(name).await
        },
    );

    view! {
        <Suspense fallback=move || view! { <Body1>"Loading data uses..."</Body1> }>
            {move || match uses_res.get() {
                Some(Ok(rows)) => view! {
                    <DataUsesPanel
                        panel_id="valence-schema-data-uses"
                        title="Data uses"
                        description="Why product code reads or changes rows in this schema."
                        rows=rows
                    />
                }
                .into_any(),
                Some(Err(err)) => view! {
                    <MessageBar intent=MessageBarIntent::Error>
                        {format!("Failed to load data uses: {err}")}
                    </MessageBar>
                }
                .into_any(),
                None => view! { <Body1>"Loading data uses..."</Body1> }.into_any(),
            }}
        </Suspense>
    }
}

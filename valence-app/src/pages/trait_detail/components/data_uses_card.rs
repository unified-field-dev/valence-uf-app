use leptos::prelude::*;

use crate::components::DataUsesPanel;
use crate::server::{get_trait_data_uses, DataUseRow};
use orbital::components::{Body1, MessageBar, MessageBarIntent};

/// Trait detail card: declared uses that target this trait only.
#[component]
pub fn TraitDataUsesCard(trait_name: String) -> impl IntoView {
    let uses_res = Resource::new(
        move || trait_name.clone(),
        |name| async move {
            if name.is_empty() {
                return Ok(Vec::<DataUseRow>::new());
            }
            get_trait_data_uses(name).await
        },
    );

    view! {
        <Suspense fallback=move || view! { <Body1>"Loading data uses..."</Body1> }>
            {move || match uses_res.get() {
                Some(Ok(rows)) => view! {
                    <DataUsesPanel
                        panel_id="valence-trait-data-uses"
                        title="Data uses"
                        description="Why product code queries or changes rows through this trait."
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

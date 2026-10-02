//! Unscoped data uses page (`/valence/unscoped-uses`).

use leptos::prelude::*;
use orbital::components::{Body1, ContentContainer, Title3};
use orbital::primitives::{Flex, FlexGap};

use crate::components::{DataUsesLoadError, DataUsesPanel};
use crate::server::{get_unscoped_data_uses, DataUseRow};

/// Lists QueryCore / raw declared uses that are not schema- or trait-scoped.
#[component]
pub fn ValenceUnscopedUsesPage() -> impl IntoView {
    let uses_res = Resource::new(|| (), |_| async move { get_unscoped_data_uses().await });

    view! {
        <ContentContainer data_testid="valence-unscoped-uses-page">
            <div id="valence-unscoped-uses">
                <Flex vertical=true gap=FlexGap::Large>
                    <Title3>"Unscoped uses"</Title3>
                    <Body1>
                        "Declared Valence access that is not tied to one schema or trait — usually QueryCore graph walks."
                    </Body1>
                    <Suspense fallback=move || view! { <Body1>"Loading unscoped uses..."</Body1> }>
                        {move || match uses_res.get() {
                            Some(Ok(rows)) => view! {
                                <DataUsesPanel
                                    panel_id="valence-unscoped-data-uses"
                                    title="Unscoped data uses"
                                    description="Purpose strings for cross-schema or raw QueryCore entry points."
                                    rows=rows
                                />
                            }
                            .into_any(),
                            Some(Err(err)) => view! {
                                <DataUsesLoadError error=err what="unscoped uses" />
                            }
                            .into_any(),
                            None => view! { <Body1>"Loading..."</Body1> }.into_any(),
                        }}
                    </Suspense>
                </Flex>
            </div>
        </ContentContainer>
    }
}

mod components;

pub use components::IterRunRow;

use leptos::prelude::*;
use orbital::components::{Caption1, ContentContainer, SpacingSize, Title3};
use orbital::primitives::*;

use components::IterRunsListSection;

#[component]
pub fn ValenceIterIndexPage() -> impl IntoView {
    view! {
        <div id="valence-iters-page">
        <ContentContainer data_testid="valence-iter-index-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Iter runs"</Title3>
                <Caption1>"Recent executions across all schemas."</Caption1>
                <div id="valence-iters-list">
                    <IterRunsListSection />
                </div>
            </Flex>
        </ContentContainer>
        </div>
    }
}

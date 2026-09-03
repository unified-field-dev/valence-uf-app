use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::use_interval_fn;
use orbital::components::{Body1, Caption1, ContentContainer, Title3};
use orbital::primitives::*;

use crate::server::{list_deletion_runs, DeletionRunView};

#[component]
pub fn ValenceDeletionIndexPage() -> impl IntoView {
    let tick = RwSignal::new(0u32);
    let res = Resource::new(
        move || tick.get(),
        |_| async move { list_deletion_runs(100).await },
    );

    use_interval_fn(
        move || {
            tick.update(|n| *n += 1);
        },
        3000,
    );

    view! {
        <div id="valence-deletions-page">
        <ContentContainer data_testid="valence-deletion-index-page">
            <Title3>"Deletions"</Title3>
            <Suspense fallback=move || view! { <Body1>"Loading…"</Body1> }>
                {move || match res.get() {
                    Some(Ok(rows)) => {
                        let rows: Vec<DeletionRunView> = rows;
                        view! {
                            <div id="valence-deletions-list">
                            <Flex vertical=true gap=FlexGap::Small>
                                <For each=move || rows.clone() key=|r| r.run_id.clone() let:run>
                                    {
                                        let href = valence_backend::valence_deletion_run_path(
                                            &run.root_table,
                                            &run.run_id,
                                        );
                                        let prog = format!(
                                            "{}/{}",
                                            run.completed_steps + run.failed_steps,
                                            run.total_steps.max(1),
                                        );
                                        view! {
                                            <Flex align=FlexAlign::Center gap=FlexGap::Medium>
                                                <Body1>{run.root_table.clone()}</Body1>
                                                <code>{run.root_record_id.clone()}</code>
                                                <Badge>{run.status.clone()}</Badge>
                                                <Caption1>{prog}</Caption1>
                                                <Caption1>{run.requested_at.clone()}</Caption1>
                                                <A href=href>
                                                    <Button appearance=ButtonAppearance::Subtle>"Open"</Button>
                                                </A>
                                            </Flex>
                                        }
                                    }
                                </For>
                            </Flex>
                            </div>
                        }.into_any()
                    }
                    Some(Err(e)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                    }.into_any(),
                    None => view! { <Body1>"…"</Body1> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
        </div>
    }
}

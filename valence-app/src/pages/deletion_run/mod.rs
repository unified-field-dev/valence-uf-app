use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use leptos_use::use_interval_fn;
use orbital::components::{Body1, Caption1, Card, CardHeader, ContentContainer, Subtitle2, Title3};
use orbital::primitives::*;

use crate::server::{
    cancel_deletion_run, get_deletion_run, list_deletion_run_steps, DeletionRunView,
    DeletionStepView,
};

#[component]
pub fn ValenceDeletionRunPage() -> impl IntoView {
    let params = use_params_map();
    let schema_name = Memo::new(move |_| {
        params
            .get()
            .get("schema_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });
    let run_id = Memo::new(move |_| {
        params
            .get()
            .get("run_id")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let tick = RwSignal::new(0u32);
    let run_res = Resource::new(
        move || (run_id.get(), tick.get()),
        |(rid, _)| async move { get_deletion_run(rid).await },
    );

    let steps_res = Resource::new(
        move || (run_id.get(), tick.get()),
        |(rid, _)| async move { list_deletion_run_steps(rid).await },
    );

    let stop_poll = RwSignal::new(false);
    Effect::new(move |_| {
        if let Some(Ok(Some(r))) = run_res.get() {
            let t = r.status.as_str();
            stop_poll.set(matches!(t, "completed" | "failed" | "cancelled"));
        }
    });

    use_interval_fn(
        move || {
            if !stop_poll.get_untracked() {
                tick.update(|n| *n += 1);
            }
        },
        2000,
    );

    let cancel = Action::new(move |rid: &String| {
        let rid = rid.clone();
        async move { cancel_deletion_run(rid).await }
    });

    let back_href = Memo::new(move |_| valence_backend::valence_schema_path(&schema_name.get()));

    view! {
        <ContentContainer data_testid="valence-deletion-run-page">
            <div id="valence-deletion-run-back">
            <A href=move || back_href.get()>
                <Button appearance=ButtonAppearance::Subtle>"← Back to schema"</Button>
            </A>
            </div>
            <Suspense fallback=move || view! { <Body1>"Loading run…"</Body1> }>
                {move || match run_res.get() {
                    Some(Ok(Some(run))) => {
                        let run: DeletionRunView = run;
                        let total = run.total_steps.max(1);
                        let done = run.completed_steps + run.failed_steps;
                        let frac = (done as f64 / total as f64).clamp(0.0, 1.0);
                        let bar = Signal::derive(move || frac);
                        let rid_cancel = run.run_id.clone();
                        let terminal = matches!(
                            run.status.as_str(),
                            "completed" | "failed" | "cancelled"
                        );
                        view! {
                            <div>
                            <div id="valence-deletion-run-header">
                            <Title3>{format!("Deletion run {}", run.run_id)}</Title3>
                            <Caption1>
                                "Status: " {run.status.clone()}
                            </Caption1>
                            </div>
                            <Card>
                                <div id="valence-deletion-run-progress">
                                <CardHeader>
                                    <Subtitle2>"Progress"</Subtitle2>
                                </CardHeader>
                                <ProgressBar value=bar />
                                <Caption1>
                                    {format!("{} / {} steps ({:.0}%)", done, run.total_steps, frac * 100.0)}
                                </Caption1>
                                </div>
                                <div id="valence-deletion-run-cancel">
                                <Button
                                    appearance=ButtonAppearance::Secondary
                                    disabled=Signal::derive({
                                        let p = cancel.pending();
                                        move || p.get() || terminal
                                    })
                                    on_click=Callback::new(move |_| { cancel.dispatch(rid_cancel.clone()); })
                                >
                                    "Cancel run"
                                </Button>
                                </div>
                            </Card>

                            <div id="valence-deletion-run-steps">
                            <Suspense fallback=move || view! { <Body1>"Loading steps…"</Body1> }>
                                {move || match steps_res.get() {
                                    Some(Ok(steps)) => {
                                        let steps: Vec<DeletionStepView> = steps;
                                        view! {
                                            <Flex vertical=true gap=FlexGap::Small>
                                                <For each=move || steps.clone() key=|s| s.step_id.clone() let:s>
                                                    {
                                                        view! {
                                                            <Flex align=FlexAlign::Center gap=FlexGap::Medium>
                                                                <Body1>{s.record_table.clone()}</Body1>
                                                                <code>{s.record_id.clone()}</code>
                                                                <Caption1>{s.action.clone()}</Caption1>
                                                                <Caption1>{s.depth}</Caption1>
                                                                <Badge>{s.status.clone()}</Badge>
                                                            </Flex>
                                                        }
                                                    }
                                                </For>
                                            </Flex>
                                        }.into_any()
                                    }
                                    Some(Err(e)) => view! {
                                        <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                                    }.into_any(),
                                    None => view! { <Body1>"…"</Body1> }.into_any(),
                                }}
                            </Suspense>
                            </div>
                            </div>
                        }.into_any()
                    }
                    Some(Ok(None)) => view! { <MessageBar intent=MessageBarIntent::Warning>"Run not found"</MessageBar> }.into_any(),
                    Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar> }.into_any(),
                    None => view! { <Body1>"…"</Body1> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
    }
}

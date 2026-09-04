use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use leptos_use::use_interval_fn;
use orbital::components::{
    Body1, Caption1, Card, CardHeader, CardSectionBorder, ContentContainer, Subtitle2, Title3,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::server::{
    cancel_iter_run, get_iter_run, list_iter_run_batches, list_iter_run_errors, IterBatchView,
    IterRowErrorView, IterRunView,
};

#[component]
pub fn ValenceIterRunPage() -> impl IntoView {
    let params = use_params_map();
    let schema_name = Memo::new(move |_| {
        params
            .get()
            .get("schema_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });
    let run_id_memo = Memo::new(move |_| {
        params
            .get()
            .get("run_id")
            .map(|s| s.to_string())
            .unwrap_or_default()
    });

    let tick = RwSignal::new(0u32);
    let run_res = Resource::new(
        move || (run_id_memo.get(), tick.get()),
        |(rid, _)| async move { get_iter_run(rid).await },
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

    let errors_res = Resource::new(
        move || (run_id_memo.get(), tick.get()),
        |(rid, _)| async move { list_iter_run_errors(rid, 0, 50).await },
    );

    let batches_res = Resource::new(
        move || (run_id_memo.get(), tick.get()),
        |(rid, _)| async move { list_iter_run_batches(rid, 0, 50).await },
    );

    let cancel_action = Action::new(move |rid: &String| {
        let rid = rid.clone();
        async move { cancel_iter_run(rid).await }
    });

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Section { display: flex; flex-direction: column; gap: 16px; }
    };

    let back_href = Memo::new(move |_| valence_backend::valence_schema_path(&schema_name.get()));

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="valence-iter-run-page">
            <div class=class_names.section>
                <A href=move || back_href.get()>
                    <Button appearance=ButtonAppearance::Subtle>"← Back to schema"</Button>
                </A>

                <Suspense fallback=move || view! { <Body1>"Loading run…"</Body1> }>
                    {move || match run_res.get() {
                        Some(Ok(Some(run))) => {
                            let run: IterRunView = run;
                            let total = run.total_rows.max(1);
                            let done = run.processed_rows + run.skipped_rows + run.failed_rows;
                            let frac = (done as f64 / total as f64).clamp(0.0, 1.0);
                            let bar = Signal::derive(move || frac);
                            let title = format!(
                                "Iter run: {} on {}",
                                run.iter_name, run.target_table
                            );
                            let rid_cancel = run.run_id.clone();
                            view! {
                                <div id="valence-iter-run-header">
                                <Title3>{title}</Title3>
                                <Caption1>
                                    "Run ID: " {run.run_id.clone()}
                                    " — status: " {run.status.clone()}
                                </Caption1>
                                </div>

                                <div id="valence-iter-run-stats">
                                <Grid config=GridConfig::with_gaps(4, 16, 0)>
                                    <GridItem><Card><Caption1>"Total rows"</Caption1><Body1>{run.total_rows}</Body1></Card></GridItem>
                                    <GridItem><Card><Caption1>"Processed"</Caption1><Body1>{run.processed_rows}</Body1></Card></GridItem>
                                    <GridItem><Card><Caption1>"Skipped"</Caption1><Body1>{run.skipped_rows}</Body1></Card></GridItem>
                                    <GridItem><Card><Caption1>"Failed"</Caption1><Body1>{run.failed_rows}</Body1></Card></GridItem>
                                </Grid>
                                </div>

                                <Card>
                                    <div id="valence-iter-run-progress">
                                    <CardHeader>
                                        <Subtitle2>"Progress"</Subtitle2>
                                    </CardHeader>
                                    <ProgressBar value=bar />
                                    <Caption1>
                                        {format!(
                                            "{} / {} rows ({:.0}%)",
                                            done,
                                            total,
                                            frac * 100.0
                                        )}
                                    </Caption1>
                                    {run.error_message.clone().map(|m| view! {
                                        <MessageBar intent=MessageBarIntent::Warning>{m}</MessageBar>
                                    })}
                                    </div>
                                    <div id="valence-iter-run-cancel">
                                    <Button
                                        appearance=ButtonAppearance::Secondary
                                        disabled=Signal::derive(move || cancel_action.pending().get())
                                        on_click=Callback::new(move |_| { cancel_action.dispatch(rid_cancel.clone()); })
                                    >
                                        "Cancel run"
                                    </Button>
                                    {move || cancel_action.value().get().map(|r| match r {
                                        Ok(()) => view! {
                                            <MessageBar intent=MessageBarIntent::Success>"Cancellation requested."</MessageBar>
                                        }.into_any(),
                                        Err(e) => view! {
                                            <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                                        }.into_any(),
                                    })}
                                    </div>
                                </Card>
                            }.into_any()
                        }
                        Some(Ok(None)) => view! {
                            <MessageBar intent=MessageBarIntent::Warning>"Run not found"</MessageBar>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                        }.into_any(),
                        None => view! { <Body1>"…"</Body1> }.into_any(),
                    }}
                </Suspense>

                <div id="valence-iter-run-errors">
                <Card>
                    <CardHeader>
                        <Subtitle2>"Row errors (latest 50)"</Subtitle2>
                    </CardHeader>
                    <Suspense fallback=move || view! { <Body1>"Loading errors…"</Body1> }>
                        {move || match errors_res.get() {
                            Some(Ok(page)) => {
                                let items: Vec<IterRowErrorView> = page.items;
                                if items.is_empty() {
                                    view! { <Caption1>"No errors recorded."</Caption1> }.into_any()
                                } else {
                                    view! {
                                        <Flex vertical=true>
                                            <For each=move || items.clone() key=|e| e.id.clone() let:row>
                                                <>
                                                    <Flex vertical=true gap=FlexGap::Size(4)>
                                                        <Body1>
                                                            <strong>{row.row_id.clone()}</strong>
                                                            " — "
                                                            {row.error_kind.clone()}
                                                        </Body1>
                                                        <Caption1>{row.error_message.clone()}</Caption1>
                                                        <Caption1>{row.created_at.clone()}</Caption1>
                                                    </Flex>
                                                    <CardSectionBorder />
                                                </>
                                            </For>
                                        </Flex>
                                    }.into_any()
                                }
                            }
                            Some(Err(e)) => view! {
                                <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                            }.into_any(),
                            None => view! { <Body1>"…"</Body1> }.into_any(),
                        }}
                    </Suspense>
                </Card>
                </div>

                <div id="valence-iter-run-batches">
                <Card>
                    <CardHeader>
                        <Subtitle2>"Batches (latest 50)"</Subtitle2>
                    </CardHeader>
                    <Suspense fallback=move || view! { <Body1>"Loading batches…"</Body1> }>
                        {move || match batches_res.get() {
                            Some(Ok(page)) => {
                                let items: Vec<IterBatchView> = page.items;
                                if items.is_empty() {
                                    view! { <Caption1>"No batch rows yet."</Caption1> }.into_any()
                                } else {
                                    view! {
                                        <Flex vertical=true>
                                            <For each=move || items.clone() key=|b| b.id.clone() let:row>
                                                <>
                                                    <Flex vertical=true gap=FlexGap::Size(4)>
                                                        <Body1>
                                                            "Batch "
                                                            {row.batch_index}
                                                            " — "
                                                            {row.status.clone()}
                                                        </Body1>
                                                        <Caption1>
                                                            {format!(
                                                                "rows {} | proc {} | skip {} | fail {}",
                                                                row.row_count,
                                                                row.processed,
                                                                row.skipped,
                                                                row.failed
                                                            )}
                                                        </Caption1>
                                                    </Flex>
                                                    <CardSectionBorder />
                                                </>
                                            </For>
                                        </Flex>
                                    }.into_any()
                                }
                            }
                            Some(Err(e)) => view! {
                                <MessageBar intent=MessageBarIntent::Error>{format!("{}", e)}</MessageBar>
                            }.into_any(),
                            None => view! { <Body1>"…"</Body1> }.into_any(),
                        }}
                    </Suspense>
                </Card>
                </div>
            </div>
        </ContentContainer>
    }
}

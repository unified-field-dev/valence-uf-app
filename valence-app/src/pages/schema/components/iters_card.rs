use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{
    Body1, Caption1, Card, CardContent, CardHeader, CardSectionBorder, EmptyState, SectionTitle,
    Stack, StackConfig,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::ValenceHelpCardHeader;
use crate::pages::iter_index::IterRunRow;
use crate::server::{
    get_schema_iters, list_recent_iter_runs_for_schema, start_iter_run, IterInfo, IterRunSummary,
};

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn ItersCard(schema_name: String) -> impl IntoView {
    let schema = StoredValue::new(schema_name);

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .TableWrap {
            border: 1px solid var(--colorNeutralStroke2);
            border-radius: 8px;
            overflow: hidden;
        }
        .CompactTable { width: 100%; table-layout: fixed; }
        .ActionCol { width: 96px; white-space: nowrap; }
        .Code {
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
            background-color: var(--colorNeutralBackground3);
            padding: 2px 6px;
            border-radius: 4px;
        }
        .Subtle { color: var(--colorNeutralForeground3); }
        .RunsTable { width: 100%; min-width: 520px; }
        .TableScroller { overflow-x: auto; }
    };

    let iters_res = Resource::new(
        move || schema.get_value().clone(),
        |name| async move { get_schema_iters(name).await },
    );

    let start_action = Action::new(move |(sname, iter): &(String, String)| {
        let sname = sname.clone();
        let iter = iter.clone();
        async move { start_iter_run(sname, iter).await }
    });

    let recent_res = Resource::new(
        move || (schema.get_value().clone(), start_action.version().get()),
        |(name, _)| async move { list_recent_iter_runs_for_schema(name, 5).await },
    );

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-iters" data-testid="valence-schema-iters-card" style="width: 100%;">
            <Card class=class_names.card gap=FlexGap::Size(0)>
                                    <ValenceHelpCardHeader
                        title="Iters"
                        description="Background jobs that scan or process records in this table."
                    />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <Caption1 class=class_names.subtle>
                            "Registered row-level operations on this schema."
                        </Caption1>

                        <div id="valence-schema-iter-start">
                        <SectionTitle>"Registered iters"</SectionTitle>
                        <Suspense fallback=move || view! { <Body1>"Loading iters..."</Body1> }>
                            {move || match iters_res.get() {
                                Some(Ok(iters)) => {
                                    let iters: Vec<IterInfo> = iters;
                                    if iters.is_empty() {
                                        view! {
                                            <EmptyState message="No iters registered" />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class=class_names.table_wrap>
                                                <Table class=class_names.compact_table>
                                                    <TableHeader>
                                                        <TableRow>
                                                            <TableHeaderCell>"Iter"</TableHeaderCell>
                                                            <TableHeaderCell class=class_names.action_col>"Start"</TableHeaderCell>
                                                        </TableRow>
                                                    </TableHeader>
                                                    <TableBody>
                                                        <For
                                                            each=move || iters.clone()
                                                            key=|i| i.name.clone()
                                                            let:iter_info
                                                        >
                                                            {
                                                                let iter_name = iter_info.name.clone();
                                                                view! {
                                                                    <TableRow>
                                                                        <TableCell>
                                                                            <code class=class_names.code>
                                                                                {iter_info.name.clone()}
                                                                            </code>
                                                                        </TableCell>
                                                                        <TableCell class=class_names.action_col>
                                                                            <Button
                                                                                appearance=ButtonAppearance::Primary
                                                                                disabled=Signal::derive({
                                                                                    let p = start_action.pending();
                                                                                    move || p.get()
                                                                                })
                                                                                on_click=Callback::new(move |_| {
                                                                                    start_action.dispatch((
                                                                                        schema.get_value().clone(),
                                                                                        iter_name.clone(),
                                                                                    ));
                                                                                })
                                                                            >
                                                                                "Start"
                                                                            </Button>
                                                                        </TableCell>
                                                                    </TableRow>
                                                                }
                                                            }
                                                        </For>
                                                    </TableBody>
                                                </Table>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                Some(Err(e)) => view! {
                                    <MessageBar intent=MessageBarIntent::Error>
                                        {format!("Failed to load iters: {}", e)}
                                    </MessageBar>
                                }.into_any(),
                                None => view! { <Body1>"Loading…"</Body1> }.into_any(),
                            }}
                        </Suspense>

                        {move || start_action.value().get().and_then(|r| match r {
                            Err(e) => Some(view! {
                                <MessageBar intent=MessageBarIntent::Error>
                                    {format!("Start failed: {}", e)}
                                </MessageBar>
                            }),
                            Ok(run_id) => {
                                let href = valence_backend::valence_iter_run_path(&schema.get_value(), &run_id);
                                Some(view! {
                                    <MessageBar intent=MessageBarIntent::Success>
                                        <span>"Run started — "</span>
                                        <A href=href>"open run"</A>
                                    </MessageBar>
                                })
                            }
                        })}
                        </div>

                        <CardSectionBorder />

                        <div id="valence-schema-iter-runs">
                        <SectionTitle>"Recent iter runs"</SectionTitle>
                        <Suspense fallback=move || view! { <Body1>"Loading recent runs…"</Body1> }>
                            {move || match recent_res.get() {
                                Some(Ok(runs)) => {
                                    let runs: Vec<IterRunSummary> = runs;
                                    if runs.is_empty() {
                                        view! {
                                            <Caption1 class=class_names.subtle>"No runs yet."</Caption1>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class=class_names.table_scroller>
                                                <div class=class_names.table_wrap>
                                                    <Table class=class_names.runs_table>
                                                        <TableHeader>
                                                            <TableRow>
                                                                <TableHeaderCell>"Run"</TableHeaderCell>
                                                                <TableHeaderCell>"Iter"</TableHeaderCell>
                                                                <TableHeaderCell>"Status"</TableHeaderCell>
                                                                <TableHeaderCell>"Progress"</TableHeaderCell>
                                                                <TableHeaderCell>"Started"</TableHeaderCell>
                                                            </TableRow>
                                                        </TableHeader>
                                                        <TableBody>
                                                            <For
                                                                each=move || runs.clone()
                                                                key=|r| r.run_id.clone()
                                                                let:run
                                                            >
                                                                <IterRunRow run=run hide_schema=true />
                                                            </For>
                                                        </TableBody>
                                                    </Table>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                Some(Err(e)) => view! {
                                    <MessageBar intent=MessageBarIntent::Error>
                                        {format!("Runs: {}", e)}
                                    </MessageBar>
                                }.into_any(),
                                None => view! { <Body1>"…"</Body1> }.into_any(),
                            }}
                        </Suspense>
                        </div>
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

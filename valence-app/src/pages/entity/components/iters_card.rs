use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, CardHeader, Stack, StackConfig};
use orbital::primitives::*;

use crate::components::{bordered_table_styles, ValenceHelpCardHeader};
use crate::server::{evaluate_iter_for_entity, run_iter_on_entity, IterEntityEvaluation};

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn EntityItersCard(schema_name: String, entity_id: String) -> impl IntoView {
    let ctx = StoredValue::new((schema_name, entity_id));
    let table_styles = bordered_table_styles();

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .ActionCol { width: 72px; white-space: nowrap; }
    };

    let eval_res = Resource::new(
        move || {
            let (s, e) = ctx.get_value();
            (s.clone(), e.clone())
        },
        |(s, e)| async move { evaluate_iter_for_entity(s, e).await },
    );

    let run_action = Action::new(move |(sname, eid, iter): &(String, String, String)| {
        let sname = sname.clone();
        let eid = eid.clone();
        let iter = iter.clone();
        async move { run_iter_on_entity(sname, eid, iter).await }
    });

    let pending = run_action.pending();

    view! {
        <style>{style_sheet}</style>
        <style>{table_styles.sheet.clone()}</style>
        <div id="valence-entity-iter-run" data-testid="valence-entity-iters-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Iters"
                        description="Background jobs that can run on this specific record."
                    />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <Caption1 class=table_styles.subtle.clone()>"Row-level operations"</Caption1>
                        <Suspense fallback=move || view! { <Body1>"Loading iter evaluation…"</Body1> }>
                            {move || match eval_res.get() {
                                Some(Ok(rows)) => {
                                    let rows: Vec<IterEntityEvaluation> = rows;
                                    if rows.is_empty() {
                                        view! {
                                            <Caption1 class=table_styles.subtle.clone()>
                                                "No iters for this schema."
                                            </Caption1>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class=table_styles.table_scroller.clone()>
                                                <div class=table_styles.table_wrap.clone()>
                                                    <Table class=table_styles.compact_table.clone()>
                                                        <TableHeader>
                                                            <TableRow>
                                                                <TableHeaderCell>"Iter"</TableHeaderCell>
                                                                <TableHeaderCell>"Evaluation"</TableHeaderCell>
                                                                <TableHeaderCell class=class_names.action_col>"Action"</TableHeaderCell>
                                                            </TableRow>
                                                        </TableHeader>
                                                        <TableBody>
                                                            <For
                                                                each=move || rows.clone()
                                                                key=|r| r.iter_name.clone()
                                                                let:row
                                                            >
                                                                {
                                                                    let iter_for_dispatch = row.iter_name.clone();
                                                                    let label = row.iter_name.clone();
                                                                    let should = row.should_run;
                                                                    let reason = row.reason.clone();
                                                                    let eval_text = if should {
                                                                        format!("Should run: Yes — {}", reason)
                                                                    } else {
                                                                        format!("Should not run — {}", reason)
                                                                    };
                                                                    view! {
                                                                        <TableRow>
                                                                            <TableCell>
                                                                                <code class=table_styles.code.clone()>{label}</code>
                                                                            </TableCell>
                                                                            <TableCell>
                                                                                <Caption1>{eval_text}</Caption1>
                                                                            </TableCell>
                                                                            <TableCell class=class_names.action_col>
                                                                                <Button
                                                                                    appearance=ButtonAppearance::Primary
                                                                                    disabled=Signal::derive(move || pending.get())
                                                                                    on_click=Callback::new(move |_| {
                                                                                        let (s, e) = ctx.get_value();
                                                                                        run_action.dispatch((
                                                                                            s.clone(),
                                                                                            e.clone(),
                                                                                            iter_for_dispatch.clone(),
                                                                                        ));
                                                                                    })
                                                                                >
                                                                                    "Run"
                                                                                </Button>
                                                                            </TableCell>
                                                                        </TableRow>
                                                                    }
                                                                }
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
                                        {format!("{}", e)}
                                    </MessageBar>
                                }.into_any(),
                                None => view! { <Body1>"…"</Body1> }.into_any(),
                            }}
                        </Suspense>

                        {move || {
                            let (schema, _) = ctx.get_value();
                            run_action.value().get().map(|r| match r {
                                Ok(run_id) => {
                                    let href = valence_backend::valence_iter_run_path(&schema, &run_id);
                                    view! {
                                        <MessageBar intent=MessageBarIntent::Success>
                                            <span>"Run enqueued — "</span>
                                            <A href=href>"open run"</A>
                                        </MessageBar>
                                    }
                                    .into_any()
                                }
                                Err(e) => view! {
                                    <MessageBar intent=MessageBarIntent::Error>
                                        {format!("Run failed: {}", e)}
                                    </MessageBar>
                                }
                                .into_any(),
                            })
                        }}
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

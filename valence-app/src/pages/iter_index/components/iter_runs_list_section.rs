use leptos::prelude::*;
use leptos_use::use_interval_fn;
use orbital::components::{
    Caption1, Card, EmptyState, OrbitalInfiniteScroll, OrbitalInfiniteScrollEmptyView,
    OrbitalInfiniteScrollEndView,
};
use orbital::primitives::*;

use crate::server::{list_iter_runs, IterRunSummary, ITERS_PAGE_SIZE};

use super::IterRunRow;

fn is_non_terminal_status(status: &str) -> bool {
    matches!(status, "pending" | "scanning" | "processing")
}

/// Card-wrapped table of iter runs with infinite scroll and live polling for active runs.
#[component]
pub fn IterRunsListSection() -> impl IntoView {
    let trigger_refetch = RwSignal::new(0u32);
    let poll_active = RwSignal::new(true);

    use_interval_fn(
        move || {
            if poll_active.get_untracked() {
                trigger_refetch.update(|n| *n += 1);
            }
        },
        3000,
    );

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; max-width: 100%; margin: 0; box-sizing: border-box; }
        .TableScroller { width: 100%; overflow-x: auto; }
        .Table { width: 100%; min-width: 720px; }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="valence-iter-runs-list-section">
            {move || {
                let _trigger = trigger_refetch.get();
                let fetch_runs = |offset: u32, limit: u32| list_iter_runs(offset, limit);

                view! {
                    <Card class=class_names.card>
                        <OrbitalInfiniteScroll
                            page_size=ITERS_PAGE_SIZE
                            fetch=fetch_runs
                            max_height="600px"
                            let:items
                        >
                            {
                                Effect::new(move |_| {
                                    let runs = items.get();
                                    let has_active = runs
                                        .iter()
                                        .any(|run: &IterRunSummary| is_non_terminal_status(&run.status));
                                    poll_active.set(has_active);
                                });
                            }
                            <OrbitalInfiniteScrollEmptyView slot>
                                <EmptyState
                                    message="No iter runs yet"
                                    description="Start an iter from a schema page to see run history here."
                                />
                            </OrbitalInfiniteScrollEmptyView>
                            <OrbitalInfiniteScrollEndView slot>
                                <Caption1>"All runs loaded"</Caption1>
                            </OrbitalInfiniteScrollEndView>
                            <div class=class_names.table_scroller>
                                <Table class=class_names.table>
                                    <TableHeader>
                                        <TableRow>
                                            <TableHeaderCell>"Run"</TableHeaderCell>
                                            <TableHeaderCell>"Iter"</TableHeaderCell>
                                            <TableHeaderCell>"Schema"</TableHeaderCell>
                                            <TableHeaderCell>"Status"</TableHeaderCell>
                                            <TableHeaderCell>"Progress"</TableHeaderCell>
                                            <TableHeaderCell>"Started"</TableHeaderCell>
                                        </TableRow>
                                    </TableHeader>
                                    <TableBody>
                                        <For
                                            each=move || items.get()
                                            key=|run| run.run_id.clone()
                                            let:run
                                        >
                                            <IterRunRow run=run />
                                        </For>
                                    </TableBody>
                                </Table>
                            </div>
                        </OrbitalInfiniteScroll>
                    </Card>
                }
            }}
        </div>
    }
}

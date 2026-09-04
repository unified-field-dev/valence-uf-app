use leptos::prelude::*;
use orbital::components::{
    Body1, Caption1, Card, CardContent, CardHeader, EmptyState, Stack, StackConfig,
};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::server::{list_deletion_runs_for_schema, DeletionRunView};

use crate::components::ValenceHelpCardHeader;

use super::deletion_run_row::DeletionRunRow;

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn SchemaDeletionsCard(schema_name: String) -> impl IntoView {
    let schema = StoredValue::new(schema_name);

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .TableWrap {
            border: 1px solid var(--colorNeutralStroke2);
            border-radius: 8px;
            overflow: hidden;
        }
        .Table { width: 100%; min-width: 560px; }
        .TableScroller { overflow-x: auto; }
        .Subtle { color: var(--colorNeutralForeground3); }
    };

    let runs_res = Resource::new(
        move || schema.get_value().clone(),
        |name| async move { list_deletion_runs_for_schema(name).await },
    );

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-deletions" data-testid="valence-schema-deletions-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Deletions"
                        description="Tracked delete operations and their progress for this table."
                    />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <Caption1 class=class_names.subtle>
                            "Recent deletion runs for this schema (root table)."
                        </Caption1>
                        <Suspense fallback=move || view! { <Body1>"Loading deletions…"</Body1> }>
                            {move || match runs_res.get() {
                                Some(Ok(runs)) => {
                                    let runs: Vec<DeletionRunView> = runs;
                                    if runs.is_empty() {
                                        view! {
                                            <EmptyState message="No deletion runs yet" />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class=class_names.table_scroller>
                                                <div class=class_names.table_wrap>
                                                    <Table class=class_names.table>
                                                        <TableHeader>
                                                            <TableRow>
                                                                <TableHeaderCell>"Record"</TableHeaderCell>
                                                                <TableHeaderCell>"Progress"</TableHeaderCell>
                                                                <TableHeaderCell>"Status"</TableHeaderCell>
                                                                <TableHeaderCell>"Requested"</TableHeaderCell>
                                                                <TableHeaderCell>"Action"</TableHeaderCell>
                                                            </TableRow>
                                                        </TableHeader>
                                                        <TableBody>
                                                            <For
                                                                each=move || runs.clone()
                                                                key=|r| r.run_id.clone()
                                                                let:run
                                                            >
                                                                <DeletionRunRow run=run />
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
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

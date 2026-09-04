use leptos::prelude::*;
use orbital::components::{Body1, Card, CardContent, CardHeader, EmptyState};
use orbital::primitives::*;

use crate::components::{bordered_table_styles, ValenceHelpCardHeader};
use crate::server::DeletionRequest;

#[component]
pub fn EntityDeletionsCard(deletions: Vec<DeletionRequest>) -> impl IntoView {
    let deletions = StoredValue::new(deletions);
    let table_styles = bordered_table_styles();

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .Code {
            font-family: var(--fontFamilyMonospace);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <style>{table_styles.sheet.clone()}</style>
        <div id="valence-entity-deletions" data-testid="valence-entity-deletions-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Deletions"
                        description="Delete requests involving this record and their status."
                    />
                <CardContent>
                    {move || {
                        let dels = deletions.get_value();
                        if dels.is_empty() {
                            view! {
                                <EmptyState message="No active deletions" />
                            }.into_any()
                        } else {
                            view! {
                                <div class=table_styles.table_scroller.clone()>
                                    <div class=table_styles.table_wrap.clone()>
                                        <Table class=table_styles.compact_table.clone()>
                                            <TableHeader>
                                                <TableRow>
                                                    <TableHeaderCell>"Request"</TableHeaderCell>
                                                    <TableHeaderCell>"Status"</TableHeaderCell>
                                                    <TableHeaderCell>"Requested"</TableHeaderCell>
                                                </TableRow>
                                            </TableHeader>
                                            <TableBody>
                                                <For
                                                    each=move || deletions.get_value()
                                                    key=|d| d.id.clone()
                                                    let:d
                                                >
                                                    <TableRow>
                                                        <TableCell>
                                                            <code class=class_names.code>{d.id.clone()}</code>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Badge>{d.status.clone()}</Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Body1 class=table_styles.subtle.clone()>
                                                                {d.requested_at.clone()}
                                                            </Body1>
                                                        </TableCell>
                                                    </TableRow>
                                                </For>
                                            </TableBody>
                                        </Table>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </CardContent>
            </Card>
        </div>
    }
}

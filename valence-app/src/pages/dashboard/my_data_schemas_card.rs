use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{
    Body1, Caption1, Card, CardContent, EmptyState, Table, TableBody, TableCell, TableHeader,
    TableHeaderCell, TableRow,
};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_my_data_top_schemas;

use super::skeletons::DashboardKpiSkeleton;

#[component]
pub fn DashboardMyDataSchemasCard() -> impl IntoView {
    let res = Resource::new(
        || (),
        |_| async move { get_dashboard_my_data_top_schemas().await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Top schemas"
                description="Schemas where you hold the most owned rows."
            />
            <CardContent>
                <div id="valence-dashboard-top-schemas" data-testid="dashboard-my-data-schemas-card">
                    <Transition fallback=move || view! { <DashboardKpiSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(rows) if rows.is_empty() => view! {
                                <EmptyState message="No owned rows yet. Browse schemas to create data." />
                            }.into_any(),
                            Ok(rows) => view! {
                                <Table>
                                    <TableHeader>
                                        <TableRow>
                                            <TableHeaderCell>"Schema"</TableHeaderCell>
                                            <TableHeaderCell>"Active rows"</TableHeaderCell>
                                            <TableHeaderCell>"Pending deletion"</TableHeaderCell>
                                        </TableRow>
                                    </TableHeader>
                                    <TableBody>
                                        <For
                                            each=move || rows.clone()
                                            key=|row| row.valence_model.clone()
                                            children=move |row| {
                                                let href = valence_backend::valence_schema_path(&row.valence_model);
                                                view! {
                                                    <TableRow>
                                                        <TableCell>
                                                            <A href=href>
                                                                <Body1>{row.valence_model.clone()}</Body1>
                                                            </A>
                                                        </TableCell>
                                                        <TableCell><Body1>{row.active_rows.to_string()}</Body1></TableCell>
                                                        <TableCell>
                                                            <Caption1>{row.pending_deletion_rows.to_string()}</Caption1>
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            }
                                        />
                                    </TableBody>
                                </Table>
                            }.into_any(),
                            Err(e) => view! {
                                <orbital::components::MessageBar intent=orbital::components::MessageBarIntent::Error>
                                    {e.to_string()}
                                </orbital::components::MessageBar>
                            }.into_any(),
                        })}
                    </Transition>
                </div>
            </CardContent>
        </Card>
    }
}

use leptos::prelude::*;
use orbital::components::{
    Card, CardContent, EmptyState, Flex, FlexGap, FlexWrap, StatCard, StatCardVariant,
};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_my_data_stats;

use super::skeletons::DashboardKpiSkeleton;

#[component]
pub fn DashboardMyDataStatsCard() -> impl IntoView {
    let res = Resource::new(
        || (),
        |_| async move { get_dashboard_my_data_stats().await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Your footprint"
                description="Rows you own across Valence schemas."
            />
            <CardContent>
                <div id="valence-dashboard-my-data" data-testid="dashboard-my-data-stats-card">
                    <Transition fallback=move || view! { <DashboardKpiSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(stats) if !stats.viewer_is_user => {
                                view! {
                                    <EmptyState message="Sign in as a user to see personal data metrics." />
                                }.into_any()
                            }
                            Ok(stats) => view! {
                                <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
                                    <StatCard
                                        label="Owned rows"
                                        value=Signal::derive(move || stats.owned_rows.to_string())
                                    />
                                    <StatCard
                                        label="Tables with data"
                                        value=Signal::derive(move || stats.tables_with_data.to_string())
                                    />
                                    <StatCard
                                        label="Pending deletion"
                                        value=Signal::derive(move || stats.pending_deletion_rows.to_string())
                                        variant=StatCardVariant::Warning
                                    />
                                </Flex>
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

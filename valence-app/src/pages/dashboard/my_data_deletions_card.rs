use leptos::prelude::*;
use orbital::components::{Body1, Card, CardContent, StatCard, StatCardVariant};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_active_deletions;

use super::skeletons::DashboardKpiSkeleton;

#[component]
pub fn DashboardMyDataDeletionsCard() -> impl IntoView {
    let res = Resource::new(
        || (),
        |_| async move { get_dashboard_active_deletions().await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Active deletions"
                description="Deletion runs you started that are still in flight."
            />
            <CardContent>
                <div id="valence-dashboard-active-deletions" data-testid="dashboard-my-data-deletions-card">
                    <Transition fallback=move || view! { <DashboardKpiSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(count) => view! {
                                <StatCard
                                    label="In-flight runs"
                                    value=Signal::derive(move || count.to_string())
                                    variant=StatCardVariant::Danger
                                />
                                <Body1>"Queued, scanning, or processing runs requested by you."</Body1>
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

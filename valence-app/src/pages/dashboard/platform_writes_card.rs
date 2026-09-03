use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, MessageBar, MessageBarIntent};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_platform_writes_breakdown;

use super::charts::area_chart_from_series;
use super::permission_denied::DashboardPermissionDeniedState;
use super::skeletons::DashboardChartSkeleton;

#[component]
pub fn DashboardPlatformWritesCard(
    range_secs: Signal<i64>,
    group_by: Signal<String>,
) -> impl IntoView {
    let res = Resource::new(
        move || (range_secs.get(), group_by.get()),
        |(secs, group)| async move { get_dashboard_platform_writes_breakdown(secs, group).await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Writes breakdown"
                description="Write rate grouped by table or database type."
            />
            <CardContent>
                <div id="valence-dashboard-writes" data-testid="dashboard-platform-writes-card">
                    <Transition fallback=move || view! { <DashboardChartSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(series) if series.is_empty() => view! {
                                <Caption1>"No write metrics in this range."</Caption1>
                            }.into_any(),
                            Ok(series) => view! {
                                <Caption1>"Writes / sec by dimension"</Caption1>
                                {area_chart_from_series(&series, "writes", 280.0)}
                            }.into_any(),
                            Err(e) if e.to_string().contains("Permission denied") => {
                                view! { <DashboardPermissionDeniedState /> }.into_any()
                            }
                            Err(e) => view! {
                                <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                            }.into_any(),
                        })}
                    </Transition>
                </div>
            </CardContent>
        </Card>
    }
}

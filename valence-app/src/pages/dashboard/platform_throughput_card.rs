use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, MessageBar, MessageBarIntent};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_platform_throughput;

use super::charts::line_chart_from_series;
use super::permission_denied::DashboardPermissionDeniedState;
use super::skeletons::DashboardChartSkeleton;

#[component]
pub fn DashboardPlatformThroughputCard(range_secs: Signal<i64>) -> impl IntoView {
    let res = Resource::new(
        move || range_secs.get(),
        |secs| async move { get_dashboard_platform_throughput(secs).await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Throughput"
                description="Reads, writes, and errors per second over the selected range."
            />
            <CardContent>
                <div id="valence-dashboard-throughput" data-testid="dashboard-platform-throughput-card">
                    <Transition fallback=move || view! { <DashboardChartSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(series) if series.is_empty() => view! {
                                <Caption1>"No metric points in this range."</Caption1>
                            }.into_any(),
                            Ok(series) => view! {
                                <Caption1>"Operations per second"</Caption1>
                                {line_chart_from_series(&series, 280.0)}
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

use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, MessageBar, MessageBarIntent};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_platform_error_offenders;

use super::charts::bar_chart_from_slices;
use super::permission_denied::DashboardPermissionDeniedState;
use super::skeletons::DashboardChartSkeleton;

#[component]
pub fn DashboardPlatformErrorsCard(range_secs: Signal<i64>) -> impl IntoView {
    let res = Resource::new(
        move || range_secs.get(),
        |secs| async move { get_dashboard_platform_error_offenders(secs).await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Error top offenders"
                description="Tables with the most Valence errors in the selected range."
            />
            <CardContent>
                <div id="valence-dashboard-errors" data-testid="dashboard-platform-errors-card">
                    <Transition fallback=move || view! { <DashboardChartSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(slices) if slices.is_empty() => view! {
                                <Caption1>"No errors logged in this range."</Caption1>
                            }.into_any(),
                            Ok(slices) => view! {
                                <Caption1>"Error count by table"</Caption1>
                                {bar_chart_from_slices(&slices, 280.0)}
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

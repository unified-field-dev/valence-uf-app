use leptos::prelude::*;
use orbital::components::{Card, CardContent, Flex, FlexGap, FlexWrap, StatCard};

use crate::components::help::ValenceHelpCardHeader;
use crate::server::get_dashboard_platform_headline;

use super::permission_denied::DashboardPermissionDeniedState;
use super::skeletons::DashboardKpiSkeleton;

#[component]
pub fn DashboardPlatformHeadlineCard(range_secs: Signal<i64>) -> impl IntoView {
    let res = Resource::new(
        move || range_secs.get(),
        |secs| async move { get_dashboard_platform_headline(secs).await },
    );

    view! {
        <Card>
            <ValenceHelpCardHeader
                title="Platform headline"
                description="Latest counter values over the selected range."
            />
            <CardContent>
                <div id="valence-dashboard-headline" data-testid="dashboard-platform-headline-card">
                    <Transition fallback=move || view! { <DashboardKpiSkeleton /> }>
                        {move || res.get().map(|r| match r {
                            Ok(headline) => view! {
                                <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
                                    {headline.cards.into_iter().map(|c| {
                                        let label: &'static str =
                                            Box::leak(c.label.clone().into_boxed_str());
                                        let value = c.value.clone();
                                        view! {
                                            <StatCard
                                                label=label
                                                value=Signal::derive(move || value.clone())
                                            />
                                        }
                                    }).collect_view()}
                                </Flex>
                            }.into_any(),
                            Err(e) if e.to_string().contains("Permission denied") => {
                                view! { <DashboardPermissionDeniedState /> }.into_any()
                            }
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

mod charts;
mod my_data_deletions_card;
mod my_data_schemas_card;
mod my_data_stats_card;
mod permission_denied;
mod platform_errors_card;
mod platform_headline_card;
mod platform_reads_card;
mod platform_throughput_card;
mod platform_writes_card;
mod search_header;
mod skeletons;

use leptos::prelude::*;
use orbital::components::{AutoGrid, ContentContainer, Flex, SpacingSize, Subtitle2};

use charts::DashboardPlatformToolbar;
use my_data_deletions_card::DashboardMyDataDeletionsCard;
use my_data_schemas_card::DashboardMyDataSchemasCard;
use my_data_stats_card::DashboardMyDataStatsCard;
use platform_errors_card::DashboardPlatformErrorsCard;
use platform_headline_card::DashboardPlatformHeadlineCard;
use platform_reads_card::DashboardPlatformReadsCard;
use platform_throughput_card::DashboardPlatformThroughputCard;
use platform_writes_card::DashboardPlatformWritesCard;
use search_header::DashboardSearchHeader;

#[component]
pub fn ValenceDashboardPage() -> impl IntoView {
    let range_secs = RwSignal::new(3600i64);
    let group_by = RwSignal::new("table".to_string());

    view! {
        <ContentContainer max_width="1200px" data_testid="valence-dashboard-page">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <DashboardSearchHeader />

                <Subtitle2>"My Data"</Subtitle2>
                <AutoGrid min="280px" gap=SpacingSize::Size160>
                    <DashboardMyDataStatsCard />
                    <DashboardMyDataSchemasCard />
                    <DashboardMyDataDeletionsCard />
                </AutoGrid>

                <Subtitle2>"Valence Platform"</Subtitle2>
                <DashboardPlatformToolbar range_secs=range_secs group_by=group_by />
                <AutoGrid min="400px" gap=SpacingSize::Size160>
                    <DashboardPlatformHeadlineCard range_secs=range_secs.into() />
                    <DashboardPlatformThroughputCard range_secs=range_secs.into() />
                    <DashboardPlatformWritesCard range_secs=range_secs.into() group_by=group_by.into() />
                    <DashboardPlatformReadsCard range_secs=range_secs.into() group_by=group_by.into() />
                    <DashboardPlatformErrorsCard range_secs=range_secs.into() />
                </AutoGrid>
            </Flex>
        </ContentContainer>
    }
}

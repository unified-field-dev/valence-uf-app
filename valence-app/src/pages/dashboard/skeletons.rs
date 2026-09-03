use leptos::prelude::*;
use orbital::components::{Skeleton, SkeletonItem};

#[component]
pub fn DashboardKpiSkeleton() -> impl IntoView {
    view! {
        <Skeleton>
            <SkeletonItem />
        </Skeleton>
    }
}

#[component]
pub fn DashboardChartSkeleton() -> impl IntoView {
    view! {
        <Skeleton>
            <SkeletonItem />
            <SkeletonItem />
        </Skeleton>
    }
}

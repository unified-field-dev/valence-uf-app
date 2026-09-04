use leptos::prelude::*;
use orbital::components::{MessageBar, MessageBarIntent};

#[component]
pub fn DashboardPermissionDeniedState() -> impl IntoView {
    view! {
        <div data-testid="dashboard-permission-denied">
            <MessageBar intent=MessageBarIntent::Warning>
                "You do not have permission to view these metrics."
            </MessageBar>
        </div>
    }
}

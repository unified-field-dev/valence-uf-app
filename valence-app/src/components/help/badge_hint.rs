use leptos::prelude::*;
use orbital::primitives::{Badge, Tooltip, TooltipAppearance};

/// Badge with a short explanatory tooltip.
#[component]
pub fn BadgeHint(label: String, tooltip: &'static str) -> impl IntoView {
    view! {
        <Tooltip content=tooltip appearance=TooltipAppearance::Inverted>
            <Badge>{label}</Badge>
        </Tooltip>
    }
}

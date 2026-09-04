use leptos::prelude::*;
use orbital::primitives::*;

/// Badge for a Valence iter run status string.
#[component]
pub fn IterRunStatusBadge(#[prop(into)] status: String) -> impl IntoView {
    let (label, appearance, color) = match status.as_str() {
        "pending" => (
            "Pending".to_string(),
            BadgeAppearance::Tint,
            BadgeColor::Informative,
        ),
        "scanning" => (
            "Scanning".to_string(),
            BadgeAppearance::Tint,
            BadgeColor::Brand,
        ),
        "processing" => (
            "Processing".to_string(),
            BadgeAppearance::Tint,
            BadgeColor::Brand,
        ),
        "completed" => (
            "Completed".to_string(),
            BadgeAppearance::Filled,
            BadgeColor::Success,
        ),
        "failed" => (
            "Failed".to_string(),
            BadgeAppearance::Filled,
            BadgeColor::Danger,
        ),
        "cancelled" => (
            "Cancelled".to_string(),
            BadgeAppearance::Outline,
            BadgeColor::Warning,
        ),
        other => (
            other.to_string(),
            BadgeAppearance::Outline,
            BadgeColor::Subtle,
        ),
    };

    view! {
        <Badge appearance=appearance color=color>{label}</Badge>
    }
}

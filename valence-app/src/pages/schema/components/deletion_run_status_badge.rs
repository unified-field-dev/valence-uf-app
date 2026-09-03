use leptos::prelude::*;
use orbital::primitives::*;

/// Badge for a Valence deletion run status string.
#[component]
pub fn DeletionRunStatusBadge(#[prop(into)] status: String) -> impl IntoView {
    let (label, appearance, color) = match status.as_str() {
        "pending" => (
            "Pending".to_string(),
            BadgeAppearance::Tint,
            BadgeColor::Informative,
        ),
        "running" | "processing" => (status.clone(), BadgeAppearance::Tint, BadgeColor::Brand),
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

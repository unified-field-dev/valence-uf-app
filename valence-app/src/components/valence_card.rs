use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardHeader, CardHeaderDescription, Subtitle2};
use turf::inline_style_sheet_values;

/// A consistent card component for Valence pages with title, subtitle, and body content.
///
/// This component provides a standard layout used across Valence cards:
/// - Full-width card with consistent styling
/// - Header section with title and subtitle via Thaw CardHeader
/// - Body section for child content
#[component]
pub fn ValenceCard(
    /// The title displayed in the card header
    title: &'static str,
    /// The subtitle displayed in the card header
    subtitle: &'static str,
    /// The body content of the card
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            max-width: 600px;
            margin: 0;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="valence-valence-card" style="width: 100%;">
            <Card class=class_names.card>
                <CardHeader>
                    <Subtitle2>{title}</Subtitle2>
                    <CardHeaderDescription slot>
                        <Caption1>{subtitle}</Caption1>
                    </CardHeaderDescription>
                </CardHeader>
                {children()}
            </Card>
        </div>
    }
}

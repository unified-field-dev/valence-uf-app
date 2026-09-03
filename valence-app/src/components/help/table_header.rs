use leptos::prelude::*;
use orbital::primitives::{InfoLabel, InfoLabelInfo};

/// Table column header with an optional info popover.
#[component]
pub fn ValenceHelpColumnHeader(
    label: &'static str,
    #[prop(optional)] info: Option<AnyView>,
) -> impl IntoView {
    view! {
        {if let Some(info_view) = info {
            view! {
                <div data-testid=format!("valence-help-col-{}", label.to_ascii_lowercase().replace(' ', "-"))>
                    <InfoLabel>
                        {label}
                        <InfoLabelInfo slot>
                            {info_view}
                        </InfoLabelInfo>
                    </InfoLabel>
                </div>
            }.into_any()
        } else {
            view! { {label} }.into_any()
        }}
    }
}

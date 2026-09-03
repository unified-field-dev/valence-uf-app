use leptos::prelude::*;
use leptos_router::components::A;
use orbital::primitives::{Button, ButtonAppearance, Tooltip, TooltipAppearance};

/// Subtle navigation button with ellipsis truncation and full-text tooltip.
#[component]
pub fn TruncatedRecordLink(
    href: String,
    display: String,
    link_wrap_class: String,
    link_button_class: String,
) -> impl IntoView {
    let tooltip = display.clone();
    view! {
        <div class=link_wrap_class>
            <Tooltip content=tooltip appearance=TooltipAppearance::Inverted>
                <A href=href>
                    <Button appearance=ButtonAppearance::Subtle>
                        <span class=link_button_class>{display}</span>
                    </Button>
                </A>
            </Tooltip>
        </div>
    }
}

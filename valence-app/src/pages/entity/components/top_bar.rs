use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::Title3;
use orbital::primitives::*;
use turf::inline_style_sheet_values;

#[component]
pub fn EntityTopBar(title: String, back_href: String, back_label: String) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .TopBar {
            gap: 12px;
            align-items: center;
            margin-bottom: 24px;
        }

        .Spacer {
            flex: 1;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div id="valence-entity-top-bar" data-testid="valence-entity-top-bar">
            <Flex class=class_names.top_bar align=FlexAlign::Center>
                <Title3>{title}</Title3>
                <div class=class_names.spacer />
                <A href=back_href>
                    <Button appearance=ButtonAppearance::Secondary>
                        {back_label}
                    </Button>
                </A>
            </Flex>
        </div>
    }
}

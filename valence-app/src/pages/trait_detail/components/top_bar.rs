use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::Title3;
use orbital::primitives::*;
use turf::inline_style_sheet_values;

#[component]
pub fn TraitTopBar(trait_name: String) -> impl IntoView {
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

    let title_string = format!("/traits/{}", trait_name);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-trait-top-bar" data-testid="valence-trait-top-bar">
            <Flex class=class_names.top_bar align=FlexAlign::Center>
                <Title3>{title_string}</Title3>
                <Badge>"Trait"</Badge>
                <div class=class_names.spacer />
                <A href=crate::paths::TRAITS>
                    <Button appearance=ButtonAppearance::Subtle>"Back to Traits"</Button>
                </A>
            </Flex>
        </div>
    }
}

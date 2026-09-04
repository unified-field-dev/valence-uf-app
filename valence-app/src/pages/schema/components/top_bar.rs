use leptos::prelude::*;
use orbital::components::Title3;
use orbital::primitives::*;
use turf::inline_style_sheet_values;

#[component]
pub fn SchemaTopBar(schema_name: String) -> impl IntoView {
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

    let title_string = format!("/schemas/{}", schema_name);
    let (title, _) = signal(title_string);

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-top-bar" data-testid="valence-schema-top-bar">
            <Flex class=class_names.top_bar align=FlexAlign::Center>
                <Title3>{move || title.get()}</Title3>
                <Badge>"Table"</Badge>
                <div class=class_names.spacer />
            </Flex>
        </div>
    }
}

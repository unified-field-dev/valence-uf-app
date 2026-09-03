use leptos::prelude::*;
use orbital::components::{Body1, Caption1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldTypeTypography {
    Caption,
    Body,
}

#[component]
pub fn FieldTypeText(
    text: String,
    typography: FieldTypeTypography,
    #[prop(optional, into)] subtle_class: MaybeProp<String>,
) -> impl IntoView {
    match typography {
        FieldTypeTypography::Caption => view! {
            <Caption1 class=subtle_class>{text}</Caption1>
        }
        .into_any(),
        FieldTypeTypography::Body => view! {
            <Body1 class=subtle_class>{text}</Body1>
        }
        .into_any(),
    }
}

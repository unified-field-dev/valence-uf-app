use leptos::prelude::*;
use orbital::components::{Body1, Caption1, ThemeColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldTypeTypography {
    Caption,
    Body,
}

#[component]
pub fn FieldTypeText(
    text: String,
    typography: FieldTypeTypography,
    /// Retained for call-site compatibility; muted color comes from [`ThemeColor`].
    #[prop(optional, into)]
    #[allow(unused_variables)]
    subtle_class: MaybeProp<String>,
) -> impl IntoView {
    match typography {
        FieldTypeTypography::Caption => view! {
            <Caption1 color=ThemeColor::NeutralForeground3>{text}</Caption1>
        }
        .into_any(),
        FieldTypeTypography::Body => view! {
            <Body1 color=ThemeColor::NeutralForeground3>{text}</Body1>
        }
        .into_any(),
    }
}

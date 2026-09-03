use leptos::prelude::*;

use super::typography::{FieldTypeText, FieldTypeTypography};

#[component]
pub fn PlainTypeLabel(
    text: String,
    typography: FieldTypeTypography,
    #[prop(optional, into)] subtle_class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <FieldTypeText text=text typography=typography subtle_class=subtle_class />
    }
}

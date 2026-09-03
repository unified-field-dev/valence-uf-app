use leptos::prelude::*;
use orbital::components::Caption1;
use orbital::primitives::{InfoLabel, InfoLabelInfo};

use super::copy::{on_delete_detail, on_delete_summary, OnDeleteContext};

/// Visible on-delete summary with an info popover explaining affected records.
#[component]
pub fn OnDeleteHint(
    schema_name: String,
    from_field: String,
    to_table: String,
    label: String,
    cardinality: String,
    on_delete: String,
    subtle_class: String,
) -> impl IntoView {
    let ctx = OnDeleteContext {
        schema_name: &schema_name,
        from_field: &from_field,
        to_table: &to_table,
        label: &label,
        cardinality: &cardinality,
        on_delete: &on_delete,
    };
    let summary = on_delete_summary(&ctx);
    let detail = on_delete_detail(&ctx);

    view! {
        <Caption1 class=subtle_class>
            {label}
            " · "
            <InfoLabel>
                {summary}
                <InfoLabelInfo slot>
                    <Caption1>{detail}</Caption1>
                </InfoLabelInfo>
            </InfoLabel>
        </Caption1>
    }
}

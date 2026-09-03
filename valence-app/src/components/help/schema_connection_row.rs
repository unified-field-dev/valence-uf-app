use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1};
use orbital::primitives::*;

use super::{
    cardinality_badge_tooltip, ref_badge_tooltip, BadgeHint, ConnectionRowLayout, OnDeleteHint,
    TruncatedRecordLink,
};
use crate::components::code_style_classes;
use crate::server::{InverseSchemaConnection, SchemaConnection};

/// Outgoing schema connection row with on-delete help.
#[component]
pub fn SchemaConnectionRow(schema_name: String, connection: SchemaConnection) -> impl IntoView {
    let styles = code_style_classes();
    let from_field = connection.from_field.clone();
    let to_table = connection.to_table.clone();
    let label = connection.label.clone();
    let cardinality = connection.cardinality.clone();
    let on_delete = connection.on_delete.clone();
    let target_trait = connection.target_trait.clone();
    let trait_source = connection.trait_source.clone();

    let target_link: AnyView = if let Some(trait_name) = target_trait {
        let href = valence_backend::valence_trait_path(&trait_name);
        view! {
            <A href=href>
                <Button appearance=ButtonAppearance::Subtle>
                    {trait_name}
                </Button>
            </A>
        }
        .into_any()
    } else {
        view! {
            <TruncatedRecordLink
                href=valence_backend::valence_schema_path(&to_table)
                display=to_table.clone()
                link_wrap_class=styles.link_wrap.clone()
                link_button_class=styles.link_button.clone()
            />
        }
        .into_any()
    };

    let trait_source_view: AnyView = match trait_source {
        Some(trait_name) => {
            let href = valence_backend::valence_trait_path(&trait_name);
            view! {
                <Caption1 class=styles.subtle.clone()>
                    "Inherited from "
                    <A href=href>
                        <Button appearance=ButtonAppearance::Subtle>
                            {trait_name}
                        </Button>
                    </A>
                </Caption1>
            }
            .into_any()
        }
        None => view! {}.into_any(),
    };

    let from_field_display = from_field.clone();
    let from_field_hint = from_field.clone();

    view! {
        <style>{styles.sheet.clone()}</style>
        <ConnectionRowLayout
            list_item_class=styles.list_item.clone()
            meta_row_class=styles.meta_row.clone()
            badge=view! {
                <BadgeHint
                    label=cardinality.clone()
                    tooltip=cardinality_badge_tooltip(&cardinality)
                />
            }.into_any()
            primary=view! {
                <Body1>
                    <code class=styles.code.clone()>{from_field_display}</code>
                    " → "
                    {target_link}
                </Body1>
            }.into_any()
            meta=view! {
                <OnDeleteHint
                    schema_name=schema_name.clone()
                    from_field=from_field_hint
                    to_table=to_table.clone()
                    label=label.clone()
                    cardinality=cardinality.clone()
                    on_delete=on_delete.clone()
                    subtle_class=styles.subtle.clone()
                />
                {trait_source_view}
            }.into_any()
        />
    }
}

/// Incoming schema connection row.
#[component]
pub fn InverseSchemaConnectionRow(connection: InverseSchemaConnection) -> impl IntoView {
    let styles = code_style_classes();
    let from_table = connection.from_table.clone();
    let from_field = connection.from_field.clone();
    let label = connection.label.clone();

    view! {
        <style>{styles.sheet.clone()}</style>
        <ConnectionRowLayout
            list_item_class=styles.list_item.clone()
            meta_row_class=styles.meta_row.clone()
            badge=view! { <BadgeHint label="Ref".to_string() tooltip=ref_badge_tooltip() /> }.into_any()
            primary=view! {
                <Body1>
                    <TruncatedRecordLink
                        href=valence_backend::valence_schema_path(&from_table)
                        display=from_table.clone()
                        link_wrap_class=styles.link_wrap.clone()
                        link_button_class=styles.link_button.clone()
                    />
                    " → "
                    <code class=styles.code.clone()>{from_field}</code>
                </Body1>
            }.into_any()
            meta=view! {
                <Caption1 class=styles.subtle.clone()>{label}</Caption1>
            }.into_any()
        />
    }
}

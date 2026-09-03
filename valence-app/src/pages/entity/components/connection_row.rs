use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1};
use orbital::primitives::*;

use crate::components::{
    code_style_classes, fk_badge_tooltip, ref_badge_tooltip, BadgeHint, ConnectionRowLayout,
    OnDeleteHint, TruncatedRecordLink,
};
use crate::server::InverseConnectionData;

#[component]
pub fn ConnectionRow(
    schema_name: String,
    from_field: String,
    to_table: String,
    label: String,
    value: String,
    cardinality: String,
    on_delete: String,
    #[prop(optional)] target_trait: String,
    #[prop(optional)] trait_source: String,
) -> impl IntoView {
    let styles = code_style_classes();

    let connection_view: AnyView = if value.is_empty() {
        if !target_trait.is_empty() {
            let href = valence_backend::valence_trait_path(&target_trait);
            view! {
                <A href=href>
                    <Button appearance=ButtonAppearance::Subtle>
                        {target_trait.clone()}
                    </Button>
                </A>
            }
            .into_any()
        } else {
            view! { <Body1 class=styles.subtle.clone()>"—"</Body1> }.into_any()
        }
    } else if value == "-----" {
        view! {
            <Body1 class=styles.subtle.clone() style="font-style: italic; letter-spacing: 2px;">
                "-----"
            </Body1>
        }
        .into_any()
    } else {
        let (resolved_table, resolved_id) = match value.split_once(':') {
            Some((table, id)) if !table.is_empty() && !id.is_empty() => {
                (table.to_string(), id.to_string())
            }
            _ => (
                to_table.clone(),
                super::strip_record_id_prefix(&value).to_string(),
            ),
        };
        let href = valence_backend::valence_entity_path(&resolved_table, &resolved_id);
        let display = format!("{}/{}", resolved_table, resolved_id);
        view! {
            <TruncatedRecordLink
                href=href
                display=display
                link_wrap_class=styles.link_wrap.clone()
                link_button_class=styles.link_button.clone()
            />
        }
        .into_any()
    };

    let trait_source_view: AnyView = if !trait_source.is_empty() {
        let href = valence_backend::valence_trait_path(&trait_source);
        view! {
            <Caption1 class=styles.subtle.clone()>
                "Inherited from "
                <A href=href>
                    <Button appearance=ButtonAppearance::Subtle>
                        {trait_source.clone()}
                    </Button>
                </A>
            </Caption1>
        }
        .into_any()
    } else {
        view! {}.into_any()
    };

    let from_field_display = from_field.clone();
    let schema_for_hint = schema_name.clone();
    let field_for_hint = from_field.clone();
    let table_for_hint = to_table.clone();
    let label_for_hint = label.clone();
    let card_for_hint = cardinality.clone();
    let delete_for_hint = on_delete.clone();

    view! {
        <style>{styles.sheet.clone()}</style>
        <div data-testid="valence-entity-connection-row" style="display: contents">
            <ConnectionRowLayout
                list_item_class=styles.list_item.clone()
                meta_row_class=styles.meta_row.clone()
                badge=view! { <BadgeHint label="FK".to_string() tooltip=fk_badge_tooltip() /> }.into_any()
                primary=view! {
                    <Body1>
                        <code class=styles.code.clone()>{from_field_display}</code>
                        " → "
                        {connection_view}
                    </Body1>
                }.into_any()
                meta=view! {
                    <OnDeleteHint
                        schema_name=schema_for_hint
                        from_field=field_for_hint
                        to_table=table_for_hint
                        label=label_for_hint
                        cardinality=card_for_hint
                        on_delete=delete_for_hint
                        subtle_class=styles.subtle.clone()
                    />
                    {trait_source_view}
                }.into_any()
            />
        </div>
    }
}

#[component]
pub fn InverseConnectionRow(inv: InverseConnectionData) -> impl IntoView {
    let styles = code_style_classes();

    let from_table = inv.from_table.clone();
    let from_field = inv.from_field.clone();
    let label = inv.label.clone();
    let ids = inv.referencing_ids.clone();
    let restricted = inv.privacy_restricted;

    let refs_view: AnyView = if ids.is_empty() {
        view! { <Body1 class=styles.subtle.clone()>"None"</Body1> }.into_any()
    } else {
        let items: Vec<_> = ids
            .into_iter()
            .map(|id| {
                let ft = from_table.clone();
                let display = format!("{}/{}", ft, id);
                if restricted {
                    view! {
                        <div class=styles.link_wrap.clone()>
                            <Button appearance=ButtonAppearance::Subtle disabled=true>
                                <span class=styles.link_button.clone()>{display}</span>
                            </Button>
                        </div>
                    }
                    .into_any()
                } else {
                    let href = valence_backend::valence_entity_path(&ft, &id);
                    view! {
                        <TruncatedRecordLink
                            href=href
                            display=display
                            link_wrap_class=styles.link_wrap.clone()
                            link_button_class=styles.link_button.clone()
                        />
                    }
                    .into_any()
                }
            })
            .collect();
        view! { <Flex vertical=true wrap=FlexWrap::Wrap>{items}</Flex> }.into_any()
    };

    view! {
        <style>{styles.sheet.clone()}</style>
        <div data-testid="valence-entity-inverse-connection-row" style="display: contents">
            <ConnectionRowLayout
                list_item_class=styles.list_item.clone()
                meta_row_class=styles.meta_row.clone()
                badge=view! { <BadgeHint label="Ref".to_string() tooltip=ref_badge_tooltip() /> }.into_any()
                primary=view! {
                    <Body1>
                        <code class=styles.code.clone()>{format!("{}.{}", from_table, from_field)}</code>
                    </Body1>
                    {refs_view}
                }.into_any()
                meta=view! {
                    <Caption1 class=styles.subtle.clone()>{label}</Caption1>
                }.into_any()
            />
        </div>
    }
}

use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, EmptyState, IdentityCard, Stack, StackConfig};
use orbital::primitives::*;

use crate::server::{Owner, OwnershipTransferRow};

const META_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(8),
    horizontal: false,
    align: None,
    justify: None,
};

fn is_placeholder(value: &str) -> bool {
    value.is_empty() || value == "—" || value.eq_ignore_ascii_case("unknown")
}

fn owner_is_unknown(owner: &Owner) -> bool {
    owner.name == "Unknown" && owner.id == "unknown"
}

fn optional_field(value: String) -> Option<String> {
    if is_placeholder(&value) {
        None
    } else {
        Some(value)
    }
}

/// Owner badges + identity card for Valence entity pages.
#[component]
pub fn ValenceOwnerIdentity(owner: Owner) -> impl IntoView {
    let owner_name = owner.name.clone();
    let owner_role = owner.role.clone();
    let owner_email = owner.email.clone();
    let owner_handle = owner.handle.clone();
    let owner_kind = owner.owner_kind.clone();
    let ownership_status = owner.ownership_status.clone();
    let owner_entity_path = owner.owner_entity_path.clone();
    let unknown = owner_is_unknown(&owner);

    let subtitle = optional_field(owner_role);
    let email = optional_field(owner_email);
    let handle = optional_field(owner_handle);

    view! {
        <Stack config=META_STACK>
            <Flex align=FlexAlign::Center gap=FlexGap::Small wrap=FlexWrap::Wrap>
                <span data-testid="valence-owner-kind">
                    <Badge>{owner_kind.clone()}</Badge>
                </span>
                <span data-testid="valence-owner-status">
                    <Badge color=BadgeColor::Subtle>{ownership_status.clone()}</Badge>
                </span>
                {owner_entity_path.clone().map(|path| {
                    view! {
                        <span data-testid="valence-owner-entity-link">
                            <A href=path.clone()>"Open owner record"</A>
                        </span>
                    }
                    .into_any()
                }).unwrap_or_else(|| view! { <span></span> }.into_any())}
            </Flex>
            {if unknown {
                view! {
                    <div data-testid="valence-owner-identity-empty">
                        <EmptyState message="No owner assigned" />
                    </div>
                }.into_any()
            } else {
                view! {
                    <div data-testid="valence-owner-identity-card">
                        <IdentityCard
                            name=owner_name.clone()
                            title=owner_name.clone()
                            subtitle=subtitle
                            email=email
                            handle=handle
                            avatar_size=40
                        />
                    </div>
                }.into_any()
            }}
        </Stack>
    }
}

/// Transfer history list for ownership changes.
#[component]
pub fn ValenceOwnerTransferHistory(transfers: Vec<OwnershipTransferRow>) -> impl IntoView {
    let transfers = StoredValue::new(transfers);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .TransferLine {
            display: flex;
            flex-direction: column;
            gap: 4px;
            margin: 8px 0;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="valence-entity-transfer-history-list">
            {if transfers.get_value().is_empty() {
                view! { <Caption1>"No transfers recorded."</Caption1> }.into_any()
            } else {
                view! {
                    <For
                        each=move || transfers.get_value()
                        key=|t| t.id.clone()
                        let:t
                    >
                        <div class=class_names.transfer_line>
                            <Body1>
                                {format!(
                                    "{} → {} · {}",
                                    t.from_owner_type, t.to_owner_type, t.transferred_at
                                )}
                            </Body1>
                            <Caption1>
                                {format!(
                                    "by {} · {} → {}",
                                    t.transferred_by, t.from_owner_id, t.to_owner_id
                                )}
                            </Caption1>
                            {t.reason.clone().map(|r| {
                                view! { <Caption1>{r}</Caption1> }.into_any()
                            }).unwrap_or_else(|| view! { <span></span> }.into_any())}
                        </div>
                    </For>
                }
                .into_any()
            }}
        </div>
    }
}

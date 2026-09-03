use leptos::prelude::*;
use orbital::components::{Caption1, Card, CardContent, CardSectionBorder, Stack, StackConfig};
use orbital::primitives::*;

use crate::components::{
    ValenceHelpCardHeader, ValenceHelpColumnHeader, ValenceOwnerIdentity,
    ValenceOwnerTransferHistory,
};
use crate::server::Owner;

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn EntityOwnerCard(owner: Owner) -> impl IntoView {
    let transfers = owner.transfers.clone();

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
    };

    view! {
        <style>{style_sheet}</style>
        <div id="valence-entity-owner" data-testid="valence-entity-owner-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Owner"
                    description="Who is responsible for this record."
                    info=view! {
                        <Caption1>
                            "Transfer history shows past ownership changes when available."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <ValenceOwnerIdentity owner=owner />
                        <CardSectionBorder />
                        <div data-testid="valence-owner-transfers">
                            <details>
                                <summary>
                                    <ValenceHelpColumnHeader
                                        label="Transfer history"
                                        info=view! {
                                            <Caption1>"Past ownership transfers for this record."</Caption1>
                                        }.into_any()
                                    />
                                </summary>
                                <ValenceOwnerTransferHistory transfers=transfers />
                            </details>
                        </div>
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

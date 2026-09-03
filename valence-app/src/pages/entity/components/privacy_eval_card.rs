use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, SectionTitle, Stack, StackConfig};
use orbital::primitives::*;

use crate::components::bordered_table_styles;
use crate::components::{ValenceHelpCardHeader, ValenceHelpColumnHeader};
use crate::server::EntityPrivacyEvalCardData;

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

const META_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(8),
    horizontal: false,
    align: None,
    justify: None,
};

#[component]
pub fn EntityPrivacyEvalCard(data: EntityPrivacyEvalCardData) -> impl IntoView {
    let table_styles = bordered_table_styles();
    let viewer_label = data.viewer_label.clone();

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .ActionCol { width: 48px; white-space: nowrap; }
        .BucketCol { width: 48px; white-space: nowrap; }
        .PolicyCell { overflow: hidden; }
    };

    let render_rows = move || {
        if data.rows.is_empty() {
            view! {
                <TableRow>
                    <TableCell attr:colspan=4>
                        <Body1 class=table_styles.subtle.clone()>"No policies configured."</Body1>
                    </TableCell>
                </TableRow>
            }
            .into_any()
        } else {
            data.rows
                .clone()
                .into_iter()
                .map(|row| {
                    let policy_name = row.policy.name.clone();
                    let trait_badge = match row.policy.trait_source {
                        Some(trait_name) => {
                            let href = valence_backend::valence_trait_path(&trait_name);
                            view! {
                                <Caption1 class=table_styles.subtle.clone()>
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
                    let (label, badge_color) = if row.policy.passes {
                        ("Pass", BadgeColor::Success)
                    } else {
                        ("Fail", BadgeColor::Danger)
                    };
                    view! {
                        <TableRow>
                            <TableCell class=class_names.action_col>
                                <Caption1>{row.action}</Caption1>
                            </TableCell>
                            <TableCell class=class_names.bucket_col>
                                <Caption1>{row.bucket}</Caption1>
                            </TableCell>
                            <TableCell class=class_names.policy_cell>
                                <Body1>
                                    <code class=table_styles.code.clone()>{policy_name}</code>
                                    {trait_badge}
                                </Body1>
                            </TableCell>
                            <TableCell>
                                <Badge color=badge_color>{label}</Badge>
                            </TableCell>
                        </TableRow>
                    }
                })
                .collect_view()
                .into_any()
        }
    };

    view! {
        <style>{style_sheet}</style>
        <style>{table_styles.sheet.clone()}</style>
        <div id="valence-entity-privacy" style="width: 100%;">
        <Card class=class_names.card>
            <ValenceHelpCardHeader
                title="Privacy Evaluation"
                description="Whether the current request actor can perform each action on this record."
                info=view! {
                    <Caption1>
                        "Evaluation always uses the signed-in (or anonymous) request actor. Pass means the action would be allowed; Fail means it would be blocked."
                    </Caption1>
                }.into_any()
            />
            <CardContent>
                <Stack config=BODY_STACK>
                    <Stack config=META_STACK>
                        <ValenceHelpColumnHeader
                            label="Request actor"
                            info=view! {
                                <Caption1>"Bound to the current request; not selectable by the client."</Caption1>
                            }.into_any()
                        />
                        <div data-testid="valence-privacy-viewer-id">
                            <Body1>
                                <code class=table_styles.code.clone()>{viewer_label}</code>
                            </Body1>
                        </div>
                    </Stack>
                    <div>
                        <SectionTitle>"All Actions"</SectionTitle>
                        <div class=table_styles.table_wrap.clone()>
                            <Table class=table_styles.compact_table.clone()>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell class=class_names.action_col>
                                            <ValenceHelpColumnHeader label="Action" />
                                        </TableHeaderCell>
                                        <TableHeaderCell class=class_names.bucket_col>
                                            <ValenceHelpColumnHeader label="Bucket" />
                                        </TableHeaderCell>
                                        <TableHeaderCell>
                                            <ValenceHelpColumnHeader label="Policy" />
                                        </TableHeaderCell>
                                        <TableHeaderCell>
                                            <ValenceHelpColumnHeader
                                                label="Result"
                                                info=view! {
                                                    <Caption1>"Pass means allowed; Fail means the action would be denied for this actor."</Caption1>
                                                }.into_any()
                                            />
                                        </TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>{render_rows}</TableBody>
                            </Table>
                        </div>
                    </div>
                </Stack>
            </CardContent>
        </Card>
        </div>
    }
}

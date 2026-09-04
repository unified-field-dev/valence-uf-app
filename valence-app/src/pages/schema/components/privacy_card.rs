use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, SectionTitle};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::{ValenceHelpCardHeader, ValenceHelpColumnHeader};
use crate::server::SchemaPrivacyCardData;

#[component]
pub fn PrivacyPoliciesCard(data: SchemaPrivacyCardData) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .Body {
            display: flex;
            flex-direction: column;
            gap: var(--spacingVerticalM);
        }

        .TableWrap {
            border: 1px solid var(--colorNeutralStroke2);
            border-radius: 8px;
            overflow: hidden;
        }

        .CompactTable {
            width: 100%;
            table-layout: fixed;
        }

        .ActionCol { width: 48px; white-space: nowrap; }
        .BucketCol { width: 48px; white-space: nowrap; }
        .PolicyCol { width: 42%; overflow: hidden; }
        .DescCol { width: 28%; overflow: hidden; }
        .PolicyCell { width: 42%; overflow: hidden; }
        .DescCell { width: 28%; overflow: hidden; }

        .DescriptionEllipsis {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            width: 100%;
            display: block;
        }

        .Code {
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
            background-color: var(--colorNeutralBackground3);
            padding: 2px 6px;
            border-radius: 4px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            max-width: 100%;
            display: inline-block;
            vertical-align: bottom;
        }

        .Subtle {
            color: var(--colorNeutralForeground3);
        }
    };

    let render_rows = move || {
        if data.rows.is_empty() {
            view! {
                <TableRow>
                    <TableCell attr:colspan=4>
                        <Body1 class=class_names.subtle>"No policies configured."</Body1>
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
                    let policy_name_title = policy_name.clone();
                    let description = row
                        .policy
                        .description
                        .unwrap_or_else(|| "—".to_string());
                    let description_title = description.clone();
                    let trait_badge = match row.policy.trait_source {
                        Some(trait_name) => {
                            let href = valence_backend::valence_trait_path(&trait_name);
                            view! {
                                <Caption1 class=class_names.subtle>
                                    "Inherited from "
                                    <A href=href>
                                        <Button appearance=ButtonAppearance::Subtle>
                                            {trait_name}
                                        </Button>
                                    </A>
                                </Caption1>
                            }.into_any()
                        }
                        None => view! {}.into_any(),
                    };
                    view! {
                        <TableRow>
                            <TableCell class=class_names.action_col><Caption1>{row.action}</Caption1></TableCell>
                            <TableCell class=class_names.bucket_col><Caption1>{row.bucket}</Caption1></TableCell>
                            <TableCell class=class_names.policy_cell>
                                <Body1>
                                    <span class=class_names.code title=policy_name_title>
                                        {policy_name}
                                    </span>
                                    {trait_badge}
                                </Body1>
                            </TableCell>
                            <TableCell class=class_names.desc_cell>
                                <Caption1>
                                    <span class=class_names.description_ellipsis title=description_title>
                                        {description}
                                    </span>
                                </Caption1>
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
        <div id="valence-schema-privacy" data-testid="valence-schema-privacy-card" style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader
                    title="Privacy Policies"
                    description="Rules checked when someone reads, creates, updates, or deletes records in this table."
                    info=view! {
                        <Caption1>
                            "Each action runs through these rules in order. If a rule in the block group matches, the action is denied and nothing changes. Rules inherited from traits show an Inherited from link."
                        </Caption1>
                    }.into_any()
                />
                <CardContent>
                    <div class=class_names.body>
                        <SectionTitle>"All Actions"</SectionTitle>
                        <div class=class_names.table_wrap>
                            <Table class=class_names.compact_table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell class=class_names.action_col>
                                            <ValenceHelpColumnHeader
                                                label="Action"
                                                info=view! {
                                                    <Caption1>"What someone is trying to do: read, create, update, or delete."</Caption1>
                                                }.into_any()
                                            />
                                        </TableHeaderCell>
                                        <TableHeaderCell class=class_names.bucket_col>
                                            <ValenceHelpColumnHeader
                                                label="Bucket"
                                                info=view! {
                                                    <Caption1>"Priority group for this rule. Block rules can deny an action; allow rules can permit it."</Caption1>
                                                }.into_any()
                                            />
                                        </TableHeaderCell>
                                        <TableHeaderCell class=class_names.policy_col>
                                            <ValenceHelpColumnHeader
                                                label="Policy"
                                                info=view! {
                                                    <Caption1>"The named rule that is evaluated for this action."</Caption1>
                                                }.into_any()
                                            />
                                        </TableHeaderCell>
                                        <TableHeaderCell class=class_names.desc_col>
                                            <ValenceHelpColumnHeader
                                                label="Description"
                                                info=view! {
                                                    <Caption1>"Plain-language summary of what the policy checks."</Caption1>
                                                }.into_any()
                                            />
                                        </TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>{render_rows}</TableBody>
                            </Table>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

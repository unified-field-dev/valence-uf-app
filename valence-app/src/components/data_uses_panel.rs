//! Shared Data uses panel: op tabs + purpose markdown + View source.

use leptos::prelude::*;
use orbital::components::{
    Caption1, Card, CardContent, EmptyState, Link, Stack, StackConfig, Tag, ThemeColor,
};
use orbital::primitives::{FlexGap, Tab, TabList};
use orbital_markdown::{render_to_html, OrbitalMarkdownOptions, RenderContext};
use turf::inline_style_sheet_values;

use crate::components::ValenceHelpCardHeader;
use crate::server::DataUseRow;

const BODY_STACK: StackConfig = StackConfig {
    gap: FlexGap::Size(16),
    horizontal: false,
    align: None,
    justify: None,
};

/// Orbital card listing declared data uses, filtered by Read/Create/Update/Delete tabs.
#[component]
pub fn DataUsesPanel(
    /// Spotlight / test id root (e.g. `valence-schema-data-uses`).
    panel_id: &'static str,
    /// Card title.
    title: &'static str,
    /// Card description under the title.
    description: &'static str,
    /// Rows from the matching server fn.
    rows: Vec<DataUseRow>,
) -> impl IntoView {
    let tab = RwSignal::new("read".to_string());
    let rows = StoredValue::new(rows);

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card {
            width: 100%;
            margin: 0;
        }

        .ListPane {
            max-height: 28rem;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: var(--spacingVerticalM);
        }

        .Row {
            display: flex;
            flex-direction: column;
            gap: var(--spacingVerticalXS);
            padding-bottom: var(--spacingVerticalM);
            border-bottom: 1px solid var(--colorNeutralStroke2);
        }

        .PurposeHtml {
            font-size: var(--fontSizeBase300);
            line-height: var(--lineHeightBase300);
            color: var(--colorNeutralForeground1);
        }

        .PurposeHtml p {
            margin: 0 0 var(--spacingVerticalXS) 0;
        }

        .Meta {
            display: flex;
            flex-wrap: wrap;
            gap: var(--spacingHorizontalS);
            align-items: center;
        }
    };

    let count_for =
        move |op: &'static str| rows.get_value().into_iter().filter(|r| r.op == op).count();

    view! {
        <style>{style_sheet}</style>
        <div id=panel_id data-testid=panel_id style="width: 100%;">
            <Card class=class_names.card>
                <ValenceHelpCardHeader title=title description=description />
                <CardContent>
                    <Stack config=BODY_STACK>
                        <TabList selected_value=tab>
                            <Tab value="read".to_string()>
                                {move || format!("Reads ({})", count_for("read"))}
                            </Tab>
                            <Tab value="create".to_string()>
                                {move || format!("Creates ({})", count_for("create"))}
                            </Tab>
                            <Tab value="update".to_string()>
                                {move || format!("Updates ({})", count_for("update"))}
                            </Tab>
                            <Tab value="delete".to_string()>
                                {move || format!("Deletes ({})", count_for("delete"))}
                            </Tab>
                        </TabList>

                        <div class=class_names.list_pane>
                            {move || {
                                let op = tab.get();
                                let filtered: Vec<DataUseRow> = rows
                                    .get_value()
                                    .into_iter()
                                    .filter(|r| r.op == op)
                                    .collect();
                                if filtered.is_empty() {
                                    view! {
                                        <EmptyState
                                            message="No declared uses"
                                            description="Nothing in this tab for the current catalog snapshot."
                                        />
                                    }
                                    .into_any()
                                } else {
                                    let row_class = class_names.row;
                                    let purpose_class = class_names.purpose_html;
                                    let meta_class = class_names.meta;
                                    filtered
                                        .into_iter()
                                        .map(move |row| {
                                            let html = render_to_html(
                                                &row.purpose,
                                                &OrbitalMarkdownOptions::default(),
                                                &RenderContext::default(),
                                            );
                                            let via = row.via_trait.clone();
                                            let source_url = row.source_url.clone();
                                            let method = row.method.clone();
                                            let file = row.file.clone();
                                            let line = row.line;
                                            let crate_name = row.crate_name.clone();
                                            view! {
                                                <div class=row_class data-testid="valence-data-use-row">
                                                    <div class=purpose_class inner_html=html />
                                                    <div class=meta_class>
                                                        {via.map(|t| view! {
                                                            <Tag>{format!("via {t}")}</Tag>
                                                        })}
                                                        <Caption1 color=ThemeColor::NeutralForeground3>
                                                            {format!("{crate_name} · {method}")}
                                                        </Caption1>
                                                        <Caption1 color=ThemeColor::NeutralForeground3>
                                                            {format!("{file}:{line}")}
                                                        </Caption1>
                                                        {(!source_url.is_empty()).then(|| {
                                                            let href = source_url.clone();
                                                            view! {
                                                                <Link href=href inline=true>
                                                                    "View source"
                                                                </Link>
                                                            }
                                                        })}
                                                    </div>
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            }}
                        </div>
                    </Stack>
                </CardContent>
            </Card>
        </div>
    }
}

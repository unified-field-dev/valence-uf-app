use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Caption1, Card, CardContent, CardHeader, EmptyState};
use orbital::primitives::*;
use turf::inline_style_sheet_values;

use crate::components::ValenceHelpCardHeader;
use crate::server::get_schema_samples;

#[component]
pub fn SamplesCard(schema_name: String) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Card { width: 100%; margin: 0; }
        .Body { display: flex; flex-direction: column; gap: var(--spacingVerticalM); }
        .TableWrap {
            overflow-x: auto;
            border: 1px solid var(--colorNeutralStroke2);
            border-radius: 8px;
            overflow: hidden;
        }
        .MessageBarWrapper { white-space: normal; overflow-wrap: break-word; word-wrap: break-word; }
        .StatusText { text-align: center; color: var(--colorNeutralForeground3); }
        .Subtle { color: var(--colorNeutralForeground3); }
    };

    let schema_name_for_resource = schema_name.clone();
    let schema_name_for_view = StoredValue::new(schema_name.clone());

    let samples_res = Resource::new(
        move || schema_name_for_resource.clone(),
        |name| async move { get_schema_samples(name, 10).await },
    );

    let samples_memo =
        Memo::new(move |_| samples_res.get().and_then(|r| r.ok()).unwrap_or_default());

    view! {
        <style>{style_sheet}</style>
        <div id="valence-schema-samples" data-testid="valence-schema-samples-card" style="width: 100%;">
            <Card class=class_names.card>
                                    <ValenceHelpCardHeader
                        title="Samples"
                        description="Recent example records — useful for exploring real data shapes."
                    />
                <CardContent>
                    <div class=class_names.body>
                        <Caption1 class=class_names.subtle>"Latest 10 rows"</Caption1>
                        <Suspense fallback=move || view! {
                            <Body1 class=class_names.status_text>"Loading samples..."</Body1>
                        }>
                            {move || match samples_res.get() {
                                Some(Ok(samples)) => {
                                    if samples.is_empty() {
                                        view! {
                                            <div data-testid="valence-samples-empty">
                                                <EmptyState message="No samples found" />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class=class_names.table_wrap data-testid="valence-samples-table">
                                                <Table>
                                                    <TableHeader>
                                                        <TableRow>
                                                            <TableHeaderCell>"id"</TableHeaderCell>
                                                        </TableRow>
                                                    </TableHeader>
                                                    <TableBody>
                                                        <For each=move || samples_memo.get() key=|s| s.id.clone() let:s>
                                                            {
                                                                let id = s.id.clone();
                                                                let href = valence_backend::valence_entity_path(&schema_name_for_view.get_value(), &id);
                                                                view! {
                                                                    <TableRow>
                                                                        <TableCell>
                                                                            <A href=href>
                                                                                <Button appearance=ButtonAppearance::Subtle>{id}</Button>
                                                                            </A>
                                                                        </TableCell>
                                                                    </TableRow>
                                                                }
                                                            }
                                                        </For>
                                                    </TableBody>
                                                </Table>
                                            </div>
                                        }.into_any()
                                    }
                                }
                                Some(Err(err)) => view! {
                                    <MessageBar intent=MessageBarIntent::Error class=class_names.message_bar_wrapper>
                                        <Body1 wrap=true>{format!("Failed to load samples: {}", err)}</Body1>
                                    </MessageBar>
                                }.into_any(),
                                None => view! {
                                    <Body1 class=class_names.status_text>"Loading..."</Body1>
                                }.into_any(),
                            }}
                        </Suspense>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

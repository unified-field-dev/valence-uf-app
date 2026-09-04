use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::server::IterRunSummary;
use orbital::components::{Caption1, Text, TextTag};
use orbital::primitives::*;

use super::IterRunStatusBadge;

fn truncate_run_id(run_id: &str) -> String {
    if run_id.len() <= 12 {
        run_id.to_string()
    } else {
        format!("{}…", &run_id[..12])
    }
}

fn format_progress(processed_rows: i64, total_rows: i64) -> String {
    if total_rows == 0 {
        "—".to_string()
    } else {
        format!("{processed_rows}/{total_rows}")
    }
}

/// A single row in the iter runs table.
#[component]
pub fn IterRunRow(
    run: IterRunSummary,
    #[prop(default = false)] hide_schema: bool,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let run_id = run.run_id.clone();
    let href = valence_backend::valence_iter_run_path(&run.target_table, &run.run_id);
    let href_for_link = href.clone();
    let display_run_id = truncate_run_id(&run_id);
    let progress = format_progress(run.processed_rows, run.total_rows);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Row { cursor: pointer; }
        .Row:hover { background: var(--orb-color-surface-canvas-hover); }
        .Link {
            color: var(--orb-color-brand-link);
            text-decoration: none;
        }
        .Link:hover {
            color: var(--orb-color-brand-link-hover);
            text-decoration: underline;
        }
    };

    let nav = nav_store.with_value(|n| n.clone());

    view! {
        <style>{style_sheet}</style>
        <TableRow
            class=class_names.row
            on:click=move |_| nav(&href, Default::default())
        >
            <TableCell>
                <A href=href_for_link attr:class=class_names.link attr:title=run_id.clone()>
                    {display_run_id}
                </A>
            </TableCell>
            <TableCell>
                <Text tag=TextTag::Code font=TextFont::Monospace>{run.iter_name.clone()}</Text>
            </TableCell>
            {(!hide_schema).then(|| {
                let schema_href = valence_backend::valence_schema_path(&run.target_table);
                view! {
                    <TableCell>
                        <A href=schema_href attr:class=class_names.link on:click=|ev| ev.stop_propagation()>
                            {run.target_table.clone()}
                        </A>
                    </TableCell>
                }
            })}
            <TableCell><IterRunStatusBadge status=run.status.clone() /></TableCell>
            <TableCell>{progress}</TableCell>
            <TableCell><Caption1>{run.created_at.clone()}</Caption1></TableCell>
        </TableRow>
    }
}

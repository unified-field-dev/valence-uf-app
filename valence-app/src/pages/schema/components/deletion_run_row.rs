use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use orbital::primitives::*;

use crate::server::DeletionRunView;

use super::deletion_run_status_badge::DeletionRunStatusBadge;

fn truncate_id(id: &str) -> String {
    if id.len() <= 16 {
        id.to_string()
    } else {
        format!("{}…", &id[..16])
    }
}

#[component]
pub fn DeletionRunRow(run: DeletionRunView) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let href = valence_backend::valence_deletion_run_path(&run.root_table, &run.run_id);
    let href_for_link = href.clone();
    let entity_href = valence_backend::valence_entity_path(&run.root_table, &run.root_record_id);
    let display_record_id = truncate_id(&run.root_record_id);
    let progress = format!(
        "{}/{}",
        run.completed_steps + run.failed_steps,
        run.total_steps.max(1),
    );

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Row { cursor: pointer; }
        .Row:hover { background: var(--colorNeutralBackground1Hover); }
        .Link {
            color: var(--colorBrandForeground1);
            text-decoration: none;
        }
        .Link:hover { text-decoration: underline; }
        .Monospace {
            font-family: var(--fontFamilyMonospace);
        }
        .TimeCell { color: var(--colorNeutralForeground3); }
        .ActionCol { width: 72px; white-space: nowrap; }
    };

    let nav = nav_store.with_value(|n| n.clone());

    view! {
        <style>{style_sheet}</style>
        <TableRow
            class=class_names.row
            on:click=move |_| nav(&href, Default::default())
        >
            <TableCell>
                <A
                    href=entity_href
                    attr:class=class_names.link
                    attr:title=run.root_record_id.clone()
                    on:click=|ev| ev.stop_propagation()
                >
                    <span class=class_names.monospace>{display_record_id}</span>
                </A>
            </TableCell>
            <TableCell>{progress}</TableCell>
            <TableCell><DeletionRunStatusBadge status=run.status.clone() /></TableCell>
            <TableCell class=class_names.time_cell>{run.requested_at.clone()}</TableCell>
            <TableCell class=class_names.action_col>
                <A href=href_for_link on:click=|ev| ev.stop_propagation()>
                    <Button appearance=ButtonAppearance::Subtle>"Open"</Button>
                </A>
            </TableCell>
        </TableRow>
    }
}

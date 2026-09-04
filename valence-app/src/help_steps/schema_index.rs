//! Spotlight steps for the schema catalog (`/valence/schema`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Schema index page intro.
#[help_spotlight_step(
    route = "/valence/schema",
    feature_highlight = "valence-schema-index-intro",
    title = "Schema catalog",
    spotlight = "valence-schema-index-page",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceSchemaIndexIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-index-intro",
        "Every registered schema appears here — like the index cards at a library front desk. Pick one to see its fields, privacy rules, and sample rows.",
        Some("If the table is empty, no schemas are registered yet."),
        &[],
    )
}

/// Search on the schema list.
#[help_spotlight_step(
    route = "/valence/schema",
    feature_highlight = "valence-schema-index-search",
    title = "Find a schema",
    spotlight = "valence-schema-index-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ValenceSchemaIndexSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-index-search",
        "Use search when the catalog is long. Type part of a schema name to narrow the list; clear the box to see every schema again.",
        None,
        &[],
    )
}

/// Schemas data table.
#[help_spotlight_step(
    route = "/valence/schema",
    feature_highlight = "valence-schema-index-table",
    title = "Reading the table",
    spotlight = "valence-schemas-list",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceSchemaIndexTableHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-index-table",
        "Each row is one schema. Compare name, description, and row counts before you open a detail page.",
        None,
        &[
            "Row count: how many records exist in that schema",
            "Traits: reusable field bundles this schema includes",
        ],
    )
}

/// Open action on a schema row.
#[help_spotlight_step(
    route = "/valence/schema",
    feature_highlight = "valence-schema-index-open",
    title = "Open a schema",
    spotlight = "valence-schemas-list",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceSchemaIndexOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-index-open",
        "Click a row or its Open link to land on the schema detail page: fields, privacy, samples, iters, and deletion history for that shelf.",
        None,
        &[],
    )
}

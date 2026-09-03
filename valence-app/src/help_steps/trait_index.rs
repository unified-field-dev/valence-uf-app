//! Spotlight steps for the trait catalog (`/valence/traits`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Trait index page intro.
#[help_spotlight_step(
    route = "/valence/traits",
    feature_highlight = "valence-traits-intro",
    title = "Trait catalog",
    spotlight = "valence-traits-page",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceTraitsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-traits-intro",
        "Traits are reusable field bundles — shared shapes that multiple schemas can include, like a standard address block used on several forms.",
        Some("If the table is empty, no traits are registered yet."),
        &[],
    )
}

/// Traits data table.
#[help_spotlight_step(
    route = "/valence/traits",
    feature_highlight = "valence-traits-table",
    title = "Reading the table",
    spotlight = "valence-traits-list",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceTraitsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-traits-table",
        "Each row is one trait. Compare name, field count, and which schemas implement it before opening detail.",
        None,
        &[],
    )
}

/// Open action on a trait row.
#[help_spotlight_step(
    route = "/valence/traits",
    feature_highlight = "valence-traits-open",
    title = "Open a trait",
    spotlight = "valence-traits-list",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceTraitsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-traits-open",
        "Click a row to open the trait detail page: field definitions, connections, and the schema list that uses this bundle.",
        None,
        &[],
    )
}

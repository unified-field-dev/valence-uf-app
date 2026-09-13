//! Spotlight steps for Unscoped uses (`/valence/unscoped-uses`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Unscoped uses page intro.
#[help_spotlight_step(
    route = "/valence/unscoped-uses",
    feature_highlight = "valence-unscoped-uses-intro",
    title = "Unscoped uses",
    spotlight = "valence-unscoped-uses",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceUnscopedUsesIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-unscoped-uses-intro",
        "Unscoped uses lists Valence calls that are not tied to one schema or trait — usually QueryCore walks across the catalog.",
        Some("Open a purpose to see why the code runs that path, then use View source when a repository URL is available."),
        &[],
    )
}

/// Unscoped data uses card tabs.
#[help_spotlight_step(
    route = "/valence/unscoped-uses",
    feature_highlight = "valence-unscoped-data-uses",
    title = "Op tabs",
    spotlight = "valence-unscoped-data-uses",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceUnscopedUsesTabsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-unscoped-data-uses",
        "Tabs group declared uses by operation: reads, creates, updates, and deletes. Counts show how many purposes land in each bucket.",
        None,
        &[],
    )
}

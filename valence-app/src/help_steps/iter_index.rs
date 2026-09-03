//! Spotlight steps for the iter runs index (`/valence/iters`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Iter index page intro.
#[help_spotlight_step(
    route = "/valence/iters",
    feature_highlight = "valence-iters-intro",
    title = "Iter runs",
    spotlight = "valence-iters-page",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceItersIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iters-intro",
        "Iter runs are batch jobs that walk records across schemas. This page lists recent runs so you can audit progress without opening each schema.",
        Some("If the list is empty, no iters have run yet."),
        &[],
    )
}

/// Iter runs list.
#[help_spotlight_step(
    route = "/valence/iters",
    feature_highlight = "valence-iters-list",
    title = "Recent runs",
    spotlight = "valence-iters-list",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceItersListHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iters-list",
        "Each row is one iter run: schema, status, and counters. Compare status chips before you open detail.",
        None,
        &[
            "Running: work is still in progress",
            "Completed: every planned record was touched",
            "Failed / Canceled: stopped early — open detail for errors",
        ],
    )
}

/// Open action on an iter run row.
#[help_spotlight_step(
    route = "/valence/iters",
    feature_highlight = "valence-iters-open",
    title = "Open a run",
    spotlight = "valence-iters-list",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceItersOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iters-open",
        "Click a row to open the iter run page: progress, errors, batches, and cancel for in-flight work.",
        None,
        &[],
    )
}

//! Spotlight steps for the deletion runs index (`/valence/deletions`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Deletion index page intro.
#[help_spotlight_step(
    route = "/valence/deletions",
    feature_highlight = "valence-deletions-intro",
    title = "Deletion runs",
    spotlight = "valence-deletions-page",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceDeletionsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletions-intro",
        "Deletion runs remove records and their dependents in cascade. This page lists recent runs across schemas.",
        Some("If the list is empty, no deletions are queued or finished yet."),
        &[],
    )
}

/// Deletion runs list.
#[help_spotlight_step(
    route = "/valence/deletions",
    feature_highlight = "valence-deletions-list",
    title = "Recent runs",
    spotlight = "valence-deletions-list",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceDeletionsListHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletions-list",
        "Each row is one deletion run: schema, root record, and status. Scan status before opening detail.",
        None,
        &[
            "Running: cascade steps are still executing",
            "Completed: root and dependents were removed",
            "Failed / Canceled: stopped early — open detail for step errors",
        ],
    )
}

/// Open action on a deletion run row.
#[help_spotlight_step(
    route = "/valence/deletions",
    feature_highlight = "valence-deletions-open",
    title = "Open a run",
    spotlight = "valence-deletions-list",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceDeletionsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletions-open",
        "Click a row to open the deletion run page: progress, cascade steps, and cancel for in-flight work.",
        None,
        &[],
    )
}

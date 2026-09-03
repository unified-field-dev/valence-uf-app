//! Spotlight steps for deletion run detail (`/valence/schema/:schema_name/deletion/:run_id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Deletion run page intro.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight = "valence-deletion-run-intro",
    title = "This deletion run",
    spotlight = "valence-deletion-run-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceDeletionRunIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletion-run-intro",
        "A deletion run removes one record and any dependent rows linked through foreign keys. The header shows run id, schema, and status.",
        None,
        &[],
    )
}

/// Progress indicator.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight = "valence-deletion-run-progress",
    title = "Progress",
    spotlight = "valence-deletion-run-progress",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceDeletionRunProgressHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletion-run-progress",
        "Progress shows how many cascade steps have finished. While status is Running, the bar advances as each dependent row is cleaned up.",
        None,
        &[],
    )
}

/// Cancel action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight = "valence-deletion-run-cancel",
    title = "Cancel run",
    spotlight = "valence-deletion-run-cancel",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceDeletionRunCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletion-run-cancel",
        "Cancel stops an in-flight deletion after the current step completes. Use it when a cascade was queued by mistake.",
        Some("Cancel requires admin permission on mutating server functions."),
        &[],
    )
}

/// Cascade steps table.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight = "valence-deletion-run-steps",
    title = "Cascade steps",
    spotlight = "valence-deletion-run-steps",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceDeletionRunStepsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletion-run-steps",
        "Each step is one row removed in the cascade — schema, record id, and outcome.",
        Some("Scan steps when a deletion stalls to see which dependent record blocked progress."),
        &[],
    )
}

/// Back link.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/deletion/:run_id",
    feature_highlight = "valence-deletion-run-back",
    title = "Back to deletions",
    spotlight = "valence-deletion-run-back",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceDeletionRunBackHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-deletion-run-back",
        "Back returns to the deletions list or schema page so you can pick another run or queue a new delete.",
        None,
        &[],
    )
}

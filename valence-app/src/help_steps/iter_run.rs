//! Spotlight steps for iter run detail (`/valence/schema/:schema_name/iter/:run_id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Iter run page intro.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-intro",
    title = "This iter run",
    spotlight = "valence-iter-run-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceIterRunIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-intro",
        "An iter run is one batch pass over records in a schema. The header shows run id, schema, and current status.",
        None,
        &[],
    )
}

/// Run counter stats.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-stats",
    title = "Run counters",
    spotlight = "valence-iter-run-stats",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ValenceIterRunStatsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-stats",
        "These four numbers summarize how far the batch has gotten.",
        None,
        &[
            "Total: records the iter planned to touch",
            "Processed: records finished successfully",
            "Skipped: records intentionally left alone",
            "Failed: records that hit an error",
        ],
    )
}

/// Progress indicator.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-progress",
    title = "Progress",
    spotlight = "valence-iter-run-progress",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceIterRunProgressHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-progress",
        "The progress bar and percentage show how much of the batch is done. While status is Running, these values update as work continues.",
        None,
        &[],
    )
}

/// Cancel action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-cancel",
    title = "Cancel run",
    spotlight = "valence-iter-run-cancel",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceIterRunCancelHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-cancel",
        "Cancel stops an in-flight iter after the current batch finishes. Use it when a run is stuck or no longer needed.",
        Some("Cancel requires admin permission on mutating server functions."),
        &[],
    )
}

/// Errors table.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-errors",
    title = "Errors",
    spotlight = "valence-iter-run-errors",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceIterRunErrorsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-errors",
        "When individual records fail, each error row shows which id broke and why.",
        Some("Fix the underlying issue, then start a fresh iter if you need to retry the batch."),
        &[],
    )
}

/// Batches table.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/iter/:run_id",
    feature_highlight = "valence-iter-run-batches",
    title = "Batches",
    spotlight = "valence-iter-run-batches",
    position = "top",
    order = 60
)]
#[component]
pub fn ValenceIterRunBatchesHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-iter-run-batches",
        "Batches split the iter into chunks for parallel work. Each row shows batch size, timing, and outcome.",
        Some("Use batches to see whether failures cluster in one chunk or spread across the run."),
        &[],
    )
}

//! Spotlight steps for the Valence dashboard (`/valence`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro: library catalog metaphor and Valence vocabulary.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-intro",
    title = "Welcome to Valence",
    order = 10
)]
#[component]
pub fn ValenceIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-intro",
        "Valence is the catalog for your app's stored data. Think of it like a library: each shelf is a schema, each book on the shelf is a record, and you can browse shelves, open a book, or watch bulk jobs finish.",
        Some("We will walk the screens one piece at a time."),
        &[
            "Schema: a table type (the shelf label)",
            "Record: one row in that table (one book)",
            "Trait: a reusable bundle of fields shared across schemas",
            "Iter: a batch job that walks many records",
            "Deletion: a cascade job that removes a record and its dependents",
        ],
    )
}

/// Schema-or-id search box.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-search",
    title = "Jump to a schema or record",
    spotlight = "valence-dashboard-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ValenceDashboardSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-search",
        "Type a schema name to open its detail page, or paste a schema plus record id to land on one row.",
        Some("If the token does not match anything, the page stays put — try a shorter name."),
        &[],
    )
}

/// My Data stats card.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-my-data",
    title = "Your data footprint",
    spotlight = "valence-dashboard-my-data",
    position = "bottom",
    order = 30
)]
#[component]
pub fn ValenceDashboardMyDataHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-my-data",
        "These counts summarize what you own in Valence right now.",
        None,
        &[
            "Owned rows: records where you are the owner",
            "Tables with data: schemas that have at least one row",
            "Pending deletion: rows queued for cascade removal",
        ],
    )
}

/// Top schemas by row count.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-top-schemas",
    title = "Your busiest schemas",
    spotlight = "valence-dashboard-top-schemas",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceDashboardTopSchemasHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-top-schemas",
        "This list ranks schemas by how many rows you own. Open a row to inspect fields, privacy, and sample records on the schema page.",
        Some("Use it to spot where most of your data lives."),
        &[],
    )
}

/// Active deletion runs card.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-active-deletions",
    title = "Active deletions",
    spotlight = "valence-dashboard-active-deletions",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceDashboardActiveDeletionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-active-deletions",
        "When someone queues a cascade delete, the run appears here while it is still working.",
        Some("Click a row to watch step-by-step progress or cancel an in-flight run."),
        &[],
    )
}

/// Platform metrics time range and legend toolbar.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-platform-toolbar",
    title = "Platform metrics window",
    spotlight = "valence-dashboard-platform-toolbar",
    position = "bottom",
    order = 60
)]
#[component]
pub fn ValenceDashboardPlatformToolbarHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-platform-toolbar",
        "Switch the chart window to see platform traffic over 15 minutes, one hour, or 24 hours.",
        None,
        &[
            "Table: operations against a single schema",
            "Database: operations across the whole Valence store",
        ],
    )
}

/// Platform headline KPI card.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-headline",
    title = "Platform at a glance",
    spotlight = "valence-dashboard-headline",
    position = "top",
    order = 70
)]
#[component]
pub fn ValenceDashboardHeadlineHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-headline",
        "The headline card summarizes overall platform health for the time window you picked.",
        Some("Come back here when you want a quick pulse before drilling into charts."),
        &[],
    )
}

/// Throughput chart.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-throughput",
    title = "Throughput",
    spotlight = "valence-dashboard-throughput",
    position = "top",
    order = 80
)]
#[component]
pub fn ValenceDashboardThroughputHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-throughput",
        "Throughput shows how many operations finished per bucket — reads and writes combined.",
        Some("A sudden drop may mean fewer traffic or a backend slowdown; compare with the writes and reads cards."),
        &[],
    )
}

/// Writes chart.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-writes",
    title = "Writes",
    spotlight = "valence-dashboard-writes",
    position = "top",
    order = 90
)]
#[component]
pub fn ValenceDashboardWritesHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-writes",
        "Writes track create, update, and delete operations against Valence tables.",
        Some("Spikes often follow batch jobs or user activity bursts."),
        &[],
    )
}

/// Reads chart.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-reads",
    title = "Reads",
    spotlight = "valence-dashboard-reads",
    position = "top",
    order = 100
)]
#[component]
pub fn ValenceDashboardReadsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-reads",
        "Reads track lookups and list queries. Healthy apps usually read more often than they write.",
        None,
        &[],
    )
}

/// Errors chart.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-dashboard-errors",
    title = "Errors",
    spotlight = "valence-dashboard-errors",
    position = "top",
    order = 110
)]
#[component]
pub fn ValenceDashboardErrorsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-dashboard-errors",
        "Errors count failed Valence operations in the selected window.",
        Some("A flat line at zero is good; sustained bumps deserve a look at recent deploys or bad queries."),
        &[],
    )
}

/// Left navigation destinations.
#[help_spotlight_step(
    route = "/valence",
    feature_highlight = "valence-nav",
    title = "Finding your way",
    spotlight = "valence-nav",
    position = "right",
    order = 120
)]
#[component]
pub fn ValenceNavHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-nav",
        "Use the left menu to open Dashboard for a health overview, Schemas for the catalog, Traits for shared field bundles, Iters for batch jobs, and Deletions for cascade runs.",
        Some("Help → Replay restarts this page's tour."),
        &[],
    )
}

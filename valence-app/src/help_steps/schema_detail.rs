//! Spotlight steps for schema detail (`/valence/schema/:schema_name`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Schema detail page intro.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-detail-intro",
    title = "This schema",
    spotlight = "valence-schema-top-bar",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceSchemaDetailIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-detail-intro",
        "You are looking at one schema — one shelf in the catalog. The top bar shows its name and links back to the full list.",
        None,
        &[],
    )
}

/// Overview card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-overview",
    title = "Overview",
    spotlight = "valence-schema-overview",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ValenceSchemaOverviewHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-overview",
        "Overview summarizes what this schema stores: description, row count, and metadata you need before opening records.",
        None,
        &[],
    )
}

/// Sample records card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-samples",
    title = "Sample records",
    spotlight = "valence-schema-samples",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceSchemaSamplesHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-samples",
        "Sample records show a few recent rows so you can see real field values without hunting for an id.",
        Some("Click a sample row to open that record's detail page."),
        &[],
    )
}

/// Open latest record action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-open-latest",
    title = "Open latest record",
    spotlight = "valence-schema-open-latest",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceSchemaOpenLatestHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-open-latest",
        "Open latest jumps straight to the most recently touched record in this schema — handy when you know data exists but not which id.",
        None,
        &[],
    )
}

/// Export action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-export",
    title = "Export",
    spotlight = "valence-schema-export",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceSchemaExportHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-export",
        "Export downloads schema metadata and sample rows for offline review or sharing with your team.",
        None,
        &[],
    )
}

/// Privacy policies card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-privacy",
    title = "Privacy policies",
    spotlight = "valence-schema-privacy",
    position = "top",
    order = 60
)]
#[component]
pub fn ValenceSchemaPrivacyHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-privacy",
        "Privacy policies describe who may read or write each field on records in this schema.",
        Some("Open a record later to see how policies evaluate for a specific row and actor."),
        &[],
    )
}

/// Fields table.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-fields",
    title = "Fields",
    spotlight = "valence-schema-fields",
    position = "top",
    order = 70
)]
#[component]
pub fn ValenceSchemaFieldsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-fields",
        "The fields table lists every column: type, optional/required, and links to related schemas.",
        None,
        &[
            "Primary key: the id column for each record",
            "Foreign key: a pointer to a row in another schema",
        ],
    )
}

/// Connections card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-connections",
    title = "Connections",
    spotlight = "valence-schema-connections",
    position = "top",
    order = 80
)]
#[component]
pub fn ValenceSchemaConnectionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-connections",
        "Connections map how this schema links to others — incoming and outgoing foreign keys drawn as a graph.",
        Some("Use it to understand cascade deletes and join paths."),
        &[],
    )
}

/// Traits card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-traits",
    title = "Traits",
    spotlight = "valence-schema-traits",
    position = "top",
    order = 90
)]
#[component]
pub fn ValenceSchemaTraitsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-traits",
        "Traits are reusable field bundles this schema implements — like mix-ins shared across tables.",
        Some("Open the Traits catalog from the left menu to inspect a trait's fields in isolation."),
        &[],
    )
}

/// Start iter action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-iter-start",
    title = "Start an iter",
    spotlight = "valence-schema-iter-start",
    position = "top",
    order = 100
)]
#[component]
pub fn ValenceSchemaIterStartHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-iter-start",
        "An iter walks many records in batch — like a librarian checking every book on a shelf. Start one here when you need to reprocess or migrate rows.",
        None,
        &[],
    )
}

/// Iter run history.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-iter-runs",
    title = "Iter run history",
    spotlight = "valence-schema-iter-runs",
    position = "top",
    order = 110
)]
#[component]
pub fn ValenceSchemaIterRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-iter-runs",
        "Past and in-flight iter runs for this schema appear here. Open a run to see progress, errors, and batches.",
        None,
        &[],
    )
}

/// Deletion runs card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name",
    feature_highlight = "valence-schema-deletions",
    title = "Deletion runs",
    spotlight = "valence-schema-deletions",
    position = "top",
    order = 120
)]
#[component]
pub fn ValenceSchemaDeletionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-schema-deletions",
        "Deletion runs track cascade removes for records in this schema. Each run lists steps as dependent rows are cleaned up.",
        Some("Open a run to watch progress or cancel work that is still going."),
        &[],
    )
}

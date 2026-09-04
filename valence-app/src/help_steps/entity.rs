//! Spotlight steps for entity detail (`/valence/schema/:schema_name/id/:entity_id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Entity page intro.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-intro",
    title = "This record",
    spotlight = "valence-entity-top-bar",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceEntityIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-intro",
        "You are looking at one record — one row on the schema shelf. The top bar shows schema name, record id, and a link back.",
        None,
        &[],
    )
}

/// Field values card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-fields",
    title = "Field values",
    spotlight = "valence-entity-fields",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceEntityFieldsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-fields",
        "Every column on this record appears here with its current value. Read-only fields still show so you know what the row stores.",
        None,
        &[],
    )
}

/// Connections card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-connections",
    title = "Connections",
    spotlight = "valence-entity-connections",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceEntityConnectionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-connections",
        "Connections list foreign keys on this record — links to rows in other schemas. Click a link to open the related record.",
        None,
        &[],
    )
}

/// Owner card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-owner",
    title = "Owner",
    spotlight = "valence-entity-owner",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceEntityOwnerHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-owner",
        "Owner shows who controls this record today and any recent ownership transfers.",
        Some("Privacy policies often key off the owner when deciding read and write access."),
        &[],
    )
}

/// Privacy evaluation card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-privacy",
    title = "Privacy evaluation",
    spotlight = "valence-entity-privacy",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceEntityPrivacyHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-privacy",
        "Privacy evaluation shows how policies apply to this record for the signed-in actor — which fields are visible or writable right now.",
        Some("Use it when debugging access denials without guessing from policy text alone."),
        &[],
    )
}

/// Iter on this record.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-iter-run",
    title = "Iter on this record",
    spotlight = "valence-entity-iter-run",
    position = "top",
    order = 60
)]
#[component]
pub fn ValenceEntityIterRunHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-iter-run",
        "Run an iter against just this record when you need to reprocess one row without scanning the whole schema.",
        Some("Recent iter runs tied to this record also appear here."),
        &[],
    )
}

/// Deletion status card.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-deletions",
    title = "Deletion status",
    spotlight = "valence-entity-deletions",
    position = "top",
    order = 70
)]
#[component]
pub fn ValenceEntityDeletionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-deletions",
        "When a cascade delete is queued for this record, the active or recent deletion run shows here.",
        Some("Admins can queue a new deletion from quick actions when policy allows."),
        &[],
    )
}

/// Export action.
#[help_spotlight_step(
    route = "/valence/schema/:schema_name/id/:entity_id",
    feature_highlight = "valence-entity-export",
    title = "Export",
    spotlight = "valence-entity-export",
    position = "top",
    order = 80
)]
#[component]
pub fn ValenceEntityExportHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-entity-export",
        "Export downloads this record's field values for offline review or support tickets.",
        None,
        &[],
    )
}

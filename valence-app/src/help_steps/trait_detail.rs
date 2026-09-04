//! Spotlight steps for trait detail (`/valence/traits/:trait_name`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Trait detail page intro.
#[help_spotlight_step(
    route = "/valence/traits/:trait_name",
    feature_highlight = "valence-trait-detail-intro",
    title = "This trait",
    spotlight = "valence-trait-top-bar",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ValenceTraitDetailIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-trait-detail-intro",
        "You are looking at one trait — a reusable field bundle. The top bar shows its name and links back to the catalog.",
        None,
        &[],
    )
}

/// Overview card.
#[help_spotlight_step(
    route = "/valence/traits/:trait_name",
    feature_highlight = "valence-trait-overview",
    title = "Overview",
    spotlight = "valence-trait-overview",
    position = "top",
    order = 20
)]
#[component]
pub fn ValenceTraitOverviewHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-trait-overview",
        "Overview summarizes what this trait adds: description, field count, and how schemas pull it in.",
        None,
        &[],
    )
}

/// Fields table.
#[help_spotlight_step(
    route = "/valence/traits/:trait_name",
    feature_highlight = "valence-trait-fields",
    title = "Fields",
    spotlight = "valence-trait-fields",
    position = "top",
    order = 30
)]
#[component]
pub fn ValenceTraitFieldsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-trait-fields",
        "The fields table lists every column this trait contributes — types and optional flags match what you will see on implementing schemas.",
        None,
        &[],
    )
}

/// Connections card.
#[help_spotlight_step(
    route = "/valence/traits/:trait_name",
    feature_highlight = "valence-trait-connections",
    title = "Connections",
    spotlight = "valence-trait-connections",
    position = "top",
    order = 40
)]
#[component]
pub fn ValenceTraitConnectionsHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-trait-connections",
        "Connections show foreign keys defined on this trait — links trait fields may create to other schemas.",
        None,
        &[],
    )
}

/// Used-by schemas card.
#[help_spotlight_step(
    route = "/valence/traits/:trait_name",
    feature_highlight = "valence-trait-used-by",
    title = "Used by schemas",
    spotlight = "valence-trait-used-by",
    position = "top",
    order = 50
)]
#[component]
pub fn ValenceTraitUsedByHelp() -> impl IntoView {
    help_stack(
        "help-step-valence-trait-used-by",
        "Used by lists every schema that includes this trait. Open a schema row to see how the shared fields appear in context.",
        None,
        &[],
    )
}

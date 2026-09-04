//! Reusable Valence UI building blocks (tables, help copy, field-type display).
//!
//! Pages compose these under [`crate::pages`]. They are crate-private; host integrators
//! mount [`crate::ValenceRoutes`] rather than importing components directly.

pub mod bordered_table;
pub mod code_styles;
pub mod download;
pub mod field_type;
pub mod help;
pub mod owner_identity;
pub mod valence_card;

pub use bordered_table::bordered_table_styles;
pub use code_styles::code_style_classes;
pub use download::download_text_file;
pub use field_type::{FieldTypeDisplay, FieldTypeTypography};
pub use help::{
    cardinality_badge_tooltip, fk_badge_tooltip, ref_badge_tooltip, BadgeHint, ConnectionRowLayout,
    InverseSchemaConnectionRow, OnDeleteHint, SchemaConnectionRow, TruncatedRecordLink,
    ValenceHelpCardHeader, ValenceHelpColumnHeader,
};
pub use owner_identity::{ValenceOwnerIdentity, ValenceOwnerTransferHistory};
pub use valence_card::ValenceCard;

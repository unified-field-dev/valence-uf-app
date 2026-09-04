//! Blank-id rejection, unsafe-id rejection, and path-segment encoding for ops
//! UI hrefs.

/// Blank, oversized, or path-unsafe schema/trait/iter name, run id, or entity id
/// rejected before lookups.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValenceIdError {
    /// Schema name was empty or whitespace-only.
    EmptySchemaName,
    /// Run id was empty or whitespace-only.
    EmptyRunId,
    /// Entity id was empty or whitespace-only.
    EmptyEntityId,
    /// Iter name was empty or whitespace-only.
    EmptyIterName,
    /// Trait name was empty or whitespace-only.
    EmptyTraitName,
    /// Schema name exceeded [`MAX_VALENCE_ID_CHARS`].
    SchemaNameTooLong,
    /// Run id exceeded [`MAX_VALENCE_ID_CHARS`].
    RunIdTooLong,
    /// Entity id exceeded [`MAX_VALENCE_ID_CHARS`].
    EntityIdTooLong,
    /// Iter name exceeded [`MAX_VALENCE_ID_CHARS`].
    IterNameTooLong,
    /// Trait name exceeded [`MAX_VALENCE_ID_CHARS`].
    TraitNameTooLong,
    /// Schema name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeSchemaName,
    /// Run id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeRunId,
    /// Entity id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeEntityId,
    /// Iter name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeIterName,
    /// Trait name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeTraitName,
    /// Record display string had no extractable id after `table:` normalization.
    InvalidRecordIdDisplay,
}

impl std::fmt::Display for ValenceIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySchemaName => write!(f, "Schema name is required"),
            Self::EmptyRunId => write!(f, "Run id is required"),
            Self::EmptyEntityId => write!(f, "Entity id is required"),
            Self::EmptyIterName => write!(f, "Iter name is required"),
            Self::EmptyTraitName => write!(f, "Trait name is required"),
            Self::SchemaNameTooLong => write!(f, "Schema name is too long"),
            Self::RunIdTooLong => write!(f, "Run id is too long"),
            Self::EntityIdTooLong => write!(f, "Entity id is too long"),
            Self::IterNameTooLong => write!(f, "Iter name is too long"),
            Self::TraitNameTooLong => write!(f, "Trait name is too long"),
            Self::UnsafeSchemaName => {
                write!(f, "Schema name contains unsafe path characters")
            }
            Self::UnsafeRunId => {
                write!(f, "Run id contains unsafe path characters")
            }
            Self::UnsafeEntityId => {
                write!(f, "Entity id contains unsafe path characters")
            }
            Self::UnsafeIterName => {
                write!(f, "Iter name contains unsafe path characters")
            }
            Self::UnsafeTraitName => {
                write!(f, "Trait name contains unsafe path characters")
            }
            Self::InvalidRecordIdDisplay => {
                write!(f, "Record id string has no extractable id")
            }
        }
    }
}

impl std::error::Error for ValenceIdError {}

/// Maximum Unicode scalar count for schema/trait/iter names, run ids, and entity
/// ids accepted by ops detail lookups.
pub const MAX_VALENCE_ID_CHARS: usize = 256;

const fn is_unsafe_ops_id_char(c: char) -> bool {
    // Avoid `char::is_control` / `RangeInclusive::contains` — not const-stable on CI nightlies.
    let u = c as u32;
    let is_control = u < 0x20 || u == 0x7f || (u >= 0x80 && u <= 0x9f);
    is_control || c == '/' || c == '\\'
}

fn check_ops_id(raw: &str) -> Result<&str, ValenceIdErrorKind> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ValenceIdErrorKind::Empty);
    }
    if trimmed.chars().count() > MAX_VALENCE_ID_CHARS {
        return Err(ValenceIdErrorKind::TooLong);
    }
    if trimmed == "." || trimmed == ".." {
        return Err(ValenceIdErrorKind::Unsafe);
    }
    if trimmed.chars().any(is_unsafe_ops_id_char) {
        return Err(ValenceIdErrorKind::Unsafe);
    }
    Ok(trimmed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValenceIdErrorKind {
    Empty,
    TooLong,
    Unsafe,
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` schema names
/// before schema / iter / deletion lookups.
///
/// # Errors
///
/// Returns a [`ValenceIdError`] variant when the name is empty/whitespace-only,
/// longer than [`MAX_VALENCE_ID_CHARS`], contains `/` `\` or ASCII controls, or is
/// exactly `.` / `..`.
pub fn validate_schema_name(schema_name: &str) -> Result<(), ValenceIdError> {
    match check_ops_id(schema_name) {
        Ok(_) => Ok(()),
        Err(ValenceIdErrorKind::Empty) => Err(ValenceIdError::EmptySchemaName),
        Err(ValenceIdErrorKind::TooLong) => Err(ValenceIdError::SchemaNameTooLong),
        Err(ValenceIdErrorKind::Unsafe) => Err(ValenceIdError::UnsafeSchemaName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` run ids
/// before iter / deletion run detail lookups.
///
/// # Errors
///
/// Returns a [`ValenceIdError`] variant when the id fails the same rules as
/// [`validate_schema_name`].
pub fn validate_run_id(run_id: &str) -> Result<(), ValenceIdError> {
    match check_ops_id(run_id) {
        Ok(_) => Ok(()),
        Err(ValenceIdErrorKind::Empty) => Err(ValenceIdError::EmptyRunId),
        Err(ValenceIdErrorKind::TooLong) => Err(ValenceIdError::RunIdTooLong),
        Err(ValenceIdErrorKind::Unsafe) => Err(ValenceIdError::UnsafeRunId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` entity ids
/// before entity / iter-on-entity / delete-queue lookups.
///
/// # Errors
///
/// Returns a [`ValenceIdError`] variant when the id fails the same rules as
/// [`validate_schema_name`].
pub fn validate_entity_id(entity_id: &str) -> Result<(), ValenceIdError> {
    match check_ops_id(entity_id) {
        Ok(_) => Ok(()),
        Err(ValenceIdErrorKind::Empty) => Err(ValenceIdError::EmptyEntityId),
        Err(ValenceIdErrorKind::TooLong) => Err(ValenceIdError::EntityIdTooLong),
        Err(ValenceIdErrorKind::Unsafe) => Err(ValenceIdError::UnsafeEntityId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` iter names
/// before start / run-on-entity lookups.
///
/// # Errors
///
/// Returns a [`ValenceIdError`] variant when the name fails the same rules as
/// [`validate_schema_name`].
pub fn validate_iter_name(iter_name: &str) -> Result<(), ValenceIdError> {
    match check_ops_id(iter_name) {
        Ok(_) => Ok(()),
        Err(ValenceIdErrorKind::Empty) => Err(ValenceIdError::EmptyIterName),
        Err(ValenceIdErrorKind::TooLong) => Err(ValenceIdError::IterNameTooLong),
        Err(ValenceIdErrorKind::Unsafe) => Err(ValenceIdError::UnsafeIterName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` trait names
/// before trait detail lookups.
///
/// # Errors
///
/// Returns a [`ValenceIdError`] variant when the name fails the same rules as
/// [`validate_schema_name`].
pub fn validate_trait_name(trait_name: &str) -> Result<(), ValenceIdError> {
    match check_ops_id(trait_name) {
        Ok(_) => Ok(()),
        Err(ValenceIdErrorKind::Empty) => Err(ValenceIdError::EmptyTraitName),
        Err(ValenceIdErrorKind::TooLong) => Err(ValenceIdError::TraitNameTooLong),
        Err(ValenceIdErrorKind::Unsafe) => Err(ValenceIdError::UnsafeTraitName),
    }
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encode a single path segment for `/valence/...` hrefs.
///
/// Leaves RFC 3986 unreserved characters alone (`ALPHA` / `DIGIT` / `-` `.` `_`
/// `~`). Encodes `/`, `\`, controls, spaces, and other bytes so Orbital
/// `paths::*` format strings cannot smuggle extra path segments.
#[must_use]
pub fn encode_ops_path_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for &b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

/// `/valence/schema/{encoded}` detail href.
#[must_use]
pub fn valence_schema_path(schema_name: &str) -> String {
    format!("/valence/schema/{}", encode_ops_path_segment(schema_name))
}

/// `/valence/schema/{encoded}/id/{encoded}` entity detail href.
#[must_use]
pub fn valence_entity_path(schema_name: &str, entity_id: &str) -> String {
    format!(
        "/valence/schema/{}/id/{}",
        encode_ops_path_segment(schema_name),
        encode_ops_path_segment(entity_id)
    )
}

/// `/valence/schema/{encoded}/iter/{encoded}` iter-run detail href.
#[must_use]
pub fn valence_iter_run_path(schema_name: &str, run_id: &str) -> String {
    format!(
        "/valence/schema/{}/iter/{}",
        encode_ops_path_segment(schema_name),
        encode_ops_path_segment(run_id)
    )
}

/// `/valence/schema/{encoded}/deletion/{encoded}` deletion-run detail href.
#[must_use]
pub fn valence_deletion_run_path(schema_name: &str, run_id: &str) -> String {
    format!(
        "/valence/schema/{}/deletion/{}",
        encode_ops_path_segment(schema_name),
        encode_ops_path_segment(run_id)
    )
}

/// `/valence/traits/{encoded}` trait detail href.
#[must_use]
pub fn valence_trait_path(trait_name: &str) -> String {
    format!("/valence/traits/{}", encode_ops_path_segment(trait_name))
}

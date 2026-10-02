//! Row builders for the declared data-use server functions.
//!
//! Pure over catalog entries and registry lookups so every surface (schema, trait,
//! Unscoped) is unit-testable without a Leptos request.

use std::sync::atomic::{AtomicBool, Ordering};

use leptos::prelude::ServerFnError;
use valence::data_use::{ConnectionHopKind, DataUse, DataUseCatalog, DataUseTarget};

use super::data_uses::DATA_USE_CATALOG_NOT_INSTALLED;
use super::types::DataUseRow;

static NOT_INSTALLED_LOGGED: AtomicBool = AtomicBool::new(false);

/// Entries of the installed catalog, or the stable not-installed error.
///
/// A missing catalog is a host wiring defect. Reporting it as zero rows would tell
/// end users the deployment declares no uses, which is false.
pub(crate) fn installed_entries<'a>(
    catalog: Option<&'a DataUseCatalog>,
    operation: &'static str,
) -> Result<&'a [DataUse], ServerFnError> {
    if let Some(catalog) = catalog {
        return Ok(catalog.entries());
    }
    if !NOT_INSTALLED_LOGGED.swap(true, Ordering::Relaxed) {
        tracing::error!(
            target: "valence.data_use.catalog",
            operation,
            "data-use catalog not installed"
        );
    }
    Err(ServerFnError::new(format!(
        "{DATA_USE_CATALOG_NOT_INSTALLED}: this host never called \
         valence::data_use::DataUseCatalog::install at boot"
    )))
}

/// Rows for a schema page: Schema(S), Trait(T) where S implements T, and Referenced
/// Reads / Updates whose peer resolves to S.
pub(crate) fn schema_rows(
    entries: &[DataUse],
    schema: &str,
    schema_repo: &str,
    traits_for_schema: &[String],
    trait_repo: impl Fn(&str) -> Option<String>,
    resolve_peer: impl Fn(&str, &str) -> Option<String>,
) -> Vec<DataUseRow> {
    let mut rows = Vec::new();
    for entry in entries {
        match &entry.target {
            DataUseTarget::Schema(name) if name == schema => {
                rows.push(to_row(entry, None, None, schema_repo));
            }
            DataUseTarget::Trait(trait_name) if traits_for_schema.contains(trait_name) => {
                let repo = trait_repo(trait_name).unwrap_or_else(|| schema_repo.to_string());
                rows.push(to_row(entry, Some(trait_name.clone()), None, &repo));
            }
            DataUseTarget::Schema(from_table) => {
                if let Some(row) = referenced_row(entry, from_table, schema, &resolve_peer) {
                    rows.push(row);
                }
            }
            DataUseTarget::Trait(_) | DataUseTarget::Unscoped => {}
        }
    }
    rows
}

/// Rows for a trait page: Trait(T) targets only.
pub(crate) fn trait_rows(
    entries: &[DataUse],
    trait_name: &str,
    trait_repo: &str,
) -> Vec<DataUseRow> {
    entries
        .iter()
        .filter(|e| matches!(&e.target, DataUseTarget::Trait(name) if name == trait_name))
        .map(|e| to_row(e, None, None, trait_repo))
        .collect()
}

/// Rows for `/valence/unscoped-uses`.
pub(crate) fn unscoped_rows(entries: &[DataUse]) -> Vec<DataUseRow> {
    entries
        .iter()
        .filter(|e| matches!(e.target, DataUseTarget::Unscoped))
        .map(|e| to_row(e, None, None, ""))
        .collect()
}

/// `file` is relative to the declaring crate's repository, so that repository wins;
/// the schema or trait repository is only a fallback for rows scanned without one.
fn source_repo<'a>(entry: &'a DataUse, fallback: &'a str) -> &'a str {
    if entry.repository.trim().is_empty() {
        fallback
    } else {
        &entry.repository
    }
}

fn github_blob_url(repository: &str, path: &str, line: u32) -> String {
    if repository.trim().is_empty() {
        return String::new();
    }
    let repo = repository.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{repo}/blob/main/{path}#L{line}")
}

fn to_row(
    entry: &DataUse,
    via_trait: Option<String>,
    source_schema: Option<String>,
    fallback_repo: &str,
) -> DataUseRow {
    DataUseRow {
        purpose: entry.purpose.clone(),
        file: entry.file.clone(),
        line: entry.line,
        crate_name: entry.crate_name.clone(),
        op: entry.op.as_str().to_string(),
        method: entry.method.clone(),
        via_trait,
        source_schema,
        source_url: github_blob_url(source_repo(entry, fallback_repo), &entry.file, entry.line),
    }
}

fn referenced_row(
    entry: &DataUse,
    from_table: &str,
    peer_schema: &str,
    resolve_peer: &impl Fn(&str, &str) -> Option<String>,
) -> Option<DataUseRow> {
    let hop = entry.connection.as_ref()?;
    let resolved = entry
        .referenced_schema
        .clone()
        .or_else(|| resolve_peer(from_table, &hop.field))?;
    if resolved != peer_schema {
        return None;
    }
    let op = match hop.kind {
        ConnectionHopKind::ForwardGet => "referenced_read",
        ConnectionHopKind::Relate => "referenced_update",
    };
    let mut row = to_row(entry, None, Some(from_table.to_string()), "");
    row.op = op.to_string();
    Some(row)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use valence::data_use::{ConnectionHop, DataOp};

    use super::*;

    fn entry(target: DataUseTarget, op: DataOp, method: &str, purpose: &str) -> DataUse {
        DataUse {
            purpose: purpose.into(),
            file: "worker/src/lib.rs".into(),
            line: 7,
            crate_name: "worker".into(),
            repository: "https://github.com/acme/worker".into(),
            target,
            op,
            method: method.into(),
            connection: None,
            referenced_schema: None,
            via_trait: None,
        }
    }

    fn hop(mut e: DataUse, field: &str, kind: ConnectionHopKind, peer: Option<&str>) -> DataUse {
        e.connection = Some(ConnectionHop {
            field: field.into(),
            kind,
        });
        e.referenced_schema = peer.map(str::to_string);
        e
    }

    fn no_trait_repo(_: &str) -> Option<String> {
        None
    }

    fn no_peer(_: &str, _: &str) -> Option<String> {
        None
    }

    #[test]
    fn rows_none_catalog_is_not_installed_error() {
        let err = installed_entries(None, "schema").unwrap_err();
        assert!(err.to_string().contains(DATA_USE_CATALOG_NOT_INSTALLED));

        let catalog = DataUseCatalog::from_entries(vec![]);
        assert_eq!(
            installed_entries(Some(&catalog), "schema").unwrap().len(),
            0
        );
    }

    #[test]
    fn schema_rows_include_trait_fanout() {
        let entries = vec![
            entry(
                DataUseTarget::Schema("counter".into()),
                DataOp::Update,
                "update",
                "tick",
            ),
            entry(
                DataUseTarget::Trait("HasOwner".into()),
                DataOp::Read,
                "query",
                "owners",
            ),
            entry(
                DataUseTarget::Trait("Other".into()),
                DataOp::Read,
                "query",
                "other",
            ),
            entry(DataUseTarget::Unscoped, DataOp::Read, "execute", "walk"),
        ];
        let rows = schema_rows(
            &entries,
            "counter",
            "https://github.com/acme/counter",
            &["HasOwner".to_string()],
            no_trait_repo,
            no_peer,
        );
        let purposes: Vec<&str> = rows.iter().map(|r| r.purpose.as_str()).collect();
        assert_eq!(purposes, vec!["tick", "owners"]);
        assert_eq!(rows[0].op, "update");
        assert_eq!(rows[1].via_trait.as_deref(), Some("HasOwner"));
        assert_eq!(
            rows[0].source_url, "https://github.com/acme/worker/blob/main/worker/src/lib.rs#L7",
            "declaring repo wins over the schema repo"
        );
    }

    #[test]
    fn schema_rows_fall_back_to_schema_repo_without_entry_repo() {
        let mut e = entry(
            DataUseTarget::Schema("counter".into()),
            DataOp::Read,
            "get",
            "read",
        );
        e.repository = String::new();
        let rows = schema_rows(
            &[e],
            "counter",
            "https://github.com/acme/counter/",
            &[],
            no_trait_repo,
            no_peer,
        );
        assert_eq!(
            rows[0].source_url,
            "https://github.com/acme/counter/blob/main/worker/src/lib.rs#L7"
        );
    }

    #[test]
    fn schema_rows_include_referenced_peer() {
        let baked = hop(
            entry(
                DataUseTarget::Schema("todo".into()),
                DataOp::Read,
                "get_owner",
                "baked",
            ),
            "owner",
            ConnectionHopKind::ForwardGet,
            Some("user"),
        );
        let resolved = hop(
            entry(
                DataUseTarget::Schema("todo".into()),
                DataOp::Update,
                "relate_to_tag",
                "relate",
            ),
            "tag",
            ConnectionHopKind::Relate,
            None,
        );
        let elsewhere = hop(
            entry(
                DataUseTarget::Schema("project".into()),
                DataOp::Read,
                "get_tasks",
                "tasks",
            ),
            "tasks",
            ConnectionHopKind::ForwardGet,
            Some("task"),
        );
        let entries = vec![baked, resolved, elsewhere];

        let user = schema_rows(&entries, "user", "", &[], no_trait_repo, no_peer);
        assert_eq!(user.len(), 1);
        assert_eq!(user[0].op, "referenced_read");
        assert_eq!(user[0].source_schema.as_deref(), Some("todo"));

        let tag = schema_rows(&entries, "tag", "", &[], no_trait_repo, |from, field| {
            (from == "todo" && field == "tag").then(|| "tag".to_string())
        });
        assert_eq!(tag.len(), 1);
        assert_eq!(tag[0].op, "referenced_update");
        assert!(tag[0]
            .source_url
            .starts_with("https://github.com/acme/worker/blob/main/"));
    }

    #[test]
    fn trait_rows_only_trait_targets() {
        let entries = vec![
            entry(
                DataUseTarget::Trait("HasOwner".into()),
                DataOp::Read,
                "query",
                "owners",
            ),
            entry(
                DataUseTarget::Schema("HasOwner".into()),
                DataOp::Read,
                "get",
                "schema",
            ),
            entry(
                DataUseTarget::Trait("Other".into()),
                DataOp::Read,
                "query",
                "other",
            ),
        ];
        let rows = trait_rows(&entries, "HasOwner", "https://github.com/acme/traits");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].purpose, "owners");
        assert!(rows[0].via_trait.is_none());
    }

    #[test]
    fn unscoped_rows_use_entry_repository() {
        let entries = vec![
            entry(DataUseTarget::Unscoped, DataOp::Read, "execute", "walk"),
            entry(
                DataUseTarget::Schema("counter".into()),
                DataOp::Read,
                "get",
                "read",
            ),
        ];
        let rows = unscoped_rows(&entries);
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].source_url,
            "https://github.com/acme/worker/blob/main/worker/src/lib.rs#L7"
        );
    }
}

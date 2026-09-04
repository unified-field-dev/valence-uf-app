//! Valence dashboard server functions (My Data + platform metrics cards).

use chrono::{DateTime, Duration, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[cfg(feature = "ssr")]
use spectra_core::{
    aggregate_request_to_filter, metrics_query_to_range, EventAggregateRequest,
    EventAggregationSpec, EventExploreView, EventMeasure, GridFilterModel, MetricPoint,
    MetricsQuery,
};
#[cfg(feature = "ssr")]
use valence::actor::Actor;
#[cfg(feature = "ssr")]
use valence::deletion::DeletionService;
#[cfg(feature = "ssr")]
use valence::owner_ref::OwnerKind;
#[cfg(feature = "ssr")]
use valence::ownership::{OwnerDataSummary, OwnershipService};

// ---------------------------------------------------------------------------
// DTOs (SSR + WASM)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardMyDataStats {
    pub viewer_is_user: bool,
    pub owned_rows: u64,
    pub tables_with_data: u64,
    pub pending_deletion_rows: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardSchemaRowCount {
    pub valence_model: String,
    pub active_rows: u64,
    pub pending_deletion_rows: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardStatCard {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardChartPoint {
    pub ts: DateTime<Utc>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardChartSeries {
    pub id: String,
    pub label: String,
    pub points: Vec<DashboardChartPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardPlatformHeadline {
    pub cards: Vec<DashboardStatCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DashboardErrorSlice {
    pub label: String,
    pub value: f64,
}

// ---------------------------------------------------------------------------
// SSR helpers
// ---------------------------------------------------------------------------

#[cfg(feature = "ssr")]
fn range_from_secs(secs: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let end = Utc::now();
    let start = end - Duration::seconds(secs);
    (start, end)
}

#[cfg(feature = "ssr")]
async fn viewer_valence() -> Result<valence::Valence, ServerFnError> {
    super::helpers::viewer_valence().await
}

#[cfg(feature = "ssr")]
async fn fetch_owner_data_summary(v: &valence::Valence) -> Result<OwnerDataSummary, ServerFnError> {
    let actor = v.actor();
    let Actor::User { user_id } = actor else {
        return Ok(OwnerDataSummary::default());
    };
    OwnershipService::owner_data_summary(user_id, OwnerKind::User.as_str(), v)
        .await
        .map_err(|e| super::helpers::io_error(e.to_string()))
}

#[cfg(feature = "ssr")]
async fn require_spectra_query(table: &str) -> Result<(), ServerFnError> {
    let table = table.trim();
    if table.is_empty() {
        return Err(super::helpers::validation_message(
            "Spectra query table name is required",
        ));
    }

    let ctx = higgs::Higgs::from_request().await.map_err(|e| {
        super::helpers::auth_error(format!("Failed to resolve request context: {e}"))
    })?;

    if ctx.actor().is_user() {
        Ok(())
    } else {
        let permission_name = format!("spectra.query.{table}");
        Err(super::helpers::permission_error(format!(
            "`{permission_name}` is required to query this table"
        )))
    }
}

#[cfg(feature = "ssr")]
async fn query_metric_points(
    metric: &str,
    range_secs: i64,
) -> Result<Vec<MetricPoint>, ServerFnError> {
    let _ = (metric, range_secs);
    // Standalone uf-app: Spectra host wiring is composed by deployment shells (Wave 7b).
    Ok(Vec::new())
}

#[cfg(feature = "ssr")]
fn label_value(labels: &serde_json::Value, key: &str) -> String {
    labels
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(feature = "ssr")]
fn bucket_counter_rates(points: &[MetricPoint], bucket_secs: i64) -> Vec<DashboardChartPoint> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut sorted: Vec<&MetricPoint> = points.iter().collect();
    sorted.sort_by_key(|p| p.ts);

    let bucket_secs = bucket_secs.max(1);
    let mut buckets: BTreeMap<i64, f64> = BTreeMap::new();
    let mut prev: Option<&MetricPoint> = None;
    for p in sorted {
        if let Some(prev) = prev {
            let delta = (p.value - prev.value).max(0.0);
            let secs = (p.ts - prev.ts).num_seconds().max(1);
            let rate = delta / secs as f64;
            let key = p.ts.timestamp() / bucket_secs;
            *buckets.entry(key).or_default() += rate;
        }
        prev = Some(p);
    }

    buckets
        .into_iter()
        .map(|(key, rate)| DashboardChartPoint {
            ts: DateTime::from_timestamp(key * bucket_secs, 0).unwrap_or_else(Utc::now),
            value: rate,
        })
        .collect()
}

#[cfg(feature = "ssr")]
fn group_points_by_label(
    points: &[MetricPoint],
    label_key: &str,
    top_n: usize,
    bucket_secs: i64,
) -> Vec<DashboardChartSeries> {
    let mut groups: BTreeMap<String, Vec<MetricPoint>> = BTreeMap::new();
    for p in points {
        groups
            .entry(label_value(&p.labels, label_key))
            .or_default()
            .push(p.clone());
    }

    let mut ranked: Vec<(String, Vec<MetricPoint>, f64)> = groups
        .into_iter()
        .map(|(label, pts)| {
            let total = pts.iter().map(|p| p.value).sum::<f64>();
            (label, pts, total)
        })
        .collect();
    ranked.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut series = Vec::new();
    let mut other_points: Vec<MetricPoint> = Vec::new();

    for (idx, (label, pts, _)) in ranked.into_iter().enumerate() {
        if idx < top_n {
            series.push(DashboardChartSeries {
                id: label.clone(),
                label: label.clone(),
                points: bucket_counter_rates(&pts, bucket_secs),
            });
        } else {
            other_points.extend(pts);
        }
    }

    if !other_points.is_empty() {
        series.push(DashboardChartSeries {
            id: "other".into(),
            label: "Other".into(),
            points: bucket_counter_rates(&other_points, bucket_secs),
        });
    }

    series
}

#[cfg(feature = "ssr")]
fn headline_from_points(label: &str, points: &[MetricPoint]) -> DashboardStatCard {
    if points.is_empty() {
        return DashboardStatCard {
            label: label.to_string(),
            value: "—".to_string(),
        };
    }
    let max = points
        .iter()
        .map(|p| p.value)
        .fold(f64::NEG_INFINITY, f64::max);
    let last = points.last().map(|p| p.value).unwrap_or(0.0);
    DashboardStatCard {
        label: label.to_string(),
        value: format!("last {last:.1} · max {max:.1}"),
    }
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

#[uf_product_macros::server]
pub async fn get_dashboard_my_data_stats() -> Result<DashboardMyDataStats, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let v = viewer_valence().await?;
        let actor = v.actor();
        let Actor::User { .. } = actor else {
            return Ok(DashboardMyDataStats {
                viewer_is_user: false,
                owned_rows: 0,
                tables_with_data: 0,
                pending_deletion_rows: 0,
            });
        };
        let summary = fetch_owner_data_summary(&v).await?;
        Ok(DashboardMyDataStats {
            viewer_is_user: true,
            owned_rows: summary.owned_rows,
            tables_with_data: summary.tables_with_data,
            pending_deletion_rows: summary.pending_deletion_rows,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_my_data_top_schemas(
) -> Result<Vec<DashboardSchemaRowCount>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let v = viewer_valence().await?;
        let actor = v.actor();
        if !matches!(actor, Actor::User { .. }) {
            return Ok(Vec::new());
        }
        let summary = fetch_owner_data_summary(&v).await?;
        Ok(summary
            .rows_by_schema
            .into_iter()
            .take(5)
            .map(|r| DashboardSchemaRowCount {
                valence_model: r.valence_model,
                active_rows: r.active_rows,
                pending_deletion_rows: r.pending_deletion_rows,
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_active_deletions() -> Result<u64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let v = viewer_valence().await?;
        let requested_by = serde_json::to_string(v.actor())
            .map_err(|e| super::helpers::io_error(e.to_string()))?;
        DeletionService::count_active_runs_for_requester(&requested_by, &v)
            .await
            .map_err(|e| super::helpers::io_error(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_platform_headline(
    range_secs: i64,
) -> Result<DashboardPlatformHeadline, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let reads = query_metric_points("valence_db_reads", range_secs).await;
        let writes = query_metric_points("valence_db_writes", range_secs).await;
        let errors = query_metric_points("valence_db_errors", range_secs).await;
        let wall = query_metric_points("valence_db_wall_ms", range_secs).await;

        Ok(DashboardPlatformHeadline {
            cards: vec![
                reads
                    .map(|p| headline_from_points("Reads", &p))
                    .unwrap_or_else(|_| DashboardStatCard {
                        label: "Reads".into(),
                        value: "—".into(),
                    }),
                writes
                    .map(|p| headline_from_points("Writes", &p))
                    .unwrap_or_else(|_| DashboardStatCard {
                        label: "Writes".into(),
                        value: "—".into(),
                    }),
                errors
                    .map(|p| headline_from_points("Errors", &p))
                    .unwrap_or_else(|_| DashboardStatCard {
                        label: "Errors".into(),
                        value: "—".into(),
                    }),
                wall.map(|p| headline_from_points("Wall ms", &p))
                    .unwrap_or_else(|_| DashboardStatCard {
                        label: "Wall ms".into(),
                        value: "—".into(),
                    }),
            ],
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = range_secs;
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_platform_throughput(
    range_secs: i64,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let bucket_secs = 60;
        let reads = query_metric_points("valence_db_reads", range_secs).await?;
        let writes = query_metric_points("valence_db_writes", range_secs).await?;
        let errors = query_metric_points("valence_db_errors", range_secs).await?;
        Ok(vec![
            DashboardChartSeries {
                id: "reads".into(),
                label: "Reads / sec".into(),
                points: bucket_counter_rates(&reads, bucket_secs),
            },
            DashboardChartSeries {
                id: "writes".into(),
                label: "Writes / sec".into(),
                points: bucket_counter_rates(&writes, bucket_secs),
            },
            DashboardChartSeries {
                id: "errors".into(),
                label: "Errors / sec".into(),
                points: bucket_counter_rates(&errors, bucket_secs),
            },
        ])
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = range_secs;
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_platform_writes_breakdown(
    range_secs: i64,
    group_by: String,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let key = match group_by.as_str() {
            "database" | "database_type" => "database_type",
            _ => "table",
        };
        let points = query_metric_points("valence_db_writes", range_secs).await?;
        Ok(group_points_by_label(&points, key, 8, 60))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (range_secs, group_by);
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_platform_reads_breakdown(
    range_secs: i64,
    group_by: String,
) -> Result<Vec<DashboardChartSeries>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let key = match group_by.as_str() {
            "database" | "database_type" => "database_type",
            _ => "table",
        };
        let points = query_metric_points("valence_db_reads", range_secs).await?;
        Ok(group_points_by_label(&points, key, 8, 60))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (range_secs, group_by);
        unreachable!("SSR only")
    }
}

#[uf_product_macros::server]
pub(crate) async fn get_dashboard_platform_error_offenders(
    range_secs: i64,
) -> Result<Vec<DashboardErrorSlice>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let _ = range_secs;
        Ok(Vec::new())
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = range_secs;
        unreachable!("SSR only")
    }
}

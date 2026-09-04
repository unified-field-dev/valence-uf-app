use std::collections::BTreeMap;

use leptos::prelude::*;
use orbital::components::{Button, Flex, FlexGap, FlexWrap};
use orbital_charts::{AxisDef, BarChart, ChartType, LineChart, ScaleType, SeriesDef};

use crate::server::{DashboardChartSeries, DashboardErrorSlice};

pub fn line_chart_from_series(series: &[DashboardChartSeries], height: f64) -> impl IntoView {
    let mut ts_labels: BTreeMap<chrono::DateTime<chrono::Utc>, String> = BTreeMap::new();
    for s in series {
        for p in &s.points {
            ts_labels
                .entry(p.ts)
                .or_insert_with(|| format!("{}:{}", p.ts.format("%H"), p.ts.format("%M")));
        }
    }
    let ts_order: Vec<_> = ts_labels.keys().copied().collect();
    let categories: Vec<String> = ts_labels.values().cloned().collect();

    let chart_series: Vec<SeriesDef> = series
        .iter()
        .map(|s| {
            let map: BTreeMap<_, _> = s.points.iter().map(|p| (p.ts, p.value)).collect();
            let data: Vec<f64> = ts_order
                .iter()
                .map(|ts| map.get(ts).copied().unwrap_or(f64::NAN))
                .collect();
            SeriesDef {
                id: s.id.clone(),
                label: Some(s.label.clone()),
                data: Some(data),
                chart_type: Some(ChartType::Line),
                connect_nulls: Some(true),
                show_markers: Some(false),
                ..Default::default()
            }
        })
        .collect();

    let x_axis = vec![AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(categories),
        ..Default::default()
    }];

    view! {
        <LineChart series=chart_series x_axis=x_axis height=height />
    }
}

pub fn area_chart_from_series(
    series: &[DashboardChartSeries],
    stack_group: &str,
    height: f64,
) -> impl IntoView {
    let mut ts_labels: BTreeMap<chrono::DateTime<chrono::Utc>, String> = BTreeMap::new();
    for s in series {
        for p in &s.points {
            ts_labels
                .entry(p.ts)
                .or_insert_with(|| format!("{}:{}", p.ts.format("%H"), p.ts.format("%M")));
        }
    }
    let ts_order: Vec<_> = ts_labels.keys().copied().collect();
    let categories: Vec<String> = ts_labels.values().cloned().collect();

    let chart_series: Vec<SeriesDef> = series
        .iter()
        .map(|s| {
            let map: BTreeMap<_, _> = s.points.iter().map(|p| (p.ts, p.value)).collect();
            let data: Vec<f64> = ts_order
                .iter()
                .map(|ts| map.get(ts).copied().unwrap_or(0.0))
                .collect();
            SeriesDef {
                id: s.id.clone(),
                label: Some(s.label.clone()),
                data: Some(data),
                chart_type: Some(ChartType::Area),
                stack_group: Some(stack_group.to_string()),
                area: Some(true),
                connect_nulls: Some(true),
                ..Default::default()
            }
        })
        .collect();

    let x_axis = vec![AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(categories),
        ..Default::default()
    }];

    view! {
        <orbital_charts::AreaChart series=chart_series x_axis=x_axis height=height />
    }
}

pub fn bar_chart_from_slices(slices: &[DashboardErrorSlice], height: f64) -> impl IntoView {
    let categories: Vec<String> = slices.iter().map(|s| s.label.clone()).collect();
    let data: Vec<f64> = slices.iter().map(|s| s.value).collect();
    let chart_series = vec![SeriesDef {
        id: "errors".into(),
        label: Some("Errors".into()),
        data: Some(data),
        chart_type: Some(ChartType::Bar),
        ..Default::default()
    }];
    let x_axis = vec![AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Band,
        data: Some(categories),
        ..Default::default()
    }];

    view! {
        <BarChart series=chart_series x_axis=x_axis height=height />
    }
}

const RANGE_PRESETS: &[(&str, i64)] = &[("15m", 900), ("1h", 3600), ("24h", 86400)];

#[component]
pub fn DashboardPlatformToolbar(
    range_secs: RwSignal<i64>,
    group_by: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div id="valence-dashboard-platform-toolbar">
        <Flex gap=FlexGap::Medium wrap=FlexWrap::Wrap>
            <Flex gap=FlexGap::Small>
                {RANGE_PRESETS.iter().map(|(label, secs)| {
                    let secs = *secs;
                    let label = *label;
                    view! {
                        <div data-testid=format!("dashboard-range-{label}")>
                            <Button on:click=move |_| range_secs.set(secs)>
                                {label}
                            </Button>
                        </div>
                    }
                }).collect_view()}
            </Flex>
            <Flex gap=FlexGap::Small>
                <div data-testid="dashboard-group-table">
                    <Button on:click=move |_| group_by.set("table".into())>"By table"</Button>
                </div>
                <div data-testid="dashboard-group-database">
                    <Button on:click=move |_| group_by.set("database".into())>"By database"</Button>
                </div>
            </Flex>
        </Flex>
        </div>
    }
}

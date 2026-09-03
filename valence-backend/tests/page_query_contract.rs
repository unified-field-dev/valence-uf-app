//! Integration contracts for schema/trait `DataTable` page-query adapters.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};
use valence_backend::{
    apply_schema_page_query, apply_trait_page_query, SchemaListItem, TraitListItem,
};

fn sample_schema(name: &str, description: Option<&str>) -> SchemaListItem {
    SchemaListItem {
        name: name.into(),
        databases: vec!["mem".into(), "sqlite".into()],
        version: "1".into(),
        description: description.map(str::to_string),
    }
}

fn sample_trait(name: &str, description: Option<&str>) -> TraitListItem {
    TraitListItem {
        name: name.into(),
        version: "0".into(),
        description: description.map(str::to_string),
    }
}

#[test]
fn schemas_datatable_quick_search_happy_path() {
    let mut items = vec![
        sample_schema("counter", Some("counts things")),
        sample_schema("user", Some("people")),
    ];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: Some("people".into()),
        filter: None,
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "user");
}

#[test]
fn schemas_datatable_or_logic_keeps_either_match_happy_path() {
    let mut items = vec![
        sample_schema("alpha", None),
        sample_schema("beta", None),
        sample_schema("gamma", None),
    ];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::Or,
            items: vec![
                FilterRuleParam {
                    field: "name".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("alpha".into()),
                },
                FilterRuleParam {
                    field: "name".into(),
                    operator: "equals".into(),
                    value: DataValue::Text("gamma".into()),
                },
            ],
        }),
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].name, "alpha");
    assert_eq!(items[1].name, "gamma");
}

#[test]
fn schemas_datatable_name_equals_happy_path() {
    let mut items = vec![sample_schema("counter", None), sample_schema("user", None)];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "name".into(),
                operator: "equals".into(),
                value: DataValue::Text("Counter".into()),
            }],
        }),
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "counter");
}

#[test]
fn schemas_datatable_name_unknown_empty_sad() {
    let mut items = vec![sample_schema("counter", None)];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "name".into(),
                operator: "equals".into(),
                value: DataValue::Text("__missing__".into()),
            }],
        }),
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 0);
}

#[test]
fn schemas_datatable_databases_contains_happy_path() {
    let mut items = vec![
        SchemaListItem {
            name: "a".into(),
            databases: vec!["mem".into()],
            version: "1".into(),
            description: None,
        },
        SchemaListItem {
            name: "b".into(),
            databases: vec!["sqlite".into()],
            version: "1".into(),
            description: None,
        },
    ];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "databases".into(),
                operator: "contains".into(),
                value: DataValue::Text("sql".into()),
            }],
        }),
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "b");
}

#[test]
fn traits_datatable_quick_search_happy_path() {
    let mut items = vec![
        sample_trait("Named", Some("has a name")),
        sample_trait("HasOwner", None),
    ];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: Some("owner".into()),
        filter: None,
        sort: None,
    };
    apply_trait_page_query(&mut items, &request);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "HasOwner");
}

#[test]
fn traits_datatable_name_filter_unknown_empty_sad() {
    let mut items = vec![sample_trait("Named", None)];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: None,
        filter: Some(FilterQuery {
            logic: FilterLogicWire::And,
            items: vec![FilterRuleParam {
                field: "name".into(),
                operator: "equals".into(),
                value: DataValue::Text("__missing__".into()),
            }],
        }),
        sort: None,
    };
    apply_trait_page_query(&mut items, &request);
    assert_eq!(items.len(), 0);
}

#[test]
fn schemas_datatable_blank_quick_search_keeps_all_happy_path() {
    let mut items = vec![sample_schema("a", None), sample_schema("b", None)];
    let request = PageRequest {
        offset: 0,
        limit: 20,
        quick_search: Some("   ".into()),
        filter: None,
        sort: None,
    };
    apply_schema_page_query(&mut items, &request);
    assert_eq!(items.len(), 2);
}

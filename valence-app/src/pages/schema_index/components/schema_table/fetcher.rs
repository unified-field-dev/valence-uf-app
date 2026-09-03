use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::PageFetcher;
use orbital_data::DataRecord;
use orbital_paging::{Page, PageRequest};

use crate::server::{get_schemas_page, SCHEMAS_PAGE_SIZE};

use super::mapper::schema_item_to_record;

pub fn build_schema_fetcher() -> PageFetcher {
    Arc::new(|request: PageRequest| {
        Box::pin(async move {
            let page = get_schemas_page(request).await?;
            Ok(Page {
                items: page.items.into_iter().map(schema_item_to_record).collect(),
                has_more: page.has_more,
                total_count: page.total_count,
                next_request_offset: page.next_request_offset,
            })
        }) as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    })
}

pub const SCHEMA_TABLE_PAGE_SIZE: u32 = SCHEMAS_PAGE_SIZE;

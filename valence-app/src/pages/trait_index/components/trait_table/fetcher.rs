use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use leptos::prelude::*;
use orbital::primitives::PageFetcher;
use orbital_data::DataRecord;
use orbital_paging::{Page, PageRequest};

use crate::server::{get_traits_page, TRAITS_PAGE_SIZE};

use super::mapper::trait_item_to_record;

pub fn build_trait_fetcher() -> PageFetcher {
    Arc::new(|request: PageRequest| {
        Box::pin(async move {
            let page = get_traits_page(request).await?;
            Ok(Page {
                items: page.items.into_iter().map(trait_item_to_record).collect(),
                has_more: page.has_more,
                total_count: page.total_count,
                next_request_offset: page.next_request_offset,
            })
        }) as Pin<Box<dyn Future<Output = Result<Page<DataRecord>, ServerFnError>> + Send>>
    })
}

pub const TRAIT_TABLE_PAGE_SIZE: u32 = TRAITS_PAGE_SIZE;

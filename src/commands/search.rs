use anyhow::Result;

use crate::api::client::ContinenteClient;
use crate::api::models::{SearchParams, SortRule};
use crate::format::{self, OutputFormat};

#[allow(clippy::too_many_arguments)]
pub async fn run(
    client: &ContinenteClient,
    query: &str,
    max: u32,
    page: u32,
    sort: Option<SortRule>,
    brand: Option<String>,
    price_min: Option<f64>,
    price_max: Option<f64>,
    filters: Vec<(String, String)>,
    output_format: OutputFormat,
) -> Result<String> {
    let params = SearchParams {
        start: (page - 1) * max,
        size: max,
        sort,
        price_min,
        price_max,
        brand,
        filters,
    };

    let response = client.search(query, &params).await?;
    Ok(format::format_products(&response, page, max, output_format))
}

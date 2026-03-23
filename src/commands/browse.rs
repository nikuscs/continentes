use anyhow::{Result, bail};

use crate::api::client::ContinenteClient;
use crate::api::models::{SearchParams, SortRule};
use crate::categories;
use crate::format::{self, OutputFormat};

pub async fn run(
    client: &ContinenteClient,
    category: &str,
    max: u32,
    page: u32,
    sort: Option<SortRule>,
    output_format: OutputFormat,
) -> Result<String> {
    if page == 0 {
        bail!("Page number must be at least 1");
    }
    if max == 0 {
        bail!("Maximum results must be at least 1");
    }

    let cgid = categories::resolve_cgid(category).unwrap_or(category);

    let params = SearchParams {
        start: (page - 1) * max,
        size: max,
        sort,
        ..Default::default()
    };

    let response = client.browse(cgid, &params).await;

    match response {
        Ok(r) => format::format_products(&r, page, max, output_format),
        Err(crate::error::ContinenteError::NoResults) => {
            bail!(
                "No products found in category '{category}'. Use 'cnt categories' to see available categories."
            )
        }
        Err(e) => Err(e.into()),
    }
}

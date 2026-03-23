use anyhow::{Context, Result, bail};

use crate::api::client::ContinenteClient;
use crate::format::{self, OutputFormat};

pub async fn run(
    client: &ContinenteClient,
    pid: &str,
    include_nutrition: bool,
    output_format: OutputFormat,
) -> Result<String> {
    let product = client
        .product(pid)
        .await
        .context("Failed to fetch product")?;

    let nutrition = if include_nutrition {
        if let (Some(ean), Some(supplier)) = (&product.ean, &product.supplier_id) {
            let info = client
                .nutrition(pid, ean, supplier)
                .await
                .context("Failed to fetch nutritional info")?;
            Some(info)
        } else {
            bail!("Nutritional info is not available for product '{pid}'")
        }
    } else {
        None
    };

    format::format_product_detail(&product, nutrition.as_ref(), output_format)
}

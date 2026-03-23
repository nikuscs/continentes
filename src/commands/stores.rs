use anyhow::Result;

use crate::api::client::ContinenteClient;
use crate::format::{self, OutputFormat};

pub async fn run(
    client: &ContinenteClient,
    lat: f64,
    lon: f64,
    radius: u32,
    output_format: OutputFormat,
) -> Result<String> {
    let stores = client.stores(lat, lon, radius).await?;
    format::format_stores(&stores, radius, output_format)
}

use anyhow::Result;

use crate::api::client::ContinenteClient;
use crate::format::{self, OutputFormat};

pub async fn run(client: &ContinenteClient, output_format: OutputFormat) -> Result<String> {
    let flyers = client.flyers().await?;
    Ok(format::format_flyers(&flyers, output_format))
}

use anyhow::Result;

use crate::api::client::ContinenteClient;
use crate::format::{self, OutputFormat};

pub async fn run(
    client: &ContinenteClient,
    query: &str,
    output_format: OutputFormat,
) -> Result<String> {
    let result = client.suggest(query).await?;
    Ok(format::format_suggestions(&result, output_format))
}

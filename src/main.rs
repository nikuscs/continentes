use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use continente::api::client::ContinenteClient;
use continente::api::models::SortRule;
use continente::config::Config;
use continente::format::OutputFormat;

#[derive(Parser)]
#[command(name = "cnt", about = "Browse Continente supermarket products")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format
    #[arg(long, default_value = "table", global = true)]
    format: OutputFormat,

    /// Enable debug logging
    #[arg(long, short, global = true)]
    verbose: bool,

    /// Config file path
    #[arg(long, env = "CONTINENTE_CONFIG", global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Search products by keyword
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,

        /// Maximum results
        #[arg(long, default_value = "24")]
        max: u32,

        /// Page number (1-indexed)
        #[arg(long, default_value = "1")]
        page: u32,

        /// Sort order
        #[arg(long)]
        sort: Option<SortRule>,

        /// Filter by brand
        #[arg(long)]
        brand: Option<String>,

        /// Minimum price
        #[arg(long)]
        price_min: Option<f64>,

        /// Maximum price
        #[arg(long)]
        price_max: Option<f64>,

        /// Show only vegan products
        #[arg(long)]
        vegan: bool,

        /// Show only gluten-free products
        #[arg(long)]
        gluten_free: bool,

        /// Show only lactose-free products
        #[arg(long)]
        lactose_free: bool,

        /// Show only organic products
        #[arg(long)]
        bio: bool,
    },

    /// Get full product details
    #[command(alias = "p")]
    Product {
        /// Product ID
        pid: String,

        /// Include nutritional info
        #[arg(long)]
        nutrition: bool,
    },

    /// Browse products by category
    #[command(alias = "b")]
    Browse {
        /// Category ID (e.g., "laticinios", "frescos")
        category: String,

        /// Maximum results
        #[arg(long, default_value = "24")]
        max: u32,

        /// Page number (1-indexed)
        #[arg(long, default_value = "1")]
        page: u32,

        /// Sort order
        #[arg(long)]
        sort: Option<SortRule>,
    },

    /// Search suggestions (autocomplete, min 5 chars)
    #[command(alias = "sg")]
    Suggest {
        /// Search query
        query: String,
    },

    /// Find nearby stores
    #[command(alias = "st")]
    Stores {
        /// Latitude
        #[arg(long)]
        lat: f64,

        /// Longitude
        #[arg(long)]
        lon: f64,

        /// Search radius in km
        #[arg(long, default_value = "10")]
        radius: u32,
    },

    /// List available categories
    #[command(alias = "cat")]
    Categories,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new(Level::DEBUG.to_string())
    } else {
        EnvFilter::from_default_env().add_directive(Level::WARN.into())
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let config = Config::load(cli.config.as_deref());
    let _client = ContinenteClient::new(&config.http)?;

    match cli.command {
        Commands::Search { query, .. } => {
            anyhow::bail!("search command not yet implemented (query: {query})")
        }
        Commands::Product { pid, .. } => {
            anyhow::bail!("product command not yet implemented (pid: {pid})")
        }
        Commands::Browse { category, .. } => {
            anyhow::bail!("browse command not yet implemented (category: {category})")
        }
        Commands::Suggest { query } => {
            anyhow::bail!("suggest command not yet implemented (query: {query})")
        }
        Commands::Stores { lat, lon, .. } => {
            anyhow::bail!("stores command not yet implemented (lat: {lat}, lon: {lon})")
        }
        Commands::Categories => {
            anyhow::bail!("categories command not yet implemented")
        }
    }
}

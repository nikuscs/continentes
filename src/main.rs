use std::io::Write as _;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use continente::api::client::ContinenteClient;
use continente::api::models::SortRule;
use continente::commands;
use continente::config::load_config;
use continente::format::OutputFormat;

#[derive(Parser)]
#[command(name = "cnt", about = "Browse Continente supermarket products")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format
    #[arg(long, global = true)]
    format: Option<OutputFormat>,

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
        /// Category ID or name (e.g., "laticinios", "frescos")
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
        #[arg(long, allow_hyphen_values = true)]
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

    let config = load_config(cli.config.as_deref())?;
    let client = ContinenteClient::new(&config.http)?;
    let output_format = cli.format.unwrap_or(config.output.format);

    let output = match cli.command {
        Commands::Search {
            query,
            max,
            page,
            sort,
            brand,
            price_min,
            price_max,
            vegan,
            gluten_free,
            lactose_free,
            bio,
        } => {
            let mut filters = Vec::new();
            if vegan {
                filters.push(("food.Vegan".to_string(), "true".to_string()));
            }
            if gluten_free {
                filters.push(("food.GlutenFree".to_string(), "true".to_string()));
            }
            if lactose_free {
                filters.push(("food.LactoseFree".to_string(), "true".to_string()));
            }
            if bio {
                filters.push(("food.Biologic".to_string(), "true".to_string()));
            }

            commands::search::run(
                &client,
                &query,
                max,
                page,
                sort,
                brand,
                price_min,
                price_max,
                filters,
                output_format,
            )
            .await?
        }

        Commands::Product { pid, nutrition } => {
            commands::product::run(&client, &pid, nutrition, output_format).await?
        }

        Commands::Browse {
            category,
            max,
            page,
            sort,
        } => commands::browse::run(&client, &category, max, page, sort, output_format).await?,

        Commands::Suggest { query } => {
            commands::suggest::run(&client, &query, output_format).await?
        }

        Commands::Stores { lat, lon, radius } => {
            commands::stores::run(&client, lat, lon, radius, output_format).await?
        }

        Commands::Categories => commands::categories::run(output_format),
    };

    std::io::stdout().write_all(output.as_bytes())?;
    Ok(())
}

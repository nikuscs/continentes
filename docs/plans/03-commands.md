# Plan 03: Commands

## Goal

Wire up CLI subcommands to the API client. Each command is an async function that calls the client, formats output, and writes to stdout.

## Steps

### 3.1 CLI definitions (`src/main.rs`)

Update clap derive definitions:

```rust
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

        /// Show only organic (bio) products
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

    /// Search suggestions (autocomplete)
    #[command(alias = "sg")]
    Suggest {
        /// Search query (minimum 5 characters)
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
```

### 3.2 Command implementations

Each command in `src/commands/` follows this pattern:

```rust
pub async fn search(
    client: &ContinenteClient,
    query: &str,
    params: SearchParams,
    format: OutputFormat,
) -> anyhow::Result<String>
```

#### `src/commands/search.rs`

1. Build `SearchParams` from CLI args (page → start offset, filters from booleans)
2. Call `client.search(query, &params).await?`
3. Format with `format::format_products(&result, format)`
4. Return formatted string

#### `src/commands/product.rs`

1. Call `client.product(pid).await?`
2. If `--nutrition` flag: extract EAN + supplier from `nutritional_info_url`, call `client.nutrition()`
3. Format with `format::format_product_detail(&detail, nutrition, format)`
4. Return formatted string

#### `src/commands/browse.rs`

1. Resolve category name to cgid (use a lookup table from Appendix A of investigation.md)
2. Call `client.browse(cgid, &params).await?`
3. Format with `format::format_products(&result, format)` (same as search)
4. Return formatted string

#### `src/commands/suggest.rs`

1. Validate query length >= 5
2. Call `client.suggest(query).await?`
3. Format suggestions (products + categories + popular terms)
4. Return formatted string

#### `src/commands/stores.rs`

1. Call `client.stores(lat, lon, radius).await?`
2. Format with `format::format_stores(&stores, format)`
3. Return formatted string

#### `src/commands/categories.rs`

1. Hardcoded category tree from investigation Appendix A
2. Display as a tree or table depending on format
3. No API call needed — categories are static

### 3.3 Category lookup table

Create `src/categories.rs` with a static lookup:

```rust
pub struct Category {
    pub cgid: &'static str,
    pub name: &'static str,
    pub parent: Option<&'static str>,
}

/// Returns all categories as a flat list
pub fn all_categories() -> &'static [Category] { ... }

/// Resolve a user-friendly name or alias to a cgid
/// e.g., "leite" → "laticinios-leite", "frescos" → "frescos"
pub fn resolve_cgid(input: &str) -> Option<&'static str> { ... }
```

The resolve function should:
1. Try exact match on cgid
2. Try exact match on name (case-insensitive)
3. Try partial match / fuzzy match on name

### 3.4 main.rs dispatch

```rust
let output = match cli.command {
    Commands::Search { query, max, page, sort, brand, ... } => {
        let params = SearchParams::from_args(max, page, sort, brand, ...);
        commands::search::search(&client, &query, params, cli.format).await?
    }
    Commands::Product { pid, nutrition } => {
        commands::product::product(&client, &pid, nutrition, cli.format).await?
    }
    // ... etc
};

// Write output (same pattern as lauyer)
std::io::Write::write_all(&mut std::io::stdout(), output.as_bytes())?;
```

## Verification

After this plan:
- `cnt search "leite"` returns products (even if formatting is basic)
- `cnt product 6879912` returns product details
- `cnt browse frescos` returns category products
- `cnt suggest "leite"` returns suggestions
- `cnt stores --lat 38.7 --lon -9.1` returns stores
- `cnt categories` lists all categories
- All commands respect `--format json`

## Files Created/Modified

| File | Action |
|------|--------|
| `src/main.rs` | Rewrite with full CLI |
| `src/commands/mod.rs` | Update exports |
| `src/commands/search.rs` | Create |
| `src/commands/product.rs` | Create |
| `src/commands/browse.rs` | Create |
| `src/commands/suggest.rs` | Create |
| `src/commands/stores.rs` | Create |
| `src/commands/categories.rs` | Create |
| `src/categories.rs` | Create (static lookup) |
| `src/lib.rs` | Add categories module |

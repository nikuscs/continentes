---
name: cnt
description: Browse Continente online supermarket products, prices, nutrition, categories, flyers, and stores in Portugal. Use for finding grocery products, comparing prices, checking nutritional info, and locating stores.
metadata: {"openclaw":{"emoji":"🛒","requires":{"bins":["cnt"]},"install":[{"id":"binary","kind":"custom","command":"# Download from https://github.com/nikuscs/continentes/releases/latest\n# macOS: cnt-macos-arm64.tar.gz\n# Linux x64: cnt-linux-x64.tar.gz\n# Linux ARM64: cnt-linux-arm64.tar.gz\ntar -xzf cnt-*.tar.gz && chmod +x cnt && sudo mv cnt /usr/local/bin/","label":"Download pre-built binary (recommended)"},{"id":"cargo","kind":"cargo","crate":"continente","bins":["cnt"],"label":"Install via Cargo (requires Rust)"}]}}
---

# Continente CLI (cnt)

Browse [Continente online](https://www.continente.pt) supermarket products from Portugal. Search products, compare prices, inspect nutritional info, browse categories, view weekly flyers, and find nearby stores.

## Installation

**No Rust or compilation required.** Download the pre-built binary for your platform from [Releases](https://github.com/nikuscs/continentes/releases/latest):

### macOS (Apple Silicon)

```bash
curl -L https://github.com/nikuscs/continentes/releases/latest/download/cnt-macos-arm64.tar.gz | tar xz
chmod +x cnt
sudo mv cnt /usr/local/bin/
```

### Linux (x64)

```bash
curl -L https://github.com/nikuscs/continentes/releases/latest/download/cnt-linux-x64.tar.gz | tar xz
chmod +x cnt
sudo mv cnt /usr/local/bin/
```

### Linux (ARM64)

```bash
curl -L https://github.com/nikuscs/continentes/releases/latest/download/cnt-linux-arm64.tar.gz | tar xz
chmod +x cnt
sudo mv cnt /usr/local/bin/
```

### From source

```bash
cargo install --git https://github.com/nikuscs/continentes
```

### Verify

```bash
cnt --help
```

## When to use (trigger phrases)

Use this skill when the user asks:

- "search Continente for..."
- "how much does X cost at Continente?"
- "compare supermarket prices in Portugal"
- "find grocery products at Continente"
- "nutritional info for Continente product..."
- "find Continente stores near..."
- "what categories does Continente have?"
- "current Continente flyers/promotions"
- "is X vegan/gluten-free at Continente?"
- "cheapest X at Continente"
- Any mention of Continente, Portuguese supermarket, grocery shopping Portugal, product prices Portugal, or Continente online

## Commands

### Search products

Search the full Continente catalog by keyword with optional filters.

```bash
# Basic search
cnt search "leite"

# Search with brand filter
cnt search "cerveja" --brand "Super Bock"

# Search with price range
cnt search "arroz" --price-min 1 --price-max 3

# Dietary filters (can be combined)
cnt search "leite" --vegan
cnt search "bolachas" --gluten-free
cnt search "iogurte" --lactose-free
cnt search "snacks" --sugar-free
cnt search "hamburguer" --vegetarian
cnt search "cereais" --bio
cnt search "snacks" --vegan --gluten-free --bio

# Sorting
cnt search "leite" --sort price-low-to-high
cnt search "vinho" --sort price-high-to-low
cnt search "agua" --sort unit-price

# Pagination
cnt search "leite" --max 10 --page 2

# JSON output for programmatic use
cnt search "leite" --format json

# Compact (TSV) for piping
cnt search "leite" --format compact | cut -f1,4
```

### Search Options

| Flag | Description | Example |
|------|-------------|---------|
| `--brand` | Filter by brand name | `--brand "Mimosa"` |
| `--price-min` | Minimum price (€) | `--price-min 1` |
| `--price-max` | Maximum price (€) | `--price-max 5` |
| `--vegan` | Vegan products only | |
| `--vegetarian` | Vegetarian products only | |
| `--gluten-free` | Gluten-free products only | |
| `--lactose-free` | Lactose-free products only | |
| `--sugar-free` | Sugar-free products only | |
| `--bio` | Organic/biological products only | |
| `--sort` | Sort order | `--sort price-low-to-high` |
| `--max` | Max results (default: 24) | `--max 10` |
| `--page` | Page number (1-indexed) | `--page 2` |

**Sort options**: `relevance`, `price-low-to-high`, `price-high-to-low`, `unit-price`, `name-asc`, `name-desc`

### Product details

Get full product information by product ID, optionally including nutritional data.

```bash
# Basic product info (price, brand, category, EAN, rating)
cnt product 6879912

# Include nutritional info (ingredients, allergens, nutrients table)
cnt product 6879912 --nutrition

# JSON for structured data
cnt product 6879912 --nutrition --format json
```

Returns: name, brand, price (with promotions), unit price, package size, rating, category path, EAN/barcode, availability, badges, producer, nutritional table (energy, fat, carbs, protein, salt, vitamins), allergens, ingredients, storage instructions, preparation instructions, serving size.

### Browse categories

Browse products within a category. Accepts category ID (cgid) or a partial name match.

```bash
# By category name (fuzzy match)
cnt browse frescos
cnt browse "leite"
cnt browse "cerveja"

# By exact cgid
cnt browse laticinios-leite
cnt browse mercearias-arroz-massa

# With sorting
cnt browse congelados --sort price-low-to-high --max 10
```

### Search suggestions (autocomplete)

Get search autocomplete suggestions (products, categories, popular terms). Minimum 5 characters.

```bash
cnt suggest "arroz"
cnt suggest "leite"
cnt suggest "cerve"
```

### Find stores

Find Continente stores near a GPS coordinate. Shows pickup and Galp station availability.

```bash
# Lisbon area
cnt stores --lat 38.7 --lon -9.1

# Porto area, larger radius
cnt stores --lat 41.1 --lon -8.6 --radius 20

# All stores in Portugal
cnt stores --lat 39.5 --lon -8.0 --radius 300

# JSON for mapping
cnt stores --lat 38.7 --lon -9.1 --format json
```

Returns: store name, address, city, postal code, GPS coordinates, phone, opening hours, pickup availability, Galp station flag.

### List categories

Show all 251 product categories with their cgid values.

```bash
cnt categories              # tree view (14 top-level with subcategories)
cnt categories --format json  # machine-readable (251 entries)
```

### Browse flyers

List current weekly flyers and catalogs from iPaper.

```bash
cnt flyers                    # table with slug, title, dates
cnt flyers --format json      # structured data with URLs and images
```

## Output Formats

| Format | Flag | Use case |
|--------|------|----------|
| Table | `--format table` | Human-readable (default) |
| JSON | `--format json` | Programmatic / LLM parsing |
| Compact | `--format compact` | Piping to other CLI tools (TSV) |

## Global Options

| Flag | Description |
|------|-------------|
| `--format <FORMAT>` | table, json, compact |
| `-v, --verbose` | Debug logging |
| `--config <PATH>` | Config file (env: `CONTINENTE_CONFIG`) |

## Common Workflows

### Compare prices for a product

```bash
# Find cheapest milk
cnt search "leite" --sort price-low-to-high --max 10

# Compare a specific brand across products
cnt search "leite" --brand "Mimosa" --format json | jq '.products[] | {name, price}'
```

### Research product nutrition

```bash
# 1. Search for products
cnt search "iogurte grego" --format json

# 2. Get full nutrition for a specific product
cnt product 1234567 --nutrition --format json
```

### Find vegan alternatives

```bash
# Search vegan products in a category
cnt search "queijo" --vegan --sort price-low-to-high

# Check nutritional details
cnt product <id> --nutrition
```

### Build a shopping list

```bash
# Search multiple products, extract IDs and prices
cnt search "leite" --format json | jq '.products[:3] | .[] | {id, name, price}'
cnt search "pão" --format json | jq '.products[:3] | .[] | {id, name, price}'
cnt search "fruta" --format json | jq '.products[:5] | .[] | {id, name, price}'
```

### Find nearest store

```bash
# Get stores near coordinates
cnt stores --lat 38.7223 --lon -9.1393 --radius 5 --format json | jq '.[0] | {name, address, city}'
```

### Check weekly promotions

```bash
# List current flyers
cnt flyers --format json | jq '.[] | {title, description, url}'
```

## Agent Guidelines

- **Always use `--format json`** when processing results programmatically
- Product IDs are numeric strings (e.g., `6879912`) — extract from search results
- **EAN/barcode** is available in product detail but not in search results
- For nutritional info, always use `--nutrition` flag with `product` command
- Categories use internal `cgid` values — use `cnt categories --format json` to get the mapping
- Store coordinates are in decimal degrees (WGS84)
- Suggestions require minimum 5 characters
- No authentication needed — all endpoints are public
- Unit prices show per-liter or per-kilogram depending on product type
- Dietary filters can be combined (e.g., `--vegan --gluten-free`)
- Use `--sort unit-price` for best value comparisons

## Data Coverage

- ~50,000+ products across 14 top-level categories and 251 subcategories
- 228 stores across mainland Portugal, Madeira, and Azores
- Full nutritional info for food products (energy, fat, carbs, protein, salt, vitamins, minerals)
- Price data includes current price, PVPR (recommended retail), and promotions
- Product badges: "Produzido em Portugal", organic, vegan, promotions
- 6 dietary filters: vegan, vegetarian, gluten-free, lactose-free, sugar-free, bio
- Weekly flyers and catalogs via iPaper integration
- Store data includes pickup availability and Galp station flags

Feel free to copy and adapt this tool's interface into your own skill definitions or MCP server configurations.

---
emoji: 🛒
requirements:
  - name: cnt
    install: cargo install --git https://github.com/nikuscs/continentes
    how_to_check: cnt --help
---

# Continente CLI (cnt)

A command-line tool for browsing [Continente](https://www.continente.pt) supermarket products from Portugal.

## Installation

### Pre-built binaries

Download from [Releases](https://github.com/nikuscs/continentes/releases):

- **macOS (Apple Silicon)**: `cnt-macos-arm64.tar.gz`
- **Linux (x64)**: `cnt-linux-x64.tar.gz`
- **Linux (ARM64)**: `cnt-linux-arm64.tar.gz`

```bash
tar -xzf cnt-*.tar.gz && chmod +x cnt && sudo mv cnt /usr/local/bin/
```

### From source

```bash
cargo install --git https://github.com/nikuscs/continentes
```

## When to use

Use this skill when the user asks about:
- Portuguese supermarket product prices, availability, or nutritional information
- Comparing grocery products at Continente
- Finding Continente store locations
- Browsing supermarket categories or deals
- Building shopping lists with Portuguese grocery data
- Looking up EAN/barcode data for Continente products

**Trigger phrases**: "Continente prices", "supermarket products Portugal", "grocery search", "product nutrition info", "find Continente stores", "browse categories Continente"

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

# Dietary filters
cnt search "leite" --vegan
cnt search "bolachas" --gluten-free
cnt search "iogurte" --lactose-free
cnt search "cereais" --bio

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

Returns: name, brand, price (with promotions), unit price, package size, rating, category path, EAN/barcode, availability, producer, nutritional table, allergens, ingredients.

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

Find Continente stores near a GPS coordinate.

```bash
# Lisbon area
cnt stores --lat 38.7 --lon -9.1

# Porto area, larger radius
cnt stores --lat 41.1 --lon -8.6 --radius 20

# JSON for mapping
cnt stores --lat 38.7 --lon -9.1 --format json
```

Returns: store name, address, city, postal code, GPS coordinates, phone, opening hours, pickup availability.

### List categories

Show all available product categories with their cgid values.

```bash
cnt categories
cnt categories --format json
```

## Output formats

| Format | Flag | Use case |
|--------|------|----------|
| Table | `--format table` | Human-readable (default) |
| JSON | `--format json` | Programmatic / LLM parsing |
| Compact | `--format compact` | Piping to other CLI tools |

## Global options

| Flag | Description |
|------|-------------|
| `--format <FORMAT>` | table, json, compact |
| `-v, --verbose` | Debug logging |
| `--config <PATH>` | Config file (env: `CONTINENTE_CONFIG`) |

## Agent guidelines

- **Always use `--format json`** when processing results programmatically
- Product IDs are numeric strings (e.g., `6879912`) — extract from search results
- **EAN/barcode** is available in product detail but not in search results
- For nutritional info, always use `--nutrition` flag with `product` command
- Categories use internal `cgid` values — use `cnt categories --format json` to get the mapping
- Store coordinates are in decimal degrees (WGS84)
- Suggestions require minimum 5 characters
- No authentication needed — all endpoints are public
- Unit prices show per-liter or per-kilogram depending on product type

## Data coverage

- ~50,000+ products across 11 top-level categories and 91 subcategories
- 228 stores across mainland Portugal, Madeira, and Azores
- Full nutritional info for food products (energy, fat, carbs, protein, salt, vitamins)
- Price data includes current price, PVPR (recommended retail), and promotions
- Product badges: "Produzido em Portugal", organic, vegan, promotions

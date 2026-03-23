# 🛒 cnt

[![CI](https://github.com/nikuscs/continentes/actions/workflows/ci.yml/badge.svg)](https://github.com/nikuscs/continentes/actions/workflows/ci.yml)
[![Release](https://github.com/nikuscs/continentes/actions/workflows/release.yml/badge.svg)](https://github.com/nikuscs/continentes/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Fast CLI for browsing [Continente online](https://www.continente.pt) supermarket products, optimized for LLM consumption. Works as a skill for [Claude Code](https://docs.anthropic.com/en/docs/claude-code), [Claude.ai](https://claude.ai), [OpenAI Codex](https://openai.com/index/openai-codex/), and any AI agent. Search products, compare prices, inspect nutritional info, browse categories, and find nearby stores from the terminal.**

> **Disclaimer:** This project is for **educational purposes and AI automation research only**.
> The authors are not responsible for any misuse or for any damages resulting from the use of this tool.
> Users are solely responsible for ensuring compliance with applicable laws and the terms of service
> of any websites accessed. This software is provided "as-is" without warranty of any kind.
>
> This project is not affiliated with Continente or Sonae MC.

> **Note:** This project was partially developed with AI assistance and may contain bugs or unexpected behavior. Use at your own risk.

## Features

- Search 50,000+ products with brand, price, and dietary filters
- Full product details with EAN/barcode, nutritional info, and allergens
- Browse 91 product categories
- Search autocomplete suggestions
- Find 228 stores across Portugal with GPS coordinates
- Multiple output formats: table, JSON, compact (TSV)
- No authentication required

## Installation

### From source

```bash
cargo install --git https://github.com/nikuscs/continentes
```

### Pre-built binaries

Download from [Releases](https://github.com/nikuscs/continentes/releases):

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `cnt-macos-arm64.tar.gz` |
| Linux (x64) | `cnt-linux-x64.tar.gz` |
| Linux (ARM64) | `cnt-linux-arm64.tar.gz` |

```bash
tar -xzf cnt-*.tar.gz && chmod +x cnt && sudo mv cnt /usr/local/bin/
```

## Usage

### Search products

```bash
cnt search "leite"                                    # basic search
cnt search "cerveja" --brand "Super Bock"             # filter by brand
cnt search "arroz" --price-min 1 --price-max 3        # price range
cnt search "leite" --vegan --sort price-low-to-high   # dietary + sort
cnt search "bolachas" --gluten-free --page 2          # paginate
```

### Product details

```bash
cnt product 6879912                  # basic info
cnt product 6879912 --nutrition      # include nutritional data
```

```
Leite UHT Meio Gordo Continente
===============================

ID:           6879912
Brand:        Continente
Price:        0,86€
Unit Price:   0,86€/lt
Package:      emb. 1 lt
Rating:       3.9
Category:     Laticínios e Ovos > Leite > Leite Meio Gordo
EAN:          5601312508007
Badge:        Produzido em Portugal
```

### Browse categories

```bash
cnt browse frescos                     # by name (fuzzy match)
cnt browse laticinios-leite            # by exact cgid
cnt browse "cerveja" --sort unit-price  # with sorting
```

### Search suggestions

```bash
cnt suggest "arroz"     # autocomplete (min 5 chars)
```

### Find stores

```bash
cnt stores --lat 38.7 --lon -9.1              # Lisbon area
cnt stores --lat 41.1 --lon -8.6 --radius 20  # Porto, 20km
```

### List categories

```bash
cnt categories            # tree view
cnt categories --format json  # machine-readable
```

## Output formats

```bash
cnt search "leite"                     # table (default)
cnt search "leite" --format json       # JSON
cnt search "leite" --format compact    # TSV for piping
```

```bash
# Pipe to jq
cnt search "leite" --format json | jq '.products[].name'

# Pipe to other tools
cnt search "leite" --format compact | sort -t$'\t' -k2 -n
```

## Dietary filters

| Flag | Description |
|------|-------------|
| `--vegan` | Vegan products only |
| `--gluten-free` | Gluten-free only |
| `--lactose-free` | Lactose-free only |
| `--bio` | Organic/biological only |

## Sort options

| Value | Description |
|-------|-------------|
| `relevance` | Default ranking |
| `price-low-to-high` | Cheapest first |
| `price-high-to-low` | Most expensive first |
| `unit-price` | By price per unit (kg/lt) |
| `name-asc` | Name A-Z |
| `name-desc` | Name Z-A |

## Configuration

Place at `./continente.toml` or `~/.config/continente/continente.toml`:

```toml
[http]
timeout_secs = 30
retries = 3
delay_ms = 100

[output]
format = "table"   # table, json, compact
```

## Global options

| Flag | Env var | Description |
|------|---------|-------------|
| `--format <FORMAT>` | | Output: table, json, compact |
| `-v, --verbose` | | Debug logging |
| `--config <PATH>` | `CONTINENTE_CONFIG` | Config file path |

## How it works

The CLI interacts with Continente's Salesforce Commerce Cloud (SFCC) storefront controllers. These are the same endpoints the website uses — no private APIs, no authentication, no scraping of rendered pages. Product data is extracted from structured `data-` attributes and JSON responses.

See [`docs/investigation.md`](docs/investigation.md) for the full reverse engineering investigation.

## Related Projects

- [⚖️ lauyer](https://github.com/nikuscs/lauyer) — Fast CLI for searching Portuguese court jurisprudence and legislation
- [🕷️ crauler](https://github.com/nikuscs/crauler) — Web crawler with proxy routing and HTML→Markdown
- [🦎 amz-crawler](https://github.com/nikuscs/amz-crawler) — Amazon product crawler with TLS fingerprinting
- [🕹️ scrauper](https://github.com/nikuscs/scrauper) — Multi-threaded ScreenScraper.fr scraper for ES-DE

## License

MIT — see `LICENSE`.

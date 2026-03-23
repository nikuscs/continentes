# cnt — Continente CLI

A command-line tool for browsing [Continente](https://www.continente.pt) supermarket products.

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

## Usage

### Search products

```bash
cnt search "leite"
cnt search "cerveja" --brand "Super Bock" --sort price-low-to-high
cnt search "leite" --vegan --max 10
cnt search "arroz" --price-min 1 --price-max 3
cnt search "bolachas" --gluten-free --page 2
```

### Product details

```bash
cnt product 6879912
cnt product 6879912 --nutrition
cnt product 6879912 --format json
```

### Browse categories

```bash
cnt browse frescos
cnt browse "leite" --max 10
cnt browse laticinios-leite --sort price-low-to-high
```

### Search suggestions

```bash
cnt suggest "arroz"
cnt suggest "leite"
```

### Find stores

```bash
cnt stores --lat 38.7 --lon -9.1
cnt stores --lat 41.1 --lon -8.6 --radius 20
```

### List categories

```bash
cnt categories
cnt categories --format json
```

## Output Formats

| Format | Flag | Description |
|--------|------|-------------|
| Table | `--format table` | Pretty tables (default) |
| JSON | `--format json` | Machine-readable JSON |
| Compact | `--format compact` | Tab-separated for piping |

```bash
# Pipe to jq
cnt search "leite" --format json | jq '.products[].name'

# Pipe to other tools
cnt search "leite" --format compact | cut -f1,4
```

## Configuration

Place at `./continente.toml` or `~/.config/continente/continente.toml`:

```toml
[http]
timeout_secs = 30
retries = 3
delay_ms = 100

[output]
format = "table"
```

## Global Options

| Flag | Description |
|------|-------------|
| `--format <FORMAT>` | Output format: table, json, compact |
| `-v, --verbose` | Enable debug logging |
| `--config <PATH>` | Config file path (env: `CONTINENTE_CONFIG`) |

## Sort Options

| Value | Description |
|-------|-------------|
| `relevance` | Default Continente ranking |
| `price-low-to-high` | Cheapest first |
| `price-high-to-low` | Most expensive first |
| `unit-price` | By price per unit (kg/lt) |
| `name-asc` | Alphabetical A-Z |
| `name-desc` | Alphabetical Z-A |

## Dietary Filters

| Flag | Description |
|------|-------------|
| `--vegan` | Vegan products |
| `--gluten-free` | Gluten-free |
| `--lactose-free` | Lactose-free |
| `--bio` | Organic/biological |

## License

MIT

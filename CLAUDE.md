# Continente CLI (cnt)

## Architecture

Single-binary Rust CLI for browsing Continente supermarket products.
Built on Salesforce Commerce Cloud (SFCC/Demandware) controller endpoints — no auth required.

### Module Structure (~2200 LOC)

- `src/main.rs` — CLI entry (clap derive) + dispatch
- `src/api/client.rs` — ContinenteClient (reqwest, HTML parsing + JSON)
- `src/api/models.rs` — SearchProduct, ProductDetail, Store, NutritionalInfo, etc.
- `src/api/scraper.rs` — HTML parsing for search results, suggestions, nutrition
- `src/commands/` — Subcommand handlers (search, product, browse, suggest, stores, categories)
- `src/format/` — Table, JSON, compact output formatting
- `src/categories.rs` — Static category tree (91 entries) with fuzzy cgid lookup
- `src/config.rs` — TOML config loading with layered defaults
- `src/error.rs` — ContinenteError (thiserror)

### Tech Stack

Rust 2024, tokio, reqwest (rustls), scraper, clap 4, serde, tracing

### Endpoints Used

| Controller | Returns | Used For |
|-----------|---------|----------|
| `Search-ShowAjax` | HTML | Product search + category browsing |
| `Product-Variation` | JSON | Product detail |
| `Product-ProductNutritionalInfoTab` | HTML | Nutrition data |
| `SearchServices-GetSuggestions` | HTML | Autocomplete |
| `Stores-FindStores` | JSON | Store locator |

Base URL: `https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/{controller}`

### Key Decisions

- No OCAPI — SFCC controllers are open, no auth needed
- Product-Variation returns JSON (best endpoint for product detail)
- Search-ShowAjax returns HTML — product data in `data-product-tile-impression` attribute as HTML-encoded JSON
- Categories are static (hardcoded from reverse engineering investigation)
- No wreq/TLS fingerprinting needed — Continente has no CDN blocking

### Build & Test

```
cargo fmt --all -- --check
cargo clippy --all-targets
cargo test
cargo test -- --ignored  # Real network tests
```

Or use the Makefile: `make all`

### Code Standards

- Error handling: anyhow in CLI (main.rs, commands), thiserror in library (error.rs)
- No unsafe, no println, no dbg!, no todo!
- Logging: tracing macros only (tracing-subscriber with env-filter)
- Tests: `tests/` directory with wiremock mocks and real HTML/JSON fixtures
- Output: all user-facing output via `format/mod.rs`, written to stdout

### Investigation Reference

Full API investigation docs in `docs/investigation.md` — includes all endpoints, security findings, auth system, category tree, and data structures discovered from reverse engineering the Android APK.

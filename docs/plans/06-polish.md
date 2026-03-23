# Plan 06: Polish

## Goal

Final touches: developer docs, user docs, release pipeline, config example. Make the project production-ready.

## Steps

### 6.1 CLAUDE.md

Developer guide for AI assistants working on this project:

```markdown
# Continente CLI (cnt)

## Architecture

Single-binary Rust CLI for browsing Continente supermarket products.
Built on Salesforce Commerce Cloud (SFCC/Demandware) controller endpoints.

### Module Structure (~X LOC)

- `src/main.rs` — CLI entry (clap derive) + dispatch
- `src/api/client.rs` — ContinenteClient (reqwest, HTML scraping + JSON)
- `src/api/models.rs` — SearchProduct, ProductDetail, Store, etc.
- `src/api/scraper.rs` — HTML parsing (scraper crate)
- `src/commands/` — Subcommand handlers (search, product, browse, suggest, stores, categories)
- `src/format/` — Table, JSON, compact output formatting
- `src/categories.rs` — Static category tree with cgid lookup
- `src/config.rs` — TOML config loading
- `src/error.rs` — ContinenteError (thiserror)

### Tech Stack

Rust 2024, tokio, reqwest (rustls), scraper, clap 4, serde, tracing

### Key Decisions

- No OCAPI: SFCC controllers are open, no auth needed
- Product-Variation returns JSON (best endpoint for detail)
- Search-ShowAjax returns HTML (parse product-tile-impression JSON attr)
- Categories are static (hardcoded from investigation)
- No wreq needed: Continente has no CDN blocking

### Build & Test

cargo fmt --all -- --check
cargo clippy --all-targets
cargo test
cargo test -- --ignored  # Network tests

### Code Standards

- Error handling: anyhow in CLI, thiserror in library
- No unsafe, no println, no dbg!, no todo!
- Logging: tracing macros only
- Tests: tests/ directory, wiremock for HTTP mocks
```

### 6.2 README.md

User-facing documentation:

- Project description
- Installation (cargo install, pre-built binaries)
- Quick start examples
- All commands with examples:
  - `cnt search "leite" --brand Mimosa --sort price-asc`
  - `cnt product 6879912 --nutrition`
  - `cnt browse frescos --max 10`
  - `cnt suggest "arroz"`
  - `cnt stores --lat 38.7 --lon -9.1 --radius 20`
  - `cnt categories`
- Output formats (table, json, compact)
- Configuration file reference
- Global flags

### 6.3 config.example.toml

```toml
# Continente CLI Configuration
# Place at ./continente.toml or ~/.config/continente/continente.toml

[http]
timeout_secs = 30
retries = 3
delay_ms = 100

[output]
format = "table"   # table, json, compact
```

### 6.4 Release workflow (`.github/workflows/release.yml`)

Same pattern as kante-kusta:
1. Format + clippy + test checks
2. Bump version in Cargo.toml
3. Build for 3 targets:
   - `aarch64-apple-darwin` (macOS ARM64)
   - `x86_64-unknown-linux-gnu` (Linux x64)
   - `aarch64-unknown-linux-gnu` (Linux ARM64)
4. Package as `.tar.gz`
5. Create GitHub release with binaries

### 6.5 Makefile

```makefile
.PHONY: check fmt clippy test build release

check:
	cargo check --all-targets

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets

test:
	cargo test

test-all:
	cargo test -- --ignored

build:
	cargo build --release

all: fmt clippy test
```

### 6.6 Final cleanup

- Verify all lints pass
- Verify all tests pass
- Verify `--help` output is clear for all commands
- Verify JSON output is valid and parseable
- Remove any leftover TODO/fixme comments
- Ensure `.gitignore` covers: `target/`, `*.apk`, `decompiled/`

## Verification

After this plan:
- `cargo fmt --check && cargo clippy && cargo test` all pass
- README has clear usage examples
- CLAUDE.md accurately describes the architecture
- Release workflow is ready (but don't trigger yet)
- `cnt --help` shows clean help text

## Files Created/Modified

| File | Action |
|------|--------|
| `CLAUDE.md` | Create |
| `README.md` | Create |
| `config.example.toml` | Create (if not already) |
| `.github/workflows/release.yml` | Create |
| `Makefile` | Create |

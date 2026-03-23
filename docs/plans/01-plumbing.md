# Plan 01: Plumbing

## Goal

Scaffold the Rust project with all infrastructure: dependencies, lints, error types, HTTP client, config, CI. No business logic yet — just the skeleton that everything else builds on.

## Steps

### 1.1 Initialize project

Create the project at the repo root (not inside `docs/` or `investigation/`).

```bash
cargo init --name continente .
```

Binary name in Cargo.toml: `cnt`

```toml
[[bin]]
name = "cnt"
path = "src/main.rs"
```

### 1.2 Cargo.toml

```toml
[package]
name = "continente"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
description = "CLI for browsing Continente supermarket products"
license = "MIT"

[[bin]]
name = "cnt"
path = "src/main.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "cookies", "gzip", "brotli", "json"] }
clap = { version = "4", features = ["derive", "env"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
scraper = "0.25"
anyhow = "1"
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
toml = "0.8"
urlencoding = "2"

[dev-dependencies]
wiremock = "0.6"
tokio-test = "0.4"
tempfile = "3"

[profile.dev]
debug = 1

[profile.dev.package."*"]
opt-level = 1

[profile.release]
lto = true
codegen-units = 1
strip = true

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
dbg_macro = "deny"
todo = "deny"
unimplemented = "deny"
print_stdout = "deny"
print_stderr = "deny"
module_name_repetitions = "allow"
must_use_candidate = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
needless_pass_by_value = "allow"
future_not_send = "allow"
cast_possible_truncation = "allow"
cast_possible_wrap = "allow"
cast_sign_loss = "allow"
format_push_string = "allow"
return_self_not_must_use = "allow"
```

### 1.3 rustfmt.toml

```toml
max_width = 100
use_field_init_shorthand = true
```

### 1.4 Source file skeleton

Create the module structure:

```
src/
├── main.rs          # Entry point: clap parse + tracing init + dispatch
├── lib.rs           # Re-exports: pub mod api, commands, format, config, error
├── error.rs         # ContinenteError enum (thiserror)
├── config.rs        # Config struct with TOML loading + defaults
├── api/
│   ├── mod.rs       # pub mod client, models, scraper
│   ├── client.rs    # ContinenteClient struct (empty impl for now)
│   └── models.rs    # Product, SearchResult, etc. (empty structs for now)
├── commands/
│   └── mod.rs       # pub mod search, product, browse, suggest, stores (empty for now)
└── format/
    └── mod.rs       # OutputFormat enum + format stubs
```

### 1.5 Error types (`src/error.rs`)

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContinenteError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse HTML from {url}: {message}")]
    Parse { url: String, message: String },

    #[error("No results found")]
    NoResults,

    #[error("Invalid configuration: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, ContinenteError>;
```

### 1.6 Config (`src/config.rs`)

Follow lauyer's pattern: TOML file with `#[serde(default)]` everywhere.

```toml
# config.example.toml
[http]
timeout_secs = 30
retries = 3
delay_ms = 100

[output]
format = "table"
```

Config search order:
1. `--config` flag (fatal if missing)
2. `./continente.toml`
3. `~/.config/continente/continente.toml`
4. Defaults

### 1.7 HTTP Client (`src/api/client.rs`)

Create `ContinenteClient` wrapping `reqwest::Client`:
- Base URL: `https://www.continente.pt`
- User-Agent: `"Mozilla/5.0 (compatible; continente-cli/0.1; +https://github.com/nikuscs/continentes)"`
- Features: gzip, brotli, rustls-tls
- Methods: placeholder `search()`, `product()`, `suggest()`, `stores()` returning `todo!()`

### 1.8 CLI skeleton (`src/main.rs`)

Clap derive with:
- Global flags: `--format`, `--verbose`, `--config`
- Subcommands enum: `Search`, `Product`, `Browse`, `Suggest`, `Stores` (all stubs)
- Tracing init (same pattern as kante-kusta)
- Config loading
- Client creation
- Command dispatch (match on subcommand, call stub functions)

### 1.9 CI workflow (`.github/workflows/ci.yml`)

Same as kante-kusta:
1. `cargo check --all-targets`
2. `cargo fmt --all -- --check`
3. `cargo clippy --all-targets`
4. `cargo test`

### 1.10 .gitignore update

Add to existing .gitignore:
```
target/
```

## Verification

After this plan:
- `cargo check` passes
- `cargo clippy` passes
- `cargo fmt --check` passes
- `cnt --help` prints usage
- `cnt search --help` prints search usage (but command itself returns an error/todo message)

## Files Created/Modified

| File | Action |
|------|--------|
| `Cargo.toml` | Create |
| `Cargo.lock` | Generated |
| `rustfmt.toml` | Create |
| `config.example.toml` | Create |
| `.gitignore` | Modify (add target/) |
| `.github/workflows/ci.yml` | Create |
| `src/main.rs` | Create |
| `src/lib.rs` | Create |
| `src/error.rs` | Create |
| `src/config.rs` | Create |
| `src/api/mod.rs` | Create |
| `src/api/client.rs` | Create |
| `src/api/models.rs` | Create |
| `src/commands/mod.rs` | Create |
| `src/format/mod.rs` | Create |

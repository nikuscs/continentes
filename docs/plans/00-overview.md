# Continente CLI - Implementation Plans

## Overview

Build `continente` (binary: `cnt`) — a Rust CLI for browsing Continente supermarket products.
Follows the same architecture as `kante-kusta` (kk) and `lauyer`.

## Plan Order (execute sequentially)

| Plan | Name | Description | Depends On |
|------|------|-------------|------------|
| 01 | Plumbing | Project scaffolding, deps, lints, CI, config, error types, HTTP client | — |
| 02 | Models & API Client | Data models, API client with all endpoints | 01 |
| 03 | Commands | CLI subcommands (search, product, browse, suggest, stores) | 02 |
| 04 | Output Formatting | Table, JSON, compact output for all data types | 03 |
| 05 | Tests | Integration tests with wiremock, CLI tests | 04 |
| 06 | Polish | CLAUDE.md, README, config.example.toml, release workflow | 05 |

## Conventions (from existing projects)

- **Edition**: 2024, MSRV 1.85
- **Error handling**: `anyhow` in CLI, `thiserror` in library code
- **HTTP**: `reqwest` with rustls-tls (no need for wreq — Continente has no CDN blocking)
- **Logging**: `tracing` + `tracing-subscriber` with env-filter, no println
- **Lints**: clippy pedantic+nursery, forbid unsafe, deny dbg/todo/print_stdout
- **Testing**: wiremock for HTTP mocks, tests in `tests/` directory
- **Release**: LTO, strip, 1 codegen unit
- **Output**: stdout only via format module, never direct println

## Key Reference Files

- **API docs**: `docs/investigation.md` (this repo)
- **Template project**: `~/projects/kante-kusta/` (closest match)
- **Advanced patterns**: `~/projects/lauyer/` (HttpFetcher trait, config layering)

# Plan 05: Tests

## Goal

Comprehensive test coverage following lauyer's patterns: integration tests in `tests/`, wiremock for HTTP mocking, no `#[cfg(test)]` in source files.

## Steps

### 5.1 Test fixtures (`tests/fixtures/`)

Create HTML/JSON fixture files from real Continente responses:

```
tests/
├── fixtures/
│   ├── search_leite.html          # Search-ShowAjax response for "leite" (trimmed to 2 products)
│   ├── product_6879912.json       # Product-Variation JSON for milk
│   ├── suggestions_leite.html     # SearchServices-GetSuggestions for "leite"
│   ├── stores.json                # Stores-FindStores JSON (3 stores)
│   └── nutrition_6879912.html     # Product-ProductNutritionalInfoTab HTML
```

Save real responses but trim to minimal size (2-3 items each). Strip tracking/ad HTML from search fixtures.

### 5.2 Scraper tests (`tests/scraper_test.rs`)

Test HTML parsing independently:

```rust
#[test]
fn parse_search_results_extracts_products() {
    let html = include_str!("fixtures/search_leite.html");
    let result = parse_search_results(html);
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.total, 1314);
    assert!(!response.products.is_empty());

    let product = &response.products[0];
    assert_eq!(product.id, "6879912");
    assert_eq!(product.name, "Leite UHT Meio Gordo Continente");
    assert!((product.price - 0.86).abs() < f64::EPSILON);
    assert_eq!(product.brand, "Continente");
}

#[test]
fn parse_search_results_extracts_images() { ... }

#[test]
fn parse_search_results_extracts_total_count() { ... }

#[test]
fn parse_search_results_empty_html_returns_no_results() { ... }

#[test]
fn parse_suggestions_extracts_all_sections() { ... }

#[test]
fn parse_nutritional_info_extracts_nutrients() { ... }
```

### 5.3 Client tests (`tests/client_test.rs`)

Use wiremock to mock Continente endpoints:

```rust
#[tokio::test]
async fn search_returns_products() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/on/demandware.store/Sites-continente-Site/default/Search-ShowAjax"))
        .and(query_param("q", "leite"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/search_leite.html"))
        )
        .mount(&mock_server)
        .await;

    let client = ContinenteClient::with_base_url(&mock_server.uri());
    let result = client.search("leite", &SearchParams::default()).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.products.is_empty());
}

#[tokio::test]
async fn product_returns_detail_from_json() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/on/demandware.store/Sites-continente-Site/default/Product-Variation"))
        .and(query_param("pid", "6879912"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/product_6879912.json"))
        )
        .mount(&mock_server)
        .await;

    let client = ContinenteClient::with_base_url(&mock_server.uri());
    let result = client.product("6879912").await;

    assert!(result.is_ok());
    let product = result.unwrap();
    assert_eq!(product.id, "6879912");
    assert_eq!(product.brand, "Continente");
}

#[tokio::test]
async fn stores_returns_json_array() { ... }

#[tokio::test]
async fn suggest_requires_min_5_chars() { ... }

#[tokio::test]
async fn search_with_brand_filter_adds_params() { ... }

#[tokio::test]
async fn search_with_sort_adds_srule() { ... }

#[tokio::test]
async fn client_handles_server_error() { ... }

#[tokio::test]
async fn client_handles_empty_results() { ... }
```

### 5.4 Model tests (`tests/models_test.rs`)

```rust
#[test]
fn deserialize_product_variation_json() {
    let json = include_str!("fixtures/product_6879912.json");
    let detail: ProductDetail = serde_json::from_str(json).unwrap();
    assert_eq!(detail.id, "6879912");
    assert!((detail.price.sales_value - 0.86).abs() < f64::EPSILON);
}

#[test]
fn extract_ean_from_nutritional_url() {
    let url = "Product-ProductNutritionalInfoTab?pid=6879912&ean=5601312508007&supplierid=5600000000403";
    let ean = extract_ean(url);
    assert_eq!(ean, Some("5601312508007".to_string()));
}

#[test]
fn sort_rule_to_string() {
    assert_eq!(SortRule::PriceLowToHigh.as_str(), "price-low-to-high");
}

#[test]
fn search_params_default_values() {
    let params = SearchParams::default();
    assert_eq!(params.start, 0);
    assert_eq!(params.size, 24);
}
```

### 5.5 Format tests (`tests/format_test.rs`)

```rust
#[test]
fn format_products_table_truncates_long_names() { ... }

#[test]
fn format_products_json_is_valid() { ... }

#[test]
fn format_products_compact_is_tab_separated() { ... }

#[test]
fn format_product_detail_shows_promotion() { ... }

#[test]
fn format_stores_table_shows_pickup() { ... }

#[test]
fn format_categories_tree() { ... }
```

### 5.6 Category tests (`tests/categories_test.rs`)

```rust
#[test]
fn resolve_cgid_exact_match() {
    assert_eq!(resolve_cgid("laticinios"), Some("laticinios"));
}

#[test]
fn resolve_cgid_by_name() {
    assert_eq!(resolve_cgid("Frescos"), Some("frescos"));
}

#[test]
fn resolve_cgid_unknown_returns_none() {
    assert_eq!(resolve_cgid("nonexistent"), None);
}

#[test]
fn all_categories_is_not_empty() {
    assert!(all_categories().len() > 100);
}
```

### 5.7 Config tests (`tests/config_test.rs`)

```rust
#[test]
fn config_default_values() { ... }

#[test]
fn config_from_toml() { ... }

#[test]
fn config_missing_file_uses_defaults() { ... }
```

### 5.8 Integration tests with real network (`tests/integration_test.rs`)

Mark with `#[ignore]` — only run manually:

```rust
#[tokio::test]
#[ignore]
async fn real_search_returns_results() {
    let client = ContinenteClient::new();
    let result = client.search("leite", &SearchParams::default()).await;
    assert!(result.is_ok());
    assert!(!result.unwrap().products.is_empty());
}

#[tokio::test]
#[ignore]
async fn real_product_returns_detail() {
    let client = ContinenteClient::new();
    let result = client.product("6879912").await;
    assert!(result.is_ok());
}
```

## Verification

After this plan:
- `cargo test` passes (all unit + mock tests)
- `cargo test -- --ignored` passes (real network tests)
- Good coverage of: scraper parsing, client methods, model deserialization, formatting, categories

## Files Created/Modified

| File | Action |
|------|--------|
| `tests/fixtures/search_leite.html` | Create (trimmed fixture) |
| `tests/fixtures/product_6879912.json` | Create (real JSON) |
| `tests/fixtures/suggestions_leite.html` | Create (trimmed fixture) |
| `tests/fixtures/stores.json` | Create (3 stores) |
| `tests/fixtures/nutrition_6879912.html` | Create (trimmed fixture) |
| `tests/scraper_test.rs` | Create |
| `tests/client_test.rs` | Create |
| `tests/models_test.rs` | Create |
| `tests/format_test.rs` | Create |
| `tests/categories_test.rs` | Create |
| `tests/config_test.rs` | Create |
| `tests/integration_test.rs` | Create |

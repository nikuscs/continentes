# Plan 02: Models & API Client

## Goal

Implement all data models and the API client with real endpoint calls. After this plan, all data fetching works — commands and formatting come next.

## Reference

All endpoints documented in `docs/investigation.md` sections 4-7.

## Steps

### 2.1 Product models (`src/api/models.rs`)

#### From Search (HTML scraping)

```rust
/// Extracted from `product-tile-impression` JSON attribute in Search-ShowAjax HTML
pub struct SearchProduct {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub category: String,   // "Laticínios e Ovos/Leite/Leite Meio Gordo"
    pub variant: String,
    pub channel: String,
    // Extracted separately from HTML:
    pub image_url: Option<String>,
    pub unit_price: Option<String>,  // "0,86€/lt"
}
```

#### Search response wrapper

```rust
pub struct SearchResponse {
    pub products: Vec<SearchProduct>,
    pub total: u32,
    pub query: String,
}
```

#### From Product-Variation (JSON endpoint)

```rust
/// Full product detail from Product-Variation JSON
pub struct ProductDetail {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub brand: String,
    pub product_type: String,
    pub short_description: Option<String>,
    pub rating: Option<f64>,
    pub available: bool,
    pub online: bool,
    pub product_url: String,
    pub price: PriceInfo,
    pub price_per_unit: Option<PricePerUnit>,
    pub measurement_note: Option<String>,  // "emb. 1 lt"
    pub min_order_quantity: u32,
    pub max_order_quantity: u32,
    pub category: CategoryInfo,
    pub badge_info: BadgeInfo,
    pub images: ProductImages,
    pub nutritional_info_url: Option<String>,
    pub ean: Option<String>,              // Extracted from nutritional_info_url
}

pub struct PriceInfo {
    pub sales_value: f64,
    pub sales_formatted: String,
    pub list_value: Option<f64>,          // Original price (when on promotion)
    pub currency: String,
    pub promotion_end: Option<String>,
}

pub struct PricePerUnit {
    pub primary_price: f64,
    pub primary_unit: String,
    pub secondary_price: Option<String>,  // "0,86€/lt"
    pub secondary_unit: Option<String>,
}

pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub top_level_id: String,
    pub top_level_name: String,
    pub gtm_path: String,                 // "Laticínios e Ovos/Leite/Leite Meio Gordo"
}

pub struct BadgeInfo {
    pub general_title: Option<String>,    // "Produzido em Portugal"
    pub promo_title: Option<String>,      // "PVP Recomendado: 1,00€/un"
}

pub struct ProductImages {
    pub tile: Option<String>,             // 280x280
    pub quick_view: Vec<String>,          // 1000x1000
    pub full: Vec<String>,               // 2000x2000
}
```

#### Suggestion models

```rust
pub struct SuggestionResult {
    pub products: Vec<SearchProduct>,     // Top 3 product suggestions
    pub categories: Vec<CategorySuggestion>,
    pub popular_terms: Vec<String>,
}

pub struct CategorySuggestion {
    pub name: String,
    pub url: String,
}
```

#### Store models

```rust
/// From Stores-FindStores JSON response
pub struct Store {
    pub id: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub postal_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: Option<String>,
    pub store_hours: Option<String>,
    pub is_pickup_store: bool,
}
```

#### Nutritional info (optional, from HTML)

```rust
pub struct NutritionalInfo {
    pub regulated_name: Option<String>,
    pub ingredients: Option<String>,
    pub allergens: Option<String>,
    pub country_of_origin: Option<String>,
    pub storage_instructions: Option<String>,
    pub net_content: Option<String>,
    pub producer_name: Option<String>,
    pub nutrients: Vec<Nutrient>,
}

pub struct Nutrient {
    pub name: String,
    pub value: f64,
    pub unit: String,
}
```

All models derive: `Debug, Clone, serde::Serialize, serde::Deserialize`
Use `#[serde(rename_all = "camelCase")]` where the JSON uses camelCase.
Use `#[serde(default)]` for optional fields.

### 2.2 HTML scraper (`src/api/scraper.rs`)

Create a new file `src/api/scraper.rs` for HTML parsing logic.

**Search results parser:**
1. Input: raw HTML string from `Search-ShowAjax`
2. Parse with `scraper::Html::parse_document()`
3. Select elements with `product-tile-impression` attribute
4. HTML-decode the attribute value, parse as JSON → `SearchProduct`
5. Extract `data-gtm-results` attribute for total count
6. Extract image URLs from `data-src` on `img` elements within product tiles
7. Extract unit price from `.pwc-tile--price-secondary`
8. Return `SearchResponse`

**Suggestion parser:**
1. Input: raw HTML from `SearchServices-GetSuggestions`
2. Parse product suggestions (same `product-tile-impression` pattern)
3. Parse category links
4. Parse popular terms
5. Return `SuggestionResult`

**Nutritional info parser:**
1. Input: raw HTML from `Product-ProductNutritionalInfoTab`
2. Extract fields by CSS class (`.ingredients`, `.allergen-statement`, etc.)
3. Parse nutrients table rows
4. Return `NutritionalInfo`

### 2.3 API Client (`src/api/client.rs`)

Implement `ContinenteClient` with these methods:

```rust
impl ContinenteClient {
    pub fn new() -> Self { ... }
    pub fn with_base_url(base_url: &str) -> Self { ... }  // For testing

    /// Search products via Search-ShowAjax (HTML → scraper)
    pub async fn search(&self, query: &str, params: &SearchParams) -> Result<SearchResponse>

    /// Get product detail via Product-Variation (JSON)
    pub async fn product(&self, pid: &str) -> Result<ProductDetail>

    /// Get nutritional info via Product-ProductNutritionalInfoTab (HTML → scraper)
    pub async fn nutrition(&self, pid: &str, ean: &str, supplier_id: &str) -> Result<NutritionalInfo>

    /// Search suggestions via SearchServices-GetSuggestions (HTML → scraper)
    pub async fn suggest(&self, query: &str) -> Result<SuggestionResult>

    /// Find stores via Stores-FindStores (JSON)
    pub async fn stores(&self, lat: f64, lon: f64, radius: u32) -> Result<Vec<Store>>

    /// Browse category via Search-ShowAjax with cgid (HTML → scraper)
    pub async fn browse(&self, cgid: &str, params: &SearchParams) -> Result<SearchResponse>
}
```

**SearchParams struct:**
```rust
pub struct SearchParams {
    pub start: u32,           // Pagination offset (default: 0)
    pub size: u32,            // Page size (default: 24)
    pub sort: Option<SortRule>,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub brand: Option<String>,
    pub filters: Vec<(String, String)>,  // prefn1/prefv1 pairs
}

pub enum SortRule {
    Relevance,       // "Continente" (default)
    PriceLowToHigh,  // "price-low-to-high"
    PriceHighToLow,  // "price-high-to-low"
    UnitPrice,       // "price-per-capacity-ascending"
    NameAsc,         // "product-name-ascending"
    NameDesc,        // "product-name-descending"
}
```

**Product-Variation JSON deserialization:**
The JSON from `Product-Variation?pid={id}` has a nested structure:
- `product.id`, `product.productName`, `product.brand`, etc.
- Need intermediate serde structs to match the nested JSON
- Extract EAN from `product.nutritionalInfoUrlString` via regex or URL parsing

### 2.4 Module exports

Update `src/api/mod.rs`:
```rust
pub mod client;
pub mod models;
mod scraper;  // private — only used by client
```

Update `src/lib.rs`:
```rust
pub mod api;
pub mod commands;
pub mod config;
pub mod error;
pub mod format;
```

## Verification

After this plan:
- All models compile and derive correctly
- `ContinenteClient::search("leite", &default_params).await` returns real products
- `ContinenteClient::product("6879912").await` returns full product JSON
- `ContinenteClient::suggest("leite").await` returns suggestions
- `ContinenteClient::stores(38.7, -9.1, 50).await` returns stores
- Can be verified with a simple test binary or `#[ignore]` integration test

## Files Created/Modified

| File | Action |
|------|--------|
| `src/api/models.rs` | Rewrite with all models |
| `src/api/client.rs` | Implement all methods |
| `src/api/scraper.rs` | Create (HTML parsing) |
| `src/api/mod.rs` | Update exports |
| `src/lib.rs` | Update exports |

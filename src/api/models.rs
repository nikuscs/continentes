use serde::{Deserialize, Serialize};

// --- Search (from HTML scraping) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProduct {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub brand: String,
    pub category: String,
    #[serde(default)]
    pub variant: String,
    #[serde(default)]
    pub channel: String,
    #[serde(skip_deserializing)]
    pub image_url: Option<String>,
    #[serde(skip_deserializing)]
    pub unit_price: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResponse {
    pub products: Vec<SearchProduct>,
    pub total: u32,
    pub query: String,
}

// --- Product Detail (from Product-Variation JSON) ---

#[derive(Debug, Clone, Serialize)]
pub struct ProductDetail {
    pub id: String,
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
    pub measurement_note: Option<String>,
    pub min_order_quantity: u32,
    pub max_order_quantity: u32,
    pub category: CategoryInfo,
    pub badge_info: BadgeInfo,
    pub images: ProductImages,
    pub nutritional_info_url: Option<String>,
    pub ean: Option<String>,
    pub supplier_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PriceInfo {
    pub sales_value: f64,
    pub sales_formatted: String,
    pub list_value: Option<f64>,
    pub currency: String,
    pub promotion_end: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PricePerUnit {
    pub primary_value: f64,
    pub primary_unit: String,
    pub secondary_formatted: Option<String>,
    pub secondary_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub top_level_id: String,
    pub top_level_name: String,
    pub gtm_path: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct BadgeInfo {
    pub general_title: Option<String>,
    pub promo_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ProductImages {
    pub tile: Option<String>,
    pub quick_view: Vec<String>,
    pub full: Vec<String>,
}

// --- Serde intermediate structs for Product-Variation JSON ---

#[derive(Debug, Deserialize)]
pub struct ProductVariationResponse {
    pub product: RawProduct,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RawProduct {
    pub id: String,
    #[serde(default)]
    pub product_name: String,
    #[serde(default)]
    pub product_type: String,
    #[serde(default)]
    pub brand: String,
    pub short_description: Option<String>,
    pub rating: Option<f64>,
    #[serde(default)]
    pub available: bool,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub ready_to_order: bool,
    #[serde(default, alias = "productURL")]
    pub product_url: Option<String>,
    #[serde(default)]
    pub measurement_note: Option<String>,
    #[serde(default)]
    pub min_order_quantity: u32,
    #[serde(default)]
    pub max_order_quantity: u32,
    #[serde(default)]
    pub gtm_category_path: Option<String>,
    pub nutritional_info_url_string: Option<String>,
    pub price: Option<RawPrice>,
    pub price_per_unit: Option<RawPricePerUnit>,
    pub category: Option<RawCategory>,
    pub badge_info: Option<RawBadgeInfo>,
    pub product_tile_image: Option<RawImage>,
    #[serde(default)]
    pub quick_view_images: Vec<RawImageEntry>,
    #[serde(default)]
    pub pdp_images: Vec<RawImageEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RawPrice {
    pub sales: Option<RawSales>,
    pub list: Option<RawSales>,
    #[serde(alias = "onlineTo")]
    pub online_to: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSales {
    pub value: f64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub formatted: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawPricePerUnit {
    pub primary_price: Option<RawPrimaryPrice>,
    pub secondary_price: Option<RawSecondaryPrice>,
}

#[derive(Debug, Deserialize)]
pub struct RawPrimaryPrice {
    pub price: Option<RawSales>,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RawSecondaryPrice {
    pub price: Option<serde_json::Value>,
    pub unit: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct RawCategory {
    #[serde(default)]
    pub primary_category_id: String,
    #[serde(default)]
    pub primary_category_display_name: String,
    #[serde(default)]
    pub primary_category_top_level_product_category_id: String,
    #[serde(default)]
    pub primary_category_top_level_product_category_display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawBadgeInfo {
    pub general: Option<RawBadge>,
    pub promo: Option<RawBadge>,
}

#[derive(Debug, Deserialize)]
pub struct RawBadge {
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawImage {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawImageEntry {
    pub url: Option<String>,
}

impl RawProduct {
    pub fn into_detail(self) -> ProductDetail {
        let (ean, supplier_id) = extract_ean_supplier(self.nutritional_info_url_string.as_deref());

        let price = self.price.map_or_else(
            || PriceInfo {
                sales_value: 0.0,
                sales_formatted: String::new(),
                list_value: None,
                currency: String::from("EUR"),
                promotion_end: None,
            },
            |p| PriceInfo {
                sales_value: p.sales.as_ref().map_or(0.0, |s| s.value),
                sales_formatted: p
                    .sales
                    .as_ref()
                    .map_or_else(String::new, |s| s.formatted.clone()),
                list_value: p.list.as_ref().map(|l| l.value),
                currency: p
                    .sales
                    .as_ref()
                    .map_or_else(|| String::from("EUR"), |s| s.currency.clone()),
                promotion_end: p.online_to,
            },
        );

        let price_per_unit = self.price_per_unit.and_then(|ppu| {
            let primary = ppu.primary_price?;
            Some(PricePerUnit {
                primary_value: primary.price.as_ref().map_or(0.0, |p| p.value),
                primary_unit: primary.unit.unwrap_or_default(),
                secondary_formatted: ppu
                    .secondary_price
                    .as_ref()
                    .and_then(|s| s.price.as_ref().and_then(|v| v.as_str().map(String::from))),
                secondary_unit: ppu.secondary_price.and_then(|s| s.unit),
            })
        });

        let category = self.category.map_or_else(
            || CategoryInfo {
                id: String::new(),
                name: String::new(),
                top_level_id: String::new(),
                top_level_name: String::new(),
                gtm_path: self.gtm_category_path.clone().unwrap_or_default(),
            },
            |c| CategoryInfo {
                id: c.primary_category_id,
                name: c.primary_category_display_name,
                top_level_id: c.primary_category_top_level_product_category_id,
                top_level_name: c.primary_category_top_level_product_category_display_name,
                gtm_path: self.gtm_category_path.clone().unwrap_or_default(),
            },
        );

        let badge_info = self
            .badge_info
            .map_or_else(BadgeInfo::default, |b| BadgeInfo {
                general_title: b.general.and_then(|g| g.title),
                promo_title: b.promo.and_then(|p| p.title),
            });

        let images = ProductImages {
            tile: self.product_tile_image.and_then(|i| i.url),
            quick_view: self
                .quick_view_images
                .into_iter()
                .filter_map(|i| i.url)
                .collect(),
            full: self.pdp_images.into_iter().filter_map(|i| i.url).collect(),
        };

        ProductDetail {
            id: self.id,
            name: self.product_name,
            brand: self.brand,
            product_type: self.product_type,
            short_description: self.short_description,
            rating: self.rating,
            available: self.available,
            online: self.online,
            product_url: self.product_url.unwrap_or_default(),
            price,
            price_per_unit,
            measurement_note: self.measurement_note,
            min_order_quantity: self.min_order_quantity,
            max_order_quantity: self.max_order_quantity,
            category,
            badge_info,
            images,
            nutritional_info_url: self.nutritional_info_url_string,
            ean,
            supplier_id,
        }
    }
}

fn extract_ean_supplier(url: Option<&str>) -> (Option<String>, Option<String>) {
    let Some(url) = url else {
        return (None, None);
    };
    let ean = url
        .split("ean=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .map(String::from);
    let supplier = url
        .split("supplierid=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .map(String::from);
    (ean, supplier)
}

// --- Suggestions ---

#[derive(Debug, Clone, Serialize)]
pub struct SuggestionResult {
    pub products: Vec<SearchProduct>,
    pub categories: Vec<CategorySuggestion>,
    pub popular_terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategorySuggestion {
    pub name: String,
    pub url: String,
}

// --- Stores ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Store {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, alias = "address1")]
    pub address: String,
    #[serde(default)]
    pub city: String,
    #[serde(default, alias = "postalCode")]
    pub postal_code: String,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default, alias = "storeHours")]
    pub store_hours: Option<String>,
    #[serde(default, alias = "isPickupStore")]
    pub is_pickup_store: bool,
}

#[derive(Debug, Deserialize)]
pub struct StoresResponse {
    #[serde(default)]
    pub stores: Vec<Store>,
}

// --- Nutritional Info ---

#[derive(Debug, Clone, Serialize, Default)]
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

#[derive(Debug, Clone, Serialize)]
pub struct Nutrient {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

// --- Sort & Search Params ---

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum SortRule {
    #[default]
    Relevance,
    PriceLowToHigh,
    PriceHighToLow,
    UnitPrice,
    NameAsc,
    NameDesc,
}

impl SortRule {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Relevance => "Continente",
            Self::PriceLowToHigh => "price-low-to-high",
            Self::PriceHighToLow => "price-high-to-low",
            Self::UnitPrice => "price-per-capacity-ascending",
            Self::NameAsc => "product-name-ascending",
            Self::NameDesc => "product-name-descending",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub start: u32,
    pub size: u32,
    pub sort: Option<SortRule>,
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub brand: Option<String>,
    pub filters: Vec<(String, String)>,
}

impl SearchParams {
    pub fn new() -> Self {
        Self {
            size: 24,
            ..Default::default()
        }
    }
}

use std::fmt::{self, Write};
use std::str::FromStr;

use crate::api::models::{NutritionalInfo, ProductDetail, SearchResponse, Store, SuggestionResult};
use crate::categories::Category;

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    clap::ValueEnum,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Table => f.write_str("table"),
            Self::Json => f.write_str("json"),
            Self::Compact => f.write_str("compact"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "compact" => Ok(Self::Compact),
            other => Err(format!("Unknown output format: '{other}'")),
        }
    }
}

// --- Search Products ---

pub fn format_products(
    response: &SearchResponse,
    page: u32,
    size: u32,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Table => format_products_table(response, page, size),
        OutputFormat::Json => serde_json::to_string_pretty(response).unwrap_or_default(),
        OutputFormat::Compact => format_products_compact(response),
    }
}

fn format_products_table(response: &SearchResponse, page: u32, size: u32) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Found {} products for \"{}\":\n",
        response.total, response.query
    );
    let _ = writeln!(
        out,
        "{:<10} {:<48} {:>7} {:<12} Brand",
        "ID", "Name", "Price", "/Unit"
    );
    let _ = writeln!(out, "{}", "-".repeat(95));

    for p in &response.products {
        let unit = p.unit_price.as_deref().unwrap_or("");
        let _ = writeln!(
            out,
            "{:<10} {:<48} {:>6.2}€ {:<12} {}",
            p.id,
            truncate(&p.name, 47),
            p.price,
            unit,
            truncate(&p.brand, 15)
        );
    }

    let total_pages = response.total.div_ceil(size);
    let shown = response.products.len() as u32;
    let _ = writeln!(
        out,
        "\nPage {page}/{total_pages} ({shown} of {} results)",
        response.total
    );
    out
}

fn format_products_compact(response: &SearchResponse) -> String {
    let mut out = String::new();
    for p in &response.products {
        let _ = writeln!(out, "{}\t{:.2}\t{}\t{}", p.id, p.price, p.brand, p.name);
    }
    out
}

// --- Product Detail ---

pub fn format_product_detail(
    product: &ProductDetail,
    nutrition: Option<&NutritionalInfo>,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Table => format_product_table(product, nutrition),
        OutputFormat::Json => nutrition.map_or_else(
            || serde_json::to_string_pretty(product).unwrap_or_default(),
            |info| {
                #[derive(serde::Serialize)]
                struct Combined<'a> {
                    product: &'a ProductDetail,
                    nutrition: &'a NutritionalInfo,
                }
                serde_json::to_string_pretty(&Combined {
                    product,
                    nutrition: info,
                })
                .unwrap_or_default()
            },
        ),
        OutputFormat::Compact => {
            format!(
                "{}\t{:.2}\t{}\t{}",
                product.id, product.price.sales_value, product.brand, product.name
            )
        }
    }
}

fn format_product_table(product: &ProductDetail, nutrition: Option<&NutritionalInfo>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}", product.name);
    let _ = writeln!(out, "{}\n", "=".repeat(product.name.len()));

    let _ = writeln!(out, "{:<14}{}", "ID:", product.id);
    let _ = writeln!(out, "{:<14}{}", "Brand:", product.brand);

    // Price with promotion info
    if let Some(list) = product.price.list_value {
        let discount = ((1.0 - product.price.sales_value / list) * 100.0) as i32;
        let _ = writeln!(
            out,
            "{:<14}{} (was {:.2}€, -{}%)",
            "Price:", product.price.sales_formatted, list, discount
        );
    } else {
        let _ = writeln!(out, "{:<14}{}", "Price:", product.price.sales_formatted);
    }

    if let Some(ppu) = &product.price_per_unit {
        if let Some(secondary) = &ppu.secondary_formatted {
            let unit = ppu.secondary_unit.as_deref().unwrap_or("");
            let _ = writeln!(out, "{:<14}{}/{}", "Unit Price:", secondary, unit);
        }
    }

    if let Some(note) = &product.measurement_note {
        let _ = writeln!(out, "{:<14}{}", "Package:", note);
    }

    if let Some(rating) = product.rating {
        let _ = writeln!(out, "{:<14}{:.1}", "Rating:", rating);
    }

    let category_path = product.category.gtm_path.replace('/', " > ");
    let _ = writeln!(out, "{:<14}{}", "Category:", category_path);
    let _ = writeln!(
        out,
        "{:<14}{}",
        "Available:",
        if product.available { "Yes" } else { "No" }
    );

    if let Some(ean) = &product.ean {
        let _ = writeln!(out, "{:<14}{}", "EAN:", ean);
    }

    if !product.product_url.is_empty() {
        let _ = writeln!(out, "{:<14}{}", "URL:", product.product_url);
    }

    if let Some(badge) = &product.badge_info.general_title {
        let _ = writeln!(out, "{:<14}{}", "Badge:", badge);
    }

    if let Some(desc) = &product.short_description {
        let _ = writeln!(out, "\n{desc}");
    }

    if let Some(info) = nutrition {
        let _ = writeln!(out);
        out.push_str(&format_nutrition(info));
    }

    out
}

fn format_nutrition(info: &NutritionalInfo) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Nutritional Info:");
    let _ = writeln!(out, "{}", "-".repeat(40));

    if let Some(name) = &info.regulated_name {
        let _ = writeln!(out, "{:<16}{}", "Name:", name);
    }
    if let Some(ingredients) = &info.ingredients {
        let _ = writeln!(out, "{:<16}{}", "Ingredients:", truncate(ingredients, 70));
    }
    if let Some(allergens) = &info.allergens {
        let _ = writeln!(out, "{:<16}{}", "Allergens:", allergens);
    }
    if let Some(origin) = &info.country_of_origin {
        let _ = writeln!(out, "{:<16}{}", "Origin:", origin);
    }
    if let Some(storage) = &info.storage_instructions {
        let _ = writeln!(out, "{:<16}{}", "Storage:", truncate(storage, 60));
    }
    if let Some(producer) = &info.producer_name {
        let _ = writeln!(out, "{:<16}{}", "Producer:", producer);
    }

    if !info.nutrients.is_empty() {
        let _ = writeln!(out);
        for n in &info.nutrients {
            let _ = writeln!(out, "  {:<24}{:.1} {}", n.name, n.value, n.unit);
        }
    }

    out
}

// --- Suggestions ---

pub fn format_suggestions(result: &SuggestionResult, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(result).unwrap_or_default(),
        OutputFormat::Table => format_suggestions_table(result),
        OutputFormat::Compact => {
            let mut out = String::new();
            for p in &result.products {
                let _ = writeln!(out, "{}\t{:.2}\t{}\t{}", p.id, p.price, p.brand, p.name);
            }
            out
        }
    }
}

fn format_suggestions_table(result: &SuggestionResult) -> String {
    let mut out = String::new();

    if !result.products.is_empty() {
        let _ = writeln!(out, "Products:");
        for p in &result.products {
            let _ = writeln!(out, "  {:<10} {:>6.2}€  {}", p.id, p.price, p.name);
        }
    }

    if !result.categories.is_empty() {
        let _ = writeln!(out, "\nCategories:");
        for c in &result.categories {
            let _ = writeln!(out, "  {} ({})", c.name, c.url);
        }
    }

    if !result.popular_terms.is_empty() {
        let _ = writeln!(out, "\nPopular searches:");
        for t in &result.popular_terms {
            let _ = writeln!(out, "  {t}");
        }
    }

    out
}

// --- Stores ---

pub fn format_stores(stores: &[Store], radius: u32, format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(stores).unwrap_or_default(),
        OutputFormat::Table => format_stores_table(stores, radius),
        OutputFormat::Compact => {
            let mut out = String::new();
            for s in stores {
                let _ = writeln!(out, "{}\t{}\t{}\t{}", s.id, s.name, s.city, s.address);
            }
            out
        }
    }
}

fn format_stores_table(stores: &[Store], radius: u32) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Found {} stores within {}km:\n", stores.len(), radius);
    let _ = writeln!(
        out,
        "{:<34} {:<40} {:<12} Pickup",
        "Name", "Address", "City"
    );
    let _ = writeln!(out, "{}", "-".repeat(95));

    for s in stores {
        let _ = writeln!(
            out,
            "{:<34} {:<40} {:<12} {}",
            truncate(&s.name, 33),
            truncate(&s.address, 39),
            truncate(&s.city, 11),
            if s.is_pickup_store { "Yes" } else { "No" }
        );
    }
    out
}

// --- Categories ---

pub fn format_categories(categories: &[Category], format: OutputFormat) -> String {
    match format {
        OutputFormat::Json => {
            #[derive(serde::Serialize)]
            struct Cat<'a> {
                cgid: &'a str,
                name: &'a str,
                parent: Option<&'a str>,
            }
            let cats: Vec<Cat> = categories
                .iter()
                .map(|c| Cat {
                    cgid: c.cgid,
                    name: c.name,
                    parent: c.parent,
                })
                .collect();
            serde_json::to_string_pretty(&cats).unwrap_or_default()
        }
        OutputFormat::Table => format_categories_tree(categories),
        OutputFormat::Compact => {
            let mut out = String::new();
            for c in categories {
                let _ = writeln!(out, "{}\t{}", c.cgid, c.name);
            }
            out
        }
    }
}

fn format_categories_tree(categories: &[Category]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Categories:\n");

    // Group: top-level categories have no parent
    let top_level: Vec<_> = categories.iter().filter(|c| c.parent.is_none()).collect();

    for top in &top_level {
        let _ = writeln!(out, "{} ({})", top.name, top.cgid);

        let children: Vec<_> = categories
            .iter()
            .filter(|c| c.parent == Some(top.cgid))
            .collect();

        for (i, child) in children.iter().enumerate() {
            let connector = if i == children.len() - 1 {
                "└──"
            } else {
                "├──"
            };
            let _ = writeln!(out, " {connector} {} ({})", child.name, child.cgid);
        }
        let _ = writeln!(out);
    }

    out
}

// --- Helpers ---

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        format!(
            "{}…",
            s.chars().take(max.saturating_sub(1)).collect::<String>()
        )
    } else {
        s.to_string()
    }
}

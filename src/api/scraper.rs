use scraper::{Html, Selector};

use crate::api::models::{
    CategorySuggestion, Flyer, Nutrient, NutritionalInfo, SearchProduct, SearchResponse,
    SuggestionResult,
};
use crate::error::{ContinenteError, Result};

pub fn parse_search_results(html: &str, query: &str) -> Result<SearchResponse> {
    let document = Html::parse_document(html);

    let total = extract_total_count(&document);
    let products = extract_products(&document);

    if products.is_empty() && total == 0 {
        return Err(ContinenteError::NoResults);
    }

    Ok(SearchResponse {
        products,
        total,
        query: query.to_string(),
    })
}

fn extract_total_count(document: &Html) -> u32 {
    // Search results use data-gtm-results, browse uses data-total-count on .grid-footer
    let gtm_selector = Selector::parse("[data-gtm-results]").expect("valid selector");
    if let Some(count) = document
        .select(&gtm_selector)
        .next()
        .and_then(|el| el.value().attr("data-gtm-results"))
        .and_then(|v| v.parse().ok())
    {
        return count;
    }

    let footer_selector = Selector::parse("[data-total-count]").expect("valid selector");
    document
        .select(&footer_selector)
        .next()
        .and_then(|el| el.value().attr("data-total-count"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn extract_products(document: &Html) -> Vec<SearchProduct> {
    let tile_selector = Selector::parse("[data-product-tile-impression]").expect("valid selector");
    let img_selector = Selector::parse("img[data-src]").expect("valid selector");
    let unit_price_selector =
        Selector::parse(".pwc-tile--price-secondary").expect("valid selector");

    let mut products = Vec::new();

    for element in document.select(&tile_selector) {
        let Some(raw_json) = element.value().attr("data-product-tile-impression") else {
            continue;
        };

        // The attribute value is HTML-encoded (e.g., &quot; instead of ")
        let decoded = html_decode(raw_json);
        let Ok(mut product) = serde_json::from_str::<SearchProduct>(&decoded) else {
            continue;
        };

        // Extract image URL from nearest img[data-src]
        if let Some(img) = element.select(&img_selector).next() {
            product.image_url = img.value().attr("data-src").map(String::from);
        }

        // Extract unit price
        if let Some(unit_el) = element.select(&unit_price_selector).next() {
            let text: String = unit_el.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                product.unit_price = Some(text);
            }
        }

        products.push(product);
    }

    products
}

pub fn parse_suggestions(html: &str) -> SuggestionResult {
    let document = Html::parse_document(html);

    let products = extract_products(&document);

    let categories = extract_category_suggestions(&document);
    let popular_terms = extract_popular_terms(&document);

    SuggestionResult {
        products,
        categories,
        popular_terms,
    }
}

fn extract_category_suggestions(document: &Html) -> Vec<CategorySuggestion> {
    let link_selector = Selector::parse(".suggestions-category a").expect("valid selector");
    let mut categories = Vec::new();

    for link in document.select(&link_selector) {
        let name: String = link.text().collect::<String>().trim().to_string();
        let url = link.value().attr("href").unwrap_or_default().to_string();
        if !name.is_empty() {
            categories.push(CategorySuggestion { name, url });
        }
    }

    categories
}

fn extract_popular_terms(document: &Html) -> Vec<String> {
    let link_selector = Selector::parse(".suggestions-popular a").expect("valid selector");
    let mut terms = Vec::new();

    for link in document.select(&link_selector) {
        let text: String = link.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            terms.push(text);
        }
    }

    terms
}

pub fn parse_nutritional_info(html: &str) -> NutritionalInfo {
    let document = Html::parse_document(html);

    NutritionalInfo {
        regulated_name: extract_text(&document, ".regulated-product-name"),
        ingredients: extract_text(&document, ".ingredients"),
        allergens: extract_text(&document, ".allergen-statement"),
        country_of_origin: extract_text(&document, ".country-origin"),
        storage_instructions: extract_text(&document, ".storage-instruction"),
        net_content: extract_text(&document, ".net-content"),
        net_content_uom: extract_text(&document, ".net-content--uom"),
        net_weight: extract_text(&document, ".net-weight"),
        producer_name: extract_text(&document, ".contact-information--name"),
        producer_address: extract_text(&document, ".contact-information--address"),
        preparation_instructions: extract_text(&document, ".preparation-instructions"),
        daily_value_intake_reference: extract_text(&document, ".daily-value-intake-reference"),
        serving_size: extract_text(&document, ".serving-size"),
        serving_size_uom: extract_text(&document, ".serving-size--uom"),
        nutrients: extract_nutrients(&document),
    }
}

fn extract_text(document: &Html, css_selector: &str) -> Option<String> {
    let selector = Selector::parse(css_selector).ok()?;
    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

fn extract_nutrients(document: &Html) -> Vec<Nutrient> {
    let row_selector = Selector::parse(".nutrients-table tr")
        .unwrap_or_else(|_| Selector::parse("table tr").expect("valid selector"));
    let cell_selector = Selector::parse("td").expect("valid selector");

    let mut nutrients = Vec::new();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|td| td.text().collect::<String>().trim().to_string())
            .collect();

        if cells.len() >= 2 {
            let name = cells[0].clone();
            let value_str = &cells[1];
            let value = value_str
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
                .collect::<String>()
                .replace(',', ".");
            let value = value.parse::<f64>().unwrap_or(0.0);
            let unit = cells.get(2).cloned().unwrap_or_default();

            if !name.is_empty() {
                nutrients.push(Nutrient { name, value, unit });
            }
        }
    }

    nutrients
}

#[allow(clippy::similar_names)]
pub fn parse_flyers(html: &str) -> Result<Vec<Flyer>> {
    let document = Html::parse_document(html);
    let tile_selector = Selector::parse(".ipaper-tile").expect("valid selector");
    let link_selector = Selector::parse("a.ipaper-tile--image-link").expect("valid selector");
    let title_selector = Selector::parse(".ipaper-tile--title").expect("valid selector");
    let desc_selector = Selector::parse(".ipaper-tile--description").expect("valid selector");
    let img_selector = Selector::parse("img[data-src]").expect("valid selector");

    let mut flyers = Vec::new();

    for tile in document.select(&tile_selector) {
        let Some(link) = tile.select(&link_selector).next() else {
            continue;
        };
        let Some(url) = link.value().attr("href") else {
            continue;
        };

        let title = tile
            .select(&title_selector)
            .next()
            .map(|el| html_decode(el.text().collect::<String>().trim()))
            .unwrap_or_default();

        let description = tile
            .select(&desc_selector)
            .next()
            .map(|el| html_decode(el.text().collect::<String>().trim()))
            .unwrap_or_default();

        let image_url = tile
            .select(&img_selector)
            .next()
            .and_then(|el| el.value().attr("data-src").map(String::from));

        let slug = extract_flyer_slug(url);

        flyers.push(Flyer {
            title,
            description,
            url: url.to_string(),
            image_url,
            slug,
        });
    }

    if flyers.is_empty() {
        return Err(ContinenteError::NoResults);
    }

    Ok(flyers)
}

fn extract_flyer_slug(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string()
}

fn html_decode(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&iacute;", "í")
        .replace("&aacute;", "á")
        .replace("&eacute;", "é")
        .replace("&oacute;", "ó")
        .replace("&uacute;", "ú")
        .replace("&atilde;", "ã")
        .replace("&otilde;", "õ")
        .replace("&ccedil;", "ç")
        .replace("&Aacute;", "Á")
        .replace("&Eacute;", "É")
}

use continente::api::models::{ProductVariationResponse, StoresResponse};
use continente::api::parse_flyers;

#[test]
fn parse_search_html_extracts_products() {
    let html = include_str!("fixtures/search_leite.html");
    let result = continente::api::parse_search_results(html, "leite");
    assert!(result.is_ok(), "Parse failed: {result:?}");

    let response = result.unwrap();
    assert!(
        response.total > 0,
        "Expected total > 0, got {}",
        response.total
    );
    assert!(!response.products.is_empty(), "Expected products");
    assert_eq!(response.query, "leite");
}

#[test]
fn parse_search_html_product_fields_populated() {
    let html = include_str!("fixtures/search_leite.html");
    let response = continente::api::parse_search_results(html, "leite").unwrap();
    let product = &response.products[0];

    assert!(!product.id.is_empty(), "id should not be empty");
    assert!(!product.name.is_empty(), "name should not be empty");
    assert!(product.price > 0.0, "price should be positive");
    assert!(!product.brand.is_empty(), "brand should not be empty");
    assert!(!product.category.is_empty(), "category should not be empty");
}

#[test]
fn parse_search_html_extracts_images() {
    let html = include_str!("fixtures/search_leite.html");
    let response = continente::api::parse_search_results(html, "leite").unwrap();

    let has_images = response.products.iter().any(|p| p.image_url.is_some());
    assert!(has_images, "Expected products with image URLs");
}

#[test]
fn parse_search_empty_html_returns_error() {
    let result = continente::api::parse_search_results("<html></html>", "test");
    assert!(result.is_err());
}

#[test]
fn parse_suggestions_html() {
    let html = include_str!("fixtures/suggestions_leite.html");
    let result = continente::api::parse_suggestions(html);

    let has_data = !result.products.is_empty()
        || !result.categories.is_empty()
        || !result.popular_terms.is_empty();
    assert!(has_data, "Expected at least some suggestion data");
}

#[test]
fn parse_nutrition_html_extracts_fields() {
    let html = include_str!("fixtures/nutrition_6879912.html");
    let info = continente::api::parse_nutritional_info(html);

    assert!(
        info.ingredients.is_some(),
        "Expected ingredients to be present"
    );
}

#[test]
fn deserialize_product_variation_json() {
    let json = include_str!("fixtures/product_6879912.json");
    let response: ProductVariationResponse = serde_json::from_str(json).unwrap();
    let detail = response.product.into_detail();

    assert_eq!(detail.id, "6879912");
    assert_eq!(detail.name, "Leite UHT Meio Gordo Continente");
    assert_eq!(detail.brand, "Continente");
    assert!(detail.price.sales_value > 0.0);
    assert_eq!(detail.ean.as_deref(), Some("5601312508007"));
}

#[test]
fn deserialize_stores_json() {
    let json = include_str!("fixtures/stores.json");
    let response: StoresResponse = serde_json::from_str(json).unwrap();

    assert!(!response.stores.is_empty());
    let store = &response.stores[0];
    assert!(!store.id.is_empty());
    assert!(!store.name.is_empty());
}

#[test]
fn parse_flyers_html_extracts_flyers() {
    let html = r#"
    <div class="ipaper-tile">
        <a class="ipaper-tile--image-link" href="https://folhetos.continente.pt/semanal-12/">
            <img data-src="https://b-cdn.ipaper.io/cover.jpg" />
        </a>
        <div class="ipaper-tile--title">Folheto Semanal</div>
        <div class="ipaper-tile--description">18 mar - 24 mar</div>
    </div>
    <div class="ipaper-tile">
        <a class="ipaper-tile--image-link" href="https://folhetos.continente.pt/fim-de-semana/">
        </a>
        <div class="ipaper-tile--title">Fim de Semana</div>
        <div class="ipaper-tile--description">21 mar - 23 mar</div>
    </div>
    "#;
    let flyers = parse_flyers(html).unwrap();
    assert_eq!(flyers.len(), 2);
    assert_eq!(flyers[0].title, "Folheto Semanal");
    assert_eq!(flyers[0].slug, "semanal-12");
    assert!(flyers[0].image_url.is_some());
    assert_eq!(flyers[1].title, "Fim de Semana");
    assert!(flyers[1].image_url.is_none());
}

#[test]
fn parse_flyers_empty_html_returns_error() {
    let result = parse_flyers("<html></html>");
    assert!(result.is_err());
}

#[test]
fn parse_flyers_tile_without_link_is_skipped() {
    let html = r#"
    <div class="ipaper-tile">
        <div class="ipaper-tile--title">No Link</div>
    </div>
    <div class="ipaper-tile">
        <a class="ipaper-tile--image-link" href="https://folhetos.continente.pt/valid/">
        </a>
        <div class="ipaper-tile--title">Valid</div>
        <div class="ipaper-tile--description">dates</div>
    </div>
    "#;
    let flyers = parse_flyers(html).unwrap();
    assert_eq!(flyers.len(), 1);
    assert_eq!(flyers[0].title, "Valid");
}

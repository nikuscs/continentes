use continente::api::client::ContinenteClient;
use continente::api::models::SearchParams;
use continente::config::HttpConfig;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_client(base_url: &str) -> ContinenteClient {
    ContinenteClient::with_base_url(base_url, &HttpConfig::default()).unwrap()
}

const SFCC_PATH: &str = "/on/demandware.store/Sites-continente-Site/default";

#[tokio::test]
async fn search_returns_products_from_html() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("q", "leite"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.search("leite", &SearchParams::new()).await;

    assert!(result.is_ok(), "Search failed: {result:?}");
    let response = result.unwrap();
    assert!(response.total > 0);
    assert!(!response.products.is_empty());

    let first = &response.products[0];
    assert!(!first.id.is_empty());
    assert!(!first.name.is_empty());
    assert!(first.price > 0.0);
    assert!(!first.brand.is_empty());
    assert!(!first.category.is_empty());
}

#[tokio::test]
async fn search_extracts_image_urls() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let response = client.search("leite", &SearchParams::new()).await.unwrap();

    let has_images = response.products.iter().any(|p| p.image_url.is_some());
    assert!(
        has_images,
        "Expected at least one product with an image URL"
    );
}

#[tokio::test]
async fn search_with_brand_filter_sends_prefn1() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("prefn1", "brand"))
        .and(query_param("prefv1", "Mimosa"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let mut params = SearchParams::new();
    params.brand = Some("Mimosa".to_string());
    let result = client.search("leite", &params).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn search_with_sort_sends_srule() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("srule", "price-low-to-high"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let mut params = SearchParams::new();
    params.sort = Some(continente::api::models::SortRule::PriceLowToHigh);
    let result = client.search("leite", &params).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn product_returns_detail_from_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .and(query_param("pid", "6879912"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/product_6879912.json")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.product("6879912").await;

    assert!(result.is_ok(), "Product failed: {result:?}");
    let product = result.unwrap();
    assert_eq!(product.id, "6879912");
    assert_eq!(product.name, "Leite UHT Meio Gordo Continente");
    assert_eq!(product.brand, "Continente");
    assert!(product.price.sales_value > 0.0);
    assert_eq!(product.price.currency, "EUR");
    assert!(product.available);
    assert!(product.online);
}

#[tokio::test]
async fn product_extracts_ean_from_nutritional_url() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/product_6879912.json")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let product = client.product("6879912").await.unwrap();

    assert_eq!(product.ean.as_deref(), Some("5601312508007"));
    assert_eq!(product.supplier_id.as_deref(), Some("5600000000403"));
}

#[tokio::test]
async fn product_extracts_category_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/product_6879912.json")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let product = client.product("6879912").await.unwrap();

    assert_eq!(product.category.id, "laticinios-leite-meio-gordo");
    assert_eq!(product.category.name, "Leite Meio Gordo");
    assert_eq!(product.category.top_level_id, "laticinios");
    assert!(!product.category.gtm_path.is_empty());
}

#[tokio::test]
async fn product_extracts_images() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/product_6879912.json")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let product = client.product("6879912").await.unwrap();

    assert!(product.images.tile.is_some());
    assert!(!product.images.quick_view.is_empty());
    assert!(!product.images.full.is_empty());
}

#[tokio::test]
async fn stores_returns_array_from_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Stores-FindStores")))
        .and(query_param("lat", "38.7"))
        .and(query_param("long", "-9.1"))
        .and(query_param("radius", "15"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/stores.json")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.stores(38.7, -9.1, 15).await;

    assert!(result.is_ok(), "Stores failed: {result:?}");
    let stores = result.unwrap();
    assert!(!stores.is_empty());

    let first = &stores[0];
    assert!(!first.id.is_empty());
    assert!(!first.name.is_empty());
    assert!(!first.city.is_empty());
    assert!(first.latitude != 0.0);
    assert!(first.longitude != 0.0);
}

#[tokio::test]
async fn suggest_returns_products_and_terms() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/SearchServices-GetSuggestions")))
        .and(query_param("q", "leite"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/suggestions_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.suggest("leite").await;

    assert!(result.is_ok(), "Suggest failed: {result:?}");
    let suggestions = result.unwrap();
    assert!(
        !suggestions.products.is_empty()
            || !suggestions.categories.is_empty()
            || !suggestions.popular_terms.is_empty(),
        "Expected at least some suggestion data"
    );
}

#[tokio::test]
async fn suggest_rejects_short_queries() {
    let server = MockServer::start().await;
    let client = test_client(&server.uri());

    let result = client.suggest("lei").await;
    assert!(result.is_err(), "Expected error for short query");
}

#[tokio::test]
async fn nutrition_returns_parsed_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "{SFCC_PATH}/Product-ProductNutritionalInfoTab"
        )))
        .and(query_param("pid", "6879912"))
        .and(query_param("ean", "5601312508007"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/nutrition_6879912.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client
        .nutrition("6879912", "5601312508007", "5600000000403")
        .await;

    assert!(result.is_ok(), "Nutrition failed: {result:?}");
    let info = result.unwrap();
    assert!(info.ingredients.is_some());
}

#[tokio::test]
async fn browse_sends_cgid_without_query() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("cgid", "laticinios-leite"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client
        .browse("laticinios-leite", &SearchParams::new())
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn client_handles_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.search("leite", &SearchParams::new()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn client_handles_empty_search_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .respond_with(
            ResponseTemplate::new(200).set_body_string("<html><body>No results</body></html>"),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.search("xyznonexistent", &SearchParams::new()).await;

    assert!(result.is_err());
}

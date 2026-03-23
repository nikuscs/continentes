use continente::api::client::ContinenteClient;
use continente::api::models::SortRule;
use continente::commands::{browse, categories, flyers, product, search, stores, suggest};
use continente::config::HttpConfig;
use continente::format::OutputFormat;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SFCC_PATH: &str = "/on/demandware.store/Sites-continente-Site/default";

fn client(server: &MockServer) -> ContinenteClient {
    ContinenteClient::with_base_url(&server.uri(), &HttpConfig::default()).unwrap()
}

#[tokio::test]
async fn search_run_builds_query_and_formats_output() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("q", "leite"))
        .and(query_param("cgid", "col-produtos"))
        .and(query_param("start", "24"))
        .and(query_param("sz", "24"))
        .and(query_param("srule", "price-low-to-high"))
        .and(query_param("prefn1", "brand"))
        .and(query_param("prefv1", "Mimosa"))
        .and(query_param("prefn2", "food.Vegan"))
        .and(query_param("prefv2", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let result = search::run(
        &client(&server),
        "leite",
        24,
        2,
        Some(SortRule::PriceLowToHigh),
        Some("Mimosa".to_string()),
        None,
        None,
        vec![("food.Vegan".to_string(), "true".to_string())],
        OutputFormat::Table,
    )
    .await
    .unwrap();

    assert!(result.contains("Found"));
    assert!(result.contains("6879912"));
}

#[tokio::test]
async fn search_run_rejects_page_zero() {
    let server = MockServer::start().await;

    let result = search::run(
        &client(&server),
        "leite",
        24,
        0,
        None,
        None,
        None,
        None,
        Vec::new(),
        OutputFormat::Table,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Page number must be at least 1"
    );
}

#[tokio::test]
async fn search_run_rejects_max_zero() {
    let server = MockServer::start().await;

    let result = search::run(
        &client(&server),
        "leite",
        0,
        1,
        None,
        None,
        None,
        None,
        Vec::new(),
        OutputFormat::Table,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Maximum results must be at least 1"
    );
}

#[tokio::test]
async fn browse_run_resolves_category_name() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("cgid", "laticinios-leite"))
        .and(query_param("start", "0"))
        .and(query_param("sz", "12"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let output = browse::run(
        &client(&server),
        "Leite",
        12,
        1,
        None,
        OutputFormat::Compact,
    )
    .await
    .unwrap();

    assert!(output.contains("6879912"));
    assert!(output.contains("Continente"));
}

#[tokio::test]
async fn browse_run_rejects_page_zero() {
    let server = MockServer::start().await;

    let result = browse::run(
        &client(&server),
        "frescos",
        24,
        0,
        None,
        OutputFormat::Table,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Page number must be at least 1"
    );
}

#[tokio::test]
async fn browse_run_rejects_max_zero() {
    let server = MockServer::start().await;

    let result = browse::run(&client(&server), "frescos", 0, 1, None, OutputFormat::Table).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Maximum results must be at least 1"
    );
}

#[tokio::test]
async fn browse_run_propagates_no_results_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("cgid", "laticinios-leite"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
        .mount(&server)
        .await;

    let result = browse::run(
        &client(&server),
        "laticinios-leite",
        24,
        1,
        None,
        OutputFormat::Table,
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn browse_run_propagates_non_no_results_errors() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let result = browse::run(
        &client(&server),
        "laticinios-leite",
        24,
        1,
        None,
        OutputFormat::Table,
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn product_run_without_nutrition_returns_detail() {
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

    let output = product::run(&client(&server), "6879912", false, OutputFormat::Table)
        .await
        .unwrap();

    assert!(output.contains("Leite UHT Meio Gordo Continente"));
    assert!(!output.contains("Nutritional Info"));
}

#[tokio::test]
async fn product_run_with_nutrition_errors_when_metadata_is_missing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .and(query_param("pid", "6879912"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
                "product": {
                    "id": "6879912",
                    "productName": "Leite UHT Meio Gordo Continente",
                    "brand": "Continente",
                    "available": true,
                    "online": true,
                    "price": {
                        "sales": {
                            "value": 0.86,
                            "currency": "EUR",
                            "formatted": "0,86 €"
                        }
                    }
                }
            }"#,
        ))
        .mount(&server)
        .await;

    let result = product::run(&client(&server), "6879912", true, OutputFormat::Table).await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Nutritional info is not available for product '6879912'"
    );
}

#[tokio::test]
async fn product_run_with_nutrition_fetches_extra_data() {
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

    Mock::given(method("GET"))
        .and(path(format!(
            "{SFCC_PATH}/Product-ProductNutritionalInfoTab"
        )))
        .and(query_param("pid", "6879912"))
        .and(query_param("ean", "5601312508007"))
        .and(query_param("supplierid", "5600000000403"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(include_str!("fixtures/nutrition_6879912.html")),
        )
        .mount(&server)
        .await;

    let output = product::run(&client(&server), "6879912", true, OutputFormat::Table)
        .await
        .unwrap();

    assert!(output.contains("Nutritional Info"));
    assert!(output.contains("Ingredients"));
}

#[tokio::test]
async fn stores_run_formats_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Stores-FindStores")))
        .and(query_param("lat", "38.7"))
        .and(query_param("long", "-9.1"))
        .and(query_param("radius", "10"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/stores.json")),
        )
        .mount(&server)
        .await;

    let output = stores::run(&client(&server), 38.7, -9.1, 10, OutputFormat::Json)
        .await
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(parsed.is_array());
    assert!(!parsed.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn suggest_run_formats_sections() {
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

    let output = suggest::run(&client(&server), "leite", OutputFormat::Table)
        .await
        .unwrap();

    assert!(
        output.contains("Products:")
            || output.contains("Categories:")
            || output.contains("Popular searches:")
    );
}

#[tokio::test]
async fn suggest_run_rejects_short_query() {
    let server = MockServer::start().await;
    let result = suggest::run(&client(&server), "lei", OutputFormat::Table).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn flyers_run_formats_results() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/folhetos/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"
            <div class="ipaper-tile">
                <a class="ipaper-tile--image-link" href="https://folhetos.continente.pt/semanal-12-jeu9/"></a>
                <div class="ipaper-tile--title">Folheto Semanal</div>
                <div class="ipaper-tile--description">18 mar - 24 mar</div>
                <img data-src="https://b-cdn.ipaper.io/cover.jpg" />
            </div>
            "#,
        ))
        .mount(&server)
        .await;

    let output = flyers::run(&client(&server), OutputFormat::Compact)
        .await
        .unwrap();

    assert!(output.contains("semanal-12-jeu9\tFolheto Semanal\t"));
}

#[test]
fn categories_run_formats_results() {
    let output = categories::run(OutputFormat::Compact).unwrap();
    assert!(output.contains("frescos\tFrescos"));
}

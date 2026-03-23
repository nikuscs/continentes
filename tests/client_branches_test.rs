use continente::api::client::ContinenteClient;
use continente::api::models::{SearchParams, SortRule};
use continente::config::HttpConfig;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_client(base_url: &str) -> ContinenteClient {
    ContinenteClient::with_base_url(base_url, &HttpConfig::default()).unwrap()
}

const SFCC_PATH: &str = "/on/demandware.store/Sites-continente-Site/default";

#[tokio::test]
async fn search_with_price_filters_and_feature_filter_sends_expected_params() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("q", "leite"))
        .and(query_param("pmin", "1.5"))
        .and(query_param("pmax", "3"))
        .and(query_param("prefn1", "food.Vegan"))
        .and(query_param("prefv1", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let params = SearchParams {
        start: 0,
        size: 24,
        sort: Some(SortRule::PriceLowToHigh),
        price_min: Some(1.5),
        price_max: Some(3.0),
        brand: None,
        filters: vec![("food.Vegan".to_string(), "true".to_string())],
    };

    let result = client.search("leite", &params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn product_invalid_json_returns_parse_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Product-Variation")))
        .and(query_param("pid", "bad"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{not-json"))
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.product("bad").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn stores_invalid_json_returns_parse_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Stores-FindStores")))
        .respond_with(ResponseTemplate::new(200).set_body_string("{not-json"))
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let result = client.stores(38.7, -9.1, 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn browse_with_sort_sends_srule() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("{SFCC_PATH}/Search-ShowAjax")))
        .and(query_param("cgid", "frescos"))
        .and(query_param("srule", "product-name-descending"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(include_str!("fixtures/search_leite.html")),
        )
        .mount(&server)
        .await;

    let client = test_client(&server.uri());
    let mut params = SearchParams::new();
    params.sort = Some(SortRule::NameDesc);
    let result = client.browse("frescos", &params).await;

    assert!(result.is_ok());
}

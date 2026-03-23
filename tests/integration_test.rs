use continente::api::client::ContinenteClient;
use continente::api::models::SearchParams;
use continente::config::HttpConfig;

#[tokio::test]
#[ignore = "requires network"]
async fn real_search_returns_results() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let params = SearchParams::new();
    let result = client.search("leite", &params).await;
    assert!(result.is_ok(), "Search failed: {result:?}");
    let response = result.unwrap();
    assert!(response.total > 0, "Expected results, got total=0");
    assert!(!response.products.is_empty(), "Expected products");

    let first = &response.products[0];
    assert!(!first.id.is_empty());
    assert!(!first.name.is_empty());
    assert!(first.price > 0.0);
}

#[tokio::test]
#[ignore = "requires network"]
async fn real_product_returns_detail() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let result = client.product("6879912").await;
    assert!(result.is_ok(), "Product failed: {result:?}");
    let product = result.unwrap();
    assert_eq!(product.id, "6879912");
    assert_eq!(product.brand, "Continente");
    assert!(product.price.sales_value > 0.0);
    assert!(product.ean.is_some());
}

#[tokio::test]
#[ignore = "requires network"]
async fn real_stores_returns_json() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let result = client.stores(38.7, -9.1, 15).await;
    assert!(result.is_ok(), "Stores failed: {result:?}");
    let stores = result.unwrap();
    assert!(!stores.is_empty(), "Expected stores near Lisbon");
}

#[tokio::test]
#[ignore = "requires network"]
async fn real_suggest_returns_results() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let result = client.suggest("leite").await;
    assert!(result.is_ok(), "Suggest failed: {result:?}");
}

#[tokio::test]
#[ignore = "requires network"]
async fn real_browse_returns_products() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let params = SearchParams::new();
    let result = client.browse("laticinios-leite", &params).await;
    assert!(result.is_ok(), "Browse failed: {result:?}");
}

#[tokio::test]
#[ignore = "requires network"]
async fn real_nutrition_returns_data() {
    let client = ContinenteClient::new(&HttpConfig::default()).unwrap();
    let result = client
        .nutrition("6879912", "5601312508007", "5600000000403")
        .await;
    assert!(result.is_ok(), "Nutrition failed: {result:?}");
    let info = result.unwrap();
    assert!(info.ingredients.is_some());
}

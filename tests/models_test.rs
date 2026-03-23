use continente::api::models::{ProductVariationResponse, RawProduct, SearchParams, SortRule};

#[test]
fn search_params_new_sets_default_page_size() {
    let params = SearchParams::new();
    assert_eq!(params.start, 0);
    assert_eq!(params.size, 24);
}

#[test]
fn sort_rule_as_str_matches_expected_values() {
    assert_eq!(SortRule::Relevance.as_str(), "Continente");
    assert_eq!(SortRule::PriceLowToHigh.as_str(), "price-low-to-high");
    assert_eq!(SortRule::PriceHighToLow.as_str(), "price-high-to-low");
    assert_eq!(SortRule::UnitPrice.as_str(), "price-per-capacity-ascending");
    assert_eq!(SortRule::NameAsc.as_str(), "product-name-ascending");
    assert_eq!(SortRule::NameDesc.as_str(), "product-name-descending");
}

#[test]
fn raw_product_into_detail_handles_missing_optional_fields() {
    let raw = RawProduct {
        id: "1".to_string(),
        product_name: "Produto".to_string(),
        product_type: String::new(),
        brand: String::new(),
        short_description: None,
        rating: None,
        available: false,
        online: false,
        ready_to_order: false,
        product_url: None,
        measurement_note: None,
        min_order_quantity: 0,
        max_order_quantity: 0,
        gtm_category_path: None,
        nutritional_info_url_string: None,
        price: None,
        price_per_unit: None,
        category: None,
        badge_info: None,
        product_tile_image: None,
        quick_view_images: Vec::new(),
        pdp_images: Vec::new(),
    };

    let detail = raw.into_detail();
    assert_eq!(detail.id, "1");
    assert!(detail.price.sales_value.abs() < f64::EPSILON);
    assert_eq!(detail.price.currency, "EUR");
    assert!(detail.ean.is_none());
    assert!(detail.supplier_id.is_none());
    assert!(detail.images.quick_view.is_empty());
}

#[test]
fn extracts_first_ean_from_multi_ean_nutrition_url() {
    let json = r#"{
      "product": {
        "id": "1",
        "productName": "Produto",
        "brand": "Marca",
        "productType": "food",
        "available": true,
        "online": true,
        "readyToOrder": true,
        "productURL": "/produto",
        "nutritionalInfoUrlString": "Product-ProductNutritionalInfoTab?pid=1&ean=111|222&supplierid=333",
        "quickViewImages": [],
        "pdpImages": []
      }
    }"#;

    let response: ProductVariationResponse = serde_json::from_str(json).unwrap();
    let detail = response.product.into_detail();
    assert_eq!(detail.ean.as_deref(), Some("111"));
    assert_eq!(detail.supplier_id.as_deref(), Some("333"));
}

use continente::api::models::{
    Nutrient, NutritionalInfo, ProductDetail, RawBadge, RawBadgeInfo, RawCategory, RawImage,
    RawImageEntry, RawPrice, RawPricePerUnit, RawPrimaryPrice, RawProduct, RawSales,
    RawSecondaryPrice, SearchProduct, SearchResponse, Store, SuggestionResult,
};
use continente::format::{
    OutputFormat, format_categories, format_product_detail, format_products, format_stores,
    format_suggestions,
};

fn raw_product_base() -> RawProduct {
    RawProduct {
        id: "6879912".to_string(),
        product_name: "Leite UHT Meio Gordo Continente".to_string(),
        product_type: "food".to_string(),
        brand: "Continente".to_string(),
        short_description: Some("Um delicioso leite meio gordo".to_string()),
        rating: Some(3.9),
        available: true,
        online: true,
        ready_to_order: false,
        product_url: Some("https://www.continente.pt/produto/leite".to_string()),
        measurement_note: Some("emb. 1 lt".to_string()),
        min_order_quantity: 1,
        max_order_quantity: 10,
        gtm_category_path: Some("Laticínios e Ovos/Leite/Leite Meio Gordo".to_string()),
        nutritional_info_url_string: Some(
            "Product-ProductNutritionalInfoTab?pid=6879912&ean=5601312508007&supplierid=5600000000403"
                .to_string(),
        ),
        price: Some(RawPrice {
            sales: Some(RawSales {
                value: 0.86,
                currency: "EUR".to_string(),
                formatted: "0,86€".to_string(),
            }),
            list: Some(RawSales {
                value: 1.0,
                currency: "EUR".to_string(),
                formatted: "1,00€".to_string(),
            }),
            online_to: Some("2026-04-01".to_string()),
        }),
        price_per_unit: Some(RawPricePerUnit {
            primary_price: Some(RawPrimaryPrice {
                price: Some(RawSales {
                    value: 0.86,
                    currency: "EUR".to_string(),
                    formatted: "0,86€".to_string(),
                }),
                unit: Some("lt".to_string()),
            }),
            secondary_price: Some(RawSecondaryPrice {
                price: Some(serde_json::json!("0,86€/lt")),
                unit: Some("lt".to_string()),
                value: Some("0,86€/lt".to_string()),
            }),
        }),
        category: Some(RawCategory {
            primary_category_id: "laticinios-leite-meio-gordo".to_string(),
            primary_category_display_name: "Leite Meio Gordo".to_string(),
            primary_category_top_level_product_category_id: "laticinios".to_string(),
            primary_category_top_level_product_category_display_name: "Laticínios e Ovos".to_string(),
        }),
        badge_info: Some(RawBadgeInfo {
            general: Some(RawBadge {
                title: Some("Produzido em Portugal".to_string()),
            }),
            promo: Some(RawBadge {
                title: Some("PVP Recomendado: 1,00€/un".to_string()),
            }),
        }),
        product_tile_image: Some(RawImage {
            url: Some("https://example.com/tile.jpg".to_string()),
        }),
        quick_view_images: vec![RawImageEntry {
            url: Some("https://example.com/quick.jpg".to_string()),
        }],
        pdp_images: vec![RawImageEntry {
            url: Some("https://example.com/full.jpg".to_string()),
        }],
    }
}

fn sample_product_detail() -> ProductDetail {
    raw_product_base().into_detail()
}

#[test]
fn parse_search_results_handles_invalid_total_and_mixed_tiles() {
    let html = r#"
        <html>
          <body>
            <div data-gtm-results="abc"></div>
            <div data-product-tile-impression="not-json"></div>
            <div
              data-product-tile-impression="{&quot;id&quot;:&quot;6879912&quot;,&quot;name&quot;:&quot;Leite UHT Meio Gordo Continente&quot;,&quot;price&quot;:0.86,&quot;brand&quot;:&quot;Continente&quot;,&quot;category&quot;:&quot;Laticínios e Ovos/Leite/Leite Meio Gordo&quot;,&quot;variant&quot;:&quot;&quot;,&quot;channel&quot;:&quot;&quot;}"
            >
              <img data-src="https://example.com/tile.jpg" />
              <span class="pwc-tile--price-secondary">0,86€/lt</span>
            </div>
            <div
              data-product-tile-impression="{&quot;id&quot;:&quot;2210946&quot;,&quot;name&quot;:&quot;Leite UHT Meio Gordo Mimosa&quot;,&quot;price&quot;:0.90,&quot;brand&quot;:&quot;Mimosa&quot;,&quot;category&quot;:&quot;Laticínios e Ovos/Leite/Leite Meio Gordo&quot;,&quot;variant&quot;:&quot;&quot;,&quot;channel&quot;:&quot;&quot;}"
            ></div>
          </body>
        </html>
    "#;

    let result = continente::api::parse_search_results(html, "leite").unwrap();
    assert_eq!(result.total, 0);
    assert_eq!(result.products.len(), 2);
    assert_eq!(
        result.products[0].image_url.as_deref(),
        Some("https://example.com/tile.jpg")
    );
    assert_eq!(result.products[0].unit_price.as_deref(), Some("0,86€/lt"));
    assert_eq!(result.products[1].image_url, None);
    assert_eq!(result.products[1].unit_price, None);
}

#[test]
fn parse_suggestions_extracts_sections_and_skips_empty_links() {
    let html = r#"
        <html>
          <body>
            <div data-gtm-results="1"></div>
            <div
              data-product-tile-impression="{&quot;id&quot;:&quot;6879912&quot;,&quot;name&quot;:&quot;Leite UHT Meio Gordo Continente&quot;,&quot;price&quot;:0.86,&quot;brand&quot;:&quot;Continente&quot;,&quot;category&quot;:&quot;Laticínios e Ovos/Leite/Leite Meio Gordo&quot;,&quot;variant&quot;:&quot;&quot;,&quot;channel&quot;:&quot;&quot;}"
            ></div>
            <div class="suggestions-category">
              <a href="/laticinios/leite/">Leite</a>
              <a href="/ignored"></a>
            </div>
            <div class="suggestions-popular">
              <a>leite magro</a>
              <a> </a>
            </div>
          </body>
        </html>
    "#;

    let result = continente::api::parse_suggestions(html);
    assert_eq!(result.products.len(), 1);
    assert_eq!(result.categories.len(), 1);
    assert_eq!(result.categories[0].name, "Leite");
    assert_eq!(result.popular_terms, vec!["leite magro".to_string()]);
}

#[test]
fn parse_nutritional_info_handles_missing_fields_and_bad_rows() {
    let html = r#"
        <html>
          <body>
            <table class="nutrients-table">
              <tr><td>Energy</td><td>200 kJ / 48 kcal</td><td>kJ</td></tr>
              <tr><td>Salt</td><td>0,1 g</td><td>g</td></tr>
              <tr><td>Broken</td><td>abc</td><td>g</td></tr>
              <tr><td></td><td>123</td><td>g</td></tr>
            </table>
          </body>
        </html>
    "#;

    let info = continente::api::parse_nutritional_info(html);
    assert!(info.regulated_name.is_none());
    assert!(info.ingredients.is_none());
    assert!(info.allergens.is_none());
    assert_eq!(info.nutrients.len(), 3);
    assert_eq!(info.nutrients[0].name, "Energy");
    assert!((info.nutrients[0].value - 20048.0).abs() < f64::EPSILON);
    assert_eq!(info.nutrients[1].name, "Salt");
    assert!((info.nutrients[1].value - 0.1).abs() < f64::EPSILON);
    assert_eq!(info.nutrients[2].name, "Broken");
    assert!(info.nutrients[2].value.abs() < f64::EPSILON);
}

#[test]
fn raw_product_into_detail_handles_missing_optionals() {
    let mut raw = raw_product_base();
    raw.product_url = None;
    raw.price = Some(RawPrice {
        sales: None,
        list: None,
        online_to: None,
    });
    raw.price_per_unit = Some(RawPricePerUnit {
        primary_price: None,
        secondary_price: Some(RawSecondaryPrice {
            price: Some(serde_json::json!(123)),
            unit: Some("lt".to_string()),
            value: None,
        }),
    });
    raw.category = None;
    raw.badge_info = None;
    raw.product_tile_image = None;
    raw.quick_view_images.clear();
    raw.pdp_images.clear();
    raw.nutritional_info_url_string = None;

    let detail = raw.into_detail();
    assert_eq!(detail.product_url, "");
    assert!(detail.price.sales_value.abs() < f64::EPSILON);
    assert_eq!(detail.price.sales_formatted, "");
    assert_eq!(detail.price.currency, "EUR");
    assert!(detail.price_per_unit.is_none());
    assert_eq!(detail.category.id, "");
    assert_eq!(detail.badge_info.general_title, None);
    assert_eq!(detail.images.tile, None);
    assert!(detail.images.quick_view.is_empty());
    assert!(detail.images.full.is_empty());
    assert_eq!(detail.ean, None);
    assert_eq!(detail.supplier_id, None);
}

#[test]
fn raw_product_into_detail_extracts_multi_ean_and_decoder_fallback() {
    let mut raw = raw_product_base();
    raw.nutritional_info_url_string = Some(
        "Product-ProductNutritionalInfoTab?pid=6879912&ean=5601312508007|9999999999999&supplierid=5600000000403%"
            .to_string(),
    );

    let detail = raw.into_detail();
    assert_eq!(detail.ean.as_deref(), Some("5601312508007"));
    assert_eq!(detail.supplier_id.as_deref(), Some("5600000000403%"));
}

#[test]
fn format_functions_cover_remaining_match_arms() {
    let search = SearchResponse {
        products: vec![SearchProduct {
            id: "6879912".to_string(),
            name: "Leite UHT Meio Gordo Continente".to_string(),
            price: 0.86,
            brand: "Continente".to_string(),
            category: "Laticínios e Ovos/Leite/Leite Meio Gordo".to_string(),
            variant: String::new(),
            channel: String::new(),
            image_url: None,
            unit_price: None,
        }],
        total: 1,
        query: "leite".to_string(),
    };
    let product = sample_product_detail();
    let nutrition = NutritionalInfo {
        regulated_name: Some("Leite".to_string()),
        ingredients: Some("LEITE UHT".to_string()),
        allergens: Some("Contém leite".to_string()),
        country_of_origin: Some("Portugal".to_string()),
        storage_instructions: Some("Guardar em local fresco".to_string()),
        net_content: Some("1 L".to_string()),
        net_content_uom: None,
        net_weight: None,
        producer_name: Some("Continente".to_string()),
        producer_address: None,
        preparation_instructions: None,
        daily_value_intake_reference: None,
        serving_size: None,
        serving_size_uom: None,
        nutrients: vec![Nutrient {
            name: "Energy".to_string(),
            value: 200.0,
            unit: "kJ".to_string(),
        }],
    };
    let stores = vec![Store {
        id: "1".to_string(),
        name: "Continente Colombo".to_string(),
        address: "Av. Lusíada".to_string(),
        city: "Lisboa".to_string(),
        postal_code: "1500-392".to_string(),
        latitude: 38.75,
        longitude: -9.18,
        phone: None,
        store_hours: None,
        is_pickup_store: true,
        is_galp_store: false,
    }];
    let categories = continente::categories::all_categories();
    let suggestions = SuggestionResult {
        products: search.products.clone(),
        categories: vec![continente::api::models::CategorySuggestion {
            name: "Leite".to_string(),
            url: "/leite".to_string(),
        }],
        popular_terms: vec!["leite magro".to_string()],
    };

    let search_json = format_products(&search, 1, 24, OutputFormat::Json);
    let search_compact = format_products(&search, 1, 24, OutputFormat::Compact);
    let detail_json = format_product_detail(&product, Some(&nutrition), OutputFormat::Json);
    let detail_compact = format_product_detail(&product, None, OutputFormat::Compact);
    let stores_json = format_stores(&stores, 10, OutputFormat::Json);
    let stores_compact = format_stores(&stores, 10, OutputFormat::Compact);
    let categories_json = format_categories(categories, OutputFormat::Json);
    let categories_compact = format_categories(categories, OutputFormat::Compact);
    let suggestions_table = format_suggestions(&suggestions, OutputFormat::Table);
    let suggestions_compact = format_suggestions(&suggestions, OutputFormat::Compact);

    assert!(search_json.contains("\"leite\""));
    assert!(search_compact.contains("6879912\t0.86\tContinente"));
    assert!(detail_json.contains("\"nutrition\""));
    assert!(detail_compact.contains("6879912\t0.86\tContinente"));
    assert!(stores_json.contains("Continente Colombo"));
    assert!(stores_compact.contains("1\tContinente Colombo"));
    assert!(categories_json.contains("frescos"));
    assert!(categories_compact.contains("frescos\tFrescos"));
    assert!(suggestions_table.contains("Products:"));
    assert!(suggestions_table.contains("Categories:"));
    assert!(suggestions_table.contains("Popular searches:"));
    assert!(suggestions_compact.contains("6879912\t0.86\tContinente"));
}

use continente::api::models::{
    BadgeInfo, CategoryInfo, Nutrient, NutritionalInfo, PriceInfo, PricePerUnit, ProductDetail,
    ProductImages, SearchProduct, SearchResponse, Store, SuggestionResult,
};
use continente::categories::all_categories;
use continente::format::{
    OutputFormat, format_categories, format_product_detail, format_products, format_stores,
    format_suggestions,
};

fn sample_search_response() -> SearchResponse {
    SearchResponse {
        products: vec![SearchProduct {
            id: "6879912".to_string(),
            name: "Leite UHT Meio Gordo Continente Muito Muito Longo".to_string(),
            price: 0.86,
            brand: "Continente".to_string(),
            category: "Laticínios e Ovos/Leite/Leite Meio Gordo".to_string(),
            variant: String::new(),
            channel: String::new(),
            image_url: Some("https://example.com/image.jpg".to_string()),
            unit_price: Some("0,86€/lt".to_string()),
        }],
        total: 1314,
        query: "leite".to_string(),
    }
}

fn sample_product_detail() -> ProductDetail {
    ProductDetail {
        id: "6879912".to_string(),
        name: "Leite UHT Meio Gordo Continente".to_string(),
        brand: "Continente".to_string(),
        product_type: "food".to_string(),
        short_description: Some("Um delicioso leite meio gordo".to_string()),
        rating: Some(3.9),
        available: true,
        online: true,
        product_url: "https://www.continente.pt/produto/leite".to_string(),
        price: PriceInfo {
            sales_value: 0.86,
            sales_formatted: "0,86€".to_string(),
            list_value: Some(1.0),
            currency: "EUR".to_string(),
            promotion_end: None,
        },
        price_per_unit: Some(PricePerUnit {
            primary_value: 0.86,
            primary_unit: "lt".to_string(),
            secondary_formatted: Some("0,86€".to_string()),
            secondary_unit: Some("lt".to_string()),
        }),
        measurement_note: Some("emb. 1 lt".to_string()),
        min_order_quantity: 1,
        max_order_quantity: 10,
        category: CategoryInfo {
            id: "laticinios-leite-meio-gordo".to_string(),
            name: "Leite Meio Gordo".to_string(),
            top_level_id: "laticinios".to_string(),
            top_level_name: "Laticínios e Ovos".to_string(),
            gtm_path: "Laticínios e Ovos/Leite/Leite Meio Gordo".to_string(),
        },
        badge_info: BadgeInfo {
            general_title: Some("Produzido em Portugal".to_string()),
            promo_title: None,
        },
        images: ProductImages::default(),
        nutritional_info_url: None,
        ean: Some("5601312508007".to_string()),
        supplier_id: Some("5600000000403".to_string()),
    }
}

#[test]
fn output_format_from_str_and_display() {
    assert_eq!(
        "table".parse::<OutputFormat>().unwrap(),
        OutputFormat::Table
    );
    assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
    assert_eq!(
        "compact".parse::<OutputFormat>().unwrap(),
        OutputFormat::Compact
    );
    assert!("xml".parse::<OutputFormat>().is_err());

    assert_eq!(OutputFormat::Table.to_string(), "table");
    assert_eq!(OutputFormat::Json.to_string(), "json");
    assert_eq!(OutputFormat::Compact.to_string(), "compact");
}

#[test]
fn format_products_table_uses_euro_and_truncates() {
    let output = format_products(&sample_search_response(), 1, 24, OutputFormat::Table);
    assert!(output.contains("0.86€"));
    assert!(output.contains("…"));
}

#[test]
fn format_products_json_is_valid() {
    let output = format_products(&sample_search_response(), 1, 24, OutputFormat::Json);
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed["query"], "leite");
    assert_eq!(parsed["products"][0]["id"], "6879912");
}

#[test]
fn format_products_compact_is_tab_separated() {
    let output = format_products(&sample_search_response(), 1, 24, OutputFormat::Compact);
    assert!(output.contains("6879912\t0.86\tContinente\t"));
}

#[test]
fn format_product_detail_shows_promotion_and_nutrition() {
    let nutrition = NutritionalInfo {
        regulated_name: None,
        ingredients: Some("LEITE UHT meio-gordo".to_string()),
        allergens: Some("Contém leite".to_string()),
        country_of_origin: Some("Portugal".to_string()),
        storage_instructions: None,
        net_content: None,
        producer_name: None,
        nutrients: vec![Nutrient {
            name: "Proteína".to_string(),
            value: 3.4,
            unit: "g".to_string(),
        }],
    };

    let output = format_product_detail(
        &sample_product_detail(),
        Some(&nutrition),
        OutputFormat::Table,
    );
    assert!(output.contains("was 1.00€"));
    assert!(output.contains("Nutritional Info"));
    assert!(output.contains("Proteína"));
}

#[test]
fn format_stores_table_shows_pickup() {
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
    }];

    let output = format_stores(&stores, 10, OutputFormat::Table);
    assert!(output.contains("Continente Colombo"));
    assert!(output.contains("Yes"));
}

#[test]
fn format_categories_tree_contains_known_category() {
    let output = format_categories(all_categories(), OutputFormat::Table);
    assert!(output.contains("Frescos (frescos)"));
    assert!(output.contains("Leite (laticinios-leite)"));
}

#[test]
fn format_suggestions_json_is_valid() {
    let suggestions = SuggestionResult {
        products: sample_search_response().products,
        categories: Vec::new(),
        popular_terms: vec!["leite magro".to_string()],
    };

    let output = format_suggestions(&suggestions, OutputFormat::Json);
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed["popular_terms"][0], "leite magro");
}

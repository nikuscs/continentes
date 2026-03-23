use std::time::Duration;

use reqwest::Client;
use tracing::debug;

use crate::api::models::{
    NutritionalInfo, ProductDetail, ProductVariationResponse, SearchParams, SearchResponse, Store,
    StoresResponse, SuggestionResult,
};
use crate::api::scraper;
use crate::config::HttpConfig;
use crate::error::{ContinenteError, Result};

const BASE_URL: &str = "https://www.continente.pt";
const USER_AGENT: &str =
    "Mozilla/5.0 (compatible; continente-cli/0.1; +https://github.com/nikuscs/continentes)";

pub struct ContinenteClient {
    client: Client,
    base_url: String,
}

impl ContinenteClient {
    pub fn new(config: &HttpConfig) -> anyhow::Result<Self> {
        Self::build(BASE_URL, config)
    }

    pub fn with_base_url(base_url: &str, config: &HttpConfig) -> anyhow::Result<Self> {
        Self::build(base_url, config)
    }

    fn build(base_url: &str, config: &HttpConfig) -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(config.timeout_secs))
            .gzip(true)
            .brotli(true)
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    fn endpoint(&self, controller: &str) -> String {
        format!(
            "{}/on/demandware.store/Sites-continente-Site/default/{}",
            self.base_url, controller
        )
    }

    pub async fn search(&self, query: &str, params: &SearchParams) -> Result<SearchResponse> {
        let url = self.endpoint("Search-ShowAjax");
        debug!("Searching for '{query}' with params: {params:?}");

        let mut request = self
            .client
            .get(&url)
            .query(&[("q", query)])
            .query(&[("cgid", "col-produtos")])
            .query(&[("start", &params.start.to_string())])
            .query(&[("sz", &params.size.to_string())])
            .query(&[("pmin", "0.01")]);

        if let Some(sort) = &params.sort {
            request = request.query(&[("srule", sort.as_str())]);
        }
        if let Some(pmin) = params.price_min {
            request = request.query(&[("pmin", &pmin.to_string())]);
        }
        if let Some(pmax) = params.price_max {
            request = request.query(&[("pmax", &pmax.to_string())]);
        }
        if let Some(brand) = &params.brand {
            request = request.query(&[("prefn1", "brand"), ("prefv1", brand)]);
        }

        // Add additional filters (prefn2/prefv2, prefn3/prefv3, etc.)
        let filter_offset = if params.brand.is_some() { 2 } else { 1 };
        for (i, (name, value)) in params.filters.iter().enumerate() {
            let n_key = format!("prefn{}", i + filter_offset);
            let v_key = format!("prefv{}", i + filter_offset);
            request = request.query(&[(&n_key, name.as_str()), (&v_key, value.as_str())]);
        }

        let response = request.send().await?.error_for_status()?;
        let html = response.text().await?;

        scraper::parse_search_results(&html, query)
    }

    pub async fn browse(&self, cgid: &str, params: &SearchParams) -> Result<SearchResponse> {
        let url = self.endpoint("Search-ShowAjax");
        debug!("Browsing category '{cgid}'");

        let mut request = self
            .client
            .get(&url)
            .query(&[("cgid", cgid)])
            .query(&[("start", &params.start.to_string())])
            .query(&[("sz", &params.size.to_string())]);

        if let Some(sort) = &params.sort {
            request = request.query(&[("srule", sort.as_str())]);
        }

        let response = request.send().await?.error_for_status()?;
        let html = response.text().await?;

        scraper::parse_search_results(&html, cgid)
    }

    pub async fn product(&self, pid: &str) -> Result<ProductDetail> {
        let url = self.endpoint("Product-Variation");
        debug!("Fetching product '{pid}'");

        let response = self
            .client
            .get(&url)
            .query(&[("pid", pid)])
            .send()
            .await?
            .error_for_status()?;

        let json: ProductVariationResponse =
            response.json().await.map_err(|e| ContinenteError::Parse {
                url: format!("Product-Variation?pid={pid}"),
                message: e.to_string(),
            })?;

        Ok(json.product.into_detail())
    }

    pub async fn nutrition(
        &self,
        pid: &str,
        ean: &str,
        supplier_id: &str,
    ) -> Result<NutritionalInfo> {
        let url = self.endpoint("Product-ProductNutritionalInfoTab");
        debug!("Fetching nutrition for pid={pid}, ean={ean}");

        let response = self
            .client
            .get(&url)
            .query(&[
                ("pid", pid),
                ("ean", ean),
                ("supplierid", supplier_id),
                ("enabledce", "true"),
            ])
            .send()
            .await?
            .error_for_status()?;

        let html = response.text().await?;
        Ok(scraper::parse_nutritional_info(&html))
    }

    pub async fn suggest(&self, query: &str) -> Result<SuggestionResult> {
        if query.len() < 5 {
            return Err(ContinenteError::Parse {
                url: String::from("SearchServices-GetSuggestions"),
                message: String::from("Query must be at least 5 characters"),
            });
        }

        let url = self.endpoint("SearchServices-GetSuggestions");
        debug!("Getting suggestions for '{query}'");

        let response = self
            .client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?
            .error_for_status()?;

        let html = response.text().await?;
        Ok(scraper::parse_suggestions(&html))
    }

    pub async fn stores(&self, lat: f64, lon: f64, radius: u32) -> Result<Vec<Store>> {
        let url = self.endpoint("Stores-FindStores");
        debug!("Finding stores near ({lat}, {lon}) radius={radius}km");

        let response = self
            .client
            .get(&url)
            .query(&[
                ("lat", &lat.to_string()),
                ("long", &lon.to_string()),
                ("radius", &radius.to_string()),
            ])
            .send()
            .await?
            .error_for_status()?;

        let stores_response: StoresResponse =
            response.json().await.map_err(|e| ContinenteError::Parse {
                url: format!("Stores-FindStores?lat={lat}&long={lon}"),
                message: e.to_string(),
            })?;

        Ok(stores_response.stores)
    }
}

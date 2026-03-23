use std::time::Duration;

use reqwest::Client;
use tracing::debug;

use crate::api::models::{
    Flyer, NutritionalInfo, ProductDetail, ProductVariationResponse, SearchParams, SearchResponse,
    Store, StoresResponse, SuggestionResult,
};
use crate::api::scraper;
use crate::config::HttpConfig;
use crate::error::{ContinenteError, Result};

// =============================================================================
// Continente SFCC Controller Endpoints Reference
// Base: https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/{Controller}
// Full investigation: docs/investigation.md
// =============================================================================
//
// --- Search & Products (implemented) ---
// Search-ShowAjax                       GET  HTML   Product search + category browsing
// SearchServices-GetSuggestions         GET  HTML   Autocomplete (min 5 chars)
// Product-Variation                     GET  JSON   Product detail (best endpoint)
// Product-ProductNutritionalInfoTab     GET  HTML   Nutritional info table
//
// --- Search & Products (not implemented) ---
// Search-UpdateGrid                     GET  HTML   Product grid update (alt search)
// Product-Show                          GET  HTML   Full product page
// Product-ShowQuickView                 GET  HTML   Quick view modal
// Product-ShowNote                      GET  HTML   Product notes
//
// --- Stores (implemented) ---
// Stores-FindStores                     GET  JSON   Find stores by lat/lon/radius
//
// --- Stores (not implemented) ---
// Stores-GetCoverageArea                GET  JSON   Delivery coverage area
// Stores-GetDelivery                    GET  JSON   Delivery options
// Stores-SetStoreContext                POST        Set active store context
//
// --- Cart & Checkout (requires auth) ---
// Cart-AddProduct                       POST        Add product to cart
// Cart-RemoveProductLineItem            POST        Remove product from cart
// Cart-RemoveAllProductLineItems        POST        Clear entire cart
// Cart-UpdateQuantity                   POST        Update product quantity
// Cart-DimensionUpdate                  POST        Update product dimensions
// Cart-MiniCart                          GET         Mini cart data
// Cart-MiniCartShow                      GET  HTML   Mini cart display
// Cart-GetCustomerBenefits              GET         Loyalty coupons/benefits
// Cart-HasEntregaZeroInCart             GET         Check for free delivery items
// CheckoutServices-BookDeliverySlot     POST        Book delivery time slot
// CheckoutServices-BookDeliverySlotForEntregaZero  POST  Free delivery slot
// CheckoutServices-GetPayPalButton      GET  HTML   PayPal integration
//
// --- Account & Auth (requires auth) ---
// Account-Login                         POST        Login
// Account-OpenSiteCredentials           GET  HTML   Credentials iframe
// Account-UpdateCredentialsCustomerData POST        Update customer data
//
// --- Wishlist (requires auth) ---
// Wishlist-AddProduct                   POST        Add to wishlist (302 redirect without auth)
// Wishlist-GetProductIds                GET  JSON   Get wishlist product IDs
// Wishlist-RemoveProduct                POST        Remove from wishlist
// INVESTIGATED: GetProductIds returns 200 with `{"productIds":[]}` without auth.
// The wishlist is session-scoped — always empty without a logged-in session.
// AddProduct returns 302 (redirect to login). Not useful without OIDC auth flow.
//
// --- Pages & Content (not implemented) ---
// Page-IncludeHeaderMenuAjax            GET  HTML   Header menu
// Page-ManagerDecisionBanner            GET  HTML   Banner management
// Page-ManagerDecisionSponsored         GET  HTML   Sponsored placements
// Page-MobileAppValidation              GET         Mobile app check
// Page-RenderTemplate                   GET  HTML   Generic template rendering
// IPaper-AddedFromIPaperTemplate        POST        Flyer product import
// LoginOnBehalf-DisplayOobHeader        GET  HTML   OOB header
// INVESTIGATED: Page-ManagerDecisionSponsored returns HTTP 247 with bot protection
// (rbzns/winsocks JS challenge). Blocked for programmatic access. Kevel ad platform
// handles sponsored product placement server-side.
//
// --- Consent & Other (not implemented) ---
// Consent-ConsentData                   GET  JSON   Cookie consent data
// ConsentTracking-GetContent            GET  HTML   Consent tracking content
// ConsentTracking-SetSession            POST        Set consent session
// EmailSubscribe-Subscribe              POST        Newsletter subscription
//
// --- Authentication (§8 — third-party: login.continente.pt) ---
// Provider: login.continente.pt (OpenID Connect)
// Issuer: identity.sonaemc.com
// Client ID: NLR6WHyO8Iba4eRS
// Grant types: authorization_code, client_credentials, refresh_token, implicit, device_code
// PKCE: S256 | Scopes: openid, email, offline_access
// Discovery: https://login.continente.pt/.well-known/openid-configuration
// Key endpoints: /connect/authorize, /connect/token, /api/auth/authorize,
//   /api/credentials/authorize, /api/username, /api/web/registration/users
//
// --- Flyers & Catalogs (§9 — third-party: iPaper) ---
// Platform: iPaper at folhetos.continente.pt (viewer.ipaper.io)
// Account hash: 2fbc450f-0fbc-450b-9a35-444356442a01
// CDN: b-cdn.ipaper.io (page images + enrichment JSON)
//
// --- OutSystems Mobile Services (§11.1 — mobile app only) ---
// screenservices/ContinenteOnline/ActionServerDataSync
// screenservices/ContinenteOnline/Common/Splash/DataActionGetFeatureToggle
// screenservices/ContinenteOnline/MainFlow/Open_IAB/ActionGetToggleAndDomain
//
// --- Third-Party Service IDs (§11.4–11.5) ---
// CQuotient Einstein: bdvs-continente (product recommendations)
// Syndigo: 710ff2e0-2f41-1551-6979-4c465865d7d1 (product content)
// Cookiebot: 86a29107-94de-4370-a781-af2d18aa1e48 (consent)
// Kevel: col (advertising/sponsored products)
// =============================================================================

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

    /// Search products via Search-ShowAjax. See investigation §4 for HTML response format.
    /// Product data is in `data-product-tile-impression` attributes as HTML-encoded JSON.
    pub async fn search(&self, query: &str, params: &SearchParams) -> Result<SearchResponse> {
        let url = self.endpoint("Search-ShowAjax");
        debug!("Searching for '{query}' with params: {params:?}");

        let mut request = self
            .client
            .get(&url)
            .query(&[("q", query)])
            .query(&[("cgid", "col-produtos")])
            .query(&[("start", &params.start.to_string())])
            .query(&[("sz", &params.size.to_string())]);

        // Only add default pmin if user didn't specify one (avoids duplicate param → 500)
        let pmin = params.price_min.unwrap_or(0.01);
        request = request.query(&[("pmin", &pmin.to_string())]);

        if let Some(sort) = &params.sort {
            request = request.query(&[("srule", sort.as_str())]);
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

    /// Browse a category via Search-ShowAjax with `cgid` param. Same HTML format as search (§4).
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

    /// Fetch product detail JSON via Product-Variation. See investigation §5.
    ///
    /// Returns aggregate `rating` (e.g., 3.9) but no individual reviews.
    /// Product-Show JSON-LD has `ratingCount` but no review text — no dedicated
    /// reviews endpoint exists. No third-party review platform detected.
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

    /// Fetch nutritional info HTML via Product-ProductNutritionalInfoTab. See investigation §5.
    /// Requires pid, ean, and `supplier_id` from the product detail response.
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

    /// Autocomplete suggestions via SearchServices-GetSuggestions. See investigation §6.
    /// Requires min 5 characters.
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

    /// Find stores by location via Stores-FindStores. See investigation §7.
    /// Returns JSON with store details (228 known stores, no per-product stock).
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

    /// Fetch current flyers from the folhetos page. See investigation §9.
    /// Parses iPaper tile HTML from `https://www.continente.pt/folhetos/`.
    pub async fn flyers(&self) -> Result<Vec<Flyer>> {
        let url = format!("{}/folhetos/", self.base_url);
        debug!("Fetching flyers from {url}");

        let response = self.client.get(&url).send().await?.error_for_status()?;

        let html = response.text().await?;
        scraper::parse_flyers(&html)
    }
}

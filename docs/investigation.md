# Continente Online - Reverse Engineering Investigation

## Overview

**App**: Continente Online (pt.continente.LojaContinente)
**Platform**: Android
**APK Size**: ~20 MB
**Version**: 5.50.1
**Investigation Date**: 2026-03-22

Goal: Understand the Continente app's API layer to build a Rust CLI for browsing products, since the website is difficult to crawl directly.

---

## 1. App Architecture

The Continente app is **not a native Android app**. It's a **Cordova/OutSystems hybrid wrapper** that loads the continente.pt website inside an InAppBrowser (WebView).

### Boot Flow

1. App launches `MainActivity` (Cordova activity)
2. Loads OutSystems runtime from `https://lojacontinenteonline.sonae.pt/ContinenteOnline/`
3. Calls `screenservices/ContinenteOnline/ActionServerDataSync` to get a `SourceUrl`
4. That URL resolves to `https://continente.pt` (production)
5. URL is loaded in InAppBrowser with `?isMobileApp=true` appended
6. All product browsing happens on the website, not via native API calls

### Tech Stack

| Layer | Technology |
|-------|------------|
| Native Shell | Apache Cordova |
| App Framework | OutSystems Platform (low-code) |
| Client Platform Version | 11.33.0.44426 |
| E-commerce Backend | Salesforce Commerce Cloud (Demandware) |
| Recommendations | CQuotient / Einstein (`bdvs-continente`) |
| Push Notifications | Firebase Cloud Messaging |
| Marketing | Salesforce Marketing Cloud |
| Analytics | Google Tag Manager (GTM-ND3VMBM), New Relic |
| A/B Testing | Visual Website Optimizer (VWO, account 1047352) |
| Flyer Platform | iPaper (`folhetos.continente.pt`) |
| Auth Provider | OpenID Connect (`login.continente.pt`, issuer: `identity.sonaemc.com`) |

### OutSystems Configuration

- **Application Key**: `5a233775-a54f-44a5-a882-b9117b1410db`
- **Environment Key**: `e7078ce4-4153-4b50-ac01-95423aebed56`
- **Environment Name**: "Production Ext"

---

## 2. Environments & Hosts

| Domain | Environment | Purpose |
|--------|-------------|---------|
| `lojacontinenteonline.sonae.pt` | Production | OutSystems app host |
| `lojacontinenteonlinepp.sonae.pt` | Pre-Production | OutSystems app host |
| `continente.pt` | Production | Website / SFCC storefront |
| `quality.continente.pt` | QA | Staging environment |
| `integration.continente.pt` | Integration | Test environment |
| `www.betacontinente.com` | Beta | Beta testing |
| `login.continente.pt` | Production | OpenID Connect auth provider |
| `cartaocontinente.pt` | Production | Loyalty card portal |
| `folhetos.continente.pt` | Production | Weekly flyers (iPaper) |
| `loja-continente.firebaseio.com` | Production | Firebase Realtime DB |
| `api.cquotient.com` | Production | Einstein recommendations |
| `app.igodigital.com` | Production | Predictive intelligence |

---

## 3. Security Findings

### 3.1 SSL Certificate Pinning

**Config file**: `assets/www/pinning/pinning.json`

Pinned hosts with SHA-256 hashes:
- `lojacontinenteonline.sonae.pt` (4 pins)
- `lojacontinenteonlinepp.sonae.pt` (4 pins)

Pin hashes (shared between both hosts):
```
sha256/ks/n3Ayi0KpFO5h+o52sfBLQHPIofYVfmfavSWjn5U0=
sha256/ASgDeYDS5eyzWVR5W6sHq/l0wOOXwpD512A7pF1m4Q0=
sha256/0zmo0NMcNBmX1HYGdJtbivu8rDh+jN/EEVxeGNnImPQ=
sha256/PO7KxZzQg34vPtbMgKTGkv9Ievp6DGy0br0cdDvO258=
```

**Implementation**: OkHttp `CertificatePinner` + OutSystems SSL Pinning Plugin (`com.outsystems.plugins.sslpinning`)

> **Note**: Pinning only applies to the OutSystems host (`lojacontinenteonline.sonae.pt`), NOT to `continente.pt` (the SFCC storefront). This means the product browsing API on continente.pt has no pinning protection.

### 3.2 Authentication & Headers

| Header | Purpose | Where Used |
|--------|---------|------------|
| `Authorization: Bearer {token}` | Marketing Cloud auth | Salesforce MC API calls |
| `X-Subscriber-Token` | Subscriber identification | Salesforce MC |
| `outsystems-device-uuid` | Device fingerprint | OutSystems cache/logger |
| `X-SDK-Version` | SDK version tracking | Salesforce MC |
| `User-Agent` | Custom UA with device info | All HTTP calls |

The OutSystems `screenservices` calls use API version tokens (e.g., `3rcavE0vRZttADx6kRfUqg`) as a form of versioning/validation, but these are not secret — they're embedded in the JS bundles.

### 3.3 Hardcoded Firebase Credentials

Found in `assets/www/prd-google-services.json`, `pp-google-services.json`, `dev-google-services.json`:

| Environment | Project ID | API Key |
|-------------|-----------|---------|
| Production | `loja-continente` | `AIzaSyB2KPjEIhE_wABj5Et6KBs3UTAKjRkVDtk` |
| Pre-Production | `app-col-pp` | `AIzaSyAuaw6qcE5INJl4XhvRYZInsiLiufwM4Pk` |
| Development | `app-col-dev` | `AIzaSyCZkultUuGweqGmYF71Ro3QJn4z6MkawrY` |

> These are standard Firebase API keys scoped to the project — not a vulnerability per se, but they reveal the full project IDs and could be used for further enumeration.

### 3.4 Encryption

- Android KeyStore with alias `com.salesforce.androidsdk.security.KEYPAIR`
- AES-256 encryption for sensitive data
- `EncryptedSharedPreferences` for tokens/registration IDs
- Keys are NOT backed up (excluded from cloud sync)

### 3.5 OCAPI (Open Commerce API)

Salesforce Commerce Cloud exposes OCAPI at:
```
https://www.continente.pt/s/continente/dw/shop/v23_2/product_search?q=...&client_id=XXXX
```

**Status**: Returns 401 `UnknownClientIdException`. The OCAPI `client_id` is **NOT exposed anywhere in the frontend JS bundles** (searched main.js, search.js, vendors.js, vendors-additional.js, pd.js, chatBot.js, dwanalytics, dwac, fullPage.js, and login app JS). Tested 30+ client_id variations — all return 401. The 30-char demo ID returns `DemoClientIdException` confirming this is a production instance running OCAPI v25.6.

**Conclusion**: Continente does NOT use OCAPI for its storefront. They use standard SFCC Controller endpoints exclusively, which is actually better for our purposes — no auth needed.

### 3.6 Rate Limiting

**No rate limiting detected.** 20 rapid sequential requests to `Search-ShowAjax` all returned HTTP 200 with consistent response times (554-683ms). No 429 or 403 responses. No progressive slowdown.

---

## 4. Product Search API (Working - No Auth Required)

### 4.1 Search Endpoint

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Search-ShowAjax
```

Returns HTML fragments containing structured product data in `data-` attributes. ~700KB per response.

There is also an alternate endpoint:
```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Search-UpdateGrid
```

### 4.2 Parameters

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `q` | string | Search query | `leite`, `arroz` |
| `cgid` | string | Category ID filter | `col-produtos`, `animais-cao` |
| `start` | int | Pagination offset | `0`, `24`, `48` |
| `sz` | int | Page size (results per page) | `24` (default), up to ~72 |
| `pmin` | float | Minimum price filter | `0.01` |
| `pmax` | float | Maximum price filter | `5.00` |
| `prefn1` | string | Filter attribute name | `brand` |
| `prefv1` | string | Filter attribute value | `Mimosa` |
| `srule` | string | Sort rule | `Continente` (default) |

### 4.3 Sort Rules

| Value | Description |
|-------|-------------|
| `Continente` | Default relevance sort |
| `price-low-to-high` | Price ascending |
| `price-high-to-low` | Price descending |
| `price-per-capacity-ascending` | Unit price ascending |
| `product-name-ascending` | Name A-Z |
| `product-name-descending` | Name Z-A |

### 4.4 Search Refinements (Filters)

| Parameter | Values | Description |
|-----------|--------|-------------|
| `prefn1=brand` | Brand name | Filter by brand |
| `prefn1=food.Biologic` | `true` | Organic products |
| `prefn1=food.GlutenFree` | `true` | Gluten free |
| `prefn1=food.LactoseFree` | `true` | Lactose free |
| `prefn1=food.SugarFree` | `true` | Sugar free |
| `prefn1=food.Vegan` | `true` | Vegan products |
| `prefn1=food.Vegetarian` | `true` | Vegetarian products |

Multiple filters can be combined: `prefn1=brand&prefv1=Mimosa&prefn2=food.LactoseFree&prefv2=true`

### 4.5 Required Headers

Only a standard browser User-Agent is needed:

```
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36
```

### 4.6 Response Format

Product data is embedded in the `product-tile-impression` attribute as HTML-encoded JSON:

```html
<div class="product" data-pid="6879912">
  <div class="ct-inner-tile product-tile-impression='{"name":"Leite UHT Meio Gordo Continente","id":"6879912","price":0.86,"brand":"Continente","category":"Laticínios e Ovos/Leite/Leite Meio Gordo","variant":"","channel":"col"}'>
```

### 4.7 Extracted Product Data Structure

```json
{
  "name": "Leite UHT Meio Gordo Continente",
  "id": "6879912",
  "price": 0.86,
  "brand": "Continente",
  "category": "Laticínios e Ovos/Leite/Leite Meio Gordo",
  "variant": "",
  "channel": "col"
}
```

### 4.8 Price Structure in HTML

Products can have multiple price tiers:

| CSS Class | Description | Example |
|-----------|-------------|---------|
| `.prices-wrapper > .list` | PVPR (recommended retail price) | `27,99EUR` |
| `.pwc-tile--price-primary` | Continente selling price | `14,95EUR` |
| `.pwc-tile--price-secondary` | Unit price | `0,86EUR/lt` |
| `.pwc-tile--price-sdr` | Deposit/SDR amount | (if applicable) |

### 4.9 Promotion Badges

Products with PVPR show promotional badges:
- Badge class: `ct-product-tile-badge--promotional`
- Badge image: `/images/badges/pvpr/col/pvpr.png`
- Discount: `<span class="ct-product-tile-badge-value--pvpr">50</span>%`

Example discount data:
| Product | PVPR | Continente Price | Discount |
|---------|------|-----------------|----------|
| Super Bock Mini 24-pack | 27,99EUR | 14,95EUR | >46% |
| Sagres Mini 24-pack | 28,99EUR | 14,35EUR | ~50% |

### 4.10 Additional Data in HTML

- **Total results count**: `data-gtm-results="1314"` attribute
- **Add to cart URL**: `Cart-AddProduct` controller endpoint
- **Category facets**: Available as `Search-ShowAjax` links with `cgid` parameter
- **Brand facets**: Available with `prefn1=brand&prefv1={brand_name}` parameters

### 4.11 Image URL Pattern

```
https://www.continente.pt/dw/image/v2/BDVS_PRD/on/demandware.static/-/Sites-col-master-catalog/default/{hash}/images/col/{id_prefix}/{product_id}-frente.jpg?sw={width}&sh={height}
```

- `{hash}` — changes per deploy (e.g., `dw66d59f15`)
- `{id_prefix}` — first 3 digits of product ID (e.g., `687` for `6879912`)
- `sw` / `sh` — requested dimensions (280x280 tile, 1000x1000 quickview, 2000x2000 full)

> The image hash changes between deploys, so image URLs should be extracted from the HTML rather than constructed manually.

### 4.12 Example Queries

```bash
# Search for milk products
GET /on/demandware.store/Sites-continente-Site/default/Search-ShowAjax?cgid=col-produtos&q=leite&start=0&sz=24&pmin=0.01

# Search for rice, filter by brand
GET /on/demandware.store/Sites-continente-Site/default/Search-ShowAjax?cgid=col-produtos&q=arroz&prefn1=brand&prefv1=Cigala&start=0&sz=24

# Browse a category
GET /on/demandware.store/Sites-continente-Site/default/Search-ShowAjax?cgid=animais-cao&start=0&sz=24

# Sort by price ascending
GET /on/demandware.store/Sites-continente-Site/default/Search-ShowAjax?q=cerveja&srule=price-low-to-high&start=0&sz=24

# Vegan products only
GET /on/demandware.store/Sites-continente-Site/default/Search-ShowAjax?q=leite&prefn1=food.Vegan&prefv1=true&start=0&sz=24
```

---

## 5. Product Detail API (JSON - No Auth Required)

### 5.1 Product-Variation Endpoint (JSON)

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Product-Variation?pid={PRODUCT_ID}
```

**This is the richest endpoint** — returns a full JSON response with comprehensive product data.

### 5.2 Response Fields

#### Core Product Data

| Field | JSON Path | Example |
|-------|-----------|---------|
| Product ID | `product.id` | `"6879912"` |
| UUID | `product.uuid` | `"2d9bf3c61057f54f2671670a20"` |
| Product Name | `product.productName` | `"Leite UHT Meio Gordo Continente"` |
| Brand | `product.brand` | `"Continente"` |
| Product Type | `product.productType` | `"standard"` |
| Short Description | `product.shortDescription` | `"Um delicioso leite meio gordo..."` |
| Rating | `product.rating` | `3.9` |
| Available | `product.available` | `true` |
| Online | `product.online` | `true` |
| Ready to Order | `product.readyToOrder` | `true` |
| Product URL (SEO) | `product.productURL` | `"/produto/leite-uht-meio-gordo-continente-continente-6879912.html"` |

#### Pricing

| Field | JSON Path | Example |
|-------|-----------|---------|
| Sale Price | `product.price.sales.value` | `0.86` |
| Sale Price Formatted | `product.price.sales.formatted` | `"0,86EUR"` |
| List/Original Price | `product.price.list.value` | `1.00` (when on promotion) |
| Currency | `product.price.sales.currency` | `"EUR"` |
| Promotion End Date | `product.price.onlineTo` | `"2026-03-23T23:59:59.000Z"` |
| Price Per Unit (primary) | `product.pricePerUnit.primaryPrice` | `{price: {value: 0.86}, unit: "un"}` |
| Price Per Unit (secondary) | `product.pricePerUnit.secondaryPrice` | `{price: "0,86EUR", unit: "lt"}` |

#### Ordering

| Field | JSON Path | Example |
|-------|-----------|---------|
| Min Order Qty | `product.minOrderQuantity` | `6` (milk) / `1` (rice) |
| Max Order Qty | `product.maxOrderQuantity` | `10` |
| Step Quantity | `product.measurementInfo.quantityConversionRates.stepQuantity` | `6` |
| Measurement Note | `product.measurementNote` | `"emb. 1 lt"` |
| Primary Unit | `product.measurementInfo.quantityConversionRates.primaryunit` | `"un"` |
| Secondary Unit | `product.measurementInfo.quantityConversionRates.secondaryunit` | `"lt"` |

#### Category

| Field | JSON Path | Example |
|-------|-----------|---------|
| Category ID | `product.category.primaryCategoryId` | `"laticinios-leite-meio-gordo"` |
| Category Name | `product.category.primaryCategoryDisplayName` | `"Leite Meio Gordo"` |
| Top Category ID | `product.category.primaryCategoryTopLevelProductCategoryId` | `"laticinios"` |
| Top Category Name | `product.category.primaryCategoryTopLevelProductCategoryDisplayName` | `"Laticinios e Ovos"` |
| GTM Category Path | `product.gtmCategoryPath` | `"Laticinios e Ovos/Leite/Leite Meio Gordo"` |

#### Badges & Promotions

| Field | JSON Path | Example |
|-------|-----------|---------|
| General Badge | `product.badgeInfo.general.title` | `"Produzido em Portugal"` |
| General Badge Image | `product.badgeInfo.general.img` | Badge PNG URL |
| Promo Badge | `product.badgeInfo.promo.title` | `"PVP Recomendado: 1,00EUR/un"` |
| Pictograms | `product.badgeInfo.pictograms[]` | Array of badge objects |
| SDR (Deposit) | `product.productSDR` | `{isDeposit: false, hasDeposit: false}` |

#### Images (Multiple Sizes)

| Size | JSON Path | Resolution |
|------|-----------|-----------|
| Confirmation | `product.confirmationImage.url` | 87x80px |
| Tile | `product.productTileImage.url` | 280x280px |
| Quick View | `product.quickViewImages[].url` | 1000x1000px |
| PDP (full res) | `product.pdpImages[].url` | 2000x2000px |

#### EAN / Barcode

EAN codes are NOT in the main JSON response. They are embedded in the `nutritionalInfoUrlString` field:
```
Product-ProductNutritionalInfoTab?pid={PID}&ean={EAN}&supplierid={SUPPLIER_ID}&enabledce=true
```

Some products have multiple EANs separated by `|` (e.g., `5601049132995|2100221094609`).

#### Product Attributes (100+ possible fields)

The `productTabs` array contains 3 tabs with attribute keys. Key populated attributes:

**Tab 1 - "Sobre este Produto" (About):**
`food.Ingredients`, `food.Allergens`, `food.NutricionalInformation`, `food.Origin`, `food.Region`, `food.Milktype`, `food.ProductType`, `food.Alcoholcontent`, `food.SizeKg`, `food.GrapeVarieties`, `food.TextureIntensity`, `food.Flavour`, `food.FishingType`, `food.CaptureZone`

**Tab 2 - "Caracteristicas" (Characteristics):**
`food.Benefits`, `food.FatContent`, `food.Conservation`, `food.Howtouse`, `food.Howtoconsume`, `food.Tips`, `food.Producer'sName`, `food.Producer'sAdress`, `food.Recommendeddailyamounts`

**Tab 3 - "Outras Informacoes" (Other):**
`common.Disclaimer`, `common.Instructions`, `common.AdditionalInformation`, `common.Precautions`, `common.Package`, `non.food.Warranty`

### 5.3 Nutritional Data Endpoint (HTML)

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Product-ProductNutritionalInfoTab?pid={PID}&ean={EAN}&supplierid={SUPPLIER_ID}&enabledce=true
```

The EAN and supplier ID are extracted from `product.nutritionalInfoUrlString` in the Product-Variation response.

#### Available Nutritional Fields

| CSS Class | Field | Example |
|-----------|-------|---------|
| `.regulated-product-name` | Regulatory product name | "LEITE MEIO GORDO UHT FONTE DE CALCIO" |
| `.contact-information--name` | Producer name | "LACTOGAL Produtos Alimentares, S.A." |
| `.contact-information--address` | Producer address | "Rua Joao Mendonca, 505..." |
| `.ingredients` | Ingredients | "LEITE UHT meio-gordo." |
| `.net-content` | Net content | "1000" |
| `.net-content--uom` | Unit of measure | "(GRM) Grama" |
| `.net-weight` | Net weight | "1000" |
| `.storage-instruction` | Storage conditions | "Local fresco e seco..." |
| `.allergen-statement` | Allergen declaration | "Contem Leite." |
| `.country-origin` | Country of origin | "Produzido em Portugal" |
| `.preparation-instructions` | Preparation | "Nao necessita ser fervido." |
| `.daily-value-intake-reference` | Reference intake | "8400kj/2000kcal" |
| `.serving-size` | Serving size | "100" |
| `.serving-size--uom` | Serving unit | "(MLT) Mililitro" |
| `.nutrients-table` | Full nutrition table | See below |

#### Nutrition Table Fields (per 100ml/100g)

| Nutrient | Portuguese | Example | Unit |
|----------|-----------|---------|------|
| Energy | Energia | 200.0 | kJ |
| Energy | Energia | 48.0 | kcal |
| Fat | Lipidos | 1.6 | g |
| Saturated fat | Saturados | 1.0 | g |
| Carbohydrates | Hidratos de carbono | 4.9 | g |
| Sugars | Acucares | 4.9 | g |
| Fiber | Fibra | 0.0 | g |
| Protein | Proteinas | 3.4 | g |
| Salt | Sal | 0.1 | g |
| Calcium | Calcio | 120.0 | mg |
| Phosphorus | Fosforo | 93.0 | mg |
| Potassium | Potassio | 159.0 | mg |
| Iodine | Iodo | 20.0 | mcg |
| Riboflavin | Riboflavina | 0.17 | mg |
| Vitamin B-12 | Vitamina B-12 | 0.2 | mcg |

> Note: The number of nutrients varies by product. Brand-name products tend to have more detailed listings than own-brand.

### 5.4 Product Quick View (HTML)

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Product-ShowQuickView?pid={PID}
```

Returns HTML with embedded product data similar to Product-Variation.

### 5.5 JSON-LD Structured Data (Product Page)

Product pages at the SEO URL contain JSON-LD:
```json
{
  "@type": "Product",
  "name": "Leite UHT Meio Gordo Continente",
  "description": "...",
  "mpn": "6879912",
  "sku": "6879912",
  "brand": {"@type": "Thing", "name": "Continente"},
  "image": ["..."],
  "offers": {
    "@type": "Offer",
    "priceCurrency": "EUR",
    "price": "0.86",
    "priceValidUntil": "2027-03-22",
    "availability": "http://schema.org/InStock"
  },
  "aggregateRating": {
    "@type": "AggregateRating",
    "ratingCount": 1,
    "ratingValue": 3.9
  }
}
```

---

## 6. Search Suggestions API (No Auth Required)

### 6.1 Endpoint

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/SearchServices-GetSuggestions?q={query}
```

**Minimum query length**: 5 characters (configured via `data-min-chars="5.0"` on the search form). Queries under 5 chars return a JSON stub with no results.

### 6.2 Response Format

Returns HTML (not JSON) with 3 sections:

**1. "Sugestoes Principais" (Top Products)** — 3 products with:
- Product ID (`data-pid`)
- Product name, brand, category
- Price (current and original if on sale)
- Image URL (280x280)
- Product SEO URL (e.g., `/produto/leite-uht-meio-gordo-continente-continente-6879912.html`)
- Unit/package info (e.g., `emb. 1 lt`)
- Unit price (e.g., `0,86 EUR/lt`)
- `data-product-tile-impression` JSON

**2. "Categorias" (Category Suggestions)** — 3 categories:
- e.g., `https://www.continente.pt/laticinios-e-ovos/leite/?q=leite`

**3. "Mais pesquisados" (Popular Terms)** — 3 terms:
- e.g., `/pesquisa/?q=leite%20magro`

### 6.3 Other Suggestion Endpoints (Non-functional)

| Endpoint | Status |
|----------|--------|
| `Search-GetSuggestions` | 500 |
| `Search-Suggest` | 500 |
| `SearchServices-Suggest` | 500 |
| `SearchServices-GetPopularSearches` | 500 |

---

## 7. Store Locator API (JSON - No Auth Required)

### 7.1 Endpoint

```
GET https://www.continente.pt/on/demandware.store/Sites-continente-Site/default/Stores-FindStores?lat={lat}&long={long}&radius={km}
```

### 7.2 Response

Clean JSON with `stores` array. Example query with `radius=300` from center of Portugal returns **228 stores**.

### 7.3 Store Data Fields

| Field | Example |
|-------|---------|
| `ID` | Store identifier |
| `name` | Store name |
| `address1` | Street address |
| `address2` | Additional address |
| `city` | City name |
| `postalCode` | Postal code |
| `latitude` | GPS latitude |
| `longitude` | GPS longitude |
| `phone` | Phone number |
| `stateCode` | State/district code |
| `countryCode` | Country code |
| `storeHours` | Opening hours |
| `isGalpStore` | Is gas station store |
| `isPickupStore` | Supports click & collect |

### 7.4 Store Types

- `col` (Continente) — 222 stores
- `Zu` (pet stores) — 6 stores
- 203 of 228 (89%) support pickup

### 7.5 Other Store Endpoints

| Endpoint | Status | Description |
|----------|--------|-------------|
| `Stores-GetCoverageArea` | Unknown | Get delivery coverage area |
| `Stores-GetDelivery` | Unknown | Get delivery options |
| `Stores-SetStoreContext` | Unknown | Set active store |
| `StoreLocator-Find` | 500 | Not functional without extra params |

> **No per-product stock data** is exposed through these endpoints.

---

## 8. Authentication & Loyalty System

### 8.1 OpenID Connect Configuration

| Setting | Value |
|---------|-------|
| Provider | `login.continente.pt` |
| Issuer | `identity.sonaemc.com` |
| OIDC Discovery | `https://login.continente.pt/.well-known/openid-configuration` |
| Client ID | `NLR6WHyO8Iba4eRS` |
| Grant Types | `authorization_code`, `client_credentials`, `refresh_token`, `implicit`, `device_code` |
| PKCE Support | Yes (S256) |
| Scopes | `openid`, `email`, `offline_access` |

### 8.2 Login API Endpoints (login.continente.pt)

| Endpoint | Purpose |
|----------|---------|
| `/api/auth/authorize` | Main authorization |
| `/api/credentials/authorize` | Credential authorization |
| `/api/credentials/authorize/sso` | SSO authorization |
| `/api/username` | Mobile sign-in (username + clientId) |
| `/api/web/registration/users` | User registration |
| `/api/otp/resend` | OTP resend |
| `/api/otp/loyalty/resend` | Loyalty card OTP resend |
| `/api/password-recover/resend-otp` | Password recovery |
| `/api/digital-asset` | Digital asset info |
| `/api/fido2/credential-options` | WebAuthn/FIDO2 support |
| `/api/fido2/assertion-options` | FIDO2 assertion |
| `/connect/authorize` | OIDC authorize endpoint |
| `/connect/token` | OIDC token endpoint |

### 8.3 Loyalty Card (Cartao Continente)

**Card number formats** (from main.js regex validation):
| Type | Pattern | Example |
|------|---------|---------|
| Loyalty card | `^185[0-9]{10}$` or `^225[0-9]{10}$` | 13 digits, starts with 185 or 225 |
| Cartao Da | `^1111[0-9]{15}$` | 19 digits, starts with 1111 |
| Voucher | `^185[0-9]{5}$` | 8 digits, starts with 185 |
| Gift card | `^185[0-9]{15}$` | 18 digits, starts with 185 |
| Security code | `[0-9]{6}$` | 6 digits |
| Display mask | `XXX XXX XX0 000X` | |

**Payment methods supporting loyalty:**
`LoyaltyCard`, `ContinentePay`, `ContinentePayDa`, `GiftCard`, `CreditCard`, `PayPal`, `MBWay`

**Loyalty integration in cart:**
- `Cart-GetCustomerBenefits` — fetch loyalty coupons/benefits
- Cart shows `loyaltyBalance` and `cartaodaBalance`
- Loyalty events tracked via GTM with event name `continente_card`

**External portal**: `cartaocontinente.pt` (separate from SFCC)

### 8.4 Deep Link Actions (Mobile App)

`sc_home`, `sc_loyaltyCard_definition`, `sc_phoneNumber_definition`, `sc_phoneNumber_update`, `sc_email_definition`, `sc_email_update`, `sc_password_definition`, `sc_password_update`, `sc_faqs`

---

## 9. Flyers & Catalogs

### 9.1 Flyer Page

```
https://www.continente.pt/folhetos/
```

Title: "Folhetos Semanais | Continente Online". Returns HTML (891KB) listing current flyers.

### 9.2 Platform

Flyers hosted on **iPaper** at `folhetos.continente.pt` (viewer: `viewer.ipaper.io`).

### 9.3 Current Flyers (March 2026 sample)

| Slug | Description |
|------|-------------|
| `semanal-12-jeu9` | Weekly flyer |
| `semanal-12-cbd-ql98` | Weekly CBD edition |
| `semanal-12-mad-aq12` | Weekly Madeira edition |
| `fim-de-semana-s12-mm11` | Weekend flyer |
| `frescos-1-ml90` | Fresh products |
| `pascoa-sa83` | Easter campaign |
| `catalogo-ta-pascoa-lm22` | Easter catalog |
| `skin-week-1-ss09` | Skin week promotion |
| `acoresgenericogs1213` | Azores edition |
| `continente-magazine-175` | Magazine |

### 9.4 iPaper Technical Details

- Page images served from `b-cdn.ipaper.io`
- Enrichment data (clickable product hotspots) available in JSON chunks
- Account hash: `2fbc450f-0fbc-450b-9a35-444356442a01`
- Flyer example (semanal-12): Paper ID 2992031, License ID 17008, 52 pages

### 9.5 Flyer-to-Cart Integration

SFCC controller `IPaper-AddedFromIPaperTemplate` allows adding products directly from flyer enrichment data.

---

## 10. Complete SFCC Controller Endpoint List

All discovered Salesforce Commerce Cloud controller endpoints:

### Search & Products
| Endpoint | Purpose |
|----------|---------|
| `Search-ShowAjax` | Product search with HTML fragments |
| `Search-UpdateGrid` | Product grid update (alternative search) |
| `SearchServices-GetSuggestions` | Search autocomplete/suggestions |
| `Product-Variation` | Product detail JSON |
| `Product-Show` | Full product page HTML |
| `Product-ShowQuickView` | Product quick view HTML |
| `Product-ProductNutritionalInfoTab` | Nutritional info HTML |
| `Product-ShowNote` | Product notes |

### Cart & Checkout
| Endpoint | Purpose |
|----------|---------|
| `Cart-AddProduct` | Add product to cart |
| `Cart-RemoveProductLineItem` | Remove product from cart |
| `Cart-RemoveAllProductLineItems` | Clear cart |
| `Cart-UpdateQuantity` | Update product quantity |
| `Cart-DimensionUpdate` | Update product dimensions |
| `Cart-MiniCart` | Mini cart data |
| `Cart-MiniCartShow` | Mini cart display |
| `Cart-GetCustomerBenefits` | Loyalty benefits/coupons |
| `Cart-HasEntregaZeroInCart` | Check for free delivery items |
| `CheckoutServices-BookDeliverySlot` | Book delivery slot |
| `CheckoutServices-BookDeliverySlotForEntregaZero` | Book free delivery slot |
| `CheckoutServices-GetPayPalButton` | PayPal integration |

### Account & Auth
| Endpoint | Purpose |
|----------|---------|
| `Account-Login` | Login |
| `Account-OpenSiteCredentials` | Open credentials iframe |
| `Account-UpdateCredentialsCustomerData` | Update customer data |

### Wishlist
| Endpoint | Purpose |
|----------|---------|
| `Wishlist-AddProduct` | Add to wishlist |
| `Wishlist-GetProductIds` | Get wishlist product IDs |
| `Wishlist-RemoveProduct` | Remove from wishlist |

### Stores
| Endpoint | Purpose |
|----------|---------|
| `Stores-FindStores` | Find stores by location (JSON) |
| `Stores-GetCoverageArea` | Delivery coverage |
| `Stores-GetDelivery` | Delivery options |
| `Stores-SetStoreContext` | Set active store |

### Pages & Content
| Endpoint | Purpose |
|----------|---------|
| `Page-IncludeHeaderMenuAjax` | Header menu AJAX |
| `Page-ManagerDecisionBanner` | Banner management |
| `Page-ManagerDecisionSponsored` | Sponsored placements |
| `Page-MobileAppValidation` | Mobile app check |
| `Page-RenderTemplate` | Template rendering |
| `IPaper-AddedFromIPaperTemplate` | Flyer product import |
| `LoginOnBehalf-DisplayOobHeader` | OOB header |

### Other
| Endpoint | Purpose |
|----------|---------|
| `Consent-ConsentData` | Cookie consent data |
| `ConsentTracking-GetContent` | Consent tracking |
| `ConsentTracking-SetSession` | Set consent session |
| `EmailSubscribe-Subscribe` | Newsletter subscription |

---

## 11. Other Endpoints Found

### 11.1 OutSystems Screen Services (Mobile App Only)

Pattern: `screenservices/{Module}/{Flow}/{Screen}/{Action}`

| Endpoint | Purpose |
|----------|---------|
| `screenservices/ContinenteOnline/ActionServerDataSync` | Get source URL + sync data |
| `screenservices/ContinenteOnline/Common/Splash/DataActionGetFeatureToggle` | Feature flags |
| `screenservices/ContinenteOnline/MainFlow/Open_IAB/ActionGetToggleAndDomain` | Get domain + toggles |
| `screenservices/ContinenteOnline/MainFlow/ChangeEnvironment/DataActionGetFeatureToggle` | Env feature flags |

### 11.2 OutSystems Module Services

| Endpoint | Purpose |
|----------|---------|
| `moduleservices/roles` | Fetch user roles |
| `moduleservices/ping` | Heartbeat |
| `moduleservices/log` | Client-side logging |
| `moduleservices/checkrequestsuspended?requestToken={token}` | Request suspension check |

### 11.3 Salesforce Marketing Cloud

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/device/v1/registration` | POST | Device registration |
| `/device/v1/event/analytic` | POST | Analytics events |
| `/device/v1/{id}/message` | GET/PATCH | Push messages |
| `/device/v1/{id}/sync/{type}` | POST | Data sync |
| `/device/v1/location/{id}/fence` | GET | Geofence messages |
| `/devicestatistics/v1/analytic` | POST | Device statistics |

### 11.4 Third-Party Services

| Service | Identifier | Purpose |
|---------|-----------|---------|
| CQuotient Einstein | `bdvs-continente` | Product recommendations |
| igoDigital | `app.igodigital.com` | Predictive intelligence |
| Syndigo | `710ff2e0-2f41-1551-6979-4c465865d7d1` | Product content syndication |
| Cookiebot | `86a29107-94de-4370-a781-af2d18aa1e48` | Cookie consent |
| Kevel | `col` client | Advertising/sponsored products |

### 11.5 Client IDs Summary

| ID | Service | Purpose |
|----|---------|---------|
| `NLR6WHyO8Iba4eRS` | login.continente.pt | OpenID Connect auth |
| `bdvs-continente` | CQuotient/Einstein | Product recommendations |
| `col` | Kevel | Advertising/audience |

---

## 12. TODO / Further Investigation

- [x] ~~Category tree~~ — Full hierarchy mapped: 14 top-level, ~130 L2, ~400+ L3 categories (see Appendix A)
- [x] ~~Product detail page~~ — Full JSON API found via `Product-Variation`
- [ ] **Price history**: No dedicated endpoint found. Could monitor `Product-Variation` price fields over time
- [x] ~~Search suggestions/autocomplete~~ — `SearchServices-GetSuggestions?q=` (min 5 chars)
- [x] ~~Rate limiting~~ — No rate limiting detected at 20 rapid requests
- [x] ~~OCAPI client_id~~ — Locked down, not needed (SFCC controllers work without auth)
- [x] ~~Store availability~~ — `Stores-FindStores` returns JSON (228 stores), but no per-product stock
- [x] ~~Promotions endpoint~~ — Promo data embedded in product tiles (PVPR badges, discount %)
- [x] ~~Flyer/catalog data~~ — iPaper platform at `folhetos.continente.pt`
- [x] ~~Continente Card integration~~ — Full OIDC auth system, loyalty card formats, cart benefits
- [ ] **Wishlist API**: Test `Wishlist-GetProductIds` and `Wishlist-AddProduct` (may require auth)
- [ ] **Delivery slots**: Test `CheckoutServices-BookDeliverySlot` for slot availability
- [ ] **Sponsored products**: Test `Page-ManagerDecisionSponsored` for ad placements
- [ ] **Product reviews**: Check if there's an endpoint for product ratings/reviews beyond the aggregate

---

## 13. Rust CLI Plan

### Recommended Crates

| Crate | Purpose |
|-------|---------|
| `reqwest` | HTTP client (async) |
| `scraper` | HTML parsing with CSS selectors |
| `serde` / `serde_json` | JSON serialization/deserialization |
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `html_escape` | Decode HTML entities in product data |
| `tabled` or `comfy-table` | Terminal table output |

### Available Data Strategy

| Data Need | Best Endpoint | Format |
|-----------|--------------|--------|
| Product search | `Search-ShowAjax` | HTML (parse `product-tile-impression`) |
| Product details | `Product-Variation` | **JSON** |
| Nutrition/ingredients | `Product-ProductNutritionalInfoTab` | HTML |
| Search suggestions | `SearchServices-GetSuggestions` | HTML |
| Store locations | `Stores-FindStores` | **JSON** |
| Category browsing | `Search-ShowAjax?cgid=` | HTML |

### MVP Features

1. **Search**: `continente search "leite"` — search products by keyword
2. **Product detail**: `continente product 6879912` — full product info via Product-Variation JSON
3. **Browse**: `continente browse "Mercearia"` — browse by category
4. **Filters**: `--brand`, `--price-min`, `--price-max`, `--vegan`, `--gluten-free`, etc.
5. **Sorting**: `--sort price-asc|price-desc|name-asc|name-desc|unit-price`
6. **Pagination**: `--page`, `--per-page` flags
7. **Nutrition**: `continente nutrition 6879912` — nutritional info
8. **Stores**: `continente stores --lat 38.7 --lon -9.1` — nearby stores
9. **Output formats**: Table (default), JSON, CSV

### Data Extraction Strategy

**For search (HTML parsing):**
1. GET `Search-ShowAjax` with query params
2. Parse HTML with `scraper`
3. Extract `product-tile-impression` attributes → JSON with name, price, brand, category
4. Extract image URLs from `data-src` on `img` tags
5. Extract total count from `data-gtm-results`
6. Extract unit prices from `.pwc-tile--price-secondary`

**For product detail (JSON):**
1. GET `Product-Variation?pid={id}` → direct JSON response
2. Extract EAN from `nutritionalInfoUrlString`
3. Optionally GET `Product-ProductNutritionalInfoTab` for nutrition data

---

## 14. Key Files in Decompiled APK

| File | Contains |
|------|----------|
| `resources/res/xml/config.xml` | Cordova config, hostnames, plugins |
| `resources/assets/www/pinning/pinning.json` | SSL pinning configuration |
| `resources/assets/www/prd-google-services.json` | Firebase production config |
| `resources/assets/www/scripts/ContinenteOnline.controller.js` | App business logic, server actions |
| `resources/assets/www/scripts/ContinenteOnline.model.js` | Data models and entities |
| `resources/assets/www/scripts/ContinenteOnline.MainFlow.HomePage.mvc.js` | Homepage logic, URL loading |
| `resources/assets/www/scripts/ContinenteOnline.MainFlow.Open_IAB.mvc.js` | InAppBrowser URL handling |
| `resources/assets/www/scripts/ContinenteOnline.appDefinition.js` | OutSystems app definition |
| `resources/assets/www/scripts/OutSystems.js` | OutSystems runtime framework |
| `sources/com/outsystems/plugins/sslpinning/` | SSL pinning plugin (Java) |
| `sources/com/salesforce/marketingcloud/` | Marketing Cloud SDK |

---

## Appendix A: Complete Category Tree

**14 top-level categories**, **~130 L2 subcategories**, **~400+ L3 leaf categories**.

> **Note**: URL paths (e.g., `/frescos/peixaria/`) differ from internal `cgid` values (e.g., `peixaria-e-talho-peixaria`). The `cgid` is what you pass to `Search-ShowAjax`.

### Top-Level cgid Mapping

| URL Path | Internal cgid |
|----------|---------------|
| `/frescos/` | `frescos` |
| `/laticinios-e-ovos/` | `laticinios` |
| `/congelados/` | `congelados` |
| `/mercearia/` | `mercearias` |
| `/bebidas-e-garrafeira/` | `bebidas` |
| `/bio-e-saudavel/` | `biologicos` |
| `/limpeza/` | `limpeza` |
| `/bebe/` | `bebe` (inferred from children using `bebe-*`) |
| `/beleza-e-higiene/` | `higiene-beleza` |
| `/animais/` | `animais` |
| `/casa-bricolage-e-jardim/` | `casa` |
| `/brinquedos-e-jogos/` | (children use `brinquedos-*`) |
| `/oportunidades/` | (promotional/seasonal) |
| `/novidades/` | (curated landing page) |

### 1. FRESCOS (`cgid=frescos`)

**Peixaria** (`cgid=peixaria-e-talho-peixaria`)
- Filetes, Lombos e Postas (`cgid=peixaria-e-talho-peixaria-filetes`)
- Peixe Fresco (`cgid=peixaria-e-talho-peixaria-fresco`)
- Peixe Congelado (`cgid=peixaria-e-talho-peixaria-congelado`)
- Bacalhau (`cgid=peixaria-e-talho-peixaria-bacalhau`)
- Polvo, Lulas e Chocos (`cgid=peixaria-e-talho-peixaria-polvo`)
- Marisco (`cgid=peixaria-e-talho-peixaria-marisco`)
- Salmao Fumado e Especialidades (`cgid=peixaria-e-talho-peixaria-salmao`)

**Talho** (`cgid=peixaria-e-talho-talho`)
- Pronto a Cozinhar (`cgid=peixaria-e-talho-talho-pronto`)
- Novilho, Vitela e Vitelao (`cgid=peixaria-e-talho-talho-novilho`)
- Frango e Peru (`cgid=peixaria-e-talho-talho-frango`)
- Porco (`cgid=peixaria-e-talho-talho-porco`)
- Pato e Coelho (`cgid=peixaria-e-talho-talho-pato`)
- Cabrito e Borrego (`cgid=peixaria-e-talho-talho-cabrito`)

**Frutas** (`cgid=frutas-legumes-frutas`)
- Frutas da Epoca (`cgid=frutas-legumes-sazonais`)
- Banana, Maca e Pera (`cgid=frutas-legumes-frutas-banana`)
- Laranja, Clementina e Limao (`cgid=frutas-legumes-frutas-laranja`)
- Melancia, Melao e Meloa (`cgid=frutas-legumes-frutas-melao`)
- Pessego, Ameixa e Kiwi (`cgid=frutas-legumes-frutas-pessego`)
- Morango e Frutos Vermelhos (`cgid=frutas-legumes-frutas-morango`)
- Uvas e Tropicais (`cgid=frutas-legumes-frutas-tropicais`)
- Frutos Secos, Desidratados e Sementes (`cgid=frutas-legumes-secos`)
- Sumos Espremidos na Hora (`cgid=frutas-legumes-sumos-naturais`)
- Cabazes de Frutas e Legumes (`cgid=frutas-legumes-cabazes`)

**Legumes** (`cgid=frutas-legumes-legumes`)
- Batata, Batata Doce e Mandioca (`cgid=frutas-legumes-legumes-batatas`)
- Cebola, Alho e Nabo (`cgid=frutas-legumes-legumes-alhos`)
- Cenoura, Abobora e Beterraba (`cgid=frutas-legumes-legumes-cenoura`)
- Curgete, Beringela e Feijao Verde (`cgid=frutas-legumes-legumes-nabo`)
- Couves, Brocolos e Espinafres (`cgid=frutas-legumes-legumes-couves`)
- Alface, Tomate, Pepino e Pimento (`cgid=frutas-legumes-legumes-alface`)
- Saladas, Sopas e Salteados (`cgid=frutas-legumes-legumes-sopas`)
- Cogumelos, Espargos e Exoticos (`cgid=frutas-legumes-legumes-cogumelos`)
- Ervas Aromaticas e Especiarias (`cgid=frutas-legumes-ervas`)
- Tremocos e Azeitonas (`cgid=frutas-legumes-tremocos-azeitonas`)

**Queijos** (`cgid=charcutaria-queijo-queijos`)
- Fatiado e Bolas (`cgid=charcutaria-queijo-queijos-fatiado`)
- Ralado (`cgid=charcutaria-queijo-queijos-ralado`)
- Fresco, Requeijao e Mozzarella (`cgid=charcutaria-queijo-queijos-fresco`)
- Snacks e Barrar (`cgid=charcutaria-queijo-queijos-snacks`)
- Amanteigado (`cgid=charcutaria-queijo-queijos-amanteigado`)
- Curado (`cgid=charcutaria-queijo-queijos-curado`)
- Queijos do Mundo (`cgid=charcutaria-queijo-queijos-mundo`)
- Tabuas e Aperitivos (`cgid=frescos-queijos-tabuas`)

**Charcutaria** (`cgid=charcutaria-queijo-charcutaria`)
- Fiambre, Mortadela e Salame (`cgid=charcutaria-queijo-charcutaria-fiambre`)
- Presunto (`cgid=charcutaria-queijo-charcutaria-presunto`)
- Salpicao, Paio e Fuet (`cgid=charcutaria-queijo-charcutaria-salpicao`)
- Alheira e Farinheira (`cgid=charcutaria-queijo-charcutaria-alheiras`)
- Chourico e Morcela (`cgid=charcutaria-queijo-charcutaria-chouricos`)
- Bacon e Fumados (`cgid=charcutaria-queijo-charcutaria-bacon`)
- Salsichas e Linguicas (`cgid=charcutaria-queijo-charcutaria-linguicas`)
- Salmao Fumado e Especialidades (`cgid=charcutaria-queijo-salmao`)
- Tabuas e Aperitivos (`cgid=destaques-charcutaria-tabuas-aperitivos`)

**Padaria e Pastelaria** (`cgid=padaria-e-pastelaria`)
- Pao do Dia e Broa (`cgid=padaria-e-pastelaria-padaria-fresco`)
- Pao de Forma e Embalado (`cgid=padaria-e-pastelaria-padaria-forma`)
- Pao de Hamburguer, Cachorro e Wraps (`cgid=padaria-e-pastelaria-padaria-hamburguer`)
- Tostas, Gressinos e Croutons (`cgid=padaria-e-pastelaria-padaria-tostas`)
- Croissants e Paes de Leite (`cgid=padaria-e-pastelaria-pastelaria-croissants`)
- Biscoitos (`cgid=padaria-e-pastelaria-pastelaria-biscoitos`)
- Pastelaria Sortida (`cgid=padaria-e-pastelaria-pastelaria-sortida`)
- Bolos e Sobremesas (`cgid=padaria-e-pastelaria-pastelaria-bolos`)
- Massas para Culinaria (`cgid=padaria-e-pastelaria-pastelaria-massas`)

**Take-Away** (`cgid=refeicoes-faceis`)
- Entradas e Salgados (`cgid=refeicoes-faceis-entradas-salgados`)
- Sopas (`cgid=refeicoes-faceis-sopas`)
- Pizzas (`cgid=refeicoes-faceis-pizzas`)
- Massas Frescas (`cgid=refeicoes-faceis-massas`)
- Grab&Go (`cgid=refeicoes-faceis-grab-go`)
- Refeicoes Prontas (`cgid=refeicoes-faceis-refeicoes-prontas`)
- Vegetariano e Vegan (`cgid=refeicoes-faceis-refeicoes-vegetarianas`)
- Sobremesas (`cgid=refeicoes-faceis-sobremesas`)

### 2. LATICINIOS E OVOS (`cgid=laticinios`)

**Leite** (`cgid=laticinios-leite`)
- Leite Magro (`cgid=laticinios-leite-magro`)
- Leite Meio Gordo (`cgid=laticinios-leite-meio-gordo`)
- Leite Inteiro (`cgid=laticinios-leite-gordo`)
- Leite Achocolatado e Aromatizado (`cgid=laticinios-leite-achocolatado-aromatizado`)
- Leite sem Lactose (`cgid=laticinios-leite-sem-lactose`)

**Iogurtes** (`cgid=laticinios-iogurtes`)
- Iogurtes Liquidos (`cgid=laticinios-iogurtes-liquidos`)
- Iogurtes Aromas e Naturais (`cgid=laticinios-iogurtes-aromas-naturais`)
- Iogurtes Magros (`cgid=laticinios-iogurtes-magros`)
- Iogurtes Bifidus (`cgid=laticinios-iogurtes-bifidus`)
- Iogurtes Proteina (`cgid=laticinios-iogurtes-skir-kefir`)
- Iogurtes Pedacos (`cgid=laticinios-iogurtes-peda`)
- Iogurtes Kefir (`cgid=laticinios-iogurtes-kefir`)
- Iogurtes Gregos (`cgid=laticinios-iogurtes-gregos`)
- Iogurtes Bebe (`cgid=laticinios-iogurtes-bebe`)
- Iogurtes Infantis (`cgid=laticinios-iogurtes-infantis`)
- Iogurtes sem Lactose (`cgid=laticinios-iogurtes-sem-lactose`)
- Vegegurtes e Yofu (`cgid=laticinios-vegegurtes-yofu`)

**Ovos** (`cgid=laticinios-ovos`)

**Manteigas e Cremes para Barrar** (`cgid=laticinios-manteigas-cremes-vegetais`)
- Manteigas (`cgid=laticinios-manteigas`)
- Cremes para Barrar (`cgid=laticinios-cremes-para-barrar`)
- Cremes Culinarios (`cgid=laticinios-cremes-culinarios`)

**Natas e Bechamel** (`cgid=laticinios-natas-bechamel-chantilly`)
- Natas para Bater e Chantilly (`cgid=laticinios-natas-frescas`)
- Natas Culinarias (`cgid=laticinios-natas-culin`)
- Cremes Vegetais (`cgid=laticinios-natas-cremes-vegetais`)
- Molho Bechamel (`cgid=laticinios-molho-bechamel`)

**Bebidas Vegetais** (`cgid=laticinios-ovos-bebidas-vegetais`)
- Bebida Soja (`cgid=laticinios-ovos-bebidas-soja`)
- Bebida Aveia (`cgid=laticinios-ovos-bebidas-aveia`)
- Bebida Amendoa (`cgid=laticinios-ovos-bebidas-amendoa`)
- Bebida Arroz (`cgid=laticinios-ovos-bebidas-arroz`)
- Outras Bebidas Vegetais (`cgid=laticinios-ovos-bebidas-outras`)

**Sobremesas** (`cgid=laticinios-sobremesas`)
- Gelatinas (`cgid=laticinios-sobremesas-gelatinas`)
- Mousses e Pudins (`cgid=laticinios-sobremesas-mousses`)

### 3. CONGELADOS (`cgid=congelados`)

**Frutas e Legumes** (`cgid=congelados-vegetais`)
- Legumes (`cgid=congelados-vegetais-legumes-congelados`)
- Misturas de Legumes (`cgid=congelados-vegetais-mistura-vegetais`)
- Frutas (`cgid=congelados-vegetais-frutas`)

**Batata Frita e Pure** (`cgid=congelados-vegetais-batatas`)

**Nuggets e Crocantes** (`cgid=congelados-douradinhos`)

**Douradinhos e Filetes** (`cgid=congelados-douradinhos-barrinhas`)

**Hamburgueres e Almondegas** (`cgid=congelados-refeicoes-hamburguer`)

**Peixe, Marisco e Carne** (`cgid=congelados-peixe`)
- Peixe (`cgid=congelados-peixe-congelado`)
- Marisco (`cgid=congelados-peixe-marisco`)
- Bacalhau (`cgid=congelados-peixe-bacalhau`)
- Polvo, Lulas e Chocos (`cgid=congelados-peixe-polvo`)
- Carne (`cgid=congelados-carne`)

**Pizzas** (`cgid=congelados-pizzas`)

**Refeicoes Prontas** (`cgid=congelados-refeicoes-massa-refeicoes`)
- Carne (`cgid=congelados-refeicoes-massa-refeicoes-carne`)
- Peixe (`cgid=congelados-refeicoes-massa-refeicoes-peixe`)
- Massas e Gnocchis (`cgid=congelados-refeicoes-massa-refeicoes-gnochis`)
- Salteados e Sopas (`cgid=congelados-refeicoes-massa-refeicoes-misturas`)

**Salgados, Folhados e Pastelaria** (`cgid=congelados-salgados-folhados`)
- Salgados (`cgid=congelados-salgados-folhados-salgados`)
- Folhados (`cgid=congelados-salgados-folhados-folhados`)
- Pastelaria Doce (`cgid=congelados-pastelaria`)
- Pao de Alho e Pao de Queijo (`cgid=congelados-salgados-folhados-pao`)

**Vegetariano e Vegan** (`cgid=congelados-vegetariano-vegan`)
- Hamburgueres e Almondegas (`cgid=congelados-vegetariano-vegan-hamburgueres`)
- Nuggets e Panados (`cgid=congelados-vegetariano-vegan-nuggets-panados`)
- Refeicoes, Pizzas e Falafel (`cgid=congelados-vegetariano-vegan-pizzas-falafel`)

**Gelados** (`cgid=congelados-gelados`)
- Gelados de Cone (`cgid=congelados-gelados-cone`)
- Gelados de Pauzinho (`cgid=congelados-gelados-pauzinho`)
- Gelados Familiares (`cgid=congelados-gelados-familiares`)
- Gelados Americanos (`cgid=congelados-gelados-americanos`)
- Mini Bites e Sandwich (`cgid=congelados-gelados-bites`)
- Tartes Geladas e Viennettas (`cgid=congelados-gelados-tartes`)
- Gelados Infantis (`cgid=congelados-gelados-infantis`)
- Gelados Vegan (`cgid=congelados-gelados-vegan`)

**Sobremesas** (`cgid=congelados-sobremesas`)
- Bolos Congelados (`cgid=congelados-sobremesas-bolos-congelados`)
- Crepes e Petit Gateau (`cgid=congelados-sobremesas-crepes-petit`)

### 4. MERCEARIA (`cgid=mercearias`)

**Cafe, Cha e Bebidas Soluveis** (`cgid=mercearias-cafe-cha`)
- Cafe em Capsulas (`cgid=mercearia-cha-cafe-achocolatados-cafe-capsulas`)
- Cafe Torrado (`cgid=mercearia-cha-cafe-achocolatados-cafe-torrado`)
- Cafe Soluvel (`cgid=mercearia-cha-cafe-achocolatados-cafe-soluvel`)
- Chas e Infusoes (`cgid=mercearia-cha-cafe-achocolatados-chas`)
- Chocolate Soluvel (`cgid=mercearia-cha-cafe-achocolatados-achocolatados`)
- Bebidas de Cereais (`cgid=mercearia-cha-cafe-achocolatados-bebidas`)

**Cereais e Barras** (`cgid=mercearias-cereais-barras`)

**Bolachas, Biscoitos e Tostas** (`cgid=mercearias-bolachas-biscoitos`)

**Chocolate, Gomas e Rebucados** (`cgid=mercearias-chocolate`)

**Arroz, Massa e Farinha** (`cgid=mercearias-arroz-massa`)

**Azeite, Oleo e Vinagre** (`cgid=mercearias-azeite-oleo-vinagre`)

**Conservas** (`cgid=mercearias-conservas`)

**Molhos, Temperos e Sal** (`cgid=mercearias-molhos-temperos`)

**Snacks e Batatas Fritas** (`cgid=mercearias-snacks`)

**Compotas, Cremes e Mel** (`cgid=mercearias-compotas`)

**Acucar e Sobremesas** (`cgid=mercearias-acucar`)

**Alimentacao Infantil** (`cgid=mercearias-alimentacao-infantil`)

### 5. BEBIDAS E GARRAFEIRA (`cgid=bebidas`)

**Sumos e Refrigerantes** (`cgid=bebidas-sumos-refrigerantes`)

**Agua** (`cgid=bebidas-agua`)
- Agua sem Gas (`cgid=bebidas-agua-sem-gas`)
- Agua com Gas (`cgid=bebidas-agua-com-gas`)
- Agua Tonica e Ginger Ale (`cgid=bebidas-agua-tonica`)
- Agua com Sabor (`cgid=bebidas-agua-sabor`)

**Bebidas Energeticas e Isotonicas** (`cgid=bebidas-bebidas-energeticas`)

**Cervejas e Sidras** (`cgid=bebidas-cervejas-sidras`)

**Vinhos** (`cgid=bebidas-vinho`)

**Bebidas Espirituosas** (`cgid=bebidas-espirituosas`)

**Champanhe e Espumante** (`cgid=bebidas-champanhe-espumante`)

### 6. BIO E SAUDAVEL (`cgid=biologicos`)

**Suplementos e Vitaminas** (`cgid=bio-suplementos`)

**Nutricao Desportiva** (`cgid=bio-nutricao-desportiva`)

**Vegetariano e Vegan** (`cgid=bio-vegetariano-vegan`)

**Biologicos** (`cgid=bio-biologicos`)

**Sem Gluten** (`cgid=bio-sem-gluten`)

**Sem Lactose** (`cgid=bio-sem-lactose`)

### 7. LIMPEZA (`cgid=limpeza`)

**Roupa** (`cgid=limpeza-roupa`)

**Cozinha** (`cgid=limpeza-cozinha`)

**Casa de Banho** (`cgid=limpeza-wc`)

**Chao e Superficies** (`cgid=limpeza-geral`)

**Guardanapos e Rolos** (`cgid=limpeza-produtos-papel`)

**Velas e Ambientadores** (`cgid=limpeza-ambientadores`)

**Sacos e Baldes do Lixo** (`cgid=limpeza-baldes-ecopontos-sacos`)

**Mopas, Esfregonas e Vassouras** (`cgid=limpeza-panos-baldes-vassouras`)

**Panos, Esfregoes e Luvas** (`cgid=limpeza-panos-esfregoes-luvas`)

**Inseticidas e Desumidificadores** (`cgid=limpeza-inseticidas`)

**Limpeza Auto e Motos** (`cgid=limpeza-auto-motos`)

### 8. BEBE

**Alimentacao Infantil** (`cgid=bebe-alimentacao-infantil`)

**Fraldas e Toalhitas** (`cgid=bebe-fraldas-toalhitas`)

**Banho e Higiene** (`cgid=bebe-banho-higiene`)

**Cadeiras Auto e Carrinhos** (`cgid=bebe-auto-passeio`)

**Cadeiras e Acessorios de Refeicao** (`cgid=bebe-cadeiras-acessorios`)

**Mobiliario e Colchoes** (`cgid=bebe-mobiliario-colchoes`)

**Banheiras e Complementos** (`cgid=bebe-banheiras-acessorios`)

**Textil de Bebe** (`cgid=bebe-textil`)

**Chupetas e Mordedores** (`cgid=bebe-chupetas-mordedores`)

**Brinquedos e Livros** (`cgid=bebe-brinquedos`)

### 9. BELEZA E HIGIENE (`cgid=higiene-beleza`)

**Cabelo** (`cgid=higiene-beleza-cabelo`)

**Corpo** (`cgid=higiene-beleza-corpo`)

**Rosto** (`cgid=higiene-beleza-rosto`)

**Maquilhagem** (`cgid=higiene-beleza-maquilhagem`)

**Higiene Oral** (`cgid=higiene-beleza-oral`)

**Higiene Intima** (`cgid=higiene-beleza-intima`)

**Homem** (`cgid=higiene-beleza-homem`)

**Preservativos e Estimuladores** (`cgid=higiene-beleza-preservativos`)

**Lencos e Cuidados de Saude** (`cgid=higiene-beleza-lencos-saude`)

**Papel Higienico** (`cgid=higiene-beleza-papel-lencos`)

**Solares e Bronzeadores** (`cgid=higiene-beleza-solares`)

**Coffrets e Presentes** (`cgid=higiene-beleza-perfumes-conjuntos`)

### 10. ANIMAIS (`cgid=animais`)

**Cao** (`cgid=animais-cao`)

**Gato** (`cgid=animais-gato`)

**Outros Animais** (`cgid=animais-outros-animais`)

### 11. CASA, BRICOLAGE E JARDIM (`cgid=casa`)

**Mobiliario e Colchoes** (`cgid=casa-mobiliario-colchoes`)

**Textil Lar** (`cgid=casa-textil-lar`)

**Decoracao** (`cgid=casa-decoracao-banho`)

**Cozinha** (`cgid=casa-cozinha`)

**Mesa** (`cgid=casa-mesa`)

**Eletrodomesticos** (`cgid=casa-eletrodomesticos`)

**Lavandaria e Organizacao** (`cgid=casa-lavandaria-organiza`)

**Festa** (`cgid=casa-festa`)

**Jardim** (`cgid=casa-jardim`)

**Pilhas e Lampadas** (`cgid=casa-pilhas-lampadas`)

**Bricolage** (`cgid=casa-bricolage`)

### 12. BRINQUEDOS E JOGOS

**LEGO** (`cgid=brinquedos-construcoes-lego-1`)

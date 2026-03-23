# Real-World Test Plan

Manual verification checklist for cnt CLI. Run against live Continente APIs.

**Last tested:** —

---

## Search

### `search` — Basic (table default)
```bash
cnt search "leite"
```
- [ ] Returns table with products
- [ ] Portuguese characters render correctly (ã, ç, õ, é)
- [ ] Shows ID, name, price, brand, category per product
- [ ] Total count shown

### `search` — JSON format
```bash
cnt search "leite" --format json | jq '.products | length'
```
- [ ] Valid JSON output
- [ ] Contains products array, total, query fields

### `search` — Compact format (TSV)
```bash
cnt search "leite" --format compact | head -5
```
- [ ] Tab-separated values
- [ ] Pipeable to `sort`, `awk`, etc.

### `search` — Brand filter
```bash
cnt search "cerveja" --brand "Super Bock"
```
- [ ] All results are Super Bock brand
- [ ] Total count lower than unfiltered search

### `search` — Price range
```bash
cnt search "arroz" --price-min 1 --price-max 3
```
- [ ] All results between 1€ and 3€

### `search` — Sort by price
```bash
cnt search "leite" --sort price-low-to-high --format compact | head -10
```
- [ ] Results sorted cheapest first

### `search` — Sort by unit price
```bash
cnt search "azeite" --sort unit-price
```
- [ ] Results sorted by price per kg/lt

### `search` — Pagination
```bash
cnt search "arroz" --page 2 --max 12
```
- [ ] Returns page 2 results (different from page 1)
- [ ] Respects max count

### `search` — Combined filters
```bash
cnt search "leite" --vegan --bio --sort price-low-to-high
```
- [ ] Applies both dietary filters simultaneously
- [ ] Results sorted by price

---

## Dietary Filters

### `--vegan`
```bash
cnt search "leite" --vegan
```
- [ ] Returns vegan products only

### `--vegetarian`
```bash
cnt search "hamburguer" --vegetarian
```
- [ ] Returns vegetarian products only

### `--gluten-free`
```bash
cnt search "pão" --gluten-free
```
- [ ] Returns gluten-free products only

### `--lactose-free`
```bash
cnt search "queijo" --lactose-free
```
- [ ] Returns lactose-free products only

### `--sugar-free`
```bash
cnt search "bolachas" --sugar-free
```
- [ ] Returns sugar-free products only

### `--bio`
```bash
cnt search "leite" --bio
```
- [ ] Returns organic/biological products only

---

## Product Detail

### `product` — Basic info
```bash
cnt product 6879912
```
- [ ] Shows product name, brand, price, EAN, category, rating
- [ ] Price formatted correctly with €
- [ ] Category path uses `>` separator

### `product` — JSON format
```bash
cnt product 6879912 --format json | jq .id
```
- [ ] Valid JSON with all product fields
- [ ] EAN present
- [ ] Badge info present

### `product` — With nutritional info
```bash
cnt product 6879912 --nutrition
```
- [ ] Shows ingredients, allergens, origin
- [ ] Nutrient table with name, value, unit
- [ ] Producer name shown
- [ ] Storage instructions shown (if available)

### `product` — Nutrition JSON
```bash
cnt product 6879912 --nutrition --format json | jq '.nutrition.nutrients | length'
```
- [ ] Valid JSON with nutrition nested object
- [ ] Nutrients array populated

---

## Browse Categories

### `browse` — By name (fuzzy match)
```bash
cnt browse frescos
```
- [ ] Returns products from Frescos category
- [ ] Or error if no products at top-level (try subcategory)

### `browse` — By exact cgid
```bash
cnt browse laticinios-leite
```
- [ ] Returns milk products
- [ ] Total count shown

### `browse` — With sort
```bash
cnt browse "cerveja" --sort unit-price
```
- [ ] Results sorted by unit price

### `browse` — Pagination
```bash
cnt browse laticinios-leite --page 2 --max 12
```
- [ ] Page 2 results differ from page 1

---

## Suggestions

### `suggest` — Basic
```bash
cnt suggest "arroz"
```
- [ ] Returns product suggestions
- [ ] May include category suggestions and popular terms

### `suggest` — JSON format
```bash
cnt suggest "leite" --format json | jq 'keys'
```
- [ ] Valid JSON with products, categories, popular_terms

### `suggest` — Short query rejected
```bash
cnt suggest "abc"
```
- [ ] Error: query must be at least 5 characters
- [ ] Non-zero exit code

---

## Stores

### `stores` — Lisbon area
```bash
cnt stores --lat 38.7 --lon -9.1
```
- [ ] Returns stores near Lisbon
- [ ] Shows name, address, city, pickup, galp columns

### `stores` — Porto with radius
```bash
cnt stores --lat 41.1 --lon -8.6 --radius 20
```
- [ ] Returns stores within 20km of Porto

### `stores` — JSON format
```bash
cnt stores --lat 38.7 --lon -9.1 --format json | jq '.[0] | keys'
```
- [ ] Valid JSON array
- [ ] Each store has id, name, address, city, latitude, longitude, is_pickup_store, is_galp_store

### `stores` — Large radius (all Portugal)
```bash
cnt stores --lat 39.5 --lon -8.0 --radius 300 --format json | jq 'length'
```
- [ ] Returns 200+ stores

---

## Categories

### `categories` — Tree view
```bash
cnt categories
```
- [ ] Shows hierarchical tree with indentation
- [ ] Top-level categories with subcategories beneath
- [ ] Portuguese characters correct

### `categories` — JSON format
```bash
cnt categories --format json | jq 'length'
```
- [ ] Returns 251 categories
- [ ] Each has cgid, name, parent fields

### `categories` — Known categories present
```bash
cnt categories --format json | jq '[.[] | select(.parent == null)] | length'
```
- [ ] 14 top-level categories (parent: null)

---

## Flyers

### `flyers` — List current flyers
```bash
cnt flyers
```
- [ ] Shows current weekly flyers
- [ ] Each has slug, title, dates
- [ ] Footer mentions iPaper viewer

### `flyers` — JSON format
```bash
cnt flyers --format json | jq '.[0] | keys'
```
- [ ] Valid JSON array
- [ ] Each flyer has title, description, url, slug, image_url

### `flyers` — Compact format
```bash
cnt flyers --format compact
```
- [ ] Tab-separated slug, title, URL

---

## Output Formats (cross-cutting)

### All commands support `--format json`
```bash
cnt search "leite" --format json | jq . > /dev/null
cnt product 6879912 --format json | jq . > /dev/null
cnt stores --lat 38.7 --lon -9.1 --format json | jq . > /dev/null
cnt suggest "leite" --format json | jq . > /dev/null
cnt categories --format json | jq . > /dev/null
cnt flyers --format json | jq . > /dev/null
```
- [ ] All produce valid JSON (jq exits 0)

### All commands support `--format compact`
```bash
cnt search "leite" --format compact | wc -l
cnt stores --lat 38.7 --lon -9.1 --format compact | wc -l
cnt flyers --format compact | wc -l
```
- [ ] All produce TSV output

---

## Configuration

### Config file loading
```bash
printf '[output]\nformat = "json"\n' > /tmp/cnt_test.toml
cnt --config /tmp/cnt_test.toml categories | jq 'length'
```
- [ ] Config file sets default format to JSON
- [ ] Returns 251 categories

### CLI flag overrides config
```bash
printf '[output]\nformat = "json"\n' > /tmp/cnt_test.toml
cnt --config /tmp/cnt_test.toml --format table categories
```
- [ ] Table output despite config saying JSON

### Verbose logging
```bash
cnt -v search "leite" 2>&1 | head -3
```
- [ ] Debug logs visible on stderr
- [ ] Shows request details

---

## Error Handling

### Invalid product ID
```bash
cnt product 0000000
```
- [ ] Returns error (HTTP 404 or parse error)
- [ ] Non-zero exit code

### Empty search results
```bash
cnt search "xyztermoqueNaoExiste999"
```
- [ ] Returns error or empty results
- [ ] Non-zero exit code

### Invalid config path
```bash
cnt --config /tmp/nonexistent.toml search "leite"
```
- [ ] Returns error about missing config
- [ ] Non-zero exit code

---

## Edge Cases

### Special characters in query
```bash
cnt search "pão de forma"
```
- [ ] Handles `ã` correctly
- [ ] Returns bread products

### Very long brand name
```bash
cnt search "leite" --brand "Continente"
```
- [ ] Brand filter works with full name

### Multiple dietary filters combined
```bash
cnt search "snacks" --vegan --gluten-free --bio
```
- [ ] All 3 filters applied (prefn1, prefn2, prefn3)
- [ ] Very few or zero results (expected)

### Negative longitude (Portugal is west of Greenwich)
```bash
cnt stores --lat 38.7 --lon -9.1
```
- [ ] Negative longitude handled correctly
- [ ] Returns Lisbon-area stores

---

## Results

| Test | Status | Notes |
|------|--------|-------|
| search basic | | |
| search JSON | | |
| search compact | | |
| search brand filter | | |
| search price range | | |
| search sort price | | |
| search pagination | | |
| search combined filters | | |
| --vegan | | |
| --vegetarian | | |
| --gluten-free | | |
| --lactose-free | | |
| --sugar-free | | |
| --bio | | |
| product basic | | |
| product JSON | | |
| product nutrition | | |
| product nutrition JSON | | |
| browse by name | | |
| browse by cgid | | |
| browse with sort | | |
| suggest basic | | |
| suggest JSON | | |
| suggest short query | | |
| stores Lisbon | | |
| stores Porto radius | | |
| stores JSON | | |
| stores all Portugal | | |
| categories tree | | |
| categories JSON (251) | | |
| categories top-level (14) | | |
| flyers list | | |
| flyers JSON | | |
| flyers compact | | |
| all formats JSON valid | | |
| all formats compact | | |
| config file loading | | |
| CLI overrides config | | |
| verbose logging | | |
| invalid product ID | | |
| empty search results | | |
| invalid config path | | |
| special characters | | |
| multiple dietary filters | | |
| negative longitude | | |

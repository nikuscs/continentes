# Plan 04: Output Formatting

## Goal

Implement proper table, JSON, and compact output for all data types. Follow kante-kusta's format module patterns.

## Steps

### 4.1 OutputFormat enum (`src/format/mod.rs`)

```rust
#[derive(Debug, Clone, Copy, clap::ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}
```

### 4.2 Search products table

```
Found 1314 products for "leite":

ID        │ Name                                           │ Price  │ /Unit      │ Brand
──────────┼────────────────────────────────────────────────┼────────┼────────────┼──────────────
6879912   │ Leite UHT Meio Gordo Continente               │  0.86€ │ 0.86€/lt   │ Continente
2210946   │ Leite UHT Meio Gordo Mimosa                   │  0.90€ │ 0.90€/lt   │ Mimosa

Page 1/55 (24 of 1314 results)
```

- ID: 10 chars
- Name: 48 chars, truncated with `…`
- Price: right-aligned, 2 decimals, `€` suffix
- Unit price: from `.pwc-tile--price-secondary` or `pricePerUnit`
- Brand: 15 chars

### 4.3 Product detail table

```
Leite UHT Meio Gordo Continente
════════════════════════════════

ID:          6879912
Brand:       Continente
Price:       0.86€ (was 1.00€, -14%)
Unit Price:  0.86€/lt
Package:     emb. 1 lt
Rating:      ★ 3.9
Category:    Laticínios e Ovos > Leite > Leite Meio Gordo
Available:   ✓
EAN:         5601312508007
URL:         https://www.continente.pt/produto/leite-uht-meio-gordo-...

Description:
Um delicioso leite meio gordo 100% português...
```

If `--nutrition` is used, append:

```
Nutritional Info (per 100ml):
─────────────────────────────
Energy:        200 kJ / 48 kcal
Fat:           1.6g (saturated: 1.0g)
Carbohydrates: 4.9g (sugars: 4.9g)
Protein:       3.4g
Salt:          0.1g
Calcium:       120mg

Ingredients: LEITE UHT meio-gordo.
Allergens:   Contém Leite.
Origin:      Produzido em Portugal
Storage:     Local fresco e seco...
```

### 4.4 Suggestions table

```
Products:
  6879912  │  0.86€ │ Leite UHT Meio Gordo Continente
  2210946  │  0.90€ │ Leite UHT Meio Gordo Mimosa
  4949515  │  1.99€ │ Arroz Basmati Continente

Categories:
  • Laticínios e Ovos > Leite (laticinios-e-ovos/leite/?q=leite)
  • Mercearia > Arroz (mercearia/arroz-massa-e-farinha/arroz/?q=arroz)

Popular searches:
  • leite magro
  • leite sem lactose
  • leite de aveia
```

### 4.5 Stores table

```
Found 15 stores within 10km:

Name                              │ Address                                    │ City        │ Pickup
──────────────────────────────────┼────────────────────────────────────────────┼─────────────┼───────
Continente Colombo                │ Av. Lusíada, Centro Colombo               │ Lisboa      │ ✓
Continente Telheiras              │ Rua Prof. Fernando da Fonseca             │ Lisboa      │ ✓
```

### 4.6 Categories tree

```
Categories:

Frescos (frescos)
├── Peixaria (peixaria-e-talho-peixaria)
├── Talho (peixaria-e-talho-talho)
├── Frutas (frutas-legumes-frutas)
├── Legumes (frutas-legumes-legumes)
├── Queijos (charcutaria-queijo-queijos)
├── Charcutaria (charcutaria-queijo-charcutaria)
├── Padaria e Pastelaria (padaria-e-pastelaria)
└── Take-Away (refeicoes-faceis)

Laticínios e Ovos (laticinios)
├── Leite (laticinios-leite)
├── Iogurtes (laticinios-iogurtes)
├── Ovos (laticinios-ovos)
...
```

### 4.7 Compact output (tab-separated)

For piping to other tools:

```
6879912\t0.86\tContinente\tLeite UHT Meio Gordo Continente
2210946\t0.90\tMimosa\tLeite UHT Meio Gordo Mimosa
```

### 4.8 JSON output

Pretty-printed `serde_json::to_string_pretty()` for all types.
The SearchResponse, ProductDetail, SuggestionResult, Store all already derive Serialize.

### 4.9 Implementation pattern

Each format function:

```rust
pub fn format_products(response: &SearchResponse, format: OutputFormat) -> String {
    match format {
        OutputFormat::Table => format_products_table(response),
        OutputFormat::Json => serde_json::to_string_pretty(&response.products).unwrap_or_default(),
        OutputFormat::Compact => format_products_compact(response),
    }
}
```

Helper for truncating strings:
```rust
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    } else {
        s.to_string()
    }
}
```

## Verification

After this plan:
- `cnt search "leite"` shows a pretty table
- `cnt search "leite" --format json` outputs valid JSON
- `cnt search "leite" --format compact` outputs TSV
- `cnt product 6879912` shows detailed product info
- `cnt product 6879912 --nutrition` includes nutrition table
- `cnt stores --lat 38.7 --lon -9.1` shows store table
- `cnt categories` shows tree view

## Files Created/Modified

| File | Action |
|------|--------|
| `src/format/mod.rs` | Rewrite with all formatters |

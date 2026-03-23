pub struct Category {
    pub cgid: &'static str,
    pub name: &'static str,
    pub parent: Option<&'static str>,
}

pub fn all_categories() -> &'static [Category] {
    &CATEGORIES
}

pub fn resolve_cgid(input: &str) -> Option<&'static str> {
    let lower = input.to_lowercase();

    // Exact match on cgid
    if let Some(cat) = CATEGORIES.iter().find(|c| c.cgid == lower) {
        return Some(cat.cgid);
    }

    // Exact match on name (case-insensitive)
    if let Some(cat) = CATEGORIES.iter().find(|c| c.name.to_lowercase() == lower) {
        return Some(cat.cgid);
    }

    // Partial match on name
    if let Some(cat) = CATEGORIES
        .iter()
        .find(|c| c.name.to_lowercase().contains(&lower))
    {
        return Some(cat.cgid);
    }

    // Partial match on cgid
    CATEGORIES
        .iter()
        .find(|c| c.cgid.contains(&lower))
        .map(|c| c.cgid)
}

static CATEGORIES: [Category; 251] = [
    // ── Top-level categories ────────────────────────────────────────────
    Category {
        cgid: "frescos",
        name: "Frescos",
        parent: None,
    },
    Category {
        cgid: "laticinios",
        name: "Laticinios e Ovos",
        parent: None,
    },
    Category {
        cgid: "congelados",
        name: "Congelados",
        parent: None,
    },
    Category {
        cgid: "mercearias",
        name: "Mercearia",
        parent: None,
    },
    Category {
        cgid: "bebidas",
        name: "Bebidas e Garrafeira",
        parent: None,
    },
    Category {
        cgid: "biologicos",
        name: "Bio e Saudavel",
        parent: None,
    },
    Category {
        cgid: "limpeza",
        name: "Limpeza",
        parent: None,
    },
    Category {
        cgid: "bebe",
        name: "Bebe",
        parent: None,
    },
    Category {
        cgid: "higiene-beleza",
        name: "Beleza e Higiene",
        parent: None,
    },
    Category {
        cgid: "animais",
        name: "Animais",
        parent: None,
    },
    Category {
        cgid: "casa",
        name: "Casa, Bricolage e Jardim",
        parent: None,
    },
    Category {
        cgid: "brinquedos",
        name: "Brinquedos e Jogos",
        parent: None,
    },
    Category {
        cgid: "oportunidades",
        name: "Oportunidades",
        parent: None,
    },
    Category {
        cgid: "novidades",
        name: "Novidades",
        parent: None,
    },
    // ── 1. FRESCOS ──────────────────────────────────────────────────────
    // L2: Peixaria
    Category {
        cgid: "peixaria-e-talho-peixaria",
        name: "Peixaria",
        parent: Some("frescos"),
    },
    // L3: Peixaria children
    Category {
        cgid: "peixaria-e-talho-peixaria-filetes",
        name: "Filetes, Lombos e Postas",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-fresco",
        name: "Peixe Fresco",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-congelado",
        name: "Peixe Congelado",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-bacalhau",
        name: "Bacalhau",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-polvo",
        name: "Polvo, Lulas e Chocos",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-marisco",
        name: "Marisco",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    Category {
        cgid: "peixaria-e-talho-peixaria-salmao",
        name: "Salmao Fumado e Especialidades",
        parent: Some("peixaria-e-talho-peixaria"),
    },
    // L2: Talho
    Category {
        cgid: "peixaria-e-talho-talho",
        name: "Talho",
        parent: Some("frescos"),
    },
    // L3: Talho children
    Category {
        cgid: "peixaria-e-talho-talho-pronto",
        name: "Pronto a Cozinhar",
        parent: Some("peixaria-e-talho-talho"),
    },
    Category {
        cgid: "peixaria-e-talho-talho-novilho",
        name: "Novilho, Vitela e Vitelao",
        parent: Some("peixaria-e-talho-talho"),
    },
    Category {
        cgid: "peixaria-e-talho-talho-frango",
        name: "Frango e Peru",
        parent: Some("peixaria-e-talho-talho"),
    },
    Category {
        cgid: "peixaria-e-talho-talho-porco",
        name: "Porco",
        parent: Some("peixaria-e-talho-talho"),
    },
    Category {
        cgid: "peixaria-e-talho-talho-pato",
        name: "Pato e Coelho",
        parent: Some("peixaria-e-talho-talho"),
    },
    Category {
        cgid: "peixaria-e-talho-talho-cabrito",
        name: "Cabrito e Borrego",
        parent: Some("peixaria-e-talho-talho"),
    },
    // L2: Frutas
    Category {
        cgid: "frutas-legumes-frutas",
        name: "Frutas",
        parent: Some("frescos"),
    },
    // L3: Frutas children
    Category {
        cgid: "frutas-legumes-sazonais",
        name: "Frutas da Epoca",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-banana",
        name: "Banana, Maca e Pera",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-laranja",
        name: "Laranja, Clementina e Limao",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-melao",
        name: "Melancia, Melao e Meloa",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-pessego",
        name: "Pessego, Ameixa e Kiwi",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-morango",
        name: "Morango e Frutos Vermelhos",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-frutas-tropicais",
        name: "Uvas e Tropicais",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-secos",
        name: "Frutos Secos, Desidratados e Sementes",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-sumos-naturais",
        name: "Sumos Espremidos na Hora",
        parent: Some("frutas-legumes-frutas"),
    },
    Category {
        cgid: "frutas-legumes-cabazes",
        name: "Cabazes de Frutas e Legumes",
        parent: Some("frutas-legumes-frutas"),
    },
    // L2: Legumes
    Category {
        cgid: "frutas-legumes-legumes",
        name: "Legumes",
        parent: Some("frescos"),
    },
    // L3: Legumes children
    Category {
        cgid: "frutas-legumes-legumes-batatas",
        name: "Batata, Batata Doce e Mandioca",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-alhos",
        name: "Cebola, Alho e Nabo",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-cenoura",
        name: "Cenoura, Abobora e Beterraba",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-nabo",
        name: "Curgete, Beringela e Feijao Verde",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-couves",
        name: "Couves, Brocolos e Espinafres",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-alface",
        name: "Alface, Tomate, Pepino e Pimento",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-sopas",
        name: "Saladas, Sopas e Salteados",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-legumes-cogumelos",
        name: "Cogumelos, Espargos e Exoticos",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-ervas",
        name: "Ervas Aromaticas e Especiarias",
        parent: Some("frutas-legumes-legumes"),
    },
    Category {
        cgid: "frutas-legumes-tremocos-azeitonas",
        name: "Tremocos e Azeitonas",
        parent: Some("frutas-legumes-legumes"),
    },
    // L2: Queijos
    Category {
        cgid: "charcutaria-queijo-queijos",
        name: "Queijos",
        parent: Some("frescos"),
    },
    // L3: Queijos children
    Category {
        cgid: "charcutaria-queijo-queijos-fatiado",
        name: "Fatiado e Bolas",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-ralado",
        name: "Ralado",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-fresco",
        name: "Fresco, Requeijao e Mozzarella",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-snacks",
        name: "Snacks e Barrar",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-amanteigado",
        name: "Amanteigado",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-curado",
        name: "Curado",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos-mundo",
        name: "Queijos do Mundo",
        parent: Some("charcutaria-queijo-queijos"),
    },
    Category {
        cgid: "frescos-queijos-tabuas",
        name: "Tabuas e Aperitivos",
        parent: Some("charcutaria-queijo-queijos"),
    },
    // L2: Charcutaria
    Category {
        cgid: "charcutaria-queijo-charcutaria",
        name: "Charcutaria",
        parent: Some("frescos"),
    },
    // L3: Charcutaria children
    Category {
        cgid: "charcutaria-queijo-charcutaria-fiambre",
        name: "Fiambre, Mortadela e Salame",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-presunto",
        name: "Presunto",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-salpicao",
        name: "Salpicao, Paio e Fuet",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-alheiras",
        name: "Alheira e Farinheira",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-chouricos",
        name: "Chourico e Morcela",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-bacon",
        name: "Bacon e Fumados",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria-linguicas",
        name: "Salsichas e Linguicas",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "charcutaria-queijo-salmao",
        name: "Salmao Fumado e Especialidades",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    Category {
        cgid: "destaques-charcutaria-tabuas-aperitivos",
        name: "Tabuas e Aperitivos",
        parent: Some("charcutaria-queijo-charcutaria"),
    },
    // L2: Padaria e Pastelaria
    Category {
        cgid: "padaria-e-pastelaria",
        name: "Padaria e Pastelaria",
        parent: Some("frescos"),
    },
    // L3: Padaria e Pastelaria children
    Category {
        cgid: "padaria-e-pastelaria-padaria-fresco",
        name: "Pao do Dia e Broa",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-padaria-forma",
        name: "Pao de Forma e Embalado",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-padaria-hamburguer",
        name: "Pao de Hamburguer, Cachorro e Wraps",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-padaria-tostas",
        name: "Tostas, Gressinos e Croutons",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-pastelaria-croissants",
        name: "Croissants e Paes de Leite",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-pastelaria-biscoitos",
        name: "Biscoitos",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-pastelaria-sortida",
        name: "Pastelaria Sortida",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-pastelaria-bolos",
        name: "Bolos e Sobremesas",
        parent: Some("padaria-e-pastelaria"),
    },
    Category {
        cgid: "padaria-e-pastelaria-pastelaria-massas",
        name: "Massas para Culinaria",
        parent: Some("padaria-e-pastelaria"),
    },
    // L2: Take-Away
    Category {
        cgid: "refeicoes-faceis",
        name: "Take-Away",
        parent: Some("frescos"),
    },
    // L3: Take-Away children
    Category {
        cgid: "refeicoes-faceis-entradas-salgados",
        name: "Entradas e Salgados",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-sopas",
        name: "Sopas",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-pizzas",
        name: "Pizzas",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-massas",
        name: "Massas Frescas",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-grab-go",
        name: "Grab&Go",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-refeicoes-prontas",
        name: "Refeicoes Prontas",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-refeicoes-vegetarianas",
        name: "Vegetariano e Vegan",
        parent: Some("refeicoes-faceis"),
    },
    Category {
        cgid: "refeicoes-faceis-sobremesas",
        name: "Sobremesas",
        parent: Some("refeicoes-faceis"),
    },
    // ── 2. LATICINIOS E OVOS ────────────────────────────────────────────
    // L2: Leite
    Category {
        cgid: "laticinios-leite",
        name: "Leite",
        parent: Some("laticinios"),
    },
    // L3: Leite children
    Category {
        cgid: "laticinios-leite-magro",
        name: "Leite Magro",
        parent: Some("laticinios-leite"),
    },
    Category {
        cgid: "laticinios-leite-meio-gordo",
        name: "Leite Meio Gordo",
        parent: Some("laticinios-leite"),
    },
    Category {
        cgid: "laticinios-leite-gordo",
        name: "Leite Inteiro",
        parent: Some("laticinios-leite"),
    },
    Category {
        cgid: "laticinios-leite-achocolatado-aromatizado",
        name: "Leite Achocolatado e Aromatizado",
        parent: Some("laticinios-leite"),
    },
    Category {
        cgid: "laticinios-leite-sem-lactose",
        name: "Leite sem Lactose",
        parent: Some("laticinios-leite"),
    },
    // L2: Iogurtes
    Category {
        cgid: "laticinios-iogurtes",
        name: "Iogurtes",
        parent: Some("laticinios"),
    },
    // L3: Iogurtes children
    Category {
        cgid: "laticinios-iogurtes-liquidos",
        name: "Iogurtes Liquidos",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-aromas-naturais",
        name: "Iogurtes Aromas e Naturais",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-magros",
        name: "Iogurtes Magros",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-bifidus",
        name: "Iogurtes Bifidus",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-skir-kefir",
        name: "Iogurtes Proteina",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-peda",
        name: "Iogurtes Pedacos",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-kefir",
        name: "Iogurtes Kefir",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-gregos",
        name: "Iogurtes Gregos",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-bebe",
        name: "Iogurtes Bebe",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-infantis",
        name: "Iogurtes Infantis",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-iogurtes-sem-lactose",
        name: "Iogurtes sem Lactose",
        parent: Some("laticinios-iogurtes"),
    },
    Category {
        cgid: "laticinios-vegegurtes-yofu",
        name: "Vegegurtes e Yofu",
        parent: Some("laticinios-iogurtes"),
    },
    // L2: Ovos
    Category {
        cgid: "laticinios-ovos",
        name: "Ovos",
        parent: Some("laticinios"),
    },
    // L2: Manteigas e Cremes para Barrar
    Category {
        cgid: "laticinios-manteigas-cremes-vegetais",
        name: "Manteigas e Cremes para Barrar",
        parent: Some("laticinios"),
    },
    // L3: Manteigas children
    Category {
        cgid: "laticinios-manteigas",
        name: "Manteigas",
        parent: Some("laticinios-manteigas-cremes-vegetais"),
    },
    Category {
        cgid: "laticinios-cremes-para-barrar",
        name: "Cremes para Barrar",
        parent: Some("laticinios-manteigas-cremes-vegetais"),
    },
    Category {
        cgid: "laticinios-cremes-culinarios",
        name: "Cremes Culinarios",
        parent: Some("laticinios-manteigas-cremes-vegetais"),
    },
    // L2: Natas e Bechamel
    Category {
        cgid: "laticinios-natas-bechamel-chantilly",
        name: "Natas e Bechamel",
        parent: Some("laticinios"),
    },
    // L3: Natas children
    Category {
        cgid: "laticinios-natas-frescas",
        name: "Natas para Bater e Chantilly",
        parent: Some("laticinios-natas-bechamel-chantilly"),
    },
    Category {
        cgid: "laticinios-natas-culin",
        name: "Natas Culinarias",
        parent: Some("laticinios-natas-bechamel-chantilly"),
    },
    Category {
        cgid: "laticinios-natas-cremes-vegetais",
        name: "Cremes Vegetais",
        parent: Some("laticinios-natas-bechamel-chantilly"),
    },
    Category {
        cgid: "laticinios-molho-bechamel",
        name: "Molho Bechamel",
        parent: Some("laticinios-natas-bechamel-chantilly"),
    },
    // L2: Bebidas Vegetais
    Category {
        cgid: "laticinios-ovos-bebidas-vegetais",
        name: "Bebidas Vegetais",
        parent: Some("laticinios"),
    },
    // L3: Bebidas Vegetais children
    Category {
        cgid: "laticinios-ovos-bebidas-soja",
        name: "Bebida Soja",
        parent: Some("laticinios-ovos-bebidas-vegetais"),
    },
    Category {
        cgid: "laticinios-ovos-bebidas-aveia",
        name: "Bebida Aveia",
        parent: Some("laticinios-ovos-bebidas-vegetais"),
    },
    Category {
        cgid: "laticinios-ovos-bebidas-amendoa",
        name: "Bebida Amendoa",
        parent: Some("laticinios-ovos-bebidas-vegetais"),
    },
    Category {
        cgid: "laticinios-ovos-bebidas-arroz",
        name: "Bebida Arroz",
        parent: Some("laticinios-ovos-bebidas-vegetais"),
    },
    Category {
        cgid: "laticinios-ovos-bebidas-outras",
        name: "Outras Bebidas Vegetais",
        parent: Some("laticinios-ovos-bebidas-vegetais"),
    },
    // L2: Sobremesas
    Category {
        cgid: "laticinios-sobremesas",
        name: "Sobremesas",
        parent: Some("laticinios"),
    },
    // L3: Sobremesas children
    Category {
        cgid: "laticinios-sobremesas-gelatinas",
        name: "Gelatinas",
        parent: Some("laticinios-sobremesas"),
    },
    Category {
        cgid: "laticinios-sobremesas-mousses",
        name: "Mousses e Pudins",
        parent: Some("laticinios-sobremesas"),
    },
    // ── 3. CONGELADOS ───────────────────────────────────────────────────
    // L2: Frutas e Legumes
    Category {
        cgid: "congelados-vegetais",
        name: "Frutas e Legumes",
        parent: Some("congelados"),
    },
    // L3: Frutas e Legumes children
    Category {
        cgid: "congelados-vegetais-legumes-congelados",
        name: "Legumes",
        parent: Some("congelados-vegetais"),
    },
    Category {
        cgid: "congelados-vegetais-mistura-vegetais",
        name: "Misturas de Legumes",
        parent: Some("congelados-vegetais"),
    },
    Category {
        cgid: "congelados-vegetais-frutas",
        name: "Frutas",
        parent: Some("congelados-vegetais"),
    },
    // L2: Batata Frita e Pure
    Category {
        cgid: "congelados-vegetais-batatas",
        name: "Batata Frita e Pure",
        parent: Some("congelados"),
    },
    // L2: Nuggets e Crocantes
    Category {
        cgid: "congelados-douradinhos",
        name: "Nuggets e Crocantes",
        parent: Some("congelados"),
    },
    // L2: Douradinhos e Filetes
    Category {
        cgid: "congelados-douradinhos-barrinhas",
        name: "Douradinhos e Filetes",
        parent: Some("congelados"),
    },
    // L2: Hamburgueres e Almondegas
    Category {
        cgid: "congelados-refeicoes-hamburguer",
        name: "Hamburgueres e Almondegas",
        parent: Some("congelados"),
    },
    // L2: Peixe, Marisco e Carne
    Category {
        cgid: "congelados-peixe",
        name: "Peixe, Marisco e Carne",
        parent: Some("congelados"),
    },
    // L3: Peixe, Marisco e Carne children
    Category {
        cgid: "congelados-peixe-congelado",
        name: "Peixe",
        parent: Some("congelados-peixe"),
    },
    Category {
        cgid: "congelados-peixe-marisco",
        name: "Marisco",
        parent: Some("congelados-peixe"),
    },
    Category {
        cgid: "congelados-peixe-bacalhau",
        name: "Bacalhau",
        parent: Some("congelados-peixe"),
    },
    Category {
        cgid: "congelados-peixe-polvo",
        name: "Polvo, Lulas e Chocos",
        parent: Some("congelados-peixe"),
    },
    Category {
        cgid: "congelados-carne",
        name: "Carne",
        parent: Some("congelados-peixe"),
    },
    // L2: Pizzas
    Category {
        cgid: "congelados-pizzas",
        name: "Pizzas",
        parent: Some("congelados"),
    },
    // L2: Refeicoes Prontas
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes",
        name: "Refeicoes Prontas",
        parent: Some("congelados"),
    },
    // L3: Refeicoes Prontas children
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes-carne",
        name: "Carne",
        parent: Some("congelados-refeicoes-massa-refeicoes"),
    },
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes-peixe",
        name: "Peixe",
        parent: Some("congelados-refeicoes-massa-refeicoes"),
    },
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes-gnochis",
        name: "Massas e Gnocchis",
        parent: Some("congelados-refeicoes-massa-refeicoes"),
    },
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes-misturas",
        name: "Salteados e Sopas",
        parent: Some("congelados-refeicoes-massa-refeicoes"),
    },
    // L2: Salgados, Folhados e Pastelaria
    Category {
        cgid: "congelados-salgados-folhados",
        name: "Salgados, Folhados e Pastelaria",
        parent: Some("congelados"),
    },
    // L3: Salgados, Folhados e Pastelaria children
    Category {
        cgid: "congelados-salgados-folhados-salgados",
        name: "Salgados",
        parent: Some("congelados-salgados-folhados"),
    },
    Category {
        cgid: "congelados-salgados-folhados-folhados",
        name: "Folhados",
        parent: Some("congelados-salgados-folhados"),
    },
    Category {
        cgid: "congelados-pastelaria",
        name: "Pastelaria Doce",
        parent: Some("congelados-salgados-folhados"),
    },
    Category {
        cgid: "congelados-salgados-folhados-pao",
        name: "Pao de Alho e Pao de Queijo",
        parent: Some("congelados-salgados-folhados"),
    },
    // L2: Vegetariano e Vegan
    Category {
        cgid: "congelados-vegetariano-vegan",
        name: "Vegetariano e Vegan",
        parent: Some("congelados"),
    },
    // L3: Vegetariano e Vegan children
    Category {
        cgid: "congelados-vegetariano-vegan-hamburgueres",
        name: "Hamburgueres e Almondegas",
        parent: Some("congelados-vegetariano-vegan"),
    },
    Category {
        cgid: "congelados-vegetariano-vegan-nuggets-panados",
        name: "Nuggets e Panados",
        parent: Some("congelados-vegetariano-vegan"),
    },
    Category {
        cgid: "congelados-vegetariano-vegan-pizzas-falafel",
        name: "Refeicoes, Pizzas e Falafel",
        parent: Some("congelados-vegetariano-vegan"),
    },
    // L2: Gelados
    Category {
        cgid: "congelados-gelados",
        name: "Gelados",
        parent: Some("congelados"),
    },
    // L3: Gelados children
    Category {
        cgid: "congelados-gelados-cone",
        name: "Gelados de Cone",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-pauzinho",
        name: "Gelados de Pauzinho",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-familiares",
        name: "Gelados Familiares",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-americanos",
        name: "Gelados Americanos",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-bites",
        name: "Mini Bites e Sandwich",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-tartes",
        name: "Tartes Geladas e Viennettas",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-infantis",
        name: "Gelados Infantis",
        parent: Some("congelados-gelados"),
    },
    Category {
        cgid: "congelados-gelados-vegan",
        name: "Gelados Vegan",
        parent: Some("congelados-gelados"),
    },
    // L2: Sobremesas
    Category {
        cgid: "congelados-sobremesas",
        name: "Sobremesas",
        parent: Some("congelados"),
    },
    // L3: Sobremesas children
    Category {
        cgid: "congelados-sobremesas-bolos-congelados",
        name: "Bolos Congelados",
        parent: Some("congelados-sobremesas"),
    },
    Category {
        cgid: "congelados-sobremesas-crepes-petit",
        name: "Crepes e Petit Gateau",
        parent: Some("congelados-sobremesas"),
    },
    // ── 4. MERCEARIA ────────────────────────────────────────────────────
    // L2: Cafe, Cha e Bebidas Soluveis
    Category {
        cgid: "mercearias-cafe-cha",
        name: "Cafe, Cha e Bebidas Soluveis",
        parent: Some("mercearias"),
    },
    // L3: Cafe, Cha children
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-cafe-capsulas",
        name: "Cafe em Capsulas",
        parent: Some("mercearias-cafe-cha"),
    },
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-cafe-torrado",
        name: "Cafe Torrado",
        parent: Some("mercearias-cafe-cha"),
    },
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-cafe-soluvel",
        name: "Cafe Soluvel",
        parent: Some("mercearias-cafe-cha"),
    },
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-chas",
        name: "Chas e Infusoes",
        parent: Some("mercearias-cafe-cha"),
    },
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-achocolatados",
        name: "Chocolate Soluvel",
        parent: Some("mercearias-cafe-cha"),
    },
    Category {
        cgid: "mercearia-cha-cafe-achocolatados-bebidas",
        name: "Bebidas de Cereais",
        parent: Some("mercearias-cafe-cha"),
    },
    // L2: Cereais e Barras
    Category {
        cgid: "mercearias-cereais-barras",
        name: "Cereais e Barras",
        parent: Some("mercearias"),
    },
    // L2: Bolachas, Biscoitos e Tostas
    Category {
        cgid: "mercearias-bolachas-biscoitos",
        name: "Bolachas, Biscoitos e Tostas",
        parent: Some("mercearias"),
    },
    // L2: Chocolate, Gomas e Rebucados
    Category {
        cgid: "mercearias-chocolate",
        name: "Chocolate, Gomas e Rebucados",
        parent: Some("mercearias"),
    },
    // L2: Arroz, Massa e Farinha
    Category {
        cgid: "mercearias-arroz-massa",
        name: "Arroz, Massa e Farinha",
        parent: Some("mercearias"),
    },
    // L2: Azeite, Oleo e Vinagre
    Category {
        cgid: "mercearias-azeite-oleo-vinagre",
        name: "Azeite, Oleo e Vinagre",
        parent: Some("mercearias"),
    },
    // L2: Conservas
    Category {
        cgid: "mercearias-conservas",
        name: "Conservas",
        parent: Some("mercearias"),
    },
    // L2: Molhos, Temperos e Sal
    Category {
        cgid: "mercearias-molhos-temperos",
        name: "Molhos, Temperos e Sal",
        parent: Some("mercearias"),
    },
    // L2: Snacks e Batatas Fritas
    Category {
        cgid: "mercearias-snacks",
        name: "Snacks e Batatas Fritas",
        parent: Some("mercearias"),
    },
    // L2: Compotas, Cremes e Mel
    Category {
        cgid: "mercearias-compotas",
        name: "Compotas, Cremes e Mel",
        parent: Some("mercearias"),
    },
    // L2: Acucar e Sobremesas
    Category {
        cgid: "mercearias-acucar",
        name: "Acucar e Sobremesas",
        parent: Some("mercearias"),
    },
    // L2: Alimentacao Infantil
    Category {
        cgid: "mercearias-alimentacao-infantil",
        name: "Alimentacao Infantil",
        parent: Some("mercearias"),
    },
    // ── 5. BEBIDAS E GARRAFEIRA ─────────────────────────────────────────
    // L2: Sumos e Refrigerantes
    Category {
        cgid: "bebidas-sumos-refrigerantes",
        name: "Sumos e Refrigerantes",
        parent: Some("bebidas"),
    },
    // L2: Agua
    Category {
        cgid: "bebidas-agua",
        name: "Agua",
        parent: Some("bebidas"),
    },
    // L3: Agua children
    Category {
        cgid: "bebidas-agua-sem-gas",
        name: "Agua sem Gas",
        parent: Some("bebidas-agua"),
    },
    Category {
        cgid: "bebidas-agua-com-gas",
        name: "Agua com Gas",
        parent: Some("bebidas-agua"),
    },
    Category {
        cgid: "bebidas-agua-tonica",
        name: "Agua Tonica e Ginger Ale",
        parent: Some("bebidas-agua"),
    },
    Category {
        cgid: "bebidas-agua-sabor",
        name: "Agua com Sabor",
        parent: Some("bebidas-agua"),
    },
    // L2: Bebidas Energeticas e Isotonicas
    Category {
        cgid: "bebidas-bebidas-energeticas",
        name: "Bebidas Energeticas e Isotonicas",
        parent: Some("bebidas"),
    },
    // L2: Cervejas e Sidras
    Category {
        cgid: "bebidas-cervejas-sidras",
        name: "Cervejas e Sidras",
        parent: Some("bebidas"),
    },
    // L2: Vinhos
    Category {
        cgid: "bebidas-vinho",
        name: "Vinhos",
        parent: Some("bebidas"),
    },
    // L2: Bebidas Espirituosas
    Category {
        cgid: "bebidas-espirituosas",
        name: "Bebidas Espirituosas",
        parent: Some("bebidas"),
    },
    // L2: Champanhe e Espumante
    Category {
        cgid: "bebidas-champanhe-espumante",
        name: "Champanhe e Espumante",
        parent: Some("bebidas"),
    },
    // ── 6. BIO E SAUDAVEL ───────────────────────────────────────────────
    // L2: Suplementos e Vitaminas
    Category {
        cgid: "bio-suplementos",
        name: "Suplementos e Vitaminas",
        parent: Some("biologicos"),
    },
    // L2: Nutricao Desportiva
    Category {
        cgid: "bio-nutricao-desportiva",
        name: "Nutricao Desportiva",
        parent: Some("biologicos"),
    },
    // L2: Vegetariano e Vegan
    Category {
        cgid: "bio-vegetariano-vegan",
        name: "Vegetariano e Vegan",
        parent: Some("biologicos"),
    },
    // L2: Biologicos
    Category {
        cgid: "bio-biologicos",
        name: "Biologicos",
        parent: Some("biologicos"),
    },
    // L2: Sem Gluten
    Category {
        cgid: "bio-sem-gluten",
        name: "Sem Gluten",
        parent: Some("biologicos"),
    },
    // L2: Sem Lactose
    Category {
        cgid: "bio-sem-lactose",
        name: "Sem Lactose",
        parent: Some("biologicos"),
    },
    // ── 7. LIMPEZA ──────────────────────────────────────────────────────
    // L2: Roupa
    Category {
        cgid: "limpeza-roupa",
        name: "Roupa",
        parent: Some("limpeza"),
    },
    // L2: Cozinha
    Category {
        cgid: "limpeza-cozinha",
        name: "Cozinha",
        parent: Some("limpeza"),
    },
    // L2: Casa de Banho
    Category {
        cgid: "limpeza-wc",
        name: "Casa de Banho",
        parent: Some("limpeza"),
    },
    // L2: Chao e Superficies
    Category {
        cgid: "limpeza-geral",
        name: "Chao e Superficies",
        parent: Some("limpeza"),
    },
    // L2: Guardanapos e Rolos
    Category {
        cgid: "limpeza-produtos-papel",
        name: "Guardanapos e Rolos",
        parent: Some("limpeza"),
    },
    // L2: Velas e Ambientadores
    Category {
        cgid: "limpeza-ambientadores",
        name: "Velas e Ambientadores",
        parent: Some("limpeza"),
    },
    // L2: Sacos e Baldes do Lixo
    Category {
        cgid: "limpeza-baldes-ecopontos-sacos",
        name: "Sacos e Baldes do Lixo",
        parent: Some("limpeza"),
    },
    // L2: Mopas, Esfregonas e Vassouras
    Category {
        cgid: "limpeza-panos-baldes-vassouras",
        name: "Mopas, Esfregonas e Vassouras",
        parent: Some("limpeza"),
    },
    // L2: Panos, Esfregoes e Luvas
    Category {
        cgid: "limpeza-panos-esfregoes-luvas",
        name: "Panos, Esfregoes e Luvas",
        parent: Some("limpeza"),
    },
    // L2: Inseticidas e Desumidificadores
    Category {
        cgid: "limpeza-inseticidas",
        name: "Inseticidas e Desumidificadores",
        parent: Some("limpeza"),
    },
    // L2: Limpeza Auto e Motos
    Category {
        cgid: "limpeza-auto-motos",
        name: "Limpeza Auto e Motos",
        parent: Some("limpeza"),
    },
    // ── 8. BEBE ─────────────────────────────────────────────────────────
    // L2: Alimentacao Infantil
    Category {
        cgid: "bebe-alimentacao-infantil",
        name: "Alimentacao Infantil",
        parent: Some("bebe"),
    },
    // L2: Fraldas e Toalhitas
    Category {
        cgid: "bebe-fraldas-toalhitas",
        name: "Fraldas e Toalhitas",
        parent: Some("bebe"),
    },
    // L2: Banho e Higiene
    Category {
        cgid: "bebe-banho-higiene",
        name: "Banho e Higiene",
        parent: Some("bebe"),
    },
    // L2: Cadeiras Auto e Carrinhos
    Category {
        cgid: "bebe-auto-passeio",
        name: "Cadeiras Auto e Carrinhos",
        parent: Some("bebe"),
    },
    // L2: Cadeiras e Acessorios de Refeicao
    Category {
        cgid: "bebe-cadeiras-acessorios",
        name: "Cadeiras e Acessorios de Refeicao",
        parent: Some("bebe"),
    },
    // L2: Mobiliario e Colchoes
    Category {
        cgid: "bebe-mobiliario-colchoes",
        name: "Mobiliario e Colchoes",
        parent: Some("bebe"),
    },
    // L2: Banheiras e Complementos
    Category {
        cgid: "bebe-banheiras-acessorios",
        name: "Banheiras e Complementos",
        parent: Some("bebe"),
    },
    // L2: Textil de Bebe
    Category {
        cgid: "bebe-textil",
        name: "Textil de Bebe",
        parent: Some("bebe"),
    },
    // L2: Chupetas e Mordedores
    Category {
        cgid: "bebe-chupetas-mordedores",
        name: "Chupetas e Mordedores",
        parent: Some("bebe"),
    },
    // L2: Brinquedos e Livros
    Category {
        cgid: "bebe-brinquedos",
        name: "Brinquedos e Livros",
        parent: Some("bebe"),
    },
    // ── 9. BELEZA E HIGIENE ─────────────────────────────────────────────
    // L2: Cabelo
    Category {
        cgid: "higiene-beleza-cabelo",
        name: "Cabelo",
        parent: Some("higiene-beleza"),
    },
    // L2: Corpo
    Category {
        cgid: "higiene-beleza-corpo",
        name: "Corpo",
        parent: Some("higiene-beleza"),
    },
    // L2: Rosto
    Category {
        cgid: "higiene-beleza-rosto",
        name: "Rosto",
        parent: Some("higiene-beleza"),
    },
    // L2: Maquilhagem
    Category {
        cgid: "higiene-beleza-maquilhagem",
        name: "Maquilhagem",
        parent: Some("higiene-beleza"),
    },
    // L2: Higiene Oral
    Category {
        cgid: "higiene-beleza-oral",
        name: "Higiene Oral",
        parent: Some("higiene-beleza"),
    },
    // L2: Higiene Intima
    Category {
        cgid: "higiene-beleza-intima",
        name: "Higiene Intima",
        parent: Some("higiene-beleza"),
    },
    // L2: Homem
    Category {
        cgid: "higiene-beleza-homem",
        name: "Homem",
        parent: Some("higiene-beleza"),
    },
    // L2: Preservativos e Estimuladores
    Category {
        cgid: "higiene-beleza-preservativos",
        name: "Preservativos e Estimuladores",
        parent: Some("higiene-beleza"),
    },
    // L2: Lencos e Cuidados de Saude
    Category {
        cgid: "higiene-beleza-lencos-saude",
        name: "Lencos e Cuidados de Saude",
        parent: Some("higiene-beleza"),
    },
    // L2: Papel Higienico
    Category {
        cgid: "higiene-beleza-papel-lencos",
        name: "Papel Higienico",
        parent: Some("higiene-beleza"),
    },
    // L2: Solares e Bronzeadores
    Category {
        cgid: "higiene-beleza-solares",
        name: "Solares e Bronzeadores",
        parent: Some("higiene-beleza"),
    },
    // L2: Coffrets e Presentes
    Category {
        cgid: "higiene-beleza-perfumes-conjuntos",
        name: "Coffrets e Presentes",
        parent: Some("higiene-beleza"),
    },
    // ── 10. ANIMAIS ─────────────────────────────────────────────────────
    // L2: Cao
    Category {
        cgid: "animais-cao",
        name: "Cao",
        parent: Some("animais"),
    },
    // L2: Gato
    Category {
        cgid: "animais-gato",
        name: "Gato",
        parent: Some("animais"),
    },
    // L2: Outros Animais
    Category {
        cgid: "animais-outros-animais",
        name: "Outros Animais",
        parent: Some("animais"),
    },
    // ── 11. CASA, BRICOLAGE E JARDIM ────────────────────────────────────
    // L2: Mobiliario e Colchoes
    Category {
        cgid: "casa-mobiliario-colchoes",
        name: "Mobiliario e Colchoes",
        parent: Some("casa"),
    },
    // L2: Textil Lar
    Category {
        cgid: "casa-textil-lar",
        name: "Textil Lar",
        parent: Some("casa"),
    },
    // L2: Decoracao
    Category {
        cgid: "casa-decoracao-banho",
        name: "Decoracao",
        parent: Some("casa"),
    },
    // L2: Cozinha
    Category {
        cgid: "casa-cozinha",
        name: "Cozinha",
        parent: Some("casa"),
    },
    // L2: Mesa
    Category {
        cgid: "casa-mesa",
        name: "Mesa",
        parent: Some("casa"),
    },
    // L2: Eletrodomesticos
    Category {
        cgid: "casa-eletrodomesticos",
        name: "Eletrodomesticos",
        parent: Some("casa"),
    },
    // L2: Lavandaria e Organizacao
    Category {
        cgid: "casa-lavandaria-organiza",
        name: "Lavandaria e Organizacao",
        parent: Some("casa"),
    },
    // L2: Festa
    Category {
        cgid: "casa-festa",
        name: "Festa",
        parent: Some("casa"),
    },
    // L2: Jardim
    Category {
        cgid: "casa-jardim",
        name: "Jardim",
        parent: Some("casa"),
    },
    // L2: Pilhas e Lampadas
    Category {
        cgid: "casa-pilhas-lampadas",
        name: "Pilhas e Lampadas",
        parent: Some("casa"),
    },
    // L2: Bricolage
    Category {
        cgid: "casa-bricolage",
        name: "Bricolage",
        parent: Some("casa"),
    },
    // ── 12. BRINQUEDOS E JOGOS ──────────────────────────────────────────
    // L2: LEGO
    Category {
        cgid: "brinquedos-construcoes-lego-1",
        name: "LEGO",
        parent: Some("brinquedos"),
    },
];

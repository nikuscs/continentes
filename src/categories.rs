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

static CATEGORIES: [Category; 91] = [
    // Top-level
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
    // Frescos
    Category {
        cgid: "peixaria-e-talho-peixaria",
        name: "Peixaria",
        parent: Some("frescos"),
    },
    Category {
        cgid: "peixaria-e-talho-talho",
        name: "Talho",
        parent: Some("frescos"),
    },
    Category {
        cgid: "frutas-legumes-frutas",
        name: "Frutas",
        parent: Some("frescos"),
    },
    Category {
        cgid: "frutas-legumes-legumes",
        name: "Legumes",
        parent: Some("frescos"),
    },
    Category {
        cgid: "charcutaria-queijo-queijos",
        name: "Queijos",
        parent: Some("frescos"),
    },
    Category {
        cgid: "charcutaria-queijo-charcutaria",
        name: "Charcutaria",
        parent: Some("frescos"),
    },
    Category {
        cgid: "padaria-e-pastelaria",
        name: "Padaria e Pastelaria",
        parent: Some("frescos"),
    },
    Category {
        cgid: "refeicoes-faceis",
        name: "Take-Away",
        parent: Some("frescos"),
    },
    // Laticinios
    Category {
        cgid: "laticinios-leite",
        name: "Leite",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-iogurtes",
        name: "Iogurtes",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-ovos",
        name: "Ovos",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-manteigas-cremes-vegetais",
        name: "Manteigas e Cremes",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-natas-bechamel-chantilly",
        name: "Natas e Bechamel",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-ovos-bebidas-vegetais",
        name: "Bebidas Vegetais",
        parent: Some("laticinios"),
    },
    Category {
        cgid: "laticinios-sobremesas",
        name: "Sobremesas Laticinios",
        parent: Some("laticinios"),
    },
    // Congelados
    Category {
        cgid: "congelados-vegetais",
        name: "Frutas e Legumes Congelados",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-vegetais-batatas",
        name: "Batata Frita e Pure",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-douradinhos",
        name: "Nuggets e Crocantes",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-refeicoes-hamburguer",
        name: "Hamburgueres e Almondegas",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-peixe",
        name: "Peixe, Marisco e Carne Congelados",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-pizzas",
        name: "Pizzas Congeladas",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-refeicoes-massa-refeicoes",
        name: "Refeicoes Prontas Congeladas",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-salgados-folhados",
        name: "Salgados e Folhados",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-vegetariano-vegan",
        name: "Vegetariano e Vegan Congelados",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-gelados",
        name: "Gelados",
        parent: Some("congelados"),
    },
    Category {
        cgid: "congelados-sobremesas",
        name: "Sobremesas Congeladas",
        parent: Some("congelados"),
    },
    // Mercearia
    Category {
        cgid: "mercearias-cafe-cha",
        name: "Cafe, Cha e Bebidas Soluveis",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-cereais-barras",
        name: "Cereais e Barras",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-bolachas-biscoitos",
        name: "Bolachas e Biscoitos",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-chocolate",
        name: "Chocolate, Gomas e Rebucados",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-arroz-massa",
        name: "Arroz, Massa e Farinha",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-azeite-oleo-vinagre",
        name: "Azeite, Oleo e Vinagre",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-conservas",
        name: "Conservas",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-molhos-temperos",
        name: "Molhos, Temperos e Sal",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-snacks",
        name: "Snacks e Batatas Fritas",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-compotas",
        name: "Compotas, Cremes e Mel",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-acucar",
        name: "Acucar e Sobremesas",
        parent: Some("mercearias"),
    },
    Category {
        cgid: "mercearias-alimentacao-infantil",
        name: "Alimentacao Infantil",
        parent: Some("mercearias"),
    },
    // Bebidas
    Category {
        cgid: "bebidas-sumos-refrigerantes",
        name: "Sumos e Refrigerantes",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-agua",
        name: "Agua",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-bebidas-energeticas",
        name: "Bebidas Energeticas",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-cervejas-sidras",
        name: "Cervejas e Sidras",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-vinho",
        name: "Vinhos",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-espirituosas",
        name: "Bebidas Espirituosas",
        parent: Some("bebidas"),
    },
    Category {
        cgid: "bebidas-champanhe-espumante",
        name: "Champanhe e Espumante",
        parent: Some("bebidas"),
    },
    // Bio
    Category {
        cgid: "bio-suplementos",
        name: "Suplementos e Vitaminas",
        parent: Some("biologicos"),
    },
    Category {
        cgid: "bio-nutricao-desportiva",
        name: "Nutricao Desportiva",
        parent: Some("biologicos"),
    },
    Category {
        cgid: "bio-vegetariano-vegan",
        name: "Vegetariano e Vegan",
        parent: Some("biologicos"),
    },
    Category {
        cgid: "bio-biologicos",
        name: "Biologicos",
        parent: Some("biologicos"),
    },
    Category {
        cgid: "bio-sem-gluten",
        name: "Sem Gluten",
        parent: Some("biologicos"),
    },
    Category {
        cgid: "bio-sem-lactose",
        name: "Sem Lactose",
        parent: Some("biologicos"),
    },
    // Limpeza
    Category {
        cgid: "limpeza-roupa",
        name: "Roupa",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-cozinha",
        name: "Cozinha Limpeza",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-wc",
        name: "Casa de Banho",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-geral",
        name: "Chao e Superficies",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-produtos-papel",
        name: "Guardanapos e Rolos",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-ambientadores",
        name: "Velas e Ambientadores",
        parent: Some("limpeza"),
    },
    Category {
        cgid: "limpeza-inseticidas",
        name: "Inseticidas",
        parent: Some("limpeza"),
    },
    // Bebe
    Category {
        cgid: "bebe-alimentacao-infantil",
        name: "Alimentacao Infantil Bebe",
        parent: Some("bebe"),
    },
    Category {
        cgid: "bebe-fraldas-toalhitas",
        name: "Fraldas e Toalhitas",
        parent: Some("bebe"),
    },
    Category {
        cgid: "bebe-banho-higiene",
        name: "Banho e Higiene Bebe",
        parent: Some("bebe"),
    },
    Category {
        cgid: "bebe-auto-passeio",
        name: "Cadeiras Auto e Carrinhos",
        parent: Some("bebe"),
    },
    // Beleza e Higiene
    Category {
        cgid: "higiene-beleza-cabelo",
        name: "Cabelo",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-corpo",
        name: "Corpo",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-rosto",
        name: "Rosto",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-maquilhagem",
        name: "Maquilhagem",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-oral",
        name: "Higiene Oral",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-intima",
        name: "Higiene Intima",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-homem",
        name: "Homem",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-papel-lencos",
        name: "Papel Higienico",
        parent: Some("higiene-beleza"),
    },
    Category {
        cgid: "higiene-beleza-solares",
        name: "Solares e Bronzeadores",
        parent: Some("higiene-beleza"),
    },
    // Animais
    Category {
        cgid: "animais-cao",
        name: "Cao",
        parent: Some("animais"),
    },
    Category {
        cgid: "animais-gato",
        name: "Gato",
        parent: Some("animais"),
    },
    Category {
        cgid: "animais-outros-animais",
        name: "Outros Animais",
        parent: Some("animais"),
    },
    // Casa
    Category {
        cgid: "casa-mobiliario-colchoes",
        name: "Mobiliario e Colchoes",
        parent: Some("casa"),
    },
    Category {
        cgid: "casa-textil-lar",
        name: "Textil Lar",
        parent: Some("casa"),
    },
    Category {
        cgid: "casa-decoracao-banho",
        name: "Decoracao",
        parent: Some("casa"),
    },
    Category {
        cgid: "casa-cozinha",
        name: "Cozinha Casa",
        parent: Some("casa"),
    },
    Category {
        cgid: "casa-eletrodomesticos",
        name: "Eletrodomesticos",
        parent: Some("casa"),
    },
    Category {
        cgid: "casa-jardim",
        name: "Jardim",
        parent: Some("casa"),
    },
];

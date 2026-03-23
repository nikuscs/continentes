use continente::categories::{all_categories, resolve_cgid};

#[test]
fn resolve_cgid_exact_match() {
    assert_eq!(resolve_cgid("laticinios"), Some("laticinios"));
}

#[test]
fn resolve_cgid_by_name_case_insensitive() {
    assert_eq!(resolve_cgid("frescos"), Some("frescos"));
    assert_eq!(resolve_cgid("Frescos"), Some("frescos"));
}

#[test]
fn resolve_cgid_partial_match() {
    assert_eq!(resolve_cgid("leite"), Some("laticinios-leite"));
}

#[test]
fn resolve_cgid_partial_name_match_returns_cgid() {
    assert_eq!(resolve_cgid("iogur"), Some("laticinios-iogurtes"));
}

#[test]
fn resolve_cgid_unknown_returns_none() {
    assert_eq!(resolve_cgid("nonexistent"), None);
}

#[test]
fn all_categories_is_not_empty() {
    assert!(all_categories().len() > 50);
}

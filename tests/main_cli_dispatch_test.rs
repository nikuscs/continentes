use std::io::Write as _;

use assert_cmd::Command;

fn cnt() -> Command {
    Command::cargo_bin("cnt").expect("binary should exist")
}

#[test]
fn categories_dispatches_with_default_table_output() {
    let assert = cnt().args(["categories"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("Categories:"));
    assert!(stdout.contains("Frescos (frescos)"));
}

#[test]
fn categories_uses_configured_json_output() {
    let mut config = tempfile::NamedTempFile::new().unwrap();
    write!(config, "[output]\nformat = \"json\"\n").unwrap();

    let assert = cnt()
        .args(["--config", config.path().to_str().unwrap(), "categories"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 251);
}

#[test]
fn format_flag_overrides_config_file() {
    let mut config = tempfile::NamedTempFile::new().unwrap();
    write!(config, "[output]\nformat = \"json\"\n").unwrap();

    let assert = cnt()
        .args([
            "--config",
            config.path().to_str().unwrap(),
            "--format",
            "compact",
            "categories",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains('\t'));
    assert!(!stdout.trim_start().starts_with('['));
}

#[test]
fn suggest_short_query_fails_before_network() {
    let assert = cnt().args(["suggest", "lei"]).assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);

    assert!(stderr.contains("Query must be at least 5 characters"));
}

#[test]
#[ignore = "requires network"]
fn search_dispatches_to_real_endpoint() {
    let assert = cnt()
        .args(["search", "leite", "--max", "1", "--format", "compact"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains('\t'));
    assert!(stdout.contains("leite") || stdout.contains("Leite"));
}

#[test]
#[ignore = "requires network"]
fn product_dispatches_to_real_endpoint() {
    let assert = cnt()
        .args(["product", "6879912", "--format", "compact"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("6879912"));
    assert!(stdout.contains("Continente"));
}

#[test]
#[ignore = "requires network"]
fn browse_dispatches_to_real_endpoint() {
    let assert = cnt()
        .args(["browse", "frescos", "--max", "1", "--format", "compact"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains('\t'));
    assert!(!stdout.trim().is_empty());
}

#[test]
#[ignore = "requires network"]
fn stores_dispatches_to_real_endpoint() {
    let assert = cnt()
        .args([
            "stores", "--lat", "38.7", "--lon", "-9.1", "--radius", "10", "--format", "compact",
        ])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains('\t'));
    assert!(!stdout.trim().is_empty());
}

#[test]
#[ignore = "requires network"]
fn suggest_dispatches_to_real_endpoint() {
    let assert = cnt()
        .args(["suggest", "leite", "--format", "compact"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(!stdout.trim().is_empty());
}

use std::io::Write as _;

use continente::config::{Config, load_config, try_load};
use continente::format::OutputFormat;

#[test]
fn default_config_values() {
    let cfg = Config::default();

    assert_eq!(cfg.http.timeout_secs, 30);
    assert_eq!(cfg.http.retries, 3);
    assert_eq!(cfg.http.delay_ms, 100);
    assert_eq!(cfg.output.format, OutputFormat::Table);
}

#[test]
fn load_config_missing_explicit_path_returns_error() {
    let path = std::path::Path::new("/tmp/continente_missing_config_xyz.toml");
    let result = load_config(Some(path));
    assert!(result.is_err(), "missing explicit path should return error");
}

#[test]
fn load_config_parses_valid_toml() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    write!(
        f,
        r#"
[http]
timeout_secs = 5
retries = 1

[output]
format = "json"
"#
    )
    .unwrap();

    let cfg = load_config(Some(f.path())).unwrap();
    assert_eq!(cfg.http.timeout_secs, 5);
    assert_eq!(cfg.http.retries, 1);
    assert_eq!(cfg.http.delay_ms, 100);
    assert_eq!(cfg.output.format, OutputFormat::Json);
}

#[test]
fn load_config_invalid_toml_returns_error_for_explicit_path() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    write!(f, "this is not valid toml {{").unwrap();
    let result = load_config(Some(f.path()));
    assert!(
        result.is_err(),
        "invalid explicit config should return error"
    );
}

#[test]
fn load_config_none_uses_defaults_when_no_file_exists() {
    let dir = tempfile::TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    let result = load_config(None);

    std::env::set_current_dir(original_dir).unwrap();

    let cfg = result.unwrap();
    assert_eq!(cfg.output.format, OutputFormat::Table);
    assert_eq!(cfg.http.timeout_secs, 30);
}

#[test]
fn load_config_none_local_invalid_toml_falls_back_to_defaults() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("continente.toml"), "not [ valid toml").unwrap();

    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    let result = load_config(None);

    std::env::set_current_dir(original_dir).unwrap();

    let cfg = result.unwrap();
    assert_eq!(cfg.output.format, OutputFormat::Table);
    assert_eq!(cfg.http.retries, 3);
}

#[test]
fn try_load_missing_returns_none() {
    let path = std::path::Path::new("/tmp/continente_try_load_missing_xyz.toml");
    let result = try_load(path).unwrap();
    assert!(result.is_none());
}

#[test]
fn try_load_valid_file_returns_config() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    write!(f, "[output]\nformat = \"compact\"\n").unwrap();

    let cfg = try_load(f.path()).unwrap().unwrap();
    assert_eq!(cfg.output.format, OutputFormat::Compact);
}

#[test]
fn try_load_invalid_file_returns_error() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    write!(f, "[output\nformat = json").unwrap();

    let result = try_load(f.path());
    assert!(result.is_err());
}

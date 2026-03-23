use std::io::Write as _;

use assert_cmd::Command;

fn cnt() -> Command {
    Command::cargo_bin("cnt").expect("binary should exist")
}

#[test]
fn help_exits_zero() {
    cnt().arg("--help").assert().success();
}

#[test]
fn search_help_exits_zero() {
    cnt().args(["search", "--help"]).assert().success();
}

#[test]
fn unknown_subcommand_exits_nonzero() {
    cnt().arg("nonexistent").assert().failure();
}

#[test]
fn format_flag_accepts_json() {
    cnt()
        .args(["--format", "json", "categories"])
        .assert()
        .success();
}

#[test]
fn explicit_missing_config_exits_nonzero() {
    cnt()
        .args([
            "--config",
            "/tmp/continente_missing_cli_config_xyz.toml",
            "categories",
        ])
        .assert()
        .failure();
}

#[test]
fn search_rejects_page_zero() {
    cnt()
        .args(["search", "leite", "--page", "0"])
        .assert()
        .failure();
}

#[test]
fn search_rejects_max_zero() {
    cnt()
        .args(["search", "leite", "--max", "0"])
        .assert()
        .failure();
}

#[test]
fn browse_rejects_page_zero() {
    cnt()
        .args(["browse", "frescos", "--page", "0"])
        .assert()
        .failure();
}

#[test]
fn stores_accepts_negative_latitude() {
    cnt()
        .args([
            "stores", "--lat", "-38.7", "--lon", "-9.1", "--radius", "10", "--format", "compact",
        ])
        .assert()
        .success();
}

#[test]
fn config_file_sets_default_output_format() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    write!(f, "[output]\nformat = \"json\"\n").unwrap();

    let assert = cnt()
        .args(["--config", f.path().to_str().unwrap(), "categories"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.trim_start().starts_with('['),
        "stdout should be json: {stdout}"
    );
}

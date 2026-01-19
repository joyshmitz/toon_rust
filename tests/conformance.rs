#![cfg(feature = "conformance")]

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureFile {
    version: Option<String>,
    category: String,
    description: String,
    tests: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    name: String,
    input: serde_json::Value,
    expected: Option<String>,
    expected_json: Option<serde_json::Value>,
    options: Option<serde_json::Value>,
    should_error: Option<bool>,
    note: Option<String>,
    spec_section: Option<String>,
}

#[test]
#[ignore]
fn conformance_encode_fixtures() {
    let root = fixture_root();
    let encode_dir = root.join("encode");
    let files = load_fixture_files(&encode_dir);

    // TODO: replace with toon_rust::encode once implemented.
    // For each test case:
    // - apply options
    // - if should_error: assert encode() errors
    // - else: compare output to expected
    eprintln!("encode fixture files loaded: {}", files.len());
}

#[test]
#[ignore]
fn conformance_decode_fixtures() {
    let root = fixture_root();
    let decode_dir = root.join("decode");
    let files = load_fixture_files(&decode_dir);

    // TODO: replace with toon_rust::decode once implemented.
    // For each test case:
    // - apply options
    // - if should_error: assert decode() errors
    // - else: compare output to expected_json or expected
    eprintln!("decode fixture files loaded: {}", files.len());
}

fn fixture_root() -> PathBuf {
    if let Ok(path) = std::env::var("TOON_SPEC_FIXTURES") {
        return PathBuf::from(path);
    }

    PathBuf::from("tests/fixtures/spec")
}

fn load_fixture_files(dir: &Path) -> Vec<FixtureFile> {
    if !dir.exists() {
        return Vec::new();
    }

    let mut fixtures = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return fixtures,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => continue,
        };

        let parsed: FixtureFile = match serde_json::from_str(&contents) {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };

        fixtures.push(parsed);
    }

    fixtures
}

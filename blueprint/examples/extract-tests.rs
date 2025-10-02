#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;

use serde_json::Value;

use blueprint::{Version, bp_string_to_json};

fn main() {
    let root: Value = serde_json::from_str(
        &bp_string_to_json(include_str!("../tests/2.0/test_book.txt")).unwrap(),
    )
    .unwrap();

    extract_book(&root, None);
}

fn extract_book(root: &Value, path: Option<&Path>) {
    let Value::Object(root) = root else {
        panic!("Expected a JSON object");
    };

    let Some(Value::Object(book)) = root.get("blueprint_book") else {
        panic!("Expected a JSON object under the key 'blueprint_book'");
    };

    if path.is_none() {
        let Some(Value::Number(version)) = book.get("version") else {
            panic!("Expected a JSON number under the key 'version'");
        };

        let version = Version::from(version.as_u64().unwrap());
        println!("Test book from game version {version}");
    }

    let path = path.unwrap_or_else(|| Path::new(""));

    let Some(Value::Array(entries)) = book.get("blueprints") else {
        panic!("Expected a JSON array under the key 'blueprints'");
    };

    for entry in entries {
        let Value::Object(entry) = entry else {
            panic!("Expected a JSON object in the array");
        };

        let mut entry = entry.clone();
        entry.remove("index");

        let Some(Value::Object(inner)) = entry.get(entry.keys().next().unwrap()) else {
            panic!("Expected a JSON object in the entry");
        };

        let Some(Value::String(name)) = inner.get("label") else {
            panic!("Book entry has no label");
        };

        if entry.contains_key("blueprint_book") {
            let sub_path = path.join(name);
            std::fs::create_dir_all(&sub_path).expect("creating book subdirectory should succeed");

            extract_book(&Value::Object(entry), Some(&sub_path));
            continue;
        }

        let target_path = path.join(format!("{name}.json"));
        println!("Extracting test book entry {}", target_path.display());
        let json = serde_json::to_string_pretty(&entry).unwrap();
        std::fs::write(target_path, json).unwrap();
    }
}

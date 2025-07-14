#![allow(clippy::unwrap_used)]

use serde_json::Value;

use blueprint::{bp_string_to_json, Version};

fn main() {
    let root: Value = serde_json::from_str(
        &bp_string_to_json(include_str!("../tests/2.0/test_book.txt")).unwrap(),
    )
    .unwrap();

    let Value::Object(root) = root else {
        panic!("Expected a JSON object");
    };

    let Some(Value::Object(book)) = root.get("blueprint_book") else {
        panic!("Expected a JSON object under the key 'blueprint_book'");
    };

    let Some(Value::Number(version)) = book.get("version") else {
        panic!("Expected a JSON number under the key 'version'");
    };

    let version = Version::from(version.as_u64().unwrap());
    println!("Test book from game version {version}");

    let Some(Value::Array(entries)) = book.get("blueprints") else {
        panic!("Expected a JSON array under the key 'blueprints'");
    };

    for (id, entry) in entries.iter().enumerate() {
        let Value::Object(entry) = entry else {
            panic!("Expected a JSON object in the array");
        };

        let mut entry = entry.clone();
        entry.remove("index");

        let Some(Value::Object(inner)) = entry.get(entry.keys().next().unwrap()) else {
            panic!("Expected a JSON object in the entry");
        };

        let Some(Value::String(name)) = inner.get("label") else {
            panic!("Test book entry {id} has no label")
        };

        println!("Extracting test book entry {id}: {name}");

        let json = serde_json::to_string_pretty(&entry).unwrap();

        std::fs::write(format!("{name}.json"), json).unwrap();
    }
}

use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct ModEntry {
    pub name: String,
    pub enabled: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModList {
    pub mods: Vec<ModEntry>,
}

impl ModList {
    #[must_use]
    pub fn load(list_path: &Path) -> Option<Self> {
        let mut bytes = Vec::new();
        File::open(list_path).ok()?.read_to_end(&mut bytes).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}

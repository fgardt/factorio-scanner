use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use anyhow::Result;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
struct ModEntry {
    pub name: String,
    pub enabled: bool,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModListFormat {
    pub mods: Vec<ModEntry>,
}

impl ModListFormat {
    #[must_use]
    fn load(list_path: &Path) -> Option<Self> {
        let mut bytes = Vec::new();
        File::open(list_path).ok()?.read_to_end(&mut bytes).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub enabled: bool,
    pub versions: Vec<(String, String)>,
    pub active_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModList<'a> {
    mods_folder: &'a Path,
    list: HashMap<String, Entry>,
}

impl<'a> From<&ModList<'a>> for ModListFormat {
    fn from(list: &ModList) -> Self {
        let mut mods = Vec::new();

        for (name, entry) in &list.list {
            mods.push(ModEntry {
                name: name.clone(),
                enabled: entry.enabled,
                version: entry.active_version.clone(),
            });
        }

        Self { mods }
    }
}

impl<'a> From<ModList<'a>> for ModListFormat {
    fn from(list: ModList) -> Self {
        (&list).into()
    }
}

impl<'a> ModList<'a> {
    #[must_use]
    pub fn load(mods_folder: &'a Path) -> Option<Self> {
        let tmp = ModListFormat::load(&mods_folder.join("mod-list.json"))?;
        let mut list = HashMap::new();

        for entry in tmp.mods {
            let versions = entry
                .version
                .as_ref()
                .map_or_else(Vec::new, |v| vec![(String::new(), v.clone())]); // the filename is just "" since we cant know it here :/

            list.insert(
                entry.name.clone(),
                Entry {
                    enabled: entry.enabled,
                    versions,
                    active_version: entry.version,
                },
            );
        }

        Some(Self { mods_folder, list })
    }

    pub fn generate(mods_folder: &'a Path) -> Result<Self> {
        let filename_extractor = regex::Regex::new(r"^(.+?)(?:_(\d+\.\d+\.\d+)(?:\.zip)?)?$")?;

        let mut list = HashMap::new();

        list.insert(
            "base".to_string(),
            Entry {
                enabled: true,
                versions: vec![],
                active_version: None,
            },
        );

        let paths = fs::read_dir(mods_folder)?;
        for path in paths {
            let Ok(path) = path else {
                continue;
            };

            let filename = path.file_name();
            let Some(filename) = filename.to_str() else {
                continue;
            };

            let path = path.path();
            let Some(extracted) = filename_extractor.captures(filename) else {
                continue;
            };
            let Some(name) = extracted.get(1).map(|n| n.as_str().to_owned()) else {
                continue;
            };
            let version = extracted.get(2).map(|v| v.as_str().to_owned());

            let version = if path.is_file() && filename.to_lowercase().ends_with(".zip") {
                match version {
                    Some(version) => version,
                    None => continue, // should not happen
                }
            } else if path.is_dir() {
                if let Some(version) = version {
                    version
                } else {
                    let Ok(info_file) = fs::read_to_string(path.join("info.json")) else {
                        continue;
                    };

                    if let Ok(info) = serde_json::from_str::<crate::mod_info::ModInfo>(&info_file) {
                        info.version
                    } else {
                        continue;
                    }
                }
            } else {
                continue;
            };

            if list.contains_key(&name) {
                let Some(entry) = list.get_mut(&name) else {
                    continue;
                };
                let entry: &mut Entry = entry;
                entry.versions.push((filename.to_owned(), version));
            } else {
                list.insert(
                    name.clone(),
                    Entry {
                        enabled: false,
                        versions: vec![(filename.to_owned(), version)],
                        active_version: None,
                    },
                );
            }
        }

        Ok(Self { mods_folder, list })
    }

    pub fn save(&self) -> Result<()> {
        let format: ModListFormat = self.into();
        let bytes = serde_json::to_vec_pretty(&format)?;
        std::fs::write(self.mods_folder.join("mod-list.json"), bytes)?;

        Ok(())
    }

    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.list.contains_key(name)
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.list.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Entry> {
        self.list.get_mut(name)
    }
}

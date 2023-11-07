use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use anyhow::Result;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::mod_info::Version;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
struct ModEntry {
    pub name: String,
    pub enabled: bool,
    pub version: Option<Version>,
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
    pub versions: HashMap<Version, String>,
    pub active_version: Option<Version>,
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
                version: entry.active_version.map(Into::into),
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
        let Some(tmp) = ModListFormat::load(&mods_folder.join("mod-list.json")) else {
            return Self::generate(mods_folder).ok();
        };

        let mut list = HashMap::new();
        for entry in tmp.mods {
            let versions = entry
                .version
                .as_ref()
                .map_or_else(Vec::new, |v| vec![(*v, String::new())]); // the filename is just "" since we cant know it here :/

            list.insert(
                entry.name.clone(),
                Entry {
                    enabled: entry.enabled,
                    versions: versions.iter().cloned().collect(),
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
                versions: HashMap::new(),
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

            let version: Version = if path.is_file() && filename.to_lowercase().ends_with(".zip")
                || version.is_some()
            {
                match version {
                    Some(version) => match version.try_into() {
                        Ok(version) => version,
                        Err(e) => {
                            println!(
                                "Failed to parse version for mod {name} at {}: {e}",
                                path.to_string_lossy()
                            );
                            continue;
                        }
                    },
                    None => continue, // should not happen
                }
            } else if path.is_dir() {
                let Ok(info_file) = fs::read_to_string(path.join("info.json")) else {
                    continue;
                };

                match serde_json::from_str::<crate::mod_info::ModInfo>(&info_file) {
                    Ok(info) => info.version,
                    Err(e) => {
                        println!(
                            "Failed to parse info.json for mod {name} at {}: {e}",
                            path.to_string_lossy()
                        );

                        // make sure the mod is disabled if its the only version
                        if !list.contains_key(&name) {
                            list.insert(
                                name.clone(),
                                Entry {
                                    enabled: false,
                                    versions: HashMap::new(),
                                    active_version: None,
                                },
                            );
                        }

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
                entry.versions.insert(version, filename.to_owned());
            } else {
                list.insert(
                    name.clone(),
                    Entry {
                        enabled: false,
                        versions: std::iter::once(&(version, filename.to_owned()))
                            .cloned()
                            .collect(),
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

    #[must_use]
    pub fn enable_used_mods(&mut self, used_mods: &crate::UsedMods) -> Vec<(String, Version)> {
        let mut missing = Vec::new();

        for (name, version) in used_mods {
            // skip wube mods
            if name == "base" || name == "core" {
                continue;
            }

            if let Some(entry) = self.list.get_mut(name) {
                entry.enabled = true;
                entry.active_version = Some(*version);

                if !entry.versions.contains_key(version) {
                    missing.push((name.clone(), *version));
                }
            } else {
                self.list.insert(
                    name.clone(),
                    Entry {
                        enabled: true,
                        versions: HashMap::new(),
                        active_version: None,
                    },
                );

                missing.push((name.clone(), *version));
            }
        }

        missing
    }
}

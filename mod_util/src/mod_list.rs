use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use anyhow::Result;

use petgraph::prelude::DiGraph;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    mod_info::{Dependency, DependencyExt, DependencyUtil, Version},
    mod_loader::Mod,
    UsedMods, UsedVersions,
};

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

#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub enabled: bool,
    pub active_version: Option<Version>,
    pub versions: HashMap<Version, String>,
    pub known_dependencies: HashMap<Version, Vec<Dependency>>,
}

#[derive(Debug, Clone)]
pub struct ModList<'a> {
    factorio_dir: &'a Path,
    list: HashMap<String, Entry>,
}

impl<'a> From<&ModList<'a>> for ModListFormat {
    fn from(list: &ModList) -> Self {
        let mut mods = Vec::new();

        for (name, entry) in &list.list {
            // core is very special and should not be in the mod list since its always enabled
            if name == "core" {
                continue;
            }

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
    pub fn load(factorio_dir: &'a Path) -> Option<Self> {
        let mut res = Self::generate(factorio_dir).ok()?;

        if let Some(tmp) = ModListFormat::load(&factorio_dir.join("mods/mod-list.json")) {
            // enable mods (and set active version) if they were found in the folder
            for entry in tmp.mods {
                res.list.entry(entry.name).and_modify(|e| {
                    if let Some(entry_v) = entry.version {
                        if !e.versions.contains_key(&entry_v) {
                            return;
                        }

                        e.active_version = Some(entry_v);
                    }

                    e.enabled = entry.enabled;
                });
            }
        };

        Some(res)
    }

    pub fn generate(factorio_dir: &'a Path) -> Result<Self> {
        let filename_extractor = regex::Regex::new(r"^(.+?)(?:_(\d+\.\d+\.\d+)(?:\.zip)?)?$")?;

        let mut list = HashMap::new();

        // add wube mods
        for w_mod in Mod::wube_mods() {
            match Mod::load(factorio_dir, w_mod) {
                Ok(m) => {
                    list.insert(
                        w_mod.to_string(),
                        Entry {
                            enabled: w_mod == "core",
                            active_version: None,
                            versions: std::iter::once((m.info.version, w_mod.into())).collect(),
                            known_dependencies: std::iter::once((
                                m.info.version,
                                m.info.dependencies,
                            ))
                            .collect(),
                        },
                    );
                }
                Err(e) => {
                    println!("Failed to load wube mod {w_mod}: {e}");
                }
            }
        }

        // add mods from mods folder
        let paths = fs::read_dir(factorio_dir.join("mods"))?;
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
                println!("skipping invalid match: {path:?}");
                continue;
            };
            let Some(name) = extracted.get(1).map(|n| n.as_str().to_owned()) else {
                println!("skipping invalid name: {path:?}");
                continue;
            };
            let path_version = extracted
                .get(2)
                .map_or_else(String::new, |v| v.as_str().to_owned());

            let m = match Mod::load(factorio_dir, filename) {
                Ok(m) => m,
                Err(e) => {
                    println!("Failed to load mod {name} at {path:?}: {e}");
                    // make sure the mod is disabled if its the only version
                    list.entry(name).or_default();
                    continue;
                }
            };

            if let Ok(path_version) = path_version.try_into() {
                if m.info.version != path_version {
                    println!(
                        "Version mismatch for mod {name} at {path:?}: {path_version} != {}",
                        m.info.version
                    );
                    continue;
                }
            }

            let entry = list.entry(name).or_default();
            entry.versions.insert(m.info.version, filename.into());
            entry
                .known_dependencies
                .insert(m.info.version, m.info.dependencies);
        }

        Ok(Self { factorio_dir, list })
    }

    pub fn save(&self) -> Result<()> {
        let format: ModListFormat = self.into();
        let bytes = serde_json::to_vec_pretty(&format)?;
        std::fs::write(self.factorio_dir.join("mods/mod-list.json"), bytes)?;

        Ok(())
    }

    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.list.contains_key(name)
    }

    #[must_use]
    pub const fn as_list(&self) -> &HashMap<String, Entry> {
        &self.list
    }

    #[must_use]
    pub fn as_list_mut(&mut self) -> &mut HashMap<String, Entry> {
        &mut self.list
    }

    /// Marks the given mods as enabled and sets the active version to the given one.
    ///
    /// Returns a list of mods that were not found in the mod list but got added.
    pub fn enable_mods(&mut self, mods: &UsedVersions) -> UsedVersions {
        let mut missing = HashMap::new();

        for (name, version) in mods {
            self.list
                .entry(name.clone())
                .and_modify(|e| {
                    e.enabled = true;
                    e.active_version = Some(*version);

                    if !e.versions.contains_key(version) {
                        missing.insert(name.clone(), *version);
                    }
                })
                .or_insert_with(|| {
                    missing.insert(name.clone(), *version);

                    Entry {
                        enabled: true,
                        ..Entry::default()
                    }
                });
        }

        missing
    }

    #[must_use]
    pub fn active_mods(&self) -> UsedMods {
        self.list
            .iter()
            .filter_map(|(name, entry)| {
                if entry.enabled {
                    let file = if let Some(version) = entry.active_version {
                        entry.versions.get(&version).cloned()?
                    } else {
                        entry.versions.values().next().cloned()?
                    };

                    let Ok(m) = Mod::load(self.factorio_dir, &file) else {
                        //println!("Failed to load mod {name} at {file}");

                        return None;
                    };
                    Some((name.clone(), m))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn set_dependency_info(
        &mut self,
        name: &String,
        known_dependencies: HashMap<Version, Vec<Dependency>>,
    ) {
        match self.list.get_mut(name) {
            Some(e) => e.known_dependencies = known_dependencies,
            None => {
                self.list.insert(
                    name.clone(),
                    Entry {
                        known_dependencies,
                        ..Entry::default()
                    },
                );
            }
        }
    }

    pub fn solve_dependencies(&self, required: &UsedVersions) -> Result<UsedVersions> {
        if required.is_empty() {
            return Ok(UsedVersions::default());
        }

        // TODO: actually solve the constraints and not just check if the latest versions work
        let mut dep_graph = DiGraph::<(&str, Version), &Dependency>::new();

        let all_deps = self
            .list
            .iter()
            .map(|(name, entry)| (name.as_str(), &entry.known_dependencies))
            .collect::<HashMap<_, _>>();
        let mut reqs = required
            .iter()
            .map(|(name, version)| (name.as_str(), *version))
            .collect::<Vec<_>>();

        // build all required mod nodes
        let mut node_map = HashMap::new();
        while let Some((name, version)) = reqs.pop() {
            if node_map.contains_key(&name) {
                continue;
            }

            node_map.insert(name, dep_graph.add_node((name, version)));

            let Some(info) = all_deps.get(name) else {
                anyhow::bail!("Dependency solver could not find info about {name}");
            };

            let Some(node_reqs) = info.get(&version) else {
                anyhow::bail!(
                    "Dependency solver could not find dependencies for {name} v{version}"
                );
            };

            // add required dependencies to the reqs queue
            for dep in node_reqs {
                if !dep.is_required() {
                    continue;
                }

                let dep_name = dep.name().as_str();
                let dep_version = dep.version();

                let Some(info) = all_deps.get(dep_name) else {
                    anyhow::bail!("Dependency solver could not find info about dep {dep_name}");
                };

                let mut dep_versions = info
                    .keys()
                    .filter(|v| dep.allows(dep_name, **v))
                    .collect::<Vec<_>>();
                dep_versions.sort();

                let Some(dep_version) = dep_versions.last() else {
                    // no more versions to try, fail?
                    anyhow::bail!(
                            "Dependency solver could not find a version for {dep_name} that satisfies {dep_version}"
                        );
                };

                reqs.push((dep_name, **dep_version));
            }
        }

        // add all dependency edges
        for node in dep_graph.node_indices() {
            let (name, version) = &dep_graph[node];
            let Some(info) = all_deps.get(name) else {
                anyhow::bail!(
                    "Dependency solver could not find info about {name} while building edges"
                );
            };

            let Some(deps) = info.get(version) else {
                anyhow::bail!("Dependency solver could not find dependencies for {name} v{version} while building edges");
            };

            for dep in deps {
                let dep_name = dep.name().as_str();

                match node_map.get(dep_name) {
                    Some(dep_node) => {
                        dep_graph.add_edge(node, *dep_node, dep);
                    }
                    None => {
                        // optional / incompatible dependencies are allowed to be missing
                        if dep.is_required() {
                            anyhow::bail!(
                                    "Dependency solver could not find required node for {dep_name} while building edges"
                                );
                        }
                    }
                }
            }
        }

        // check if all requirements are satisfied
        for node in dep_graph.node_indices() {
            let (name, version) = &dep_graph[node];

            let conflicts = dep_graph
                .edges_directed(node, petgraph::Direction::Incoming)
                .map(|e| *e.weight())
                .collect_conflicts::<Vec<_>>(name, *version);

            if !conflicts.is_empty() {
                anyhow::bail!(
                    "Dependency solver found conflicts for {name} v{version}: {conflicts:?}"
                );
            }
        }

        // dependencies are satisfied, hurray!
        // collect all used mods + versions
        Ok(dep_graph
            .raw_nodes()
            .iter()
            .map(|n| (n.weight.0.to_string(), n.weight.1))
            .collect())
    }
}

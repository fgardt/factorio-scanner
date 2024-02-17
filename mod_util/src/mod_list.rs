use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Read,
    path::Path,
};

use petgraph::prelude::DiGraph;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    mod_info::{Dependency, DependencyExt, DependencyUtil, DependencyVersion, Version},
    mod_loader::Mod,
    DependencyList, UsedMods, UsedVersions,
};

#[derive(Debug, thiserror::Error)]
pub enum ModListError {
    #[error("failed to read file: {0}")]
    FileError(#[from] std::io::Error),

    #[error("failed to load mod list: {0}")]
    ModListDeserializationError(#[from] serde_json::Error),

    #[error("failed to load wube mod {0}: {1}")]
    WubeModLoadError(String, #[source] crate::mod_loader::ModError),

    #[error("dependency solver could not find info about {0}")]
    SolverMissingInfo(String),

    #[error("dependency solver could not find dependencies for {0} v{1}")]
    SolverNoDependencies(String, Version),

    #[error("dependency solver could not find info about dep {0}")]
    SolverNoInfoOnDependency(String),

    #[error("dependency solver could not find a version for {0} that satisfies {1}")]
    SolverUnsatisfiable(String, DependencyVersion),

    #[error("dependency solver could not find info about {0} while building edges")]
    SolverEdgesMissingInfo(String),

    #[error("dependency solver could not find dependencies for {0} v{1} while building edges")]
    SolverEdgesNoInfoOnDependency(String, Version),

    #[error("dependency solver could not find required node for {0} while building edges")]
    SolverEdgesNodeNotFound(String),

    #[error("dependency solver found conflicts for {0} v{1}: {2:?}")]
    SolverFoundConflicts(String, Version, Vec<Dependency>),
}

type Result<T> = std::result::Result<T, ModListError>;

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
    fn load(list_path: &Path) -> Result<Self> {
        if !list_path.is_file() {
            return Ok(Self { mods: Vec::new() });
        }

        let mut bytes = Vec::new();
        File::open(list_path)?.read_to_end(&mut bytes)?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub enabled: bool,
    pub active_version: Option<Version>,
    pub versions: HashMap<Version, Option<String>>,
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
    pub fn load(factorio_dir: &'a Path) -> Result<Self> {
        let mut res = Self::generate(factorio_dir)?;
        let list = ModListFormat::load(&factorio_dir.join("mods/mod-list.json"))?;

        // enable mods (and set active version) if they were found in the folder
        for entry in list.mods {
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

        Ok(res)
    }

    pub fn generate(factorio_dir: &'a Path) -> Result<Self> {
        #[allow(clippy::unwrap_used)]
        let filename_extractor = Regex::new(r"^(.+?)(?:_(\d+\.\d+\.\d+)(?:\.zip)?)?$").unwrap();

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
                            versions: std::iter::once((m.info.version, Some(w_mod.into())))
                                .collect(),
                            known_dependencies: std::iter::once((
                                m.info.version,
                                m.info.dependencies,
                            ))
                            .collect(),
                        },
                    );
                }
                Err(e) => {
                    // core and base should always be available
                    if w_mod == "core" || w_mod == "base" {
                        return Err(ModListError::WubeModLoadError(w_mod.to_string(), e));
                    }
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
            if path.is_file() && path.extension() != Some("zip".as_ref()) {
                //println!("skipping {path:?}");
                continue;
            }

            let Some(extracted) = filename_extractor.captures(filename) else {
                //println!("skipping invalid match: {path:?}");
                continue;
            };
            let Some(name) = extracted.get(1).map(|n| n.as_str().to_owned()) else {
                //println!("skipping invalid name: {path:?}");
                continue;
            };

            let version: Version = if let Some(v) = extracted.get(2) {
                let Ok(version) = v.as_str().try_into() else {
                    continue;
                };
                version
            } else {
                let Ok(version) = Mod::load(factorio_dir, filename).map(|m| m.info.version) else {
                    continue;
                };
                version
            };

            let entry = list.entry(name).or_default();
            entry.versions.insert(version, Some(filename.into()));
            // entry
            //     .known_dependencies
            //     .insert(m.info.version, m.info.dependencies);
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

                    match e.versions.get(version) {
                        Some(Some(_)) => {}
                        _ => {
                            missing.insert(name.clone(), *version);
                        }
                    }
                })
                .or_insert_with(|| {
                    missing.insert(name.clone(), *version);

                    Entry {
                        enabled: true,
                        active_version: Some(*version),
                        ..Entry::default()
                    }
                });
        }

        missing
    }

    #[must_use]
    pub fn active_mods(&self) -> UsedMods {
        let mods = self.factorio_dir.join("mods");
        self.list
            .iter()
            .filter_map(|(name, entry)| {
                if !entry.enabled {
                    return None;
                }

                let file = {
                    let mut versions = entry.versions.keys().copied().collect::<Vec<_>>();
                    versions.sort_unstable();

                    let version = entry.active_version.unwrap_or(versions.last().copied()?);

                    let versioned = format!("{name}_{version}");
                    if mods.join(versioned.clone() + ".zip").exists() {
                        versioned + ".zip"
                    } else if mods.join(versioned.clone()).exists() {
                        versioned
                    } else {
                        name.clone()
                    }
                };

                match Mod::load(self.factorio_dir, &file) {
                    Ok(m) => Some((name.clone(), m)),
                    Err(e) => {
                        println!("Failed to load mod {name} at {file}: {e}");
                        None
                    }
                }
            })
            .collect()
    }

    pub fn load_local_dependency_info(&mut self, mods: &DependencyList) {
        let mut queue = mods
            .iter()
            .map(|(n, v)| (n.clone(), *v))
            .collect::<Vec<_>>();
        let mut loaded = HashSet::<(String, DependencyVersion)>::new();

        while let Some((name, version)) = queue.pop() {
            let Some(entry) = self.list.get_mut(&name) else {
                continue;
            };

            if loaded.contains(&(name.clone(), version)) {
                continue;
            }

            loaded.insert((name.clone(), version));

            let Some(version) = version
                .get_allowed_version(
                    entry
                        .versions
                        .keys()
                        .copied()
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .copied()
            else {
                continue;
            };

            let Some(Some(filename)) = entry.versions.get(&version) else {
                continue;
            };

            let Ok(m) = Mod::load(self.factorio_dir, filename) else {
                continue;
            };

            if version != m.info.version {
                println!(
                    "Version mismatch for {name}: {version} != {}",
                    m.info.version
                );
                continue;
            }

            for dep in &m.info.dependencies {
                let dep = (dep.name().clone(), *dep.version());
                if !loaded.contains(&dep) {
                    queue.push(dep);
                }
            }

            entry
                .known_dependencies
                .insert(m.info.version, m.info.dependencies);
        }
    }

    pub fn set_dependency_info(
        &mut self,
        name: &str,
        known_dependencies: HashMap<Version, Vec<Dependency>>,
    ) {
        let e = self.list.entry(name.to_owned()).or_default();

        for version in known_dependencies.keys() {
            e.versions.entry(*version).or_insert(None);
        }

        e.known_dependencies = known_dependencies;
    }

    #[allow(clippy::too_many_lines)]
    pub fn solve_dependencies(&self, required: &DependencyList) -> Result<UsedVersions> {
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

        let mut reqs = Vec::new();
        for (name, version) in required {
            let Some(info) = self.list.get(name) else {
                return Err(ModListError::SolverMissingInfo(name.to_string()));
            };

            let info_versions = info.versions.keys().copied().collect::<Vec<_>>();
            let Some(version) = version.get_allowed_version(&info_versions) else {
                return Err(ModListError::SolverMissingInfo(name.to_string()));
            };

            reqs.push((name.as_str(), *version));
        }

        // build all required mod nodes
        let mut node_map = HashMap::new();
        while let Some((name, version)) = reqs.pop() {
            if node_map.contains_key(&name) {
                continue;
            }

            node_map.insert(name, dep_graph.add_node((name, version)));

            let Some(info) = all_deps.get(name) else {
                return Err(ModListError::SolverMissingInfo(name.to_string()));
            };

            let Some(node_reqs) = info.get(&version) else {
                return Err(ModListError::SolverNoDependencies(
                    name.to_string(),
                    version,
                ));
            };

            // add required dependencies to the reqs queue
            for dep in node_reqs {
                if !dep.is_required() {
                    continue;
                }

                let dep_name = dep.name().as_str();
                let dep_version = dep.version();

                let Some(info) = all_deps.get(dep_name) else {
                    return Err(ModListError::SolverNoInfoOnDependency(dep_name.to_string()));
                };

                let mut dep_versions = info
                    .keys()
                    .filter(|v| dep.allows(dep_name, **v))
                    .collect::<Vec<_>>();
                dep_versions.sort();

                let Some(dep_version) = dep_versions.last() else {
                    // no more versions to try, fail?
                    return Err(ModListError::SolverUnsatisfiable(
                        dep_name.to_string(),
                        *dep_version,
                    ));
                };

                reqs.push((dep_name, **dep_version));
            }
        }

        // add all dependency edges
        for node in dep_graph.node_indices() {
            let (name, version) = &dep_graph[node];
            let Some(info) = all_deps.get(name) else {
                return Err(ModListError::SolverEdgesMissingInfo((*name).to_string()));
            };

            let Some(deps) = info.get(version) else {
                return Err(ModListError::SolverEdgesNoInfoOnDependency(
                    (*name).to_string(),
                    *version,
                ));
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
                            return Err(ModListError::SolverEdgesNodeNotFound(
                                dep_name.to_string(),
                            ));
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
                .cloned()
                .collect_conflicts::<Vec<_>>(name, *version);

            if !conflicts.is_empty() {
                return Err(ModListError::SolverFoundConflicts(
                    (*name).to_string(),
                    *version,
                    conflicts,
                ));
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

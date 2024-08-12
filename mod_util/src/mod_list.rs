use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use petgraph::prelude::DiGraph;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tracing::{debug, instrument, warn};

use crate::{
    mod_info::{Dependency, DependencyExt, DependencyUtil, DependencyVersion, Version},
    mod_loader::{self, Mod},
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

    #[error("dependency solver found circular dependencies")]
    SolverCircularDependencies,
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
    #[instrument(skip_all)]
    fn load<P: AsRef<Path>>(list_path: P) -> Result<Self> {
        if !list_path.as_ref().is_file() {
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

impl Entry {
    pub fn selected_version(&self) -> Option<Version> {
        let mut versions = self.versions.keys().copied().collect::<Vec<_>>();
        versions.sort_unstable();

        Some(self.active_version.unwrap_or(versions.last().copied()?))
    }
}

#[derive(Debug, Clone)]
pub struct ModList {
    pub factorio_dir: PathBuf,
    pub mod_dir: PathBuf,
    pub list: HashMap<String, Entry>,
}

impl From<&ModList> for ModListFormat {
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

impl From<ModList> for ModListFormat {
    fn from(list: ModList) -> Self {
        (&list).into()
    }
}

impl ModList {
    pub fn load(&mut self) -> Result<&mut Self> {
        let list = ModListFormat::load(self.mod_dir.join("mod-list.json").canonicalize()?)?;

        // enable mods (and set active version) if they were found in the folder
        for entry in list.mods {
            self.list.entry(entry.name).and_modify(|e| {
                if let Some(entry_v) = entry.version {
                    if !e.versions.contains_key(&entry_v) {
                        return;
                    }

                    e.active_version = Some(entry_v);
                }

                e.enabled = entry.enabled;
            });
        }

        Ok(self)
    }

    pub fn generate(factorio_dir: impl AsRef<Path>) -> Result<Self> {
        Self::generate_custom(
            factorio_dir.as_ref().join("data"),
            factorio_dir.as_ref().join("mods"),
        )
    }

    #[instrument(name = "generate", skip_all)]
    pub fn generate_custom(data_dir: impl AsRef<Path>, mod_dir: impl AsRef<Path>) -> Result<Self> {
        #[allow(clippy::unwrap_used)]
        let filename_extractor = Regex::new(r"^(.+?)(?:_(\d+\.\d+\.\d+)(?:\.zip)?)?$").unwrap();

        let mut list = HashMap::new();

        // add wube mods
        for w_mod in Mod::wube_mods() {
            match Mod::load_wube(&data_dir, w_mod) {
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
        let paths = fs::read_dir(&mod_dir)?;
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
                debug!("skipping {path:?}");
                continue;
            }

            let Some(extracted) = filename_extractor.captures(filename) else {
                debug!("skipping invalid match: {path:?}");
                continue;
            };
            let Some(name) = extracted.get(1).map(|n| n.as_str().to_owned()) else {
                debug!("skipping invalid name: {path:?}");
                continue;
            };

            let version: Version = if let Some(v) = extracted.get(2) {
                let Ok(version) = v.as_str().try_into() else {
                    continue;
                };
                version
            } else {
                let Ok(version) = Mod::load_from_path(path).map(|m| m.info.version) else {
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

        Ok(Self {
            factorio_dir: data_dir.as_ref().to_owned(),
            mod_dir: mod_dir.as_ref().to_owned(),
            list,
        })
    }

    pub fn save(&self) -> Result<()> {
        let format: ModListFormat = self.into();
        let bytes = serde_json::to_vec_pretty(&format)?;
        std::fs::write(self.mod_dir.join("mod-list.json"), bytes)?;

        Ok(())
    }

    /// Marks the given mods as enabled and sets the active version to the given one.
    ///
    /// Returns a list of mods that were not found in the mod list but got added.
    #[instrument(skip_all, fields(mod_count = mods.len()))]
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
    pub fn load_mod(&self, name: &str) -> std::result::Result<Option<Mod>, mod_loader::ModError> {
        let Some(entry) = self.list.get(name) else {
            return Ok(None);
        };

        if !entry.enabled {
            return Ok(None);
        }

        let Some(version) = entry.selected_version() else {
            return Ok(None);
        };

        Ok(Some(Mod::load_custom(
            &self.factorio_dir,
            &self.mod_dir,
            name,
            version,
        )?))
    }

    #[must_use]
    pub fn is_enabled(&self, name: &str) -> bool {
        self.list.get(name).map_or(false, |e| e.enabled)
    }

    #[must_use]
    #[instrument(skip_all)]
    pub fn active_mods(&self) -> UsedMods {
        self.list
            .iter()
            .filter_map(|(name, entry)| {
                if !entry.enabled {
                    return None;
                }

                let version = entry.selected_version()?;
                match Mod::load_custom(&self.factorio_dir, &self.mod_dir, &name, version) {
                    Ok(m) => Some((name.clone(), m)),
                    Err(e) => {
                        warn!("Failed to load mod {name}@{version}: {e}");
                        None
                    }
                }
            })
            .collect()
    }

    #[must_use]
    #[instrument(skip_all)]
    pub fn active_with_order(&self) -> (UsedMods, Vec<String>) {
        let active = self.active_mods();
        let mut tmp = dep_chain_len(&active);

        tmp.sort_unstable_by(|(a, _), (b, _)| natord::compare_ignore_case(a, b));
        tmp.sort_by_key(|(_, chain)| *chain);

        (active, tmp.iter().map(|(name, _)| name.clone()).collect())
    }

    #[instrument(name = "load_all_local_deps", skip_all)]
    pub fn load_all_local_deps(&mut self, mods: &DependencyList) {
        let mut queue = mods
            .iter()
            .map(|(n, v)| (n.clone(), *v))
            .collect::<Vec<_>>();
        let mut loaded = HashSet::<(String, DependencyVersion)>::new();

        while let Some((name, version)) = queue.pop() {
            if loaded.contains(&(name.clone(), version)) {
                continue;
            }

            loaded.insert((name.clone(), version));
            self.load_local_dependency_info(name.as_str(), &version);
        }
    }

    pub fn load_local_dependency_info(
        &mut self,
        name: &str,
        version: &DependencyVersion,
    ) -> Option<HashMap<String, Dependency>> {
        let entry = self.list.get_mut(name)?;
        let version = version
            .get_allowed_version(
                entry
                    .versions
                    .keys()
                    .copied()
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .copied()?;

        let Ok(m) = Mod::load_custom(&self.factorio_dir, &self.mod_dir, name, version) else {
            return None;
        };

        let res = m
            .info
            .dependencies
            .iter()
            .map(|d| (d.name().clone(), d.clone()))
            .collect();

        entry
            .known_dependencies
            .insert(m.info.version, m.info.dependencies);

        Some(res)
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

    #[instrument(name = "solve_deps", skip_all)]
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
            if Mod::wube_mods().contains(&name.as_str()) {
                let Some(version) = info_versions.first() else {
                    return Err(ModListError::SolverMissingInfo(format!(
                        "{name}, a static wube mod!"
                    )));
                };

                reqs.push((name.as_str(), *version));
                continue;
            }

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

        // check for circular dependencies
        {
            let mut dep_graph = dep_graph.clone();

            // remove all edges that do not affect the load order
            let mut to_remove = Vec::new();
            for e_idx in dep_graph.edge_indices() {
                if !dep_graph[e_idx].affects_load_order() {
                    to_remove.push(e_idx);
                }
            }

            // sort and reverse since graph.remove_edge() invalidates
            // the last index and moves it to the removed index
            to_remove.sort_unstable();
            to_remove.reverse();

            for e_idx in to_remove {
                dep_graph.remove_edge(e_idx);
            }

            if petgraph::algo::is_cyclic_directed(&dep_graph) {
                return Err(ModListError::SolverCircularDependencies);
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

fn dep_chain_len(active: &UsedMods) -> Vec<(String, usize)> {
    let mut cache = HashMap::new();
    let mut visit_list = HashSet::new();
    active
        .iter()
        .map(|(name, _)| {
            (
                name.clone(),
                dep_chain_recur(name, active, &mut cache, &mut visit_list),
            )
        })
        .collect()
}

fn dep_chain_recur<'a>(
    target: &'a str,
    active: &'a UsedMods,
    cache: &mut HashMap<&'a str, usize>,
    visit_list: &mut HashSet<&'a str>,
) -> usize {
    // core is always first
    if target == "core" {
        return 0;
    }

    if let Some(len) = cache.get(target) {
        return *len;
    }

    // circular dependency?
    if visit_list.contains(target) {
        return usize::MAX >> 1;
    }

    let Some(m) = active.get(target) else {
        return 0;
    };

    visit_list.insert(target);

    let mut max = 0;
    for dep in &m.info.dependencies {
        if !dep.affects_load_order() {
            continue;
        }

        let len = dep_chain_recur(dep.name(), active, cache, visit_list);
        if len > max {
            max = len;
        }
    }

    max += 1;
    cache.insert(target, max);
    max
}

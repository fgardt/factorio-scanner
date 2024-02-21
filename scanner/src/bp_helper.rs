use std::collections::{BTreeMap, HashMap};

use mod_util::{mod_info::DependencyVersion, AnyBasic, DependencyList};
use strum::IntoEnumIterator;

use crate::preset::Preset;

#[must_use]
pub fn get_used_versions(bp: &blueprint::Blueprint) -> DependencyList {
    let mut auto_detected = DependencyList::new();

    for entity in &bp.entities {
        if entity.tags.contains_key("bp_meta_info") {
            let Some(info) = entity.tags.get("bp_meta_info") else {
                continue;
            };

            let AnyBasic::Table(data) = info else {
                continue;
            };

            let Some(mods) = data.get("mods") else {
                continue;
            };

            let AnyBasic::Table(mods) = mods else {
                continue;
            };

            let mut result = HashMap::with_capacity(mods.len());

            for (mod_name, mod_version) in mods {
                let AnyBasic::String(mod_version) = mod_version else {
                    continue;
                };

                let Ok(version) = mod_version.try_into() else {
                    continue;
                };

                result.insert(mod_name.clone(), DependencyVersion::Exact(version));
            }

            return result;
        }

        // trying to auto detect mods
        check_prefix(&entity.name, &mut auto_detected);

        if !entity.recipe.is_empty() {
            check_prefix(&entity.recipe, &mut auto_detected);
        }

        if !entity.filter.is_empty() {
            check_prefix(&entity.filter, &mut auto_detected);
        }

        for filter in &entity.filters {
            check_prefix(filter, &mut auto_detected);
        }

        for item in entity.items.keys() {
            check_prefix(item, &mut auto_detected);
        }
    }

    auto_detected
}

fn check_prefix(id: &str, dep_list: &mut DependencyList) {
    for preset in Preset::iter() {
        let Some(prefix) = preset.known_prefix() else {
            continue;
        };

        if !id.starts_with(prefix) {
            continue;
        }

        dep_list.extend(preset.used_mods());
    }
}

#[must_use]
pub fn get_used_startup_settings(bp: &blueprint::Blueprint) -> Option<&BTreeMap<String, AnyBasic>> {
    for entity in &bp.entities {
        if entity.tags.contains_key("bp_meta_info") {
            let info = entity.tags.get("bp_meta_info")?;

            let AnyBasic::Table(data) = info else {
                continue;
            };

            let settings = data.get("startup")?;

            let AnyBasic::Table(settings) = settings else {
                continue;
            };

            return Some(settings);
        }
    }

    None
}

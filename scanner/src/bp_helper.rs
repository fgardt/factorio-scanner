use std::collections::{BTreeMap, HashSet};

use blueprint::GetIDs;
use mod_util::{AnyBasic, DependencyList};
use strum::IntoEnumIterator;

use crate::preset::Preset;

#[must_use]
pub fn get_used_versions(bp: &blueprint::Blueprint) -> DependencyList {
    if let Some(meta_info) = bp.get_meta_info_mods() {
        return meta_info;
    }

    let mut auto_detected = DependencyList::new();

    let ids = bp.get_ids();
    let mut all_ids = HashSet::new();
    all_ids.extend(ids.recipe.iter().map(|i| i.as_str()));
    all_ids.extend(ids.entity.iter().map(|i| i.as_str()));
    all_ids.extend(ids.tile.iter().map(|i| i.as_str()));
    all_ids.extend(ids.fluid.iter().map(|i| i.as_str()));
    all_ids.extend(ids.item.iter().map(|i| i.as_str()));
    all_ids.extend(ids.equipment.iter().map(|i| i.as_str()));
    all_ids.extend(ids.virtual_signal.iter().map(|i| i.as_str()));
    all_ids.extend(ids.quality.iter().map(|i| i.as_str()));
    all_ids.extend(ids.space_location.iter().map(|i| i.as_str()));
    all_ids.extend(ids.asteroid_chunk.iter().map(|i| i.as_str()));
    all_ids.extend(ids.other.iter().map(std::string::String::as_str));

    for id in all_ids {
        check_prefix(id, &mut auto_detected);
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

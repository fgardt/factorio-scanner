use std::collections::HashMap;

use mod_util::{AnyBasic, UsedVersions};

#[must_use]
pub fn get_used_versions(bp: &blueprint::Blueprint) -> Option<UsedVersions> {
    for entity in &bp.entities {
        if entity.tags.contains_key("bp_meta_info") {
            let info = entity.tags.get("bp_meta_info")?;

            let AnyBasic::Table(data) = info else {
                continue;
            };

            let mods = data.get("mods")?;

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

                result.insert(mod_name.clone(), version);
            }

            return Some(result);
        }
    }

    None
}

#[must_use]
pub fn get_used_startup_settings(bp: &blueprint::Blueprint) -> Option<&HashMap<String, AnyBasic>> {
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

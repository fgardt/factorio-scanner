use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/LandMinePrototype`](https://lua-api.factorio.com/latest/prototypes/LandMinePrototype.html)
pub type LandMinePrototype = EntityWithOwnerPrototype<LandMineData>;

/// [`Prototypes/LandMinePrototype`](https://lua-api.factorio.com/latest/prototypes/LandMinePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct LandMineData {
    pub picture_safe: Option<Sprite>,
    pub picture_set: Option<Sprite>,
    pub trigger_radius: f64,

    pub picture_set_enemy: Option<Sprite>,

    #[serde(
        default = "helper::u32_120",
        skip_serializing_if = "helper::is_120_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub timeout: u32,

    #[serde(
        default = "helper::u32_10",
        skip_serializing_if = "helper::is_10_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub trigger_interval: u32,

    pub ammo_category: Option<AmmoCategoryID>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub force_die_on_attack: bool,

    #[serde(
        default = "ForceCondition::enemy",
        skip_serializing_if = "ForceCondition::is_enemy"
    )]
    pub trigger_force: ForceCondition,

    pub trigger_collision_mask: Option<CollisionMaskConnector>,
    // not implemented
    // pub action: Option<Trigger>,
}

impl super::Renderable for LandMineData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture_set.as_ref().and_then(|ps| {
            ps.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

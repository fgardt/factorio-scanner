use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
pub type RadarPrototype = EntityWithOwnerPrototype<WireEntityData<EnergyEntityData<RadarData>>>;

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarData {
    pub energy_usage: Energy,
    pub energy_per_sector: Energy,
    pub energy_per_nearby_scan: Energy,
    pub pictures: Option<RotatedSprite>,
    pub frozen_patch: Option<Sprite>,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance_of_sector_revealed: u32,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub max_distance_of_nearby_sector_revealed: u32,

    pub radius_minimap_visualisation_color: Option<Color>,

    #[serde(
        default = "helper::f64_001",
        skip_serializing_if = "helper::is_001_f64"
    )]
    pub rotation_speed: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub connects_to_other_radars: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub reset_orientation_when_frozen: bool,

    #[serde(default = "helper::f32_09", skip_serializing_if = "helper::is_09_f32")]
    pub energy_fraction_to_connect: f32,

    #[serde(default = "helper::f32_01", skip_serializing_if = "helper::is_01_f32")]
    pub energy_fraction_to_disconnect: f32,
}

impl super::Renderable for RadarData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.pictures.as_ref().and_then(|p| {
            p.render(
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

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct RadarPrototype(EntityWithOwnerPrototype<RadarData>);

impl super::Renderable for RadarPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/RadarPrototype`](https://lua-api.factorio.com/latest/prototypes/RadarPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RadarData {
    pub energy_usage: Energy,
    pub energy_per_sector: Energy,
    pub energy_per_nearby_scan: Energy,
    pub energy_source: AnyEnergySource,
    pub pictures: RotatedSprite,

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
}

impl super::Renderable for RadarData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.pictures
            .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}

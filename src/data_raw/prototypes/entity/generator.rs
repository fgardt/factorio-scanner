use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, EntityWithOwnerPrototype};
use crate::data_raw::types::*;

/// [`Prototypes/GeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/GeneratorPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorPrototype(EntityWithOwnerPrototype<GeneratorData>);

impl super::Renderable for GeneratorPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/GeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/GeneratorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorData {
    pub energy_source: ElectricEnergySource,
    pub fluid_box: FluidBox,
    pub horizontal_animation: Animation,
    pub vertical_animation: Animation,
    pub effectivity: f64,
    pub fluid_usage_per_tick: f64,
    pub maximum_temperature: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub burns_fluid: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub scale_fluid_usage: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub destroy_non_fuel_fluid: bool,

    #[serde(
        default = "helper::f64_quarter",
        skip_serializing_if = "helper::is_quarter_f64"
    )]
    pub min_perceived_performance: f64,

    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub performance_to_sound_speedup: f64,

    pub max_power_output: Option<Energy>,
    // not implemented
    // pub smoke: Vec<SmokeSource>,
}

impl super::Renderable for GeneratorData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        match options.direction.unwrap_or(Direction::North) {
            Direction::North | Direction::South => &self.vertical_animation,
            Direction::East | Direction::West => &self.horizontal_animation,
            _ => panic!("Invalid direction, generators only support cardinal directions"),
        }
        .render(options.factorio_dir, &options.used_mods, &options.into())
    }
}

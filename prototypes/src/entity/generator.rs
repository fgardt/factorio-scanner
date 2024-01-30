use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, FluidBoxEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/GeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/GeneratorPrototype.html)
pub type GeneratorPrototype = EntityWithOwnerPrototype<FluidBoxEntityData<GeneratorData>>;

/// [`Prototypes/GeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/GeneratorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratorData {
    pub energy_source: ElectricEnergySource,
    pub horizontal_animation: Option<Animation>,
    pub vertical_animation: Option<Animation>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
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
    // pub smoke: FactorioArray<SmokeSource>,
}

impl super::Renderable for GeneratorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = match options.direction {
            Direction::North | Direction::South => &self.vertical_animation,
            Direction::East | Direction::West => &self.horizontal_animation,
            _ => panic!("Invalid direction, generators only support cardinal directions"),
        }
        .as_ref()
        .and_then(|a| {
            a.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }
}

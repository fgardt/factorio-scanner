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

    pub output_fluid_box: Option<FluidBox>,

    pub pictures: Option<GeneratorPictureSet>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,

    pub fluid_usage_per_tick: FluidAmount,
    pub maximum_temperature: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub burns_fluid: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub scale_fluid_usage: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub destroy_non_fuel_fluid: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub two_direction_only: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,

    pub max_power_output: Option<Energy>,

    pub spent_fluid: Option<SpentFluidSpecification>,
    // not implemented
    // pub smoke: FactorioArray<SmokeSource>,
}

impl super::Entity for GeneratorData {}

impl super::Renderable for GeneratorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self
            .pictures
            .as_ref()
            .and_then(|p| p.get(options.direction))
            .and_then(|g| g.animation.as_ref())
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

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        if let Some(out_fb) = self.output_fluid_box.as_ref() {
            return out_fb.fluid_box_connections(options);
        }

        Vec::with_capacity(0)
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        if let Some(out_fb) = self.output_fluid_box.as_ref() {
            out_fb.render_debug(options, used_mods, render_layers);
        }
    }
}

// TODO: abstract out into generic 4WaySet<T>
/// [`Prototypes/GeneratorPrototype/GeneratorPictureSet`](https://lua-api.factorio.com/latest/prototypes/GeneratorPrototype.html#pictures)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorPictureSet {
    pub north: Option<GeneratorPictures>,
    pub east: Option<GeneratorPictures>,
    pub south: Option<GeneratorPictures>,
    pub west: Option<GeneratorPictures>,
}

impl GeneratorPictureSet {
    #[must_use]
    pub fn get(&self, direction: Direction) -> Option<&GeneratorPictures> {
        match direction {
            Direction::North => self.north.as_ref(),
            Direction::East => self.east.as_ref(),
            Direction::South => self.south.as_ref().or(self.north.as_ref()),
            Direction::West => self.west.as_ref().or(self.east.as_ref()),
            _ => None,
        }
    }
}

/// [`Types/GeneratorPictures`](https://lua-api.factorio.com/latest/types/GeneratorPictures.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorPictures {
    pub animation: Option<Animation>,
    pub frozen_patch: Option<Sprite>,
}

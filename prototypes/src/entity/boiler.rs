use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, FluidBoxEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/BoilerPrototype`](https://lua-api.factorio.com/latest/prototypes/BoilerPrototype.html)
pub type BoilerPrototype = EntityWithOwnerPrototype<FluidBoxEntityData<BoilerData>>;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BoilerData {
    pub energy_source: AnyEnergySource,
    pub output_fluid_box: FluidBox,
    pub energy_consumption: Energy,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub burning_cooldown: u16,
    pub target_temperature: f64,
    pub structure: BoilerStructure,
    pub fire: BoilerFire,
    pub fire_glow: BoilerFireGlow,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fire_glow_flicker_enabled: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fire_flicker_enabled: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mode: BoilerMode,

    pub patch: Option<BoilerPatch>,
}

impl super::Renderable for BoilerData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let structure: Animation4Way = self.structure.clone().into();
        let res = structure.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        self.output_fluid_box.connection_points(options.direction)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        Vec::with_capacity(0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerStructure {
    pub north: Animation,
    pub east: Animation,
    pub south: Animation,
    pub west: Animation,
}

impl From<BoilerStructure> for Animation4Way {
    fn from(value: BoilerStructure) -> Self {
        Self::Struct {
            north: value.north,
            east: Some(value.east),
            south: Some(value.south),
            west: Some(value.west),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerFire {
    pub north: Option<Animation>,
    pub east: Option<Animation>,
    pub south: Option<Animation>,
    pub west: Option<Animation>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerFireGlow {
    pub north: Option<Animation>,
    pub east: Option<Animation>,
    pub south: Option<Animation>,
    pub west: Option<Animation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BoilerMode {
    #[default]
    HeatWaterInside,
    OutputToSeparatePipe,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoilerPatch {
    pub north: Option<Sprite>,
    pub east: Option<Sprite>,
    pub south: Option<Sprite>,
    pub west: Option<Sprite>,
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/BoilerPrototype`](https://lua-api.factorio.com/latest/prototypes/BoilerPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct BoilerPrototype(EntityWithOwnerPrototype<BoilerData>);

impl super::Renderable for BoilerPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BoilerData {
    pub energy_source: AnyEnergySource,
    pub fluid_box: FluidBox,
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

    // TODO: skip serializing if default
    #[serde(default)]
    pub mode: BoilerMode,

    pub patch: Option<BoilerPatch>,
}

impl super::Renderable for BoilerData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        let structure: Animation4Way = self.structure.clone().into();
        structure.render(options.factorio_dir, &options.used_mods, &options.into())
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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

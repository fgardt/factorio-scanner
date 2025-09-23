use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/ThrusterPrototype`](https://lua-api.factorio.com/latest/prototypes/ThrusterPrototype.html)
pub type ThrusterPrototype = EntityWithOwnerPrototype<ThrusterData>;

/// [`Prototypes/ThrusterPrototype`](https://lua-api.factorio.com/latest/prototypes/ThrusterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ThrusterData {
    pub min_performance: ThrusterPerformancePoint,
    pub max_performance: ThrusterPerformancePoint,

    pub fuel_fluid_box: FluidBox,
    pub oxidizer_fluid_box: FluidBox,

    pub graphics_set: Option<WorkingVisualisations<ThrusterGraphicsSetData>>,
    // pub plumes: Option<PlumesSpecification>,
}

impl super::Renderable for ThrusterData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.graphics_set.as_ref().and_then(|gs| {
            gs.render(
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
        let mut res = self
            .fuel_fluid_box
            .connection_points(options.direction, options.mirrored);
        res.extend(
            self.oxidizer_fluid_box
                .connection_points(options.direction, options.mirrored),
        );

        res
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        self.fuel_fluid_box
            .render_debug(options, used_mods, render_layers);
        self.oxidizer_fluid_box
            .render_debug(options, used_mods, render_layers);
    }
}

/// [`Types/ThrusterPerformancePoint`](https://lua-api.factorio.com/latest/types/ThrusterPerformancePoint.html)
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ThrusterPerformancePoint {
    Struct {
        fluid_volume: f64,
        fluid_usage: FluidAmount,
        effectivity: f64,
    },
    Array(f64, FluidAmount, f64),
}

impl ThrusterPerformancePoint {
    #[must_use]
    pub const fn fluid_volume(&self) -> f64 {
        match self {
            Self::Array(fluid_volume, _, _) | Self::Struct { fluid_volume, .. } => *fluid_volume,
        }
    }

    #[must_use]
    pub const fn fluid_volume_mut(&mut self) -> &mut f64 {
        match self {
            Self::Struct { fluid_volume, .. } | Self::Array(fluid_volume, _, _) => fluid_volume,
        }
    }

    #[must_use]
    pub const fn fluid_usage(&self) -> FluidAmount {
        match self {
            Self::Struct { fluid_usage, .. } | Self::Array(_, fluid_usage, _) => *fluid_usage,
        }
    }

    #[must_use]
    pub const fn fluid_usage_mut(&mut self) -> &mut FluidAmount {
        match self {
            Self::Struct { fluid_usage, .. } | Self::Array(_, fluid_usage, _) => fluid_usage,
        }
    }

    #[must_use]
    pub const fn effectivity(&self) -> f64 {
        match self {
            Self::Struct { effectivity, .. } | Self::Array(_, _, effectivity) => *effectivity,
        }
    }

    #[must_use]
    pub const fn effectivity_mut(&mut self) -> &mut f64 {
        match self {
            Self::Struct { effectivity, .. } | Self::Array(_, _, effectivity) => effectivity,
        }
    }
}

/// [`Types/ThrusterGraphicsSet`](https://lua-api.factorio.com/latest/types/ThrusterGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ThrusterGraphicsSetData {
    pub flame: Option<Sprite>,
    pub flame_effect: Option<EffectTexture>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub flame_position: Vector,

    #[serde(
        default = "helper::f32_31_25",
        skip_serializing_if = "helper::is_31_25_f32"
    )]
    pub flame_effect_height: f32,
    #[serde(default = "helper::f32_6", skip_serializing_if = "helper::is_6_f32")]
    pub flame_effect_width: f32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub flame_half_height: f32,
    #[serde(
        default = "helper::f32_1_5625",
        skip_serializing_if = "helper::is_1_5625_f32"
    )]
    pub flame_effect_offset: f32,

    pub water_reflection: Option<WaterReflectionDefinition>,
}

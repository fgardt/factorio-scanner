use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/FusionGeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/FusionGeneratorPrototype.html)
pub type FusionGeneratorPrototype = EntityWithOwnerPrototype<FusionGeneratorData>;

/// [`Prototypes/FusionGeneratorPrototype`](https://lua-api.factorio.com/latest/prototypes/FusionGeneratorPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionGeneratorData {
    pub energy_source: ElectricEnergySource,

    pub graphics_set: Option<FusionGeneratorGraphicsSet>,

    pub input_fluid_box: FluidBox,
    pub output_fluid_box: FluidBox,
    pub max_fluid_usage: FluidAmount,

    pub perceived_performance: Option<PerceivedPerformance>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub burns_fluid: bool,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub effectivity: f64,
}

impl super::Renderable for FusionGeneratorData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self
            .graphics_set
            .as_ref()
            .and_then(|gs| match options.direction {
                Direction::North => Some(&gs.north_graphics_set),
                Direction::East => Some(&gs.east_graphics_set),
                Direction::South => Some(&gs.south_graphics_set),
                Direction::West => Some(&gs.west_graphics_set),
                _ => None,
            })?
            .animation
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

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<(MapPosition, Direction)> {
        let mut res = self
            .input_fluid_box
            .connection_points(options.direction, options.mirrored);

        res.extend(
            self.output_fluid_box
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
        self.input_fluid_box
            .render_debug(options, used_mods, render_layers);
        self.output_fluid_box
            .render_debug(options, used_mods, render_layers);
    }
}

/// [`Types/FusionGeneratorGraphicsSet`](https://lua-api.factorio.com/latest/types/FusionGeneratorGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionGeneratorGraphicsSet {
    pub north_graphics_set: FusionGeneratorDirectionGraphicsSet,
    pub east_graphics_set: FusionGeneratorDirectionGraphicsSet,
    pub south_graphics_set: FusionGeneratorDirectionGraphicsSet,
    pub west_graphics_set: FusionGeneratorDirectionGraphicsSet,

    #[serde(
        default = "RenderLayer::object",
        skip_serializing_if = "RenderLayer::is_object"
    )]
    pub render_layer: RenderLayer,

    pub light: Option<LightDefinition>,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub glow_color: Color,

    pub water_reflection: Option<WaterReflectionDefinition>,
}

/// [`Types/FusionGeneratorDirectionGraphicsSet`](https://lua-api.factorio.com/latest/types/FusionGeneratorDirectionGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionGeneratorDirectionGraphicsSet {
    pub animation: Option<Animation>,
    pub working_light: Option<Animation>,
    pub fusion_effect_uv_map: Option<Animation>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fluid_input_graphics: FactorioArray<FusionGeneratorFluidInputGraphics>,
}

/// [`Types/FusionGeneratorFluidInputGraphics`](https://lua-api.factorio.com/latest/types/FusionGeneratorFluidInputGraphics.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct FusionGeneratorFluidInputGraphics {
    pub sprite: Option<Sprite>,
    pub working_light: Option<Sprite>,
    pub fusion_effect_uv_map: Option<Sprite>,
}

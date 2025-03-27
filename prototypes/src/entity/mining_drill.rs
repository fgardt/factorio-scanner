use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/MiningDrillPrototype`](https://lua-api.factorio.com/latest/prototypes/MiningDrillPrototype.html)
pub type MiningDrillPrototype =
    EntityWithOwnerPrototype<WireEntityData<EnergyEntityData<MiningDrillData>>>;

/// [`Prototypes/MiningDrillPrototype`](https://lua-api.factorio.com/latest/prototypes/MiningDrillPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct MiningDrillData {
    pub vector_to_place_result: Vector,
    pub resource_searching_radius: f64,
    pub energy_usage: Energy,
    pub mining_speed: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_categories: FactorioArray<ResourceCategoryID>,

    pub output_fluid_box: Option<FluidBox>,
    pub input_fluid_box: Option<FluidBox>,

    pub graphics_set: Option<MiningDrillGraphicsSet>,
    pub wet_mining_graphics_set: Option<MiningDrillGraphicsSet>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,

    pub base_picture: Option<Sprite4Way>,

    pub effect_receiver: Option<EffectReceiver>,
    pub module_slots: Option<ItemStackIndex>,
    pub allowed_effects: Option<EffectTypeLimitation>,
    pub allowed_module_categories: Option<FactorioArray<ModuleCategoryID>>,

    pub radius_visualisation_picture: Option<Sprite>,
    pub base_render_layer: Option<RenderLayer>,

    #[serde(
        default = "helper::u8_100",
        skip_serializing_if = "helper::is_100_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub resource_drain_rate_percent: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub shuffle_resources_to_mine: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub drops_full_belt_stacks: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub base_productivity: f64,

    pub monitor_visualization_tint: Option<Color>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub filter_count: u8,
    // not implemented
    // pub moving_sound: Option<InterruptibleSound>,
    // pub drilling_sound: Option<InterruptibleSound>,
    // pub drilling_sound_animation_start_frame: Option<u16>,
    // pub drilling_sound_animation_end_frame: Option<u16>,
}

impl super::Renderable for MiningDrillData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = merge_renders(
            &[
                self.base_picture.as_ref().and_then(|s| {
                    s.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
                self.graphics_set.as_ref().and_then(|s| {
                    s.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        let mut input_cons = self.input_fluid_box.as_ref().map_or_else(
            || Vec::with_capacity(0),
            |b| b.connection_points(options.direction, options.mirrored),
        );

        let mut output_cons = self.output_fluid_box.as_ref().map_or_else(
            || Vec::with_capacity(0),
            |b| b.connection_points(options.direction, options.mirrored),
        );

        input_cons.append(&mut output_cons);
        input_cons
    }

    fn render_debug(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        if let Some(fb_out) = self.output_fluid_box.as_ref() {
            fb_out.render_debug(options, used_mods, render_layers);
        }

        if let Some(fb_in) = self.input_fluid_box.as_ref() {
            fb_in.render_debug(options, used_mods, render_layers);
        }
    }
}

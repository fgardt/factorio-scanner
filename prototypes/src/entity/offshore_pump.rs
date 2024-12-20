use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, FluidBoxEntityData, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
pub type OffshorePumpPrototype = EntityWithOwnerPrototype<
    WireEntityData<EnergyEntityData<FluidBoxEntityData<OffshorePumpData>>>,
>;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpData {
    pub pumping_speed: FluidAmount,
    pub fluid_source_offset: Vector,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub perceived_performance: PerceivedPerformance,

    pub graphics_set: Option<OffshorePumpGraphicsSet>,

    pub energy_usage: Option<Energy>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub remove_on_tile_collision: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw_fluid: bool,
}

impl super::Renderable for OffshorePumpData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.graphics_set
            .as_ref()
            .and_then(|gs| gs.render(options, used_mods, render_layers, image_cache))
    }
}

/// [`Types/OffshorePumpGraphicsSet`](https://lua-api.factorio.com/latest/types/OffshorePumpGraphicsSet.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpGraphicsSet {
    pub animation: Option<Animation4Way>,

    // TODO: default value
    pub base_render_layer: Option<RenderLayer>,

    #[serde(
        default = "helper::i8_1",
        skip_serializing_if = "helper::is_1_i8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub underwater_layer_offset: i8,

    pub fluid_animation: Option<Animation4Way>,
    pub glass_pictures: Option<Sprite4Way>,
    pub base_pictures: Option<Sprite4Way>,
    pub underwater_pictures: Option<Sprite4Way>,
}

impl super::Renderable for OffshorePumpGraphicsSet {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = merge_renders(
            &[
                self.base_pictures.as_ref().and_then(|b| {
                    b.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
                self.animation.as_ref().and_then(|a| {
                    a.render(
                        render_layers.scale(),
                        used_mods,
                        image_cache,
                        &options.into(),
                    )
                }),
                self.glass_pictures.as_ref().and_then(|g| {
                    g.render(
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
}

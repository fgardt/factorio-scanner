use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
pub type OffshorePumpPrototype = EntityWithOwnerPrototype<OffshorePumpData>;

/// [`Prototypes/OffshorePumpPrototype`](https://lua-api.factorio.com/latest/prototypes/OffshorePumpPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpData {
    pub fluid_box: FluidBox,
    pub pumping_speed: f64,
    pub fluid: FluidID,

    #[serde(flatten)]
    pub graphics: OffshorePumpGraphicsVariant,

    #[serde(
        default = "helper::f64_quarter",
        skip_serializing_if = "helper::is_quarter_f64"
    )]
    pub min_perceived_performance: f64,

    pub fluid_box_tile_collision_test: Option<CollisionMask>,
    pub adjacent_tile_collision_test: Option<CollisionMask>,
    pub center_collision_mask: Option<CollisionMask>,
    pub adjacent_tile_collision_box: Option<BoundingBox>,
    pub placeable_position_visualization: Option<Sprite>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub remove_on_tile_collision: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw_fluid: bool,

    pub check_bounding_box_collides_with_tiles: Option<bool>,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_wire_connection_points: Option<(
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    )>,
    pub circuit_connector_sprites: Option<(
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
    )>,
}

impl super::Renderable for OffshorePumpData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.graphics
            .render(options, used_mods, render_layers, image_cache)
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OffshorePumpGraphicsVariant {
    GraphicsSet {
        graphics_set: OffshorePumpGraphicsSet,
    },
    Deprecated {
        picture: Animation4Way,
    },
}

impl super::Renderable for OffshorePumpGraphicsVariant {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        match self {
            Self::GraphicsSet { graphics_set } => {
                graphics_set.render(options, used_mods, render_layers, image_cache)
            }
            Self::Deprecated { picture } => {
                let res = picture.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )?;

                render_layers.add_entity(res, &options.position);

                Some(())
            }
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct OffshorePumpGraphicsSet {
    pub animation: Animation4Way,

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
                self.animation.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
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

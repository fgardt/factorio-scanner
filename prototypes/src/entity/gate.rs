use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/GatePrototype`](https://lua-api.factorio.com/latest/prototypes/GatePrototype.html)
pub type GatePrototype = EntityWithOwnerPrototype<GateData>;

/// [`Prototypes/GatePrototype`](https://lua-api.factorio.com/latest/prototypes/GatePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct GateData {
    pub vertical_animation: Option<Animation>,
    pub horizontal_animation: Option<Animation>,

    pub vertical_rail_base: Option<Animation>,
    pub vertical_rail_animation_left: Option<Animation>,
    pub vertical_rail_animation_right: Option<Animation>,

    pub horizontal_rail_base: Option<Animation>,
    pub horizontal_rail_animation_left: Option<Animation>,
    pub horizontal_rail_animation_right: Option<Animation>,

    pub wall_patch: Option<Animation>,

    pub opening_speed: f32,
    pub activation_distance: f64,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub timeout_to_close: u32,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub fadeout_interval: u32,

    pub opened_collision_mask: Option<CollisionMaskConnector>,
    // not implemented
    // pub opening_sound: Option<Sound>,
    // pub closing_sound: Option<Sound>,
}

impl super::Renderable for GateData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = match options.direction {
            Direction::North | Direction::South => {
                let renders = if options.draw_gate_patch {
                    [
                        self.vertical_animation.as_ref().and_then(|a| {
                            a.render(
                                render_layers.scale(),
                                used_mods,
                                image_cache,
                                &options.into(),
                            )
                        }),
                        self.wall_patch.as_ref().and_then(|a| {
                            a.render(
                                render_layers.scale(),
                                used_mods,
                                image_cache,
                                &options.into(),
                            )
                        }),
                    ]
                } else {
                    [
                        self.vertical_animation.as_ref().and_then(|a| {
                            a.render(
                                render_layers.scale(),
                                used_mods,
                                image_cache,
                                &options.into(),
                            )
                        }),
                        None,
                    ]
                };
                merge_renders(&renders, render_layers.scale())
            }
            Direction::West | Direction::East => self.horizontal_animation.as_ref().and_then(|a| {
                a.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            }),
            _ => None,
        }?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

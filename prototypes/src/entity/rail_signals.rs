use std::ops::{Deref, Rem};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RailSignalBasePrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalBasePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalBaseData {
    pub animation: RotatedAnimation,
    pub rail_piece: Option<Animation>,
    pub red_light: Option<LightDefinition>,
    pub green_light: Option<LightDefinition>,
    pub orange_light: Option<LightDefinition>,
    pub default_red_output_signal: Option<SignalIDConnector>,
    pub default_green_output_signal: Option<SignalIDConnector>,
    pub default_orange_output_signal: Option<SignalIDConnector>,
}

impl super::Renderable for RailSignalBaseData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let rail_piece = self.rail_piece.as_ref()?;
        let frame_count = rail_piece.frame_count();

        // <https://forums.factorio.com/109688>
        let prog = if frame_count == 10 {
            let r = options.position.y().rem(2.0).ceil().abs();

            match options.direction {
                Direction::North => {
                    if r >= 0.9 {
                        0.0
                    } else {
                        0.8
                    }
                }
                Direction::South => {
                    if r >= 0.9 {
                        0.35
                    } else {
                        0.9
                    }
                }
                _ => ((options.direction.to_orientation() * 8.0) / 10.0).into(),
            }
        } else {
            options.direction.to_orientation().into()
        };

        let rail_piece_opts = AnimationRenderOpts {
            progress: prog,
            runtime_tint: options.runtime_tint,
        };

        if let Some(res) = self.rail_piece.as_ref().and_then(|r| {
            r.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &rail_piece_opts,
            )
        }) {
            render_layers.add(
                res,
                &options.position,
                crate::InternalRenderLayer::RailBackplate,
            );
        }

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
pub type RailChainSignalPrototype = EntityWithOwnerPrototype<WireEntityData<RailChainSignalData>>;

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailChainSignalData {
    pub selection_box_offsets: FactorioArray<Vector>,
    pub blue_light: Option<LightDefinition>,
    pub default_blue_output_signal: Option<SignalIDConnector>,

    #[serde(flatten)]
    parent: RailSignalBaseData,
}

impl Deref for RailChainSignalData {
    type Target = RailSignalBaseData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for RailChainSignalData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        // thanks to bilka: <https://discord.com/channels/139677590393716737/306402592265732098/1173539478669897768>
        let offset = {
            let (mut offset_x, mut offset_y) = options.direction.right90().get_offset().as_tuple();

            match options.direction {
                Direction::South => offset_x -= 1.0,
                Direction::West => offset_y -= 1.0,
                _ => (),
            }

            Vector::new(offset_x, offset_y)
        };

        let animation_opts = RotatedAnimationRenderOpts {
            progress: (1.0 / 5.0) * 2.5, // green light
            orientation: options.direction.to_orientation(),
            runtime_tint: options.runtime_tint,
            override_index: None,
        };

        if let Some((img, shift)) = self.animation.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &animation_opts,
        ) {
            render_layers.add_entity((img, shift + offset), &options.position);
        }

        self.parent.render(
            &super::RenderOpts {
                position: options.position.clone() + MapPosition::from(offset),
                ..options.clone()
            },
            used_mods,
            render_layers,
            image_cache,
        )
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
pub type RailSignalPrototype = EntityWithOwnerPrototype<WireEntityData<RailSignalData>>;

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalData {
    #[serde(flatten)]
    parent: RailSignalBaseData,
}

impl Deref for RailSignalData {
    type Target = RailSignalBaseData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for RailSignalData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        if let Some(animation) = self.animation.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            render_layers.add_entity(animation, &options.position);
        }

        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}

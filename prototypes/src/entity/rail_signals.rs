use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
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

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_wire_connection_points: FactorioArray<WireConnectionPoint>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_connector_sprites: FactorioArray<CircuitConnectorSprites>,
}

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
pub type RailChainSignalPrototype = EntityWithOwnerPrototype<RailChainSignalData>;

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
    ) -> crate::RenderOutput {
        let mut empty = true;

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

        let rail_piece_opts = AnimationRenderOpts {
            progress: (options.direction.to_orientation() * 8.0) / 10.0, // this is so weird, whyyyy <https://forums.factorio.com/109688>
            runtime_tint: options.runtime_tint,
        };

        if let Some((img, shift)) = self.rail_piece.as_ref().and_then(|r| {
            r.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &rail_piece_opts,
            )
        }) {
            empty = false;

            render_layers.add(
                (img, shift + offset),
                &options.position,
                InternalRenderLayer::RailTies,
            );
        }

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
            empty = false;

            render_layers.add_entity((img, shift + offset), &options.position);
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
pub type RailSignalPrototype = EntityWithOwnerPrototype<RailSignalData>;

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
    ) -> crate::RenderOutput {
        let mut empty = true;

        let rail_piece_opts = AnimationRenderOpts {
            progress: (options.direction.to_orientation() * 8.0) / 10.0, // this is so weird, whyyyy <https://forums.factorio.com/109688>
            runtime_tint: options.runtime_tint,
        };

        if let Some(rail_piece) = self.rail_piece.as_ref().and_then(|r| {
            r.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &rail_piece_opts,
            )
        }) {
            empty = false;

            render_layers.add(
                rail_piece,
                &options.position,
                InternalRenderLayer::RailMetal,
            );
        }

        if let Some(animation) = self.animation.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        ) {
            empty = false;

            render_layers.add_entity(animation, &options.position);
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

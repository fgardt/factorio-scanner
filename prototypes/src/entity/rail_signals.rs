use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Types/RailSignalPictureSet`](https://lua-api.factorio.com/latest/types/RailSignalPictureSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalPictureSet {
    pub structure: RotatedAnimation,
    #[serde(default = "rl_floor_mech", skip_serializing_if = "is_rl_floor_mech")]
    pub structure_render_layer: RenderLayer,
    pub signal_color_to_structure_frame_index: RailSignalColorToFrameIndex,
    pub rail_piece: Option<RailSignalStaticSpriteLayer>,

    pub upper_rail_piece: Option<RailSignalStaticSpriteLayer>,

    pub lights: RailSignalLights,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub circuit_connector: FactorioArray<CircuitConnectorDefinition>,
    #[serde(default = "rl_object", skip_serializing_if = "is_rl_object")]
    pub circuit_connector_render_layer: RenderLayer,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure_align_to_animation_index: FactorioArray<u8>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection_box_shift: FactorioArray<Vector>,
}

const fn rl_floor_mech() -> RenderLayer {
    RenderLayer::FloorMechanics
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_rl_floor_mech(layer: &RenderLayer) -> bool {
    *layer == rl_floor_mech()
}

const fn rl_object() -> RenderLayer {
    RenderLayer::Object
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_rl_object(layer: &RenderLayer) -> bool {
    *layer == rl_object()
}

/// [`Types/RailSignalColorToFrameIndex`](https://lua-api.factorio.com/latest/types/RailSignalColorToFrameIndex.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalColorToFrameIndex {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub none: u8,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub red: u8,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub green: u8,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub blue: u8,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub yellow: u8,
}

/// [`Types/RailSignalStaticSpriteLayer`](https://lua-api.factorio.com/latest/types/RailSignalStaticSpriteLayer.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalStaticSpriteLayer {
    pub sprites: Animation,
    #[serde(default = "rl_rcsm", skip_serializing_if = "is_rl_rcsm")]
    pub render_layer: RenderLayer,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub hide_if_simulation: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub hide_if_not_connected_to_rails: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shifts: FactorioArray<MapPosition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub align_to_frame_index: FactorioArray<u8>,
}

const fn get_frame_override_index(ati: &[u8], opts: &super::RenderOpts) -> Option<u8> {
    const EXPECTED_LEN: usize = Direction::COUNT * 4 * 3;
    if ati.is_empty() || ati.len() != EXPECTED_LEN {
        return None;
    }

    // TODO: odd / even X/Y pos, left / straight & mix / right
    Some(ati[opts.direction as usize * Direction::COUNT])

    // <https://forums.factorio.com/109688>
    // let prog = if frame_count == 10 {
    //     let r = opts.position.y().rem(2.0).ceil().abs();
    //     match opts.direction {
    //         Direction::North => {
    //             if r >= 0.9 {
    //                 0.0
    //             } else {
    //                 0.8
    //             }
    //         }
    //         Direction::South => {
    //             if r >= 0.9 {
    //                 0.35
    //             } else {
    //                 0.9
    //             }
    //         }
    //         _ => ((opts.direction.to_orientation() * 8.0) / 10.0).into(),
    //     }
    // } else {
    //     opts.direction.to_orientation().into()
    // };
}

const fn rl_rcsm() -> RenderLayer {
    RenderLayer::RailChainSignalMetal
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_rl_rcsm(layer: &RenderLayer) -> bool {
    *layer == rl_rcsm()
}

/// [`Types/RailSignalLights`](https://lua-api.factorio.com/latest/types/RailSignalLights.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalLights {
    pub red: Option<RailSignalLightDefinition>,
    pub green: Option<RailSignalLightDefinition>,
    pub blue: Option<RailSignalLightDefinition>,
    pub yellow: Option<RailSignalLightDefinition>,
}

/// [`Types/RailSignalLightDefinition`](https://lua-api.factorio.com/latest/types/RailSignalLightDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalLightDefinition {
    pub light: LightDefinition,
    pub shift: Option<Vector>,
}

/// [`Prototypes/RailSignalBasePrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalBasePrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalBaseData {
    pub ground_picture_set: RailSignalPictureSet,
    pub elevated_picture_set: RailSignalPictureSet,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub circuit_wire_max_distance: f64,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub default_red_output_signal: Option<SignalIDConnector>,
    pub default_orange_output_signal: Option<SignalIDConnector>,
    pub default_green_output_signal: Option<SignalIDConnector>,
    pub default_blue_output_signal: Option<SignalIDConnector>,

    pub elevated_collision_mask: Option<CollisionMaskConnector>,
    #[serde(default = "helper::u8_55", skip_serializing_if = "helper::is_55_u8")]
    pub elevated_selection_priority: u8,
}

impl RailSignalBaseData {
    const fn picture_set(&self, opts: &super::RenderOpts) -> &RailSignalPictureSet {
        if opts.elevated {
            &self.elevated_picture_set
        } else {
            &self.ground_picture_set
        }
    }

    fn render_structure(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &super::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let picture_set = self.picture_set(opts);
        let rot = get_frame_override_index(&picture_set.structure_align_to_animation_index, opts)?;
        let idx = picture_set.signal_color_to_structure_frame_index.green;

        picture_set.structure.render(
            scale,
            used_mods,
            image_cache,
            &RotatedRenderOpts::new_override(
                rot.into(),
                AnimationRenderOpts::new_override(idx.into(), opts.into()),
            ),
        )
    }
}

impl super::Renderable for RailSignalBaseData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let picture_set = self.picture_set(opts);
        let rail_piece = picture_set.rail_piece.as_ref()?;

        // TODO: opts for connected rail
        if rail_piece.hide_if_not_connected_to_rails {
            return None;
        }

        let idx = get_frame_override_index(&rail_piece.align_to_frame_index, opts)?;
        let rail_piece_opts = AnimationRenderOpts::new_override(idx.into(), opts.into());

        let res = picture_set.rail_piece.as_ref()?.sprites.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &rail_piece_opts,
        )?;
        render_layers.add(res, &opts.position, RenderLayer::RailScrew);

        Some(())
    }
}

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
pub type RailChainSignalPrototype = EntityWithOwnerPrototype<WireEntityData<RailChainSignalData>>;

/// [`Prototypes/RailChainSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailChainSignalPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailChainSignalData(RailSignalBaseData);

impl Deref for RailChainSignalData {
    type Target = RailSignalBaseData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl super::Renderable for RailChainSignalData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        // thanks to bilka: <https://discord.com/channels/139677590393716737/306402592265732098/1173539478669897768>
        let offset = {
            let (mut offset_x, mut offset_y) = opts.direction.right90().get_offset().as_tuple();

            match opts.direction {
                Direction::South => offset_x -= 1.0,
                Direction::West => offset_y -= 1.0,
                _ => (),
            }

            Vector::new(offset_x, offset_y)
        };

        if let Some((img, shift)) =
            self.render_structure(render_layers.scale(), used_mods, image_cache, opts)
        {
            render_layers.add_entity((img, shift + offset), &opts.position);
        }

        self.0.render(
            &super::RenderOpts {
                position: opts.position + MapPosition::from(offset),
                ..opts.clone()
            },
            used_mods,
            render_layers,
            image_cache,
        )
    }
}

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
pub type RailSignalPrototype = EntityWithOwnerPrototype<WireEntityData<RailSignalData>>;

/// [`Prototypes/RailSignalPrototype`](https://lua-api.factorio.com/latest/prototypes/RailSignalPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailSignalData(RailSignalBaseData);

impl Deref for RailSignalData {
    type Target = RailSignalBaseData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl super::Renderable for RailSignalData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let picture_set = self.picture_set(opts);
        let rot = get_frame_override_index(&picture_set.structure_align_to_animation_index, opts)?;
        let idx = picture_set.signal_color_to_structure_frame_index.green;

        if let Some(structure) =
            self.render_structure(render_layers.scale(), used_mods, image_cache, opts)
        {
            render_layers.add_entity(structure, &opts.position);
        }

        self.0.render(opts, used_mods, render_layers, image_cache)
    }
}

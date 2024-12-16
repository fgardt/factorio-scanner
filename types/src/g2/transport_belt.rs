use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;
use tracing::warn;

use super::{
    Animation, AnimationRenderOpts, AnimationVariations, DirectionalRenderOpts, GraphicsOutput,
    RenderableGraphics, RotatedAnimation, RotatedRenderOpts, Sprite4Way, SpriteVariations,
    TintableRenderOpts,
};
use crate::{ConnectedDirections, Direction, ImageCache, RealOrientation, Vector};

/// [`Types/TransportBeltAnimationSet`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSet.html)
#[derive(Debug, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct TransportBeltAnimationSet {
    pub animation_set: RotatedAnimation,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_index: u8,
    #[serde(
        default = "helper::u8_2",
        skip_serializing_if = "helper::is_2_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_index: u8,
    #[serde(
        default = "helper::u8_3",
        skip_serializing_if = "helper::is_3_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_index: u8,
    #[serde(
        default = "helper::u8_4",
        skip_serializing_if = "helper::is_4_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_index: u8,

    #[serde(
        default = "helper::u8_13",
        skip_serializing_if = "helper::is_13_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_south_index: u8,
    #[serde(
        default = "helper::u8_14",
        skip_serializing_if = "helper::is_14_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_south_index: u8,
    #[serde(
        default = "helper::u8_15",
        skip_serializing_if = "helper::is_15_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_west_index: u8,
    #[serde(
        default = "helper::u8_16",
        skip_serializing_if = "helper::is_16_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_west_index: u8,
    #[serde(
        default = "helper::u8_17",
        skip_serializing_if = "helper::is_17_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_north_index: u8,
    #[serde(
        default = "helper::u8_18",
        skip_serializing_if = "helper::is_18_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_north_index: u8,
    #[serde(
        default = "helper::u8_19",
        skip_serializing_if = "helper::is_19_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub starting_east_index: u8,
    #[serde(
        default = "helper::u8_20",
        skip_serializing_if = "helper::is_20_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub ending_east_index: u8,

    pub ending_patch: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ends_with_stopper: bool,
}

impl RenderableGraphics for TransportBeltAnimationSet {
    type RenderOpts = DirectionalRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // -1 because the index is 1-based. Lua stuff :)
        let index = match opts.direction {
            Direction::North => self.north_index,
            Direction::East => self.east_index,
            Direction::South => self.south_index,
            Direction::West => self.west_index,
            _ => {
                warn!("belts only support cardinal directions");
                return None;
            }
        } - 1;

        self.animation_set.render(
            scale,
            used_mods,
            image_cache,
            &RotatedRenderOpts::new_override(index, **opts),
        )
    }
}

/// [`Types/TransportBeltAnimationSetWithCorners`](https://lua-api.factorio.com/latest/types/TransportBeltAnimationSetWithCorners.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltAnimationSetWithCorners {
    #[serde(
        default = "helper::u8_5",
        skip_serializing_if = "helper::is_5_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_to_north_index: u8,
    #[serde(
        default = "helper::u8_6",
        skip_serializing_if = "helper::is_6_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_to_east_index: u8,
    #[serde(
        default = "helper::u8_7",
        skip_serializing_if = "helper::is_7_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_to_north_index: u8,
    #[serde(
        default = "helper::u8_8",
        skip_serializing_if = "helper::is_8_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub north_to_west_index: u8,
    #[serde(
        default = "helper::u8_9",
        skip_serializing_if = "helper::is_9_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_to_east_index: u8,
    #[serde(
        default = "helper::u8_10",
        skip_serializing_if = "helper::is_10_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub east_to_south_index: u8,
    #[serde(
        default = "helper::u8_11",
        skip_serializing_if = "helper::is_11_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub south_to_west_index: u8,
    #[serde(
        default = "helper::u8_12",
        skip_serializing_if = "helper::is_12_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub west_to_south_index: u8,

    #[serde(flatten)]
    pub animation_set: TransportBeltAnimationSet,
}

pub struct ConnectedRenderOpts<M = TintableRenderOpts> {
    pub connections: Option<ConnectedDirections>,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for ConnectedRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

impl<M> std::ops::DerefMut for ConnectedRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M> ConnectedRenderOpts<M> {
    pub const fn new(connections: Option<ConnectedDirections>, more: M) -> Self {
        Self { connections, more }
    }
}

impl RenderableGraphics for TransportBeltAnimationSetWithCorners {
    type RenderOpts = ConnectedRenderOpts<DirectionalRenderOpts<AnimationRenderOpts>>;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let connections = opts.connections.unwrap_or_default();
        let index = match opts.direction {
            Direction::North => match connections {
                ConnectedDirections::Left | ConnectedDirections::UpLeft => self.west_to_north_index,
                ConnectedDirections::Right | ConnectedDirections::UpRight => {
                    self.east_to_north_index
                }
                _ => self.animation_set.north_index,
            },
            Direction::South => match connections {
                ConnectedDirections::Left | ConnectedDirections::DownLeft => {
                    self.west_to_south_index
                }
                ConnectedDirections::Right | ConnectedDirections::DownRight => {
                    self.east_to_south_index
                }
                _ => self.animation_set.south_index,
            },
            Direction::East => match connections {
                ConnectedDirections::Up | ConnectedDirections::UpRight => self.north_to_east_index,
                ConnectedDirections::Down | ConnectedDirections::DownRight => {
                    self.south_to_east_index
                }
                _ => self.animation_set.east_index,
            },
            Direction::West => match connections {
                ConnectedDirections::Up | ConnectedDirections::UpLeft => self.north_to_west_index,
                ConnectedDirections::Down | ConnectedDirections::DownLeft => {
                    self.south_to_west_index
                }
                _ => self.animation_set.west_index,
            },
            _ => unreachable!("Belts only support cardinal directions"),
        } - 1;

        self.animation_set.animation_set.render(
            scale,
            used_mods,
            image_cache,
            &RotatedRenderOpts::new_override(index, ***opts),
        )
    }
}

/// [`Types/TransportBeltConnectorFrame`](https://lua-api.factorio.com/latest/types/TransportBeltConnectorFrame.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBeltConnectorFrame {
    pub frame_main: AnimationVariations,
    pub frame_shadow: AnimationVariations,
    pub frame_main_scanner: Animation,
    pub frame_main_scanner_movement_speed: f32,
    pub frame_main_scanner_horizontal_start_shift: Vector,
    pub frame_main_scanner_horizontal_end_shift: Vector,
    pub frame_main_scanner_horizontal_y_scale: f32,
    pub frame_main_scanner_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_vertical_start_shift: Vector,
    pub frame_main_scanner_vertical_end_shift: Vector,
    pub frame_main_scanner_vertical_y_scale: f32,
    pub frame_main_scanner_vertical_rotation: RealOrientation,
    pub frame_main_scanner_cross_horizontal_start_shift: Vector,
    pub frame_main_scanner_cross_horizontal_end_shift: Vector,
    pub frame_main_scanner_cross_horizontal_y_scale: f32,
    pub frame_main_scanner_cross_horizontal_rotation: RealOrientation,
    pub frame_main_scanner_cross_vertical_start_shift: Vector,
    pub frame_main_scanner_cross_vertical_end_shift: Vector,
    pub frame_main_scanner_cross_vertical_y_scale: f32,
    pub frame_main_scanner_cross_vertical_rotation: RealOrientation,
    pub frame_main_scanner_nw_ne: Animation,
    pub frame_main_scanner_sw_se: Animation,
    pub frame_back_patch: Option<SpriteVariations>,
    pub frame_front_patch: Option<SpriteVariations>,
}

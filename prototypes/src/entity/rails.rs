use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use tracing::warn;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
pub type RailPrototype = EntityWithOwnerPrototype<RailData>;

/// [`Prototypes/RailPrototype`](https://lua-api.factorio.com/latest/prototypes/RailPrototype.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailData {
    pub pictures: RailPictureSet,
    pub fence_pictures: Option<RailFenceGraphicsSet>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub extra_planner_penalty: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub extra_planner_goal_penalty: f64,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub forced_fence_segment_count: u8,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ending_shifts: FactorioArray<Vector>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deconstruction_marker_positions: FactorioArray<Vector>,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub remove_soft_decoratives: bool,
    // TODO: override `build_grid_size` and `selection_box`

    // not implemented
    // pub walking_sound: Option<Sound>,
}

impl super::Renderable for RailData {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.pictures
            .render(opts, used_mods, render_layers, image_cache)
    }
}

macro_rules! deref_newtype {
    ($inner:ident, $name:ident) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name($inner);

        impl Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl super::Renderable for $name {
            fn render(
                &self,
                opts: &super::RenderOpts,
                used_mods: &UsedMods,
                render_layers: &mut crate::RenderLayerBuffer,
                image_cache: &mut ImageCache,
            ) -> super::RenderOutput {
                self.0.render(opts, used_mods, render_layers, image_cache)
            }
        }
    };
    ($inner:ident, $($name:ident),+) => {
        $(deref_newtype!($inner, $name);)+
    }
}

deref_newtype! {
    RailPrototype,

    CurvedRailAPrototype,
    CurvedRailBPrototype,
    HalfDiagonalRailPrototype,
    StraightRailPrototype,

    ElevatedCurvedRailAPrototype,
    ElevatedCurvedRailBPrototype,
    ElevatedHalfDiagonalRailPrototype,
    ElevatedStraightRailPrototype,

    LegacyCurvedRailPrototype,
    LegacyStraightRailPrototype
}

/// [`Prototypes/RailRampPrototype`](https://lua-api.factorio.com/latest/prototypes/RailRampPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailRampPrototype {
    #[serde(default = "helper::f32_15", skip_serializing_if = "helper::is_15_f32")]
    pub support_range: f32,
    pub collision_mask_allow_on_deep_oil_ocean: Option<CollisionMaskConnector>,

    #[serde(flatten)]
    parent: RailPrototype,
}

impl Deref for RailRampPrototype {
    type Target = RailPrototype;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for RailRampPrototype {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        self.parent
            .render(opts, used_mods, render_layers, image_cache)
    }
}

/// [`Types/RailPictureSet`](https://lua-api.factorio.com/latest/types/RailPictureSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPictureSet {
    pub north: Box<RailPieceLayers>,
    pub northeast: Box<RailPieceLayers>,
    pub east: Box<RailPieceLayers>,
    pub southeast: Box<RailPieceLayers>,
    pub south: Box<RailPieceLayers>,
    pub southwest: Box<RailPieceLayers>,
    pub west: Box<RailPieceLayers>,
    pub northwest: Box<RailPieceLayers>,

    pub front_rail_endings: Option<Box<Sprite16Way>>,
    pub back_rail_endings: Option<Box<Sprite16Way>>,
    pub rail_endings: Option<Box<Sprite16Way>>,

    pub segment_visualisation_endings: Option<Box<RotatedAnimation>>, // 16 directions, 6 frames each

    pub render_layers: RailRenderLayers,
    pub secondary_render_layers: Option<RailRenderLayers>,

    pub slice_origin: Option<RailsSliceOffsets>,
    // not implemented
    // pub fog_mask: Option<RailsFogMaskDefinitions>,
}

impl super::Renderable for RailPictureSet {
    fn render(
        &self,
        opts: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let piece = match opts.direction {
            Direction::North => &self.north,
            Direction::NorthEast => &self.northeast,
            Direction::East => &self.east,
            Direction::SouthEast => &self.southeast,
            Direction::South => &self.south,
            Direction::SouthWest => &self.southwest,
            Direction::West => &self.west,
            Direction::NorthWest => &self.northwest,
            _ => {
                warn!("Invalid direction for rail");
                return None;
            }
        }
        .as_ref();

        let mut empty = true;
        let rl = &self.render_layers;
        let scale = render_layers.scale();
        let render_opts = opts.into();

        if let Some(metals) = piece
            .metals
            .as_ref()
            .and_then(|m| m.render(scale, used_mods, image_cache, &render_opts))
        {
            empty = false;
            render_layers.add(metals, &opts.position, rl.metal);
        }

        if let Some(backplates) = piece
            .backplates
            .as_ref()
            .and_then(|bp| bp.render(scale, used_mods, image_cache, &render_opts))
        {
            empty = false;
            render_layers.add(backplates, &opts.position, rl.screw);
        }

        if let Some(ties) = piece
            .ties
            .as_ref()
            .and_then(|t| t.render(scale, used_mods, image_cache, &render_opts))
        {
            empty = false;
            render_layers.add(ties, &opts.position, rl.tie);
        }

        if let Some(stone_path) = piece
            .stone_path
            .as_ref()
            .and_then(|sp| sp.render(scale, used_mods, image_cache, &render_opts))
        {
            empty = false;
            render_layers.add(stone_path, &opts.position, rl.stone_path);
        }

        if let Some(stone_path_lower) = piece
            .stone_path_background
            .as_ref()
            .and_then(|spb| spb.render(scale, used_mods, image_cache, &render_opts))
        {
            empty = false;
            render_layers.add(stone_path_lower, &opts.position, rl.stone_path_lower);
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

/// [`Types/RailRenderLayers`](https://lua-api.factorio.com/latest/types/RailRenderLayers.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailRenderLayers {
    #[serde(default = "rspl", skip_serializing_if = "is_rspl")]
    pub stone_path_lower: RenderLayer,
    #[serde(default = "rsp", skip_serializing_if = "is_rsp")]
    pub stone_path: RenderLayer,
    #[serde(default = "tie", skip_serializing_if = "is_tie")]
    pub tie: RenderLayer,
    #[serde(default = "screw", skip_serializing_if = "is_screw")]
    pub screw: RenderLayer,
    #[serde(default = "metal", skip_serializing_if = "is_metal")]
    pub metal: RenderLayer,
    pub front_end: Option<RenderLayer>,
    pub back_end: Option<RenderLayer>,

    #[serde(default = "helper::i8_1", skip_serializing_if = "helper::is_1_i8")]
    pub underwater_layer_offset: i8,
}

impl RailRenderLayers {
    #[must_use]
    pub const fn front_end(&self) -> RenderLayer {
        match self.front_end {
            Some(fe) => fe,
            None => self.screw,
        }
    }

    #[must_use]
    pub const fn back_end(&self) -> RenderLayer {
        match self.back_end {
            Some(be) => be,
            None => self.screw,
        }
    }
}

/// [`Types/RailPictureSet/RailsSliceOffsets`](https://lua-api.factorio.com/latest/types/RailPictureSet.html#slice_origin)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailsSliceOffsets {
    pub north: Option<Vector>,
    pub east: Option<Vector>,
    pub south: Option<Vector>,
    pub west: Option<Vector>,
}

/// [`Types/RailPieceLayers`](https://lua-api.factorio.com/latest/types/RailPieceLayers.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailPieceLayers {
    pub metals: Option<SpriteVariations>,
    pub backplates: Option<SpriteVariations>,
    pub ties: Option<SpriteVariations>,
    pub stone_path: Option<SpriteVariations>,
    pub stone_path_background: Option<SpriteVariations>,

    pub segment_visualisation_middle: Option<Sprite>,
    pub water_reflection: Option<Sprite>,
    pub underwater_structure: Option<Sprite>,
    pub shadow_subtract_mask: Option<Sprite>,
    pub shadow_mask: Option<Sprite>,
}

impl super::Renderable for RailPieceLayers {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let mut empty = true;

        if let Some(res) = self.stone_path_background.as_ref().and_then(|spb| {
            spb.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(res, &options.position, RenderLayer::RailStonePathLower);
        }

        if let Some(res) = self.stone_path.as_ref().and_then(|sp| {
            sp.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(res, &options.position, RenderLayer::RailStonePath);
        }

        if let Some(res) = self.ties.as_ref().and_then(|t| {
            t.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(res, &options.position, RenderLayer::RailTie);
        }

        if let Some(res) = self.backplates.as_ref().and_then(|b| {
            b.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(res, &options.position, RenderLayer::RailScrew);
        }

        if let Some(res) = self.metals.as_ref().and_then(|m| {
            m.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(res, &options.position, RenderLayer::RailMetal);
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

/// [`Types/RailFenceGraphicsSet`](https://lua-api.factorio.com/latest/types/RailFenceGraphicsSet.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailFenceGraphicsSet {
    pub segment_count: RailFenceGraphicsSetSegmentCount, // u8, must be 2 or 4

    #[serde(default = "elo", skip_serializing_if = "is_elo")]
    pub back_fence_render_layer: RenderLayer,
    #[serde(default = "eho", skip_serializing_if = "is_eho")]
    pub front_fence_render_layer: RenderLayer,
    #[serde(default = "elo", skip_serializing_if = "is_elo")]
    pub back_fence_render_layer_secondary: RenderLayer,
    #[serde(default = "eho", skip_serializing_if = "is_eho")]
    pub front_fence_render_layer_secondary: RenderLayer,

    #[serde(rename = "side_A")]
    pub side_a: RailFencePictureSet,
    #[serde(rename = "side_B")]
    pub side_b: RailFencePictureSet,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RailFenceGraphicsSetSegmentCount {
    Two = 2,
    Four = 4,
}

/// [`Types/RailFencePictureSet`](https://lua-api.factorio.com/latest/types/RailFencePictureSet.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RailFencePictureSet {
    pub ends: [Box<RailFenceDirectionSet>; 4],
    pub fence: Box<RailFenceDirectionSet>,
    pub ends_upper: Option<[Box<RailFenceDirectionSet>; 4]>,
    pub fence_upper: Option<Box<RailFenceDirectionSet>>,
}

/// [`Types/RailFenceDirectionSet`](https://lua-api.factorio.com/latest/types/RailFenceDirectionSet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RailFenceDirectionSet {
    pub north: Option<SpriteVariations>,
    pub northeast: Option<SpriteVariations>,
    pub east: Option<SpriteVariations>,
    pub southeast: Option<SpriteVariations>,
    pub south: Option<SpriteVariations>,
    pub southwest: Option<SpriteVariations>,
    pub west: Option<SpriteVariations>,
    pub northwest: Option<SpriteVariations>,
}

#[expect(clippy::trivially_copy_pass_by_ref)]
mod rl_help {
    use crate::RenderLayer;

    pub const fn rspl() -> RenderLayer {
        RenderLayer::RailStonePathLower
    }

    pub const fn rsp() -> RenderLayer {
        RenderLayer::RailStonePath
    }

    pub const fn tie() -> RenderLayer {
        RenderLayer::RailTie
    }

    pub const fn screw() -> RenderLayer {
        RenderLayer::RailScrew
    }

    pub const fn metal() -> RenderLayer {
        RenderLayer::RailMetal
    }

    pub const fn elo() -> RenderLayer {
        RenderLayer::ElevatedLowerObject
    }

    pub const fn eho() -> RenderLayer {
        RenderLayer::ElevatedHigherObject
    }

    pub fn is_rspl(rl: &RenderLayer) -> bool {
        *rl == rspl()
    }

    pub fn is_rsp(rl: &RenderLayer) -> bool {
        *rl == rsp()
    }

    pub fn is_tie(rl: &RenderLayer) -> bool {
        *rl == tie()
    }

    pub fn is_screw(rl: &RenderLayer) -> bool {
        *rl == screw()
    }

    pub fn is_metal(rl: &RenderLayer) -> bool {
        *rl == metal()
    }

    pub fn is_elo(rl: &RenderLayer) -> bool {
        *rl == elo()
    }

    pub fn is_eho(rl: &RenderLayer) -> bool {
        *rl == eho()
    }
}
use rl_help::*;

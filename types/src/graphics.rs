use std::num::NonZeroU32;

use image::{imageops, DynamicImage, GenericImageView, Rgba};
use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{FactorioArray, ImageCache, MapPosition, RealOrientation};

use super::{helper, Color, Direction, FileName, Vector};

/// [`Types/SpritePriority`](https://lua-api.factorio.com/latest/types/SpritePriority.html)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SpritePriority {
    ExtraHighNoScale,
    ExtraHigh,
    High,
    #[default]
    Medium,
    Low,
    VeryLow,
    NoAtlas,
}

/// Union used in [`Types/SpriteFlags`](https://lua-api.factorio.com/latest/types/SpriteFlags.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteFlag {
    NoCrop,
    NotCompressed,
    AlwaysCompressed,
    Mipmap,
    LinearMinification,
    LinearMagnification,
    LinearMipLevel,
    AlphaMask,
    NoScale,
    Mask,
    Icon,
    Gui,
    GuiIcon,
    Light,
    Terrain,
    TerrainEffectMap,
    Shadow,
    Smoke,
    Decal,
    LowObject,
    TrilinearFiltering,
    #[serde(rename = "group=none")]
    GroupNone,
    #[serde(rename = "group=terrain")]
    GroupTerrain,
    #[serde(rename = "group=shadow")]
    GroupShadow,
    #[serde(rename = "group=smoke")]
    GroupSmoke,
    #[serde(rename = "group=decal")]
    GroupDecal,
    #[serde(rename = "group=low-object")]
    GroupLowObject,
    #[serde(rename = "group=gui")]
    GroupGui,
    #[serde(rename = "group=icon")]
    GroupIcon,
    #[serde(rename = "group=icon-background")]
    GroupIconBackground,
    Compressed,
}

/// [`Types/SpriteFlags`](https://lua-api.factorio.com/latest/types/SpriteFlags.html)
pub type SpriteFlags = FactorioArray<SpriteFlag>;

/// [`Types/SpriteSizeType`](https://lua-api.factorio.com/latest/types/SpriteSizeType.html)
pub type SpriteSizeType = i16;

/// [`Types/BlendMode`](https://lua-api.factorio.com/latest/types/BlendMode.html)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BlendMode {
    #[default]
    Normal,
    Additive,
    AdditiveSoft,
    Multiplicative,
    MultiplicativeWithAlpha,
    Overwrite,
}

/// [`Types/RenderLayer`](https://lua-api.factorio.com/latest/types/RenderLayer.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RenderLayer {
    Zero,
    BackgroundTransitions,
    UnderTiles,
    Decals,
    AboveTiles,
    #[serde(rename = "ground-layer-1")]
    GroundLayer1,
    #[serde(rename = "ground-layer-2")]
    GroundLayer2,
    #[serde(rename = "ground-layer-3")]
    GroundLayer3,
    #[serde(rename = "ground-layer-4")]
    GroundLayer4,
    #[serde(rename = "ground-layer-5")]
    GroundLayer5,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    RailStonePathLower,
    RailStonePath,
    RailTie,
    Decorative,
    GroundPatch,
    GroundPatchHigher,
    GroundPatchHigher2,
    RailChainSignalMetal,
    RailScrew,
    RailMetal,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    TransportBeltReader,
    LowerObject,
    TransportBeltCircuitConnector,
    LowerObjectAboveShadow,
    LowerObjectOverlay,
    ObjectUnder,
    Object,
    CargoHatch,
    HigherObjectUnder,
    HigherObjectAbove,
    TrainStopTop,
    ItemInInserterHand,
    AboveInserters,
    Wires,
    UnderElevated,
    ElevatedRailStonePathLower,
    ElevatedRailStonePath,
    ElevatedRailTie,
    ElevatedRailScrew,
    ElevatedRailMetal,
    ElevatedLowerObject,
    ElevatedObject,
    ElevatedHigherObject,
    FluidVisualization,
    WiresAbove,
    EntityInfoIcon,
    EntityInfoIconAbove,
    Explosion,
    Projectile,
    Smoke,
    AirObject,
    AirEntityInfoIcon,
    LightEffect,
    SelectionBox,
    HigherSelectionBox,
    CollisionSelectionBox,
    Arrow,
    Cursor,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteSizeParam {
    Size {
        #[serde(deserialize_with = "helper::truncating_deserializer")]
        size: SpriteSizeType,
    },
    Size2 {
        // TODO: truncating deserializer (in case someone is very funny and has floats here....)
        size: (SpriteSizeType, SpriteSizeType),
    },
    Explicit {
        #[serde(deserialize_with = "helper::truncating_deserializer")]
        width: SpriteSizeType,

        #[serde(deserialize_with = "helper::truncating_deserializer")]
        height: SpriteSizeType,
    },
}

pub trait FetchSprite {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput>;

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput>;

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput>;

    fn get_position(&self) -> (i16, i16);
    fn get_size(&self) -> (i16, i16);
}

// blocked until specialization is stabilized
// impl<S, T> FetchSprite for S
// where
//     S: std::ops::Deref<Target = T>,
//     T: FetchSprite,
// {
//     fn fetch(
//         &self,
//         scale: f64,
//         filename: &FileName,
//         used_mods: &UsedMods,
//         image_cache: &mut ImageCache,
//         runtime_tint: Option<Color>,
//     ) -> Option<GraphicsOutput> {
//         self.deref()
//             .fetch(scale, filename, used_mods, image_cache, runtime_tint)
//     }
//
//     fn fetch_offset(
//         &self,
//         scale: f64,
//         filename: &FileName,
//         used_mods: &UsedMods,
//         image_cache: &mut ImageCache,
//         runtime_tint: Option<Color>,
//         offset: (i16, i16),
//     ) -> Option<GraphicsOutput> {
//         self.fetch_offset(
//             scale,
//             filename,
//             used_mods,
//             image_cache,
//             runtime_tint,
//             offset,
//         )
//     }
//
//     fn fetch_offset_by_pixels(
//         &self,
//         scale: f64,
//         filename: &FileName,
//         used_mods: &UsedMods,
//         image_cache: &mut ImageCache,
//         runtime_tint: Option<Color>,
//         offset: (i16, i16),
//     ) -> Option<GraphicsOutput> {
//         self.fetch_offset_by_pixels(
//             scale,
//             filename,
//             used_mods,
//             image_cache,
//             runtime_tint,
//             offset,
//         )
//     }
//
//     fn get_position(&self) -> (i16, i16) {
//         self.get_position()
//     }
//
//     fn get_size(&self) -> (i16, i16) {
//         self.get_size()
//     }
// }

pub type GraphicsOutput = (DynamicImage, Vector);
pub trait RenderableGraphics {
    type RenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput>;
}

pub fn merge_layers<O, T: RenderableGraphics<RenderOpts = O>>(
    layers: &[T],
    scale: f64,
    used_mods: &UsedMods,
    image_cache: &mut ImageCache,
    opts: &O,
) -> Option<GraphicsOutput> {
    let layers = layers
        .iter()
        .map(|layer| layer.render(scale, used_mods, image_cache, opts))
        .collect::<Vec<_>>();

    merge_renders(layers.as_slice(), scale)
}

#[must_use]
pub fn merge_renders(renders: &[Option<GraphicsOutput>], scale: f64) -> Option<GraphicsOutput> {
    const TILE_RES: f64 = 32.0;

    let renders = renders
        .iter()
        .filter_map(|x| x.as_ref())
        .collect::<Vec<_>>();

    if renders.is_empty() {
        return None;
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for (img, shift) in &renders {
        let (shift_x, shift_y) = shift.as_tuple();
        let (width, height) = img.dimensions();
        let width = f64::from(width) * scale / TILE_RES;
        let height = f64::from(height) * scale / TILE_RES;

        let x = shift_x - (width / 2.0);
        let y = shift_y - (height / 2.0);

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }

    let px_per_tile = TILE_RES / scale;
    let width = (max_x - min_x) * px_per_tile;
    let height = (max_y - min_y) * px_per_tile;
    let res_shift = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    let center = (
        res_shift.0.mul_add(-px_per_tile, width / 2.0),
        res_shift.1.mul_add(-px_per_tile, height / 2.0),
    );

    let mut combined = DynamicImage::new_rgba8(width.ceil() as u32, height.ceil() as u32);

    for (img, shift) in &renders {
        let (shift_x, shift_y) = shift.as_tuple();
        let (post_width, post_height) = img.dimensions();
        let x = shift_x.mul_add(px_per_tile, center.0 - (f64::from(post_width) / 2.0));
        let y = shift_y.mul_add(px_per_tile, center.1 - (f64::from(post_height) / 2.0));

        imageops::overlay(&mut combined, img, x.round() as i64, y.round() as i64);
    }

    Some((combined, res_shift.into()))
}

pub trait Scale {
    fn scale(&self) -> f64;
}

// blocked until specialization is stabilized
// impl<S, T> Scale for S
// where
//     S: std::ops::Deref<Target = T>,
//     T: Scale,
// {
//     fn scale(&self) -> f64 {
//         self.deref().scale()
//     }
// }

// /// [`Types/SpriteSource`](https://lua-api.factorio.com/latest/types/SpriteSource.html)
// #[skip_serializing_none]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SpriteSource<T> {
//     pub filename: FileName,
// }

/// [`Types/SpriteParameters`](https://lua-api.factorio.com/latest/types/SpriteParameters.html)
///
/// **MISSING THE `filename` FIELD**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteParams {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub priority: SpritePriority,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: SpriteFlags,

    #[serde(flatten)]
    pub size: SpriteSizeParam,

    // TODO: turn position, x, y into some enum?
    // TODO: truncating deserializer
    #[serde(default)]
    pub position: Option<(SpriteSizeType, SpriteSizeType)>,
    #[serde(default, deserialize_with = "helper::truncating_deserializer")]
    pub x: SpriteSizeType,
    #[serde(default, deserialize_with = "helper::truncating_deserializer")]
    pub y: SpriteSizeType,

    #[serde(default)]
    pub shift: Vector,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_shadow: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_glow: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub mipmap_count: u8,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub apply_runtime_tint: bool,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub blend_mode: BlendMode,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub load_in_minimal_mode: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub premul_alpha: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generate_sdf: bool,
}

impl Scale for SpriteParams {
    fn scale(&self) -> f64 {
        self.scale
    }
}

impl FetchSprite for SpriteParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (0, 0),
        )
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (width, height) = self.get_size();
        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (offset.0 * width, offset.1 * height),
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        // TODO: add extra output for shadows
        // rendering shadows / glow / light is not supported
        if self.draw_as_shadow || self.draw_as_light {
            //|| self.draw_as_glow {
            return None;
        }

        let (x, y) = self.get_position();
        let (offset_x, offset_y) = offset;
        let (width, height) = self.get_size();

        let img = filename.load(used_mods, image_cache)?.crop_imm(
            (x + offset_x) as u32,
            (y + offset_y) as u32,
            width as u32,
            height as u32,
        );

        let mut img = img.resize(
            (f64::from(img.width()) * self.scale / scale).round() as u32,
            (f64::from(img.height()) * self.scale / scale).round() as u32,
            image::imageops::FilterType::Nearest,
        );

        // apply tint if applicable
        let tint = if self.apply_runtime_tint {
            runtime_tint.unwrap_or(self.tint)
        } else {
            self.tint
        };

        if !Color::is_white(&tint) {
            let mut img_buf = img.to_rgba8();
            let [tint_r, tint_g, tint_b, tint_a] = tint.to_rgba();

            for Rgba([r, g, b, a]) in img_buf.pixels_mut() {
                *r = (f64::from(*r) * tint_r).round() as u8;
                *g = (f64::from(*g) * tint_g).round() as u8;
                *b = (f64::from(*b) * tint_b).round() as u8;
                *a = (f64::from(*a) * tint_a).round() as u8;
            }
            img = img_buf.into();
        }

        //img.save("test.png").unwrap();

        Some((img, self.shift))
    }

    fn get_position(&self) -> (i16, i16) {
        match self.position {
            None => (self.x, self.y),
            Some((x, y)) => {
                if self.x == 0 && self.y == 0 {
                    (x, y)
                } else {
                    (self.x, self.y)
                }
            }
        }
    }

    fn get_size(&self) -> (i16, i16) {
        match self.size {
            SpriteSizeParam::Size { size } => (size, size),
            SpriteSizeParam::Size2 { size } => size,
            SpriteSizeParam::Explicit { width, height } => (width, height),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleGraphics<T: FetchSprite> {
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: Box<T>,
    },
    Layered {
        layers: FactorioArray<Self>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraphicsRenderOpts {
    pub runtime_tint: Option<Color>,
}

impl<T: FetchSprite + Scale> RenderableGraphics for SimpleGraphics<T> {
    type RenderOpts = SimpleGraphicsRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match &self {
            Self::Layered { layers } => merge_layers(layers, scale, used_mods, image_cache, opts),
            Self::Simple { filename, data } => {
                data.fetch(scale, filename, used_mods, image_cache, opts.runtime_tint)
            }
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiFileGraphics<Single: RenderableGraphics, Multi: RenderableGraphics> {
    Simple {
        #[serde(flatten)]
        data: Box<Single>,
    },
    MultiFile {
        #[serde(flatten)]
        data: Box<Multi>,
    },
    Layered {
        layers: FactorioArray<Self>,
    },
}

impl<O, S, M> Scale for MultiFileGraphics<S, M>
where
    S: RenderableGraphics<RenderOpts = O> + Scale,
    M: RenderableGraphics<RenderOpts = O> + Scale,
{
    fn scale(&self) -> f64 {
        match self {
            Self::Simple { data } => data.scale(),
            Self::MultiFile { data } => data.scale(),
            Self::Layered { layers } => layers.first().map_or(1.0, Self::scale),
        }
    }
}

impl<O, S, M> RenderableGraphics for MultiFileGraphics<S, M>
where
    S: RenderableGraphics<RenderOpts = O> + Scale,
    M: RenderableGraphics<RenderOpts = O> + Scale,
{
    type RenderOpts = O;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Layered { layers } => merge_layers(layers, scale, used_mods, image_cache, opts),
            Self::Simple { data } => data.render(scale, used_mods, image_cache, opts),
            Self::MultiFile { data } => data.render(scale, used_mods, image_cache, opts),
        }
    }
}

/// [`Types/Sprite`](https://lua-api.factorio.com/latest/types/Sprite.html)
pub type Sprite = SimpleGraphics<SpriteParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParams {
    pub filename: FileName,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub direction_count: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub counterclockwise: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_low_quality_rotation: bool,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for RotatedSpriteParams {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl Scale for RotatedSpriteParams {
    fn scale(&self) -> f64 {
        self.sprite_params.scale()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedSpriteRenderOpts {
    pub orientation: RealOrientation,
    pub runtime_tint: Option<Color>,
}

fn direction_count_to_index(
    direction_count: u16,
    orientation: RealOrientation,
    back_equals_front: bool,
) -> u16 {
    let orientation = if back_equals_front {
        orientation * 2.0 % 1.0
    } else {
        orientation
    };

    (f64::from(direction_count) * orientation).round() as u16 % direction_count
}

impl RenderableGraphics for RotatedSpriteParams {
    type RenderOpts = RotatedSpriteRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let mut index = direction_count_to_index(
            self.direction_count,
            opts.orientation,
            self.back_equals_front,
        );
        if self.counterclockwise {
            index = self.direction_count - index - 1;
        }

        // TODO: support `axially_symmetrical` and `apply_projection` (and `allow_low_quality_rotation`?)

        let line_length = if self.line_length == 0 {
            self.direction_count
        } else {
            self.line_length as u16
        };

        let row = index / line_length;
        let column = index % line_length;

        self.fetch_offset(
            scale,
            &self.filename,
            used_mods,
            image_cache,
            opts.runtime_tint,
            (column as i16, row as i16),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParamsMultiFile {
    pub filenames: FactorioArray<FileName>,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub direction_count: u16,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub counterclockwise: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_low_quality_rotation: bool,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for RotatedSpriteParamsMultiFile {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl Scale for RotatedSpriteParamsMultiFile {
    fn scale(&self) -> f64 {
        self.sprite_params.scale()
    }
}

impl RenderableGraphics for RotatedSpriteParamsMultiFile {
    type RenderOpts = RotatedSpriteRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if self.lines_per_file == 0 {
            return None;
        }

        let mut index = direction_count_to_index(
            self.direction_count,
            opts.orientation,
            self.back_equals_front,
        );
        if self.counterclockwise {
            index = self.direction_count - index - 1;
        }

        // TODO: support `axially_symmetrical` and `apply_projection` (and `allow_low_quality_rotation`?)

        let line_length = if self.line_length == 0 {
            self.direction_count
        } else {
            self.line_length as u16
        };

        let file_index = (index / line_length) / self.lines_per_file as u16;
        let row = (index / line_length) % self.lines_per_file as u16;
        let column = index % line_length;

        self.sprite_params.fetch_offset(
            scale,
            self.filenames.get(file_index as usize)?,
            used_mods,
            image_cache,
            opts.runtime_tint,
            (column as i16, row as i16),
        )
    }
}

/// [`Types/RotatedSprite`](https://lua-api.factorio.com/latest/types/RotatedSprite.html)
pub type RotatedSprite = MultiFileGraphics<RotatedSpriteParams, RotatedSpriteParamsMultiFile>;

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
/// variant for `Sprite4Way`
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite4WaySheet {
    pub filename: FileName,

    #[serde(
        default = "helper::u32_4",
        skip_serializing_if = "helper::is_4_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub frames: u32,

    pub hr_version: Option<Box<Self>>,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for Sprite4WaySheet {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl Scale for Sprite4WaySheet {
    fn scale(&self) -> f64 {
        self.hr_version
            .as_ref()
            .map_or_else(|| self.sprite_params.scale(), |hr| hr.scale())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpriteNWayRenderOpts {
    pub direction: Direction,
    pub runtime_tint: Option<Color>,
}

impl From<&SpriteNWayRenderOpts> for SimpleGraphicsRenderOpts {
    fn from(value: &SpriteNWayRenderOpts) -> Self {
        Self {
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for Sprite4WaySheet {
    type RenderOpts = SpriteNWayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: option to enable/disable HR mode
        #[allow(clippy::option_if_let_else)]
        if let Some(hr_version) = &self.hr_version {
            if scale < self.sprite_params.scale() {
                return hr_version.render(scale, used_mods, image_cache, opts);
            }
        }

        let direction = match opts.direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
            _ => unreachable!("Sprite4WaySheet does not support diagonals"),
        } % self.frames;

        self.sprite_params.fetch_offset(
            scale,
            &self.filename,
            used_mods,
            image_cache,
            opts.runtime_tint,
            (direction as i16, 0),
        )
    }
}

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
/// variant for `Sprite8Way`
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite8WaySheet {
    pub filename: FileName,

    #[serde(
        default = "helper::u32_8",
        skip_serializing_if = "helper::is_8_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub frames: u32,

    pub hr_version: Option<Box<Self>>,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for Sprite8WaySheet {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl Scale for Sprite8WaySheet {
    fn scale(&self) -> f64 {
        self.hr_version
            .as_ref()
            .map_or_else(|| self.sprite_params.scale(), |hr| hr.scale())
    }
}

impl RenderableGraphics for Sprite8WaySheet {
    type RenderOpts = SpriteNWayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: option to enable/disable HR mode
        #[allow(clippy::option_if_let_else)]
        if let Some(hr_version) = &self.hr_version {
            if scale < self.sprite_params.scale() {
                return hr_version.render(scale, used_mods, image_cache, opts);
            }
        }

        let direction = opts.direction as u32 % self.frames;

        let (width, _) = self.sprite_params.get_size();
        self.sprite_params.fetch_offset(
            scale,
            &self.filename,
            used_mods,
            image_cache,
            opts.runtime_tint,
            (direction as i16, 0),
        )
    }
}

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite4Way {
    Sprite(Sprite),
    Sheets {
        sheets: FactorioArray<Sprite4WaySheet>,
    },
    Sheet {
        sheet: Sprite4WaySheet,
    },
    Directions {
        north: Sprite,
        east: Sprite,
        south: Sprite,
        west: Sprite,
    },
}

impl RenderableGraphics for Sprite4Way {
    type RenderOpts = SpriteNWayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Sprite(sprite) => sprite.render(scale, used_mods, image_cache, &opts.into()),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Directions {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north,
                Direction::East => east,
                Direction::South => south,
                Direction::West => west,
                _ => {
                    unimplemented!("Sprite4Way does not support diagonals")
                }
            }
            .render(scale, used_mods, image_cache, &opts.into()),
        }
    }
}

/// [`Types/Sprite8Way`](https://lua-api.factorio.com/latest/types/Sprite8Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite8Way {
    Sheets {
        sheets: FactorioArray<Sprite8WaySheet>,
    },
    Sheet {
        sheet: Sprite8WaySheet,
    },
    Directions {
        north: Sprite,
        north_east: Sprite,
        east: Sprite,
        south_east: Sprite,
        south: Sprite,
        south_west: Sprite,
        west: Sprite,
        north_west: Sprite,
    },
}

impl RenderableGraphics for Sprite8Way {
    type RenderOpts = SpriteNWayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => todo!(),
            Self::Directions {
                north,
                north_east,
                east,
                south_east,
                south,
                south_west,
                west,
                north_west,
            } => match opts.direction {
                Direction::North => north,
                Direction::NorthEast => north_east,
                Direction::East => east,
                Direction::SouthEast => south_east,
                Direction::South => south,
                Direction::SouthWest => south_west,
                Direction::West => west,
                Direction::NorthWest => north_west,
                _ => {
                    unimplemented!("Sprite8Way does not support half-diagonals");
                }
            }
            .render(scale, used_mods, image_cache, &opts.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSheetParams {
    #[serde(
        default = "helper::u32_1",
        skip_serializing_if = "helper::is_1_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub variation_count: u32,

    #[serde(
        default = "helper::u32_1",
        skip_serializing_if = "helper::is_1_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub repeat_count: u32,

    // TODO: support the default based on variation_count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_length: Option<u32>,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for SpriteSheetParams {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl FetchSprite for SpriteSheetParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch(scale, filename, used_mods, image_cache, runtime_tint)
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn get_position(&self) -> (i16, i16) {
        self.sprite_params.get_position()
    }

    fn get_size(&self) -> (i16, i16) {
        self.sprite_params.get_size()
    }
}

impl Scale for SpriteSheetParams {
    fn scale(&self) -> f64 {
        self.sprite_params.scale()
    }
}

/// [`Types/SpriteSheet`](https://lua-api.factorio.com/latest/types/SpriteSheet.html)
pub type SpriteSheet = SimpleGraphics<SpriteSheetParams>;

/// [`Types/SpriteVariations`](https://lua-api.factorio.com/latest/types/SpriteVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteVariations {
    Struct { sheet: SpriteSheet },
    SpriteSheet(SpriteSheet),
    Array(FactorioArray<Sprite>),
}

#[derive(Debug, Clone, Copy)]
pub struct SpriteVariationsRenderOpts {
    pub variation: NonZeroU32,
    pub runtime_tint: Option<Color>,
}

impl Default for SpriteVariationsRenderOpts {
    #[allow(unsafe_code)]
    fn default() -> Self {
        Self {
            variation: unsafe { NonZeroU32::new_unchecked(1) },
            runtime_tint: None,
        }
    }
}

impl From<&SpriteVariationsRenderOpts> for SimpleGraphicsRenderOpts {
    fn from(value: &SpriteVariationsRenderOpts) -> Self {
        Self {
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for SpriteVariations {
    type RenderOpts = SpriteVariationsRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Struct { sheet } | Self::SpriteSheet(sheet) => {
                // TODO: implement variations here
                sheet.render(scale, used_mods, image_cache, &opts.into())
            }
            Self::Array(variations) => variations.get((opts.variation.get() - 1) as usize)?.render(
                scale,
                used_mods,
                image_cache,
                &opts.into(),
            ),
        }
    }
}

// ======================= //
// =======[ Tiles ]======= //
// ======================= //

#[derive(Debug, Clone, Copy)]
pub struct TileRenderOpts {
    pub position: MapPosition,
    pub runtime_tint: Option<Color>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TileGraphics<T: FetchSprite> {
    pub picture: FileName,

    #[serde(flatten)]
    data: Box<T>,
}

impl<T: FetchSprite> std::ops::Deref for TileGraphics<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: FetchSprite> RenderableGraphics for TileGraphics<T> {
    type RenderOpts = TileRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let x = opts.position.x().ceil() as i16;
        let y = opts.position.y().ceil() as i16;

        self.fetch_offset(
            scale,
            &self.picture,
            used_mods,
            image_cache,
            opts.runtime_tint,
            (x, y),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSpriteLayout {
    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub x: SpriteSizeType,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub y: SpriteSizeType,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub line_length: u32,

    #[serde(
        default,
        deserialize_with = "helper::truncating_deserializer",
        skip_serializing_if = "helper::is_default"
    )]
    pub count: u32,
}

impl FetchSprite for TileSpriteLayout {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (0, 0),
        )
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (width, height) = self.get_size();

        let size = 8; // TODO: verify this? https://forums.factorio.com/113757
        let count = self.count as i16;
        let mut offset_x = offset.0.rem_euclid(size * count);
        let mut offset_y = offset.1.rem_euclid(size);

        if self.line_length > 0 {
            let line_length = self.line_length as i16;
            offset_y += offset_x / line_length * size;
            offset_x %= line_length;
        }

        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (offset_x * width, offset_y * height),
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (x, y) = self.get_position();
        let (offset_x, offset_y) = offset;
        let (width, height) = self.get_size();

        let img = filename.load(used_mods, image_cache)?.crop_imm(
            (x + offset_x) as u32,
            (y + offset_y) as u32,
            width as u32,
            height as u32,
        );

        let mut img = img.resize(
            (f64::from(img.width()) * self.scale / scale).round() as u32,
            (f64::from(img.height()) * self.scale / scale).round() as u32,
            image::imageops::FilterType::Nearest,
        );

        if let Some(tint) = runtime_tint {
            if !Color::is_white(&tint) {
                let mut img_buf = img.to_rgba8();
                let [tint_r, tint_g, tint_b, tint_a] = tint.to_rgba();

                for Rgba([r, g, b, a]) in img_buf.pixels_mut() {
                    *r = (f64::from(*r) * tint_r).round() as u8;
                    *g = (f64::from(*g) * tint_g).round() as u8;
                    *b = (f64::from(*b) * tint_b).round() as u8;
                    *a = (f64::from(*a) * tint_a).round() as u8;
                }
                img = img_buf.into();
            }
        }

        Some((img, Vector::default()))
    }

    fn get_position(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (i16, i16) {
        let size = (32.0 / self.scale).round() as i16;
        (size, size)
    }
}

impl Scale for TileSpriteLayout {
    fn scale(&self) -> f64 {
        self.scale
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileLightPicturesParams {
    pub size: u32,

    #[serde(flatten)]
    tile_sprite_layout: TileSpriteLayout,
}

impl FetchSprite for TileLightPicturesParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (0, 0),
        )
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (width, height) = self.get_size();

        let size = self.size as i16;
        let count = self.count as i16;
        let line_length = self.line_length as i16;
        let mut offset_x = offset.0.rem_euclid(size * count);
        let mut offset_y = offset.1.rem_euclid(size);

        if self.line_length > 0 {
            let line_length = self.line_length as i16;
            offset_y += offset_x / line_length * size;
            offset_x %= line_length;
        }

        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (offset_x * width, offset_y * height),
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (x, y) = self.get_position();
        let (offset_x, offset_y) = offset;
        let (width, height) = self.get_size();

        let img = filename.load(used_mods, image_cache)?.crop_imm(
            (x + offset_x) as u32,
            (y + offset_y) as u32,
            width as u32,
            height as u32,
        );

        let mut img = img.resize(
            (f64::from(img.width()) * self.scale / scale).round() as u32,
            (f64::from(img.height()) * self.scale / scale).round() as u32,
            image::imageops::FilterType::Nearest,
        );

        if let Some(tint) = runtime_tint {
            if !Color::is_white(&tint) {
                let mut img_buf = img.to_rgba8();
                let [tint_r, tint_g, tint_b, tint_a] = tint.to_rgba();

                for Rgba([r, g, b, a]) in img_buf.pixels_mut() {
                    *r = (f64::from(*r) * tint_r).round() as u8;
                    *g = (f64::from(*g) * tint_g).round() as u8;
                    *b = (f64::from(*b) * tint_b).round() as u8;
                    *a = (f64::from(*a) * tint_a).round() as u8;
                }
                img = img_buf.into();
            }
        }

        Some((img, Vector::default()))
    }

    fn get_position(&self) -> (i16, i16) {
        self.tile_sprite_layout.get_position()
    }

    fn get_size(&self) -> (i16, i16) {
        let size = self.size as i16;
        let (w, h) = self.tile_sprite_layout.get_size();
        (w * size, h * size)
    }
}

impl Scale for TileLightPicturesParams {
    fn scale(&self) -> f64 {
        self.tile_sprite_layout.scale()
    }
}

impl std::ops::Deref for TileLightPicturesParams {
    type Target = TileSpriteLayout;

    fn deref(&self) -> &Self::Target {
        &self.tile_sprite_layout
    }
}

/// [`Types/TileLightPictures`](https://lua-api.factorio.com/latest/types/TileLightPictures.html)
pub type TileLightPictures = TileGraphics<TileLightPicturesParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMainPicturesParams {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub size: u32,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: FactorioArray<f64>,

    #[serde(flatten)]
    tile_sprite_layout: TileSpriteLayout,
}

impl FetchSprite for TileMainPicturesParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (0, 0),
        )
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (width, height) = self.get_size();

        let size = self.size as i16;
        let count = self.count as i16;
        let line_length = self.line_length as i16;
        let mut offset_x = offset.0.rem_euclid(size * count);
        let mut offset_y = offset.1.rem_euclid(size);

        if self.line_length > 0 {
            let line_length = self.line_length as i16;
            offset_y += offset_x / line_length * size;
            offset_x %= line_length;
        }

        self.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            (offset_x * width, offset_y * height),
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (x, y) = self.get_position();
        let (offset_x, offset_y) = offset;
        let (width, height) = self.get_size();

        let img = filename.load(used_mods, image_cache)?.crop_imm(
            (x + offset_x) as u32,
            (y + offset_y) as u32,
            width as u32,
            height as u32,
        );

        let mut img = img.resize(
            (f64::from(img.width()) * self.scale / scale).round() as u32,
            (f64::from(img.height()) * self.scale / scale).round() as u32,
            image::imageops::FilterType::Nearest,
        );

        if let Some(tint) = runtime_tint {
            if !Color::is_white(&tint) {
                let mut img_buf = img.to_rgba8();
                let [tint_r, tint_g, tint_b, tint_a] = tint.to_rgba();

                for Rgba([r, g, b, a]) in img_buf.pixels_mut() {
                    *r = (f64::from(*r) * tint_r).round() as u8;
                    *g = (f64::from(*g) * tint_g).round() as u8;
                    *b = (f64::from(*b) * tint_b).round() as u8;
                    *a = (f64::from(*a) * tint_a).round() as u8;
                }
                img = img_buf.into();
            }
        }

        Some((img, Vector::default()))
    }

    fn get_position(&self) -> (i16, i16) {
        self.tile_sprite_layout.get_position()
    }

    fn get_size(&self) -> (i16, i16) {
        let size = self.size as i16;
        let (w, h) = self.tile_sprite_layout.get_size();
        (w * size, h * size)
    }
}

impl Scale for TileMainPicturesParams {
    fn scale(&self) -> f64 {
        self.tile_sprite_layout.scale()
    }
}

impl std::ops::Deref for TileMainPicturesParams {
    type Target = TileSpriteLayout;

    fn deref(&self) -> &Self::Target {
        &self.tile_sprite_layout
    }
}

/// [`Types/TileMainPictures`](https://lua-api.factorio.com/latest/types/TileMainPictures.html)
pub type TileMainPictures = TileGraphics<TileMainPicturesParams>;

// ======================== //
// =====[ Animations ]===== //
// ======================== //

// TODO: truncating deserializer for arrays....
/// [`Types/AnimationFrameSequence`](https://lua-api.factorio.com/latest/types/AnimationFrameSequence.html)
pub type AnimationFrameSequence = FactorioArray<u16>;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationRunMode {
    #[default]
    Forward,
    Backward,
    ForwardThenBackward,
}

/// [`Types/Stripe`](https://lua-api.factorio.com/latest/types/Stripe.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stripe {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub width_in_frames: u32,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub height_in_frames: Option<u32>, // TODO: is only optional when used in RotatedAnimation
    pub filename: FileName,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub x: u32,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub y: u32,
}

impl Stripe {
    #[must_use]
    pub fn frame_count(&self) -> u32 {
        self.width_in_frames * self.height_in_frames.unwrap_or(1)
    }

    #[must_use]
    pub fn rotated_frame_count(&self, direction_count: u32) -> u32 {
        self.frame_count() * self.height_in_frames.unwrap_or(direction_count)
    }
}

/// [`Types/AnimationParameters`](https://lua-api.factorio.com/latest/types/AnimationParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationParams {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub run_mode: AnimationRunMode,

    // #[serde(
    //     default = "helper::u32_1",
    //     skip_serializing_if = "helper::is_1_u32",
    //     deserialize_with = "helper::truncating_deserializer"
    // )]

    // overridden in AnimationSheet
    pub frame_count: Option<u32>,

    // #[serde(
    //     default,
    //     skip_serializing_if = "helper::is_default",
    //     deserialize_with = "helper::truncating_deserializer"
    // )]

    // overridden in AnimationSheet
    pub line_length: Option<u32>,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub animation_speed: f64,

    #[serde(
        default = "helper::f64_max",
        skip_serializing_if = "helper::is_max_f64"
    )]
    pub max_advance: f64,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub repeat_count: u8,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frame_sequence: AnimationFrameSequence,

    #[serde(flatten)]
    sprite_params: SpriteParams,
}

impl std::ops::Deref for AnimationParams {
    type Target = SpriteParams;

    fn deref(&self) -> &Self::Target {
        &self.sprite_params
    }
}

impl FetchSprite for AnimationParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch(scale, filename, used_mods, image_cache, runtime_tint)
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn get_position(&self) -> (i16, i16) {
        self.sprite_params.get_position()
    }

    fn get_size(&self) -> (i16, i16) {
        self.sprite_params.get_size()
    }
}

impl Scale for AnimationParams {
    fn scale(&self) -> f64 {
        self.sprite_params.scale()
    }
}

impl AnimationParams {
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn frame_index(&self, progress: f64) -> u32 {
        let mode_factor = match self.run_mode {
            AnimationRunMode::Forward => 1,
            AnimationRunMode::Backward => 1,
            AnimationRunMode::ForwardThenBackward => 2,
        };

        let frame_count = if self.frame_sequence.is_empty() {
            self.frame_count.unwrap_or(1)
        } else {
            self.frame_sequence.len() as u32
        };

        let total_frame = (progress
            * f64::from(frame_count * mode_factor * u32::from(self.repeat_count)))
        .round() as u32;

        let non_repeat_frame = total_frame % (frame_count * mode_factor);

        let non_mode_frame = match self.run_mode {
            AnimationRunMode::Forward => non_repeat_frame,
            AnimationRunMode::Backward => non_repeat_frame, //frame_count - non_repeat_frame - 1, // TODO: figure out what backward runmode does
            AnimationRunMode::ForwardThenBackward => {
                if non_repeat_frame < frame_count {
                    non_repeat_frame
                } else {
                    frame_count * 2 - non_repeat_frame - 1
                }
            }
        };

        if self.frame_sequence.is_empty() {
            non_mode_frame
        } else {
            u32::from(
                self.frame_sequence
                    .get(non_mode_frame as usize)
                    .copied()
                    .unwrap_or_default(),
            )
        }
    }

    #[must_use]
    pub fn line_length(&self) -> u32 {
        let line_length = self.line_length.unwrap_or_default();

        if line_length == 0 {
            self.frame_count.unwrap_or(1)
        } else {
            line_length
        }
    }
}

/// [`Types/Animation`](https://lua-api.factorio.com/latest/types/Animation.html)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Animation {
    Layered {
        layers: FactorioArray<Self>,
    },
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: Box<AnimationParams>,

        hr_version: Option<Box<Self>>,
    },
    Striped {
        stripes: FactorioArray<Stripe>,

        #[serde(flatten)]
        data: Box<AnimationParams>,

        hr_version: Option<Box<Self>>,
    },
}

impl Animation {
    #[must_use]
    pub fn frame_count(&self) -> u32 {
        match self {
            Self::Layered { layers } => layers.first().map_or(1, Self::frame_count),
            Self::Simple { data, .. } | Self::Striped { data, .. } => data.frame_count.unwrap_or(1),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationRenderOpts {
    pub progress: f64,
    pub runtime_tint: Option<Color>,
}

impl RenderableGraphics for Animation {
    type RenderOpts = AnimationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match &self {
            Self::Layered { layers } => merge_layers(layers, scale, used_mods, image_cache, opts),
            Self::Striped {
                stripes,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                if let Some(hr_version) = hr_version {
                    if scale < data.scale() {
                        return hr_version.render(scale, used_mods, image_cache, opts);
                    }
                }

                // TODO: add extra output for shadows
                // rendering shadows / glow / light is not supported
                // theoretically this is not needed here
                if data.sprite_params.draw_as_shadow
                        //|| data.sprite_params.draw_as_glow
                        || data.sprite_params.draw_as_light
                {
                    return None;
                }

                let index = data.frame_index(opts.progress);
                let mut curr_stripe_max = 0;

                for stripe in stripes {
                    let stripe_max = stripe.frame_count();

                    if index >= stripe_max + curr_stripe_max {
                        curr_stripe_max += stripe_max;
                        continue;
                    }

                    // prevent division by 0 panic
                    // tho that should already be prevented by skipping over stripes with 0 frames
                    if stripe.width_in_frames == 0 {
                        return None;
                    }

                    let stripe_index = index - curr_stripe_max;
                    let row = stripe_index / stripe.width_in_frames;
                    let column = stripe_index % stripe.width_in_frames;

                    return data.fetch_offset(
                        scale,
                        &stripe.filename,
                        used_mods,
                        image_cache,
                        opts.runtime_tint,
                        (column as i16, row as i16),
                    );
                }

                None
            }
            Self::Simple {
                filename,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                if let Some(hr_version) = hr_version {
                    if scale < data.scale() {
                        return hr_version.render(scale, used_mods, image_cache, opts);
                    }
                }

                // line_length = 0 means all frames are in a single line
                let line_length = if data.line_length.unwrap_or(0) == 0 {
                    data.frame_count.unwrap_or(1)
                } else {
                    data.line_length.unwrap_or(0)
                };

                // prevent division by 0 panic
                if line_length == 0 {
                    return None;
                }

                let index = data.frame_index(opts.progress);
                let row = index / line_length;
                let column = index % line_length;

                data.fetch_offset(
                    scale,
                    filename,
                    used_mods,
                    image_cache,
                    opts.runtime_tint,
                    (column as i16, row as i16),
                )
            }
        }
    }
}

/// [`Types/Animation4Way`](https://lua-api.factorio.com/latest/types/Animation4Way.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Animation4Way {
    Animation(Animation),
    Struct {
        north: Animation,
        east: Option<Animation>,
        south: Option<Animation>,
        west: Option<Animation>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Animation4WayRenderOpts {
    pub direction: Direction,
    pub progress: f64,
    pub runtime_tint: Option<Color>,
}

impl From<&Animation4WayRenderOpts> for AnimationRenderOpts {
    fn from(value: &Animation4WayRenderOpts) -> Self {
        Self {
            progress: value.progress,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for Animation4Way {
    type RenderOpts = Animation4WayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Animation(animation) => {
                animation.render(scale, used_mods, image_cache, &opts.into())
            }
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north,
                Direction::East => east.as_ref().unwrap_or(north),
                Direction::South => south.as_ref().unwrap_or(north),
                Direction::West => west
                    .as_ref()
                    .unwrap_or_else(|| east.as_ref().unwrap_or(north)),
                _ => {
                    unimplemented!("Animation4Way only supports cardinal directions")
                }
            }
            .render(scale, used_mods, image_cache, &opts.into()),
        }
    }
}

/// [`Types/AnimationElement`](https://lua-api.factorio.com/latest/types/AnimationElement.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationElement {
    pub render_layer: Option<RenderLayer>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub secondary_draw_order: Option<i8>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_as_sprite: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_as_light: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub apply_tint: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw: bool,

    pub animation: Option<Animation>,
}

impl RenderableGraphics for AnimationElement {
    type RenderOpts = AnimationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if !self.draw_as_sprite || self.draw_as_light {
            return None;
        }

        self.animation
            .as_ref()
            .and_then(|animation| animation.render(scale, used_mods, image_cache, opts))
    }
}

// TODO: check & properly implement AnimationSheet & AnimationVariations
/// [`Types/AnimationSheet`](https://lua-api.factorio.com/latest/types/AnimationSheet.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationSheet {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub variation_count: u32,

    pub hr_version: Option<Box<Self>>,

    #[serde(flatten)]
    animation_params: AnimationParams,
}

impl std::ops::Deref for AnimationSheet {
    type Target = AnimationParams;

    fn deref(&self) -> &Self::Target {
        &self.animation_params
    }
}

/// [`Types/AnimationVariations`](https://lua-api.factorio.com/latest/types/AnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimationVariations {
    Animation(Animation),
    Array(FactorioArray<Animation>),
    Sheet {
        sheet: AnimationSheet,
    },
    Sheets {
        sheets: FactorioArray<AnimationSheet>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationVariationsRenderOpts {
    pub variation: NonZeroU32,
    pub progress: f64,
    pub runtime_tint: Option<Color>,
}

impl Default for AnimationVariationsRenderOpts {
    fn default() -> Self {
        #[allow(unsafe_code)]
        Self {
            variation: unsafe { NonZeroU32::new_unchecked(1) },
            progress: 0.0,
            runtime_tint: None,
        }
    }
}

impl From<&AnimationVariationsRenderOpts> for AnimationRenderOpts {
    fn from(value: &AnimationVariationsRenderOpts) -> Self {
        Self {
            progress: value.progress,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for AnimationVariations {
    type RenderOpts = AnimationVariationsRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Animation(animation) => {
                animation.render(scale, used_mods, image_cache, &opts.into())
            }
            Self::Array(animations) => animations.get((opts.variation.get() - 1) as usize)?.render(
                scale,
                used_mods,
                image_cache,
                &opts.into(),
            ),
            Self::Sheets { sheets } => todo!(), //merge_layers(sheets,  used_mods, image_cache, opts),
            Self::Sheet { sheet } => todo!(),
        }
    }
}

/// [`Types/ShiftAnimationWaypoints`](https://lua-api.factorio.com/latest/types/ShiftAnimationWaypoints.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftAnimationWaypoints {
    pub north: FactorioArray<Vector>,
    pub east: FactorioArray<Vector>,
    pub south: FactorioArray<Vector>,
    pub west: FactorioArray<Vector>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedAnimationParams {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub direction_count: u32,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub lines_per_file: Option<u32>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "helper::truncating_opt_deserializer"
    )]
    pub slice: Option<u32>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub still_frame: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub counterclockwise: bool,

    #[serde(default = "helper::f64_05", skip_serializing_if = "helper::is_05_f64")]
    pub middle_orientation: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub orientation_range: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(flatten)]
    animation_params: AnimationParams,
}

impl std::ops::Deref for RotatedAnimationParams {
    type Target = AnimationParams;

    fn deref(&self) -> &Self::Target {
        &self.animation_params
    }
}

impl Scale for RotatedAnimationParams {
    fn scale(&self) -> f64 {
        self.animation_params.scale()
    }
}

impl FetchSprite for RotatedAnimationParams {
    fn fetch(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.animation_params
            .fetch(scale, filename, used_mods, image_cache, runtime_tint)
    }

    fn fetch_offset(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.animation_params.fetch_offset(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        scale: f64,
        filename: &FileName,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.animation_params.fetch_offset_by_pixels(
            scale,
            filename,
            used_mods,
            image_cache,
            runtime_tint,
            offset,
        )
    }

    fn get_position(&self) -> (i16, i16) {
        self.animation_params.get_position()
    }

    fn get_size(&self) -> (i16, i16) {
        self.animation_params.get_size()
    }
}

impl RotatedAnimationParams {
    #[must_use]
    pub fn orientation_index(&self, orientation: RealOrientation) -> u32 {
        let index =
            (orientation * f64::from(self.direction_count)).round() as u32 % self.direction_count;

        if self.counterclockwise {
            self.direction_count - index - 1
        } else {
            index
        }
    }
}

/// [`Types/RotatedAnimation`](https://lua-api.factorio.com/latest/types/RotatedAnimation.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimation {
    Layered {
        layers: FactorioArray<Self>,
    },
    Single {
        filename: FileName,

        #[serde(flatten)]
        data: Box<RotatedAnimationParams>,

        hr_version: Option<Box<Self>>,
    },
    Multi {
        filenames: FactorioArray<FileName>,

        #[serde(flatten)]
        data: Box<RotatedAnimationParams>,

        hr_version: Option<Box<Self>>,
    },
    Striped {
        stripes: FactorioArray<Stripe>,

        #[serde(flatten)]
        data: Box<RotatedAnimationParams>,

        hr_version: Option<Box<Self>>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedAnimationRenderOpts {
    pub orientation: RealOrientation,
    pub progress: f64,
    pub runtime_tint: Option<Color>,

    pub override_index: Option<u8>,
}

impl From<&RotatedAnimationRenderOpts> for AnimationRenderOpts {
    fn from(value: &RotatedAnimationRenderOpts) -> Self {
        Self {
            progress: value.progress,
            runtime_tint: value.runtime_tint,
        }
    }
}

impl RenderableGraphics for RotatedAnimation {
    type RenderOpts = RotatedAnimationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Layered { layers } => merge_layers(layers, scale, used_mods, image_cache, opts),
            Self::Striped {
                stripes,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                if let Some(hr_version) = hr_version {
                    if scale < data.scale() {
                        return hr_version.render(scale, used_mods, image_cache, opts);
                    }
                }

                None
            }
            Self::Multi {
                filenames,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                if let Some(hr_version) = hr_version {
                    if scale < data.scale() {
                        return hr_version.render(scale, used_mods, image_cache, opts);
                    }
                }

                let orientation_index = opts
                    .override_index
                    .map_or_else(|| data.orientation_index(opts.orientation), u32::from);
                let file_index = orientation_index / data.lines_per_file.unwrap_or(1);
                let frame_index = data.animation_params.frame_index(opts.progress);
                let line_length = data.animation_params.line_length();

                let column = frame_index % line_length;
                let row = frame_index / line_length;

                data.fetch_offset(
                    scale,
                    filenames.get(file_index as usize)?,
                    used_mods,
                    image_cache,
                    opts.runtime_tint,
                    (column as i16, (row + orientation_index) as i16),
                )
            }
            Self::Single {
                filename,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                if let Some(hr_version) = hr_version {
                    if scale < data.scale() {
                        return hr_version.render(scale, used_mods, image_cache, opts);
                    }
                }

                let orientation_index = opts
                    .override_index
                    .map_or_else(|| data.orientation_index(opts.orientation), u32::from);
                let frame_index = data.animation_params.frame_index(opts.progress);
                let line_length = data.animation_params.line_length();

                let column = frame_index % line_length;
                let row = frame_index / line_length;

                data.fetch_offset(
                    scale,
                    filename,
                    used_mods,
                    image_cache,
                    opts.runtime_tint,
                    (column as i16, (row + orientation_index) as i16),
                )
            }
        }
    }
}

/// [`Types/RotatedAnimation4Way`](https://lua-api.factorio.com/latest/types/RotatedAnimation4Way.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimation4Way {
    RotatedAnimation(RotatedAnimation),
    Struct {
        north: RotatedAnimation,
        east: Option<RotatedAnimation>,
        south: Option<RotatedAnimation>,
        west: Option<RotatedAnimation>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedAnimation4WayRenderOpts {
    pub direction: Direction,
    pub orientation: RealOrientation,
    pub progress: f64,
    pub runtime_tint: Option<Color>,
}

impl From<&RotatedAnimation4WayRenderOpts> for RotatedAnimationRenderOpts {
    fn from(value: &RotatedAnimation4WayRenderOpts) -> Self {
        Self {
            orientation: value.orientation,
            progress: value.progress,
            runtime_tint: value.runtime_tint,
            override_index: None,
        }
    }
}

impl RenderableGraphics for RotatedAnimation4Way {
    type RenderOpts = RotatedAnimation4WayRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::RotatedAnimation(animation) => {
                animation.render(scale, used_mods, image_cache, &opts.into())
            }
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north,
                Direction::East => east.as_ref().unwrap_or(north),
                Direction::South => south.as_ref().unwrap_or(north),
                Direction::West => west
                    .as_ref()
                    .unwrap_or_else(|| east.as_ref().unwrap_or(north)),
                _ => {
                    unimplemented!("RotatedAnimation4Way does not support diagonals")
                }
            }
            .render(scale, used_mods, image_cache, &opts.into()),
        }
    }
}

/// [`Types/RotatedAnimationVariations`](https://lua-api.factorio.com/latest/types/RotatedAnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimationVariations {
    Animation(RotatedAnimation),
    Array(FactorioArray<RotatedAnimation>),
}

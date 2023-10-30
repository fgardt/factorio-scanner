use std::collections::HashMap;

use image::{
    imageops::{self, FilterType},
    DynamicImage, GenericImageView, Rgba,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{helper, Color, Direction, FileName, Vector};

/// [`Types/SpritePriority`](https://lua-api.factorio.com/latest/types/SpritePriority.html)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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
pub type SpriteFlags = Vec<SpriteFlag>;

/// [`Types/SpriteSizeType`](https://lua-api.factorio.com/latest/types/SpriteSizeType.html)
pub type SpriteSizeType = i16;

/// [`Types/BlendMode`](https://lua-api.factorio.com/latest/types/BlendMode.html)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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
    WaterTile,
    GroundTile,
    TileTransition,
    Decals,
    LowerRadiusVisualization,
    RadiusVisualization,
    TransportBeltIntegration,
    Resource,
    BuildingSmoke,
    Decorative,
    GroundPatch,
    GroundPatchHigher,
    GroundPatchHigher2,
    Remnants,
    Floor,
    TransportBelt,
    TransportBeltEndings,
    TransportBeltCircuitConnector,
    FloorMechanicsUnderCorpse,
    Corpse,
    FloorMechanics,
    Item,
    LowerObject,
    LowerObjectAboveShadow,
    Object,
    HigherObjectUnder,
    HigherObjectAbove,
    ItemInInserterHand,
    Wires,
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
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput>;

    fn fetch_offset(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput>;

    fn fetch_offset_by_pixels(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput>;

    fn get_position(&self) -> (i16, i16);
    fn get_size(&self) -> (i16, i16);
}

pub type GraphicsOutput = (DynamicImage, f64, Vector);
pub trait RenderableGraphics {
    type RenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput>;
}

pub fn merge_layers<O, T: RenderableGraphics<RenderOpts = O>>(
    layers: &[T],
    factorio_dir: &str,
    used_mods: &HashMap<&str, &str>,
    opts: &O,
) -> Option<GraphicsOutput> {
    let layers = layers
        .iter()
        .map(|layer| layer.render(factorio_dir, used_mods, opts))
        .collect::<Vec<_>>();

    merge_renders(layers.as_slice())
}

pub fn merge_renders(renders: &[Option<GraphicsOutput>]) -> Option<GraphicsOutput> {
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
    let mut min_scale = f64::MAX;

    for (img, scale, (shift_x, shift_y)) in &renders {
        let (width, height) = img.dimensions();
        let width = f64::from(width) * scale / TILE_RES;
        let height = f64::from(height) * scale / TILE_RES;

        let x = shift_x - (width / 2.0);
        let y = shift_y - (height / 2.0);

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
        min_scale = min_scale.min(*scale);

        //println!("{width}x{height} x{scale} ({shift_x}, {shift_y})");
    }

    if min_scale <= 0.0 {
        return None;
    }

    let px_per_tile = TILE_RES / min_scale;
    let width = (max_x - min_x) * px_per_tile;
    let height = (max_y - min_y) * px_per_tile;
    let res_shift = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    let center = (
        res_shift.0.mul_add(-px_per_tile, width / 2.0),
        res_shift.1.mul_add(-px_per_tile, height / 2.0),
    );

    //println!("px/tile: {px_per_tile}\nborders: {min_x} {max_x} | {min_y} {max_y}\nsize: {width}, {height}\nshift: {res_shift:?}\ncenter: {center:?}");

    let mut combined = DynamicImage::new_rgba8(width.ceil() as u32, height.ceil() as u32);

    for (img, scale, (shift_x, shift_y)) in &renders {
        let effective_scale = scale / min_scale;
        let (pre_width, pre_height) = img.dimensions();
        let scaled = img.resize(
            (f64::from(pre_width) * effective_scale).round() as u32,
            (f64::from(pre_height) * effective_scale).round() as u32,
            FilterType::Triangle,
        );
        let (post_width, post_height) = scaled.dimensions();
        let x = center.0 - (f64::from(post_width) / 2.0) + (shift_x * px_per_tile);
        let y = center.1 - (f64::from(post_height) / 2.0) + (shift_y * px_per_tile);

        imageops::overlay(&mut combined, &scaled, x.round() as i64, y.round() as i64);
    }

    Some((combined, min_scale, res_shift))
}

/// [`Types/SpriteParameters`](https://lua-api.factorio.com/latest/types/SpriteParameters.html)
///
/// **MISSING THE `filename` FIELD**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteParams {
    // TODO: skip serializing if default
    #[serde(default)]
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
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub mipmap_count: u8,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub apply_runtime_tint: bool,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    // TODO: skip serializing if default
    #[serde(default)]
    pub blend_mode: BlendMode,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub load_in_minimal_mode: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub premul_alpha: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub generate_sdf: bool,
}

impl FetchSprite for SpriteParams {
    fn fetch(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.fetch_offset_by_pixels(factorio_dir, filename, used_mods, runtime_tint, (0, 0))
    }

    fn fetch_offset(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        let (width, height) = self.get_size();
        self.fetch_offset_by_pixels(
            factorio_dir,
            filename,
            used_mods,
            runtime_tint,
            (offset.0 * width, offset.1 * height),
        )
    }

    fn fetch_offset_by_pixels(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        //println!("{data:?}");

        // TODO: add extra output for shadows
        // rendering shadows / glow / light is not supported
        if self.draw_as_shadow 
        //|| self.draw_as_glow
        || self.draw_as_light {
            return None;
        }

        let (x, y) = self.get_position();
        let (offset_x, offset_y) = offset;
        let (width, height) = self.get_size();

        let mut img = filename.load(factorio_dir, &used_mods)?.crop_imm(
            (x + offset_x) as u32,
            (y + offset_y) as u32,
            width as u32,
            height as u32,
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

        Some((img, self.scale, self.shift))
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
        data: T,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    Layered {
        layers: Vec<Self>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleGraphicsRenderOpts {
    pub runtime_tint: Option<Color>,
}

impl<T: FetchSprite> RenderableGraphics for SimpleGraphics<T> {
    type RenderOpts = SimpleGraphicsRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match &self {
            Self::Layered { layers } => merge_layers(layers, factorio_dir, used_mods, opts),
            Self::Simple {
                filename,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    data.fetch(factorio_dir, filename, used_mods, opts.runtime_tint)
                }
            }
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiFileGraphics<Single, Multi> {
    Simple {
        #[serde(flatten)]
        data: Single,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    MultiFile {
        #[serde(flatten)]
        data: Multi,

        #[serde(skip_serializing_if = "Option::is_none")]
        hr_version: Option<Box<Self>>,
    },
    Layered {
        layers: Vec<Self>,
    },
}

impl<O, S, M> RenderableGraphics for MultiFileGraphics<S, M>
where
    S: RenderableGraphics<RenderOpts = O>,
    M: RenderableGraphics<RenderOpts = O>,
{
    type RenderOpts = O;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Layered { layers } => merge_layers(layers, factorio_dir, used_mods, opts),
            Self::Simple { data, hr_version } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    data.render(factorio_dir, used_mods, opts)
                }
            }
            Self::MultiFile { data, hr_version } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    data.render(factorio_dir, used_mods, opts)
                }
            }
        }
    }
}

/// [`Types/Sprite`](https://lua-api.factorio.com/latest/types/Sprite.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite(SimpleGraphics<SpriteParams>);

impl RenderableGraphics for Sprite {
    type RenderOpts = SimpleGraphicsRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        self.0.render(factorio_dir, used_mods, opts)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParams {
    filename: FileName,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    direction_count: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u64",
        deserialize_with = "helper::truncating_deserializer"
    )]
    lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    counterclockwise: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    allow_low_quality_rotation: bool,

    #[serde(flatten)]
    pub sprite_params: SpriteParams,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedSpriteRenderOpts {
    pub orientation: f64,
    pub runtime_tint: Option<Color>,
}

fn direction_count_to_index(direction_count: u16, orientation: super::RealOrientation) -> u16 {
    (f64::from(direction_count) * orientation).round() as u16 % direction_count
}

impl RenderableGraphics for RotatedSpriteParams {
    type RenderOpts = RotatedSpriteRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        let mut index = direction_count_to_index(self.direction_count, opts.orientation);
        if self.counterclockwise {
            index = self.direction_count - index - 1;
        }

        // TODO: support `axially_symmetrical`, `back_equals_front` and `apply_projection` (and `allow_low_quality_rotation`?)

        let line_length = if self.line_length == 0 {
            self.direction_count
        } else {
            self.line_length as u16
        };

        let row = index / line_length;
        let column = index % line_length;

        self.sprite_params.fetch_offset(
            factorio_dir,
            &self.filename,
            used_mods,
            opts.runtime_tint,
            (column as i16, row as i16),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSpriteParamsMultiFile {
    filenames: Vec<FileName>,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    direction_count: u16,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    lines_per_file: u64,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    apply_projection: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    counterclockwise: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    line_length: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    allow_low_quality_rotation: bool,

    #[serde(flatten)]
    pub sprite_params: SpriteParams,
}

impl RenderableGraphics for RotatedSpriteParamsMultiFile {
    type RenderOpts = RotatedSpriteRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if self.lines_per_file == 0 {
            return None;
        }

        let mut index = direction_count_to_index(self.direction_count, opts.orientation);
        if self.counterclockwise {
            index = self.direction_count - index - 1;
        }

        // TODO: support `axially_symmetrical`, `back_equals_front` and `apply_projection` (and `allow_low_quality_rotation`?)

        let line_length = if self.line_length == 0 {
            self.direction_count
        } else {
            self.line_length as u16
        };

        let file_index = (index / line_length) / self.lines_per_file as u16;
        let row = (index / line_length) % self.lines_per_file as u16;
        let column = index % line_length;

        self.sprite_params.fetch_offset(
            factorio_dir,
            self.filenames.get(file_index as usize)?,
            used_mods,
            opts.runtime_tint,
            (column as i16, row as i16),
        )
    }
}

/// [`Types/RotatedSprite`](https://lua-api.factorio.com/latest/types/RotatedSprite.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSprite(MultiFileGraphics<RotatedSpriteParams, RotatedSpriteParamsMultiFile>);

impl RenderableGraphics for RotatedSprite {
    type RenderOpts = RotatedSpriteRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        self.0.render(factorio_dir, used_mods, opts)
    }
}

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
    pub sprite_params: SpriteParams,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: option to enable/disable HR mode
        #[allow(clippy::option_if_let_else)]
        if let Some(hr_version) = &self.hr_version {
            hr_version.render(factorio_dir, used_mods, opts)
        } else {
            let direction = match opts.direction {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 2,
                Direction::West => 3,
                _ => unreachable!("Sprite4WaySheet does not support diagonals"),
            } % self.frames;

            self.sprite_params.fetch_offset(
                factorio_dir,
                &self.filename,
                used_mods,
                opts.runtime_tint,
                (direction as i16, 0),
            )
        }
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
    pub sprite_params: SpriteParams,
}

impl RenderableGraphics for Sprite8WaySheet {
    type RenderOpts = SpriteNWayRenderOpts;

    fn render(
        &self,
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        // TODO: option to enable/disable HR mode
        #[allow(clippy::option_if_let_else)]
        if let Some(hr_version) = &self.hr_version {
            hr_version.render(factorio_dir, used_mods, opts)
        } else {
            let direction = opts.direction as u32 % self.frames;

            let (width, _) = self.sprite_params.get_size();
            self.sprite_params.fetch_offset(
                factorio_dir,
                &self.filename,
                used_mods,
                opts.runtime_tint,
                (direction as i16, 0),
            )
        }
    }
}

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite4Way {
    Sprite(Sprite),
    Sheets {
        sheets: Vec<Sprite4WaySheet>,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Sprite(sprite) => sprite.0.render(factorio_dir, used_mods, &opts.into()),
            Self::Sheet { sheet } => sheet.render(factorio_dir, used_mods, opts),
            Self::Sheets { sheets } => merge_layers(sheets, factorio_dir, used_mods, opts),
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
            .0
            .render(factorio_dir, used_mods, &opts.into()),
        }
    }
}

/// [`Types/Sprite8Way`](https://lua-api.factorio.com/latest/types/Sprite8Way.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sprite8Way {
    Sheets {
        sheets: Vec<Sprite8WaySheet>,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Sheets { sheets } => merge_layers(sheets, factorio_dir, used_mods, opts),
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
            }
            .0
            .render(factorio_dir, used_mods, &opts.into()),
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
    pub sprite_params: SpriteParams,
}

impl FetchSprite for SpriteSheetParams {
    fn fetch(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch(factorio_dir, filename, used_mods, runtime_tint)
    }

    fn fetch_offset(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch_offset(factorio_dir, filename, used_mods, runtime_tint, offset)
    }

    fn fetch_offset_by_pixels(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset_by_pixels(
            factorio_dir,
            filename,
            used_mods,
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

/// [`Types/SpriteSheet`](https://lua-api.factorio.com/latest/types/SpriteSheet.html)
pub type SpriteSheet = SimpleGraphics<SpriteSheetParams>;

/// [`Types/SpriteVariations`](https://lua-api.factorio.com/latest/types/SpriteVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteVariations {
    Struct { sheet: SpriteSheet },
    SpriteSheet(SpriteSheet),
    Array(Vec<Sprite>),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpriteVariationsRenderOpts {
    pub variation: u32,
    pub runtime_tint: Option<Color>,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Struct { sheet } | Self::SpriteSheet(sheet) => {
                sheet.render(factorio_dir, used_mods, &opts.into())
            }
            Self::Array(variations) => {
                variations
                    .get(0)?
                    .render(factorio_dir, used_mods, &opts.into())
            }
        }
    }
}

/// [`Types/WaterReflectionDefinition`](https://lua-api.factorio.com/latest/types/WaterReflectionDefinition.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct WaterReflectionDefinition {
    pub pictures: Option<SpriteVariations>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub orientation_to_variation: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub rotate: bool,
}

// ======================= //
// =======[ Tiles ]======= //
// ======================= //

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TileGraphics<T> {
    pub data: T,
    pub hr_version: Option<Box<Self>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSpriteParams {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub count: u32,

    pub picture: FileName,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub x: SpriteSizeType,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_i16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub y: SpriteSizeType,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub line_length: u32,
}

/// [`Types/TileSprite`](https://lua-api.factorio.com/latest/types/TileSprite.html)
pub type TileSprite = TileGraphics<TileSpriteParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSpriteProbabilityParams {
    #[serde(flatten)]
    pub tile_sprite_params: TileSpriteParams,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub size: u32,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<f64>,
}

/// [`Types/TileSpriteWithProbability`](https://lua-api.factorio.com/latest/types/TileSpriteWithProbability.html)
pub type TileSpriteWithProbability = TileGraphics<TileSpriteProbabilityParams>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TileTransitionSpriteParams {
    #[serde(flatten)]
    pub tile_sprite_params: TileSpriteParams,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub tall: bool,
}

/// [`Types/TileTransitionSprite`](https://lua-api.factorio.com/latest/types/TileTransitionSprite.html)
pub type TileTransitionSprite = TileGraphics<TileTransitionSpriteParams>;

// ======================== //
// =====[ Animations ]===== //
// ======================== //

// TODO: truncating deserializer for arrays....
/// [`Types/AnimationFrameSequence`](https://lua-api.factorio.com/latest/types/AnimationFrameSequence.html)
pub type AnimationFrameSequence = Vec<u16>;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub x: u32,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub y: u32,
}

impl Stripe {
    pub fn frame_count(&self) -> u32 {
        self.width_in_frames * self.height_in_frames.unwrap_or(1)
    }

    pub fn rotated_frame_count(&self, direction_count: u32) -> u32 {
        self.frame_count() * self.height_in_frames.unwrap_or(direction_count)
    }
}

/// [`Types/AnimationParameters`](https://lua-api.factorio.com/latest/types/AnimationParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationParams {
    #[serde(flatten)]
    pub sprite_params: SpriteParams,

    // TODO: skip searializing if default
    #[serde(default)]
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
    //     skip_serializing_if = "helper::is_0_u32",
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
}

impl FetchSprite for AnimationParams {
    fn fetch(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch(factorio_dir, filename, used_mods, runtime_tint)
    }

    fn fetch_offset(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params
            .fetch_offset(factorio_dir, filename, used_mods, runtime_tint, offset)
    }

    fn fetch_offset_by_pixels(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.sprite_params.fetch_offset_by_pixels(
            factorio_dir,
            filename,
            used_mods,
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

impl AnimationParams {
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
        layers: Vec<Self>,
    },
    Simple {
        filename: FileName,

        #[serde(flatten)]
        data: AnimationParams,

        hr_version: Option<Box<Self>>,
    },
    Striped {
        stripes: Vec<Stripe>,

        #[serde(flatten)]
        data: AnimationParams,

        hr_version: Option<Box<Self>>,
    },
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match &self {
            Self::Layered { layers } => merge_layers(layers, factorio_dir, used_mods, opts),
            Self::Striped {
                stripes,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
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
                            factorio_dir,
                            &stripe.filename,
                            used_mods,
                            opts.runtime_tint,
                            (column as i16, row as i16),
                        );
                    }

                    None
                }
            }
            Self::Simple {
                filename,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
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
                        factorio_dir,
                        filename,
                        used_mods,
                        opts.runtime_tint,
                        (column as i16, row as i16),
                    )
                }
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Animation(animation) => animation.render(factorio_dir, used_mods, &opts.into()),
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north,
                Direction::NorthEast => {
                    unimplemented!("Animation4Way does not support diagonals")
                }
                Direction::East => east.as_ref().unwrap_or(north),
                Direction::SouthEast => {
                    unimplemented!("Animation4Way does not support diagonals")
                }
                Direction::South => south.as_ref().unwrap_or(north),
                Direction::SouthWest => {
                    unimplemented!("Animation4Way does not support diagonals")
                }
                Direction::West => west.as_ref().unwrap_or(east.as_ref().unwrap_or(north)),
                Direction::NorthWest => {
                    unimplemented!("Animation4Way does not support diagonals")
                }
            }
            .render(factorio_dir, used_mods, &opts.into()),
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        if !self.draw_as_sprite || self.draw_as_light {
            return None;
        }

        self.animation
            .as_ref()
            .and_then(|animation| animation.render(factorio_dir, used_mods, opts))
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
    pub animation_params: AnimationParams,
}

/// [`Types/AnimationVariations`](https://lua-api.factorio.com/latest/types/AnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimationVariations {
    Animation(Animation),
    Array(Vec<Animation>),
    Sheet { sheet: AnimationSheet },
    Sheets { sheets: Vec<AnimationSheet> },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationVariationsRenderOpts {
    pub variation: u32,
    pub progress: f64,
    pub runtime_tint: Option<Color>,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Animation(animation) => animation.render(factorio_dir, used_mods, &opts.into()),
            Self::Array(animations) => animations.get(opts.variation as usize)?.render(
                factorio_dir,
                used_mods,
                &opts.into(),
            ),
            Self::Sheets { sheets } => todo!(), //merge_layers(sheets, factorio_dir, used_mods, opts),
            Self::Sheet { sheet } => todo!(),
        }
    }
}

/// [`Types/ShiftAnimationWaypoints`](https://lua-api.factorio.com/latest/types/ShiftAnimationWaypoints.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftAnimationWaypoints {
    pub north: Vec<Vector>,
    pub east: Vec<Vector>,
    pub south: Vec<Vector>,
    pub west: Vec<Vector>,
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
        skip_serializing_if = "helper::is_0_u32",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub still_frame: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub axially_symmetrical: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub counterclockwise: bool,

    #[serde(
        default = "helper::f64_half",
        skip_serializing_if = "helper::is_half_f64"
    )]
    pub middle_orientation: f64,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub orientation_range: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(flatten)]
    pub animation_params: AnimationParams,
}

impl FetchSprite for RotatedAnimationParams {
    fn fetch(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
    ) -> Option<GraphicsOutput> {
        self.animation_params
            .fetch(factorio_dir, filename, used_mods, runtime_tint)
    }

    fn fetch_offset(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.animation_params
            .fetch_offset(factorio_dir, filename, used_mods, runtime_tint, offset)
    }

    fn fetch_offset_by_pixels(
        &self,
        factorio_dir: &str,
        filename: &FileName,
        used_mods: &HashMap<&str, &str>,
        runtime_tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<GraphicsOutput> {
        self.animation_params.fetch_offset_by_pixels(
            factorio_dir,
            filename,
            used_mods,
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
    pub fn orientation_index(&self, orientation: f64) -> u32 {
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
        layers: Vec<Self>,
    },
    Single {
        filename: FileName,

        #[serde(flatten)]
        data: RotatedAnimationParams,

        hr_version: Option<Box<Self>>,
    },
    Multi {
        filenames: Vec<FileName>,

        #[serde(flatten)]
        data: RotatedAnimationParams,

        hr_version: Option<Box<Self>>,
    },
    Striped {
        stripes: Vec<Stripe>,

        #[serde(flatten)]
        data: RotatedAnimationParams,

        hr_version: Option<Box<Self>>,
    },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedAnimationRenderOpts {
    pub orientation: f64,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::Layered { layers } => merge_layers(layers, factorio_dir, used_mods, opts),
            Self::Striped {
                stripes,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    // TODO: support stripes
                    None
                }
            }
            Self::Multi {
                filenames,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    let orientation_index = match opts.override_index {
                        Some(index) => u32::from(index),
                        None => data.orientation_index(opts.orientation),
                    };
                    let file_index = orientation_index / data.lines_per_file.unwrap_or(1);
                    let frame_index = data.animation_params.frame_index(opts.progress);
                    let line_length = data.animation_params.line_length();

                    let column = frame_index % line_length;
                    let row = frame_index / line_length;

                    data.fetch_offset(
                        factorio_dir,
                        filenames.get(file_index as usize)?,
                        used_mods,
                        opts.runtime_tint,
                        (column as i16, (row + orientation_index) as i16),
                    )
                }
            }
            Self::Single {
                filename,
                data,
                hr_version,
            } => {
                // TODO: option to enable/disable HR mode
                #[allow(clippy::option_if_let_else)]
                if let Some(hr_version) = hr_version {
                    hr_version.render(factorio_dir, used_mods, opts)
                } else {
                    let orientation_index = match opts.override_index {
                        Some(index) => u32::from(index),
                        None => data.orientation_index(opts.orientation),
                    };
                    let frame_index = data.animation_params.frame_index(opts.progress);
                    let line_length = data.animation_params.line_length();

                    let column = frame_index % line_length;
                    let row = frame_index / line_length;

                    data.fetch_offset(
                        factorio_dir,
                        filename,
                        used_mods,
                        opts.runtime_tint,
                        (column as i16, (row + orientation_index) as i16),
                    )
                }
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
    pub orientation: f64,
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
        factorio_dir: &str,
        used_mods: &HashMap<&str, &str>,
        opts: &Self::RenderOpts,
    ) -> Option<GraphicsOutput> {
        match self {
            Self::RotatedAnimation(animation) => {
                animation.render(factorio_dir, used_mods, &opts.into())
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
            .render(factorio_dir, used_mods, &opts.into()),
        }
    }
}

/// [`Types/RotatedAnimationVariations`](https://lua-api.factorio.com/latest/types/RotatedAnimationVariations.html)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimationVariations {
    Animation(RotatedAnimation),
    Array(Vec<RotatedAnimation>),
}

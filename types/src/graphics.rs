use image::DynamicImage;
use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use crate::{FactorioArray, FileName, ImageCache, Vector};

mod animation;
mod beacon;
mod rotated;
mod sheet;
mod sprite;
mod tile;
mod transport_belt;
mod util;
mod variations;
mod working_visualisation;

pub use animation::*;
pub use beacon::*;
pub use rotated::*;
pub use sheet::*;
pub use sprite::*;
pub use tile::*;
pub use transport_belt::*;
pub use util::*;
pub use variations::*;
pub use working_visualisation::*;

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl RenderLayer {
    #[must_use]
    pub const fn object() -> Self {
        Self::Object
    }

    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object)
    }
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

pub trait SourceProvider {
    type FetchArgs;

    fn fetch(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        position: (SpriteSizeType, SpriteSizeType),
        size: (SpriteSizeType, SpriteSizeType),
        fetch_args: Self::FetchArgs,
    ) -> Option<DynamicImage>;
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSource {
    filename: FileName,
}

impl SourceProvider for SingleSource {
    type FetchArgs = ();

    fn fetch(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        (x, y): (SpriteSizeType, SpriteSizeType),
        (width, height): (SpriteSizeType, SpriteSizeType),
        (): Self::FetchArgs,
    ) -> Option<DynamicImage> {
        let img = self.filename.load(used_mods, image_cache)?.crop_imm(
            x as u32,
            y as u32,
            width as u32,
            height as u32,
        );
        Some(img)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiSingleSource {
    Multi {
        filenames: FactorioArray<FileName>,
        /// technically [`RotatedSprite`] requires this to be u64 but
        /// 2^32 lines in a single file (with a max size of 8k) is nonsense
        lines_per_file: u32,
    },
    Single(SingleSource),
}

#[derive(Debug, Clone, Copy)]
pub struct MultiSingleSourceFetchArgs {
    pub index: u32,
    pub line_length: u32,
}

impl MultiSingleSource {
    fn multi_source(
        filenames: &[FileName],
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        (x, y): (SpriteSizeType, SpriteSizeType),
        (width, height): (SpriteSizeType, SpriteSizeType),
        lines_per_file: u32,
        fetch_args: MultiSingleSourceFetchArgs,
    ) -> Option<DynamicImage> {
        let line_idx = fetch_args.index / fetch_args.line_length;
        let col_idx = fetch_args.index % fetch_args.line_length;
        let row_idx = line_idx % lines_per_file;
        let file_idx = line_idx / lines_per_file;

        let width = width as u32;
        let height = height as u32;
        let x = x as u32 + col_idx * width;
        let y = y as u32 + row_idx * height;

        let img = filenames
            .get(file_idx as usize)?
            .load(used_mods, image_cache)?
            .crop_imm(x, y, width, height);

        Some(img)
    }
}

impl SourceProvider for MultiSingleSource {
    type FetchArgs = MultiSingleSourceFetchArgs;

    fn fetch(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        position: (SpriteSizeType, SpriteSizeType),
        size: (SpriteSizeType, SpriteSizeType),
        fetch_args: Self::FetchArgs,
    ) -> Option<DynamicImage> {
        match self {
            Self::Multi {
                filenames,
                lines_per_file,
            } => Self::multi_source(
                filenames,
                used_mods,
                image_cache,
                position,
                size,
                *lines_per_file,
                fetch_args,
            ),
            Self::Single(s) => s.fetch(used_mods, image_cache, position, size, ()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Stripe {
    pub width_in_frames: u32,
    pub height_in_frames: Option<u32>,

    pub filename: FileName,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub x: u32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum StripeMultiSingleSource {
    Stripe {
        stripes: FactorioArray<Stripe>,
    },
    Multi {
        filenames: FactorioArray<FileName>,
        slice: Option<u32>,
        /// only [`RotatedSprite`] requires this to be u64, otherwise it's u32
        lines_per_file: u32,
    },
    Single(SingleSource),
}

#[derive(Debug, Clone, Copy)]
pub struct StripeMultiSingleSourceFetchArgs {
    pub index: u32,
    pub line_length: u32,

    /// [`Stripe::height_in_frames`] is optional when [`StripeMultiSingleSource`] is used
    /// in [`RotatedAnimation`] which then defaults to [`RotatedAnimationData::direction_count`].
    pub direction_count: Option<u32>,
}

impl SourceProvider for StripeMultiSingleSource {
    type FetchArgs = StripeMultiSingleSourceFetchArgs;

    fn fetch(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        position: (SpriteSizeType, SpriteSizeType),
        size: (SpriteSizeType, SpriteSizeType),
        fetch_args: Self::FetchArgs,
    ) -> Option<DynamicImage> {
        match self {
            Self::Stripe { stripes } => {
                let mut target_idx = fetch_args.index;
                for stripe in stripes {
                    let Some(stripe_height) =
                        stripe.height_in_frames.or(fetch_args.direction_count)
                    else {
                        continue;
                    };

                    let stripe_width = stripe.width_in_frames;
                    let stripe_frames = stripe_width * stripe_height;

                    if target_idx >= stripe_frames {
                        target_idx -= stripe_frames;
                        continue;
                    }

                    let stripe_col = target_idx % stripe_width;
                    let stripe_row = target_idx / stripe_width;
                    let width = size.0 as u32;
                    let height = size.1 as u32;
                    // unsure if adding position x/y to stripe x/y is correct
                    let x = position.0 as u32 + stripe.x + stripe_col * width;
                    let y = position.1 as u32 + stripe.y + stripe_row * height;

                    let img = stripe
                        .filename
                        .load(used_mods, image_cache)?
                        .crop_imm(x, y, width, height);

                    return Some(img);
                }

                None
            }
            Self::Multi {
                filenames,
                slice,
                lines_per_file,
            } => MultiSingleSource::multi_source(
                filenames,
                used_mods,
                image_cache,
                position,
                size,
                *lines_per_file,
                MultiSingleSourceFetchArgs {
                    index: fetch_args.index,
                    line_length: fetch_args.line_length,
                },
            ),
            Self::Single(s) => s.fetch(used_mods, image_cache, position, size, ()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayeredGraphic<G: RenderableGraphics> {
    Layered { layers: FactorioArray<Self> },
    Data(G),
}

impl<G, O> RenderableGraphics for LayeredGraphic<G>
where
    G: RenderableGraphics<RenderOpts = O>,
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
            Self::Data(g) => g.render(scale, used_mods, image_cache, opts),
        }
    }
}

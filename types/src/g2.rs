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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

pub trait SourceProvider {}

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

impl SourceProvider for SingleSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiSingleSource {
    Multi {
        filenames: FactorioArray<FileName>,
        lines_per_file: u32,
    },
    Single(SingleSource),
}

impl SourceProvider for MultiSingleSource {}

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
        lines_per_file: u32,
    },
    Single(SingleSource),
}

impl SourceProvider for StripeMultiSingleSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayeredGraphic<G: RenderableGraphics> {
    Layered { layers: FactorioArray<G> },
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
            Self::Layered { layers } => todo!(),
            Self::Data(g) => g.render(scale, used_mods, image_cache, opts),
        }
    }
}

use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::SpriteSizeType;
use crate::{FactorioArray, FileName};

/// [`Types/TileRenderLayer`](https://lua-api.factorio.com/latest/types/TileRenderLayer.html)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum TileRenderLayer {
    Zero,
    Water,
    WaterOverlay,
    GroundNatural,
    GroundArtificial,
    Top,
}

/// [`Types/TileTransitionsVariants`](https://lua-api.factorio.com/latest/types/TileTransitionsVariants.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct TileTransitionsVariants {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main: FactorioArray<TileMainPictures>,

    #[serde(default = "helper::u8_8", skip_serializing_if = "helper::is_8_u8")]
    pub material_texture_width_in_tiles: u8,
    #[serde(default = "helper::u8_8", skip_serializing_if = "helper::is_8_u8")]
    pub material_texture_height_in_tiles: u8,
    pub material_background: Option<MaterialTextureParameters>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub light: FactorioArray<TileLightPictures>,
    pub material_light: Option<MaterialTextureParameters>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub empty_transitions: bool,
    pub transition: Option<TileTransitions>,
}

/// [`Types/TileSpriteLayout`](https://lua-api.factorio.com/latest/types/TileSpriteLayout.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileSpriteLayout {
    pub picture: FileName,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub scale: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub x: SpriteSizeType,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub y: SpriteSizeType,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub line_length: u8,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub count: u8,
}

/// [`Types/TileMainPictures`](https://lua-api.factorio.com/latest/types/TileMainPictures.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMainPictures {
    pub size: u32,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub probability: f64,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: FactorioArray<f64>,

    #[serde(flatten)]
    parent: TileSpriteLayout,
}

impl std::ops::Deref for TileMainPictures {
    type Target = TileSpriteLayout;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Types/TileLightPictures`](https://lua-api.factorio.com/latest/types/TileLightPictures.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileLightPictures {
    pub size: u32,

    #[serde(flatten)]
    parent: TileSpriteLayout,
}

impl std::ops::Deref for TileLightPictures {
    type Target = TileSpriteLayout;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Types/MaterialTextureParameters`](https://lua-api.factorio.com/latest/types/MaterialTextureParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTextureParameters {
    pub count: u32,
    pub picture: FileName,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub scale: f32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub x: SpriteSizeType,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub y: SpriteSizeType,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub line_length: u32,
}

/// [`Types/TileTransitions`](https://lua-api.factorio.com/latest/types/TileTransitions.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileTransitions {
    // TODO: lots of fun properties
}

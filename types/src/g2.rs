use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use crate::{
    BlendMode, Color, FileName, SpriteFlags, SpritePriority, SpriteSizeParam, SpriteSizeType,
    Vector,
};

mod sheet;

/// [`Types/SpriteSource`](https://lua-api.factorio.com/latest/types/SpriteSource.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct SpriteSource {
    pub filename: FileName,

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

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub load_in_minimal_mode: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub premul_alpha: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_forced_downscale: bool,
}

/// [`Types/SpriteParameters`](https://lua-api.factorio.com/latest/types/SpriteParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct SpriteParameters {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub priority: SpritePriority,

    pub flags: Option<SpriteFlags>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub shift: Vector,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub rotate_shift: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_special_effect: bool,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub scale: f64,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_as_shadow: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_as_glow: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_as_light: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub mipmap_count: u8,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_runtime_tint: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub tint_as_overlay: bool,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub invert_colors: bool,

    #[serde(default = "Color::white", skip_serializing_if = "Color::is_white")]
    pub tint: Color,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub blend_mode: BlendMode,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub generate_sdf: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub surface: SpriteUsageSurfaceHint,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub usage: SpriteUsageHint,

    #[serde(flatten)]
    parent: SpriteSource,
}

impl std::ops::Deref for SpriteParameters {
    type Target = SpriteSource;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Types/SpriteUsageSurfaceHint`](https://lua-api.factorio.com/latest/types/SpriteUsageSurfaceHint.html)
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteUsageSurfaceHint {
    #[default]
    Any,
    Nauvis,
    Vulcanus,
    Gleba,
    Fulgora,
    Aquilo,
    Space,
}

/// [`Types/SpriteUsageHint`](https://lua-api.factorio.com/latest/types/SpriteUsageHint.html)
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpriteUsageHint {
    #[default]
    Any,
    Mining,
    TileArtifical,
    CorpseDecay,
    Enemy,
    Player,
    Train,
    Vehicle,
    Explosion,
    Rail,
    ElevatedRail,
    Air,
    Remnant,
    Decorative,
}

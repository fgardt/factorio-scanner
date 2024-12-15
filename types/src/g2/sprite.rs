use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    BlendMode, LayeredGraphic, RenderLayer, RenderableGraphics, SingleSource, SourceProvider,
    SpriteFlags, SpritePriority, SpriteSizeParam, SpriteSizeType,
};
use crate::{Color, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct TintableRenderOpts {
    pub runtime_tint: Option<Color>,
}

/// [`Types/SpriteSource`](https://lua-api.factorio.com/latest/types/SpriteSource.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct SpriteSource<S: SourceProvider = SingleSource> {
    #[serde(flatten)]
    pub source: S,

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

/// [`Types/EffectTexture`](https://lua-api.factorio.com/latest/types/EffectTexture.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectTexture(SpriteSource);

impl RenderableGraphics for EffectTexture {
    type RenderOpts = ();

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        todo!()
    }
}

/// [`Types/SpriteParameters`](https://lua-api.factorio.com/latest/types/SpriteParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct SpriteParameters<S: SourceProvider = SingleSource> {
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
    parent: SpriteSource<S>,
}

impl<S: SourceProvider> std::ops::Deref for SpriteParameters<S> {
    type Target = SpriteSource<S>;

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

/// [`Types/Sprite`](https://lua-api.factorio.com/latest/types/Sprite.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteData {
    // technically there are `dice` fields here but we'll ignore them
    #[serde(flatten)]
    parent: SpriteParameters,
}

impl std::ops::Deref for SpriteData {
    type Target = SpriteParameters;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for SpriteData {
    type RenderOpts = TintableRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        todo!()
    }
}

/// [`Types/Sprite`](https://lua-api.factorio.com/latest/types/Sprite.html)
pub type Sprite = LayeredGraphic<SpriteData>;

/// [`Types/LayeredSprite`](https://lua-api.factorio.com/latest/types/LayeredSprite.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredSpriteData {
    pub render_layer: RenderLayer,

    // technically there are `dice` fields here but we'll ignore them
    #[serde(flatten)]
    parent: SpriteParameters,
}

impl std::ops::Deref for LayeredSpriteData {
    type Target = SpriteParameters;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for LayeredSpriteData {
    type RenderOpts = TintableRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        todo!()
    }
}

/// [`Types/LayeredSprite`](https://lua-api.factorio.com/latest/types/LayeredSprite.html)
pub type LayeredSprite = LayeredGraphic<LayeredSpriteData>;

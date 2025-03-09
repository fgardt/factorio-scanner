use image::Rgba;
use mod_util::UsedMods;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    merge_layers, BlendMode, LayeredGraphic, RenderLayer, RenderableGraphics, SingleSource,
    SourceProvider, SpriteFlags, SpritePriority, SpriteSizeParam, SpriteSizeType,
};
use crate::{Color, ImageCache, SingleOrArray, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct TintableRenderOpts {
    pub runtime_tint: Option<Color>,
}

impl From<Color> for TintableRenderOpts {
    fn from(color: Color) -> Self {
        Self {
            runtime_tint: Some(color),
        }
    }
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

impl<S: SourceProvider> SpriteSource<S> {
    pub const fn get_position(&self) -> (SpriteSizeType, SpriteSizeType) {
        if self.x == 0 && self.y == 0 {
            match self.position {
                Some((x, y)) => (x, y),
                None => (0, 0),
            }
        } else {
            (self.x, self.y)
        }
    }

    pub const fn get_size(&self) -> (SpriteSizeType, SpriteSizeType) {
        match self.size {
            SpriteSizeParam::Size { size } => (size, size),
            SpriteSizeParam::Size2 { size } => size,
            SpriteSizeParam::Explicit { width, height } => (width, height),
        }
    }

    pub fn fetch(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        fetch_args: S::FetchArgs,
    ) -> Option<image::DynamicImage> {
        self.source.fetch(
            used_mods,
            image_cache,
            self.get_position(),
            self.get_size(),
            fetch_args,
        )
    }

    pub fn fetch_offset(
        &self,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        fetch_args: S::FetchArgs,
        (offset_x, offset_y): (i16, i16),
    ) -> Option<image::DynamicImage> {
        let (x, y) = self.get_position();
        let (width, height) = self.get_size();
        self.source.fetch(
            used_mods,
            image_cache,
            (x + offset_x * width, y + offset_y * width),
            (width, height),
            fetch_args,
        )
    }
}

/// [`Types/EffectTexture`](https://lua-api.factorio.com/latest/types/EffectTexture.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectTexture(SpriteSource);

impl std::ops::Deref for EffectTexture {
    type Target = SpriteSource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderableGraphics for EffectTexture {
    type RenderOpts = ();

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        (): &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        Some((self.fetch(used_mods, image_cache, ())?, (Vector::default())))
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

impl<S: SourceProvider> SpriteParameters<S> {
    pub fn fetch_scale_tint(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        fetch_args: S::FetchArgs,
        tint: Option<Color>,
    ) -> Option<super::GraphicsOutput> {
        self.fetch_offset_scale_tint(scale, used_mods, image_cache, fetch_args, tint, (0, 0))
    }

    pub fn fetch_offset_scale_tint(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        fetch_args: S::FetchArgs,
        tint: Option<Color>,
        offset: (i16, i16),
    ) -> Option<super::GraphicsOutput> {
        if self.draw_as_shadow || self.draw_as_glow || self.draw_as_light {
            return None;
        }

        let img = self.fetch_offset(used_mods, image_cache, fetch_args, offset)?;

        let mut img = if (scale - self.scale).abs() < f64::EPSILON {
            img
        } else {
            let scalar = self.scale / scale;
            img.resize(
                (f64::from(img.width()) * scalar).round() as u32,
                (f64::from(img.height()) * scalar).round() as u32,
                image::imageops::FilterType::Nearest,
            )
        };

        let tint = if self.apply_runtime_tint {
            tint.unwrap_or(self.tint)
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

        Some((img, self.shift))
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
        self.fetch_scale_tint(scale, used_mods, image_cache, (), opts.runtime_tint)
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
    parent: Sprite,
}

impl std::ops::Deref for LayeredSpriteData {
    type Target = Sprite;

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
        // TODO: respect the render_layer property, requires major refactor
        self.parent.render(scale, used_mods, image_cache, opts)
    }
}

/// [`Types/LayeredSprite`](https://lua-api.factorio.com/latest/types/LayeredSprite.html)
pub type LayeredSprite = SingleOrArray<LayeredSpriteData>;

impl RenderableGraphics for SingleOrArray<LayeredSpriteData> {
    type RenderOpts = TintableRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &UsedMods,
        image_cache: &mut ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::Single(g) => g.render(scale, used_mods, image_cache, opts),
            Self::Array(layers) => merge_layers(layers, scale, used_mods, image_cache, opts),
        }
    }
}

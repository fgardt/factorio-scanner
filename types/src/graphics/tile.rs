use image::Rgba;
use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{RenderableGraphics, SpriteSizeType, TintableRenderOpts};
use crate::{Color, FactorioArray, FileName, MapPosition, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct LocationalRenderOpts<M = TintableRenderOpts> {
    pub position: MapPosition,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for LocationalRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

impl<M> std::ops::DerefMut for LocationalRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M> LocationalRenderOpts<M> {
    pub const fn new(position: MapPosition, more: M) -> Self {
        Self { position, more }
    }

    pub const fn get_offset(&self) -> (i32, i32) {
        let (x, y) = self.position.as_tuple();
        (x.ceil() as i32, y.ceil() as i32)
    }
}

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

    // technically line_length and count are u8 but we'll use u32
    // to reuse this struct for MaterialTextureParameters
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub line_length: u32,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub count: u32,
}

impl TileSpriteLayout {
    #[allow(clippy::cast_possible_wrap)]
    fn fetch_offset_scale_size_tint(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        size: u32,
        tint: Option<Color>,
        offset: (i32, i32),
    ) -> Option<image::DynamicImage> {
        let raw_size = (32.0 / self.scale).round() as u32 * size;
        let cols = if self.line_length == 0 {
            self.count
        } else {
            self.line_length
        };
        let rows = (f64::from(self.count) / f64::from(cols)).ceil() as u32;
        let (mut x_offset, mut y_offset) = offset;
        x_offset %= cols as i32;
        y_offset %= rows as i32;
        if x_offset < 0 {
            x_offset += cols as i32;
        }
        if y_offset < 0 {
            y_offset += rows as i32;
        }

        let img = self.picture.load(used_mods, image_cache)?.crop_imm(
            self.x as u32 + x_offset as u32 * raw_size,
            self.y as u32 + y_offset as u32 * raw_size,
            raw_size,
            raw_size,
        );

        let mut img = if (scale - f64::from(self.scale)).abs() < f64::EPSILON {
            img
        } else {
            let scalar = f64::from(self.scale) / scale;
            img.resize(
                (f64::from(img.width()) * scalar).round() as u32,
                (f64::from(img.height()) * scalar).round() as u32,
                image::imageops::FilterType::Nearest,
            )
        };

        let tint = tint.unwrap_or_else(Color::white);

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

        Some(img)
    }
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

impl RenderableGraphics for TileMainPictures {
    type RenderOpts = LocationalRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        Some((
            self.fetch_offset_scale_size_tint(
                scale,
                used_mods,
                image_cache,
                self.size,
                opts.runtime_tint,
                opts.get_offset(),
            )?,
            Vector::default(),
        ))
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

impl RenderableGraphics for TileLightPictures {
    type RenderOpts = LocationalRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        Some((
            self.fetch_offset_scale_size_tint(
                scale,
                used_mods,
                image_cache,
                self.size,
                opts.runtime_tint,
                opts.get_offset(),
            )?,
            Vector::default(),
        ))
    }
}

/// [`Types/MaterialTextureParameters`](https://lua-api.factorio.com/latest/types/MaterialTextureParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTextureParameters {
    #[serde(flatten)]
    data: TileSpriteLayout,
}

impl std::ops::Deref for MaterialTextureParameters {
    type Target = TileSpriteLayout;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl RenderableGraphics for MaterialTextureParameters {
    type RenderOpts = LocationalRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        Some((
            self.fetch_offset_scale_size_tint(
                scale,
                used_mods,
                image_cache,
                1,
                opts.runtime_tint,
                opts.get_offset(),
            )?,
            Vector::default(),
        ))
    }
}

/// [`Types/TileTransitions`](https://lua-api.factorio.com/latest/types/TileTransitions.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileTransitions {
    // TODO: lots of fun properties
}

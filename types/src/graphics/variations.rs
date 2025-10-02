use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use super::{
    Animation, AnimationRenderOpts, AnimationSheet, LayeredSprite, RenderableGraphics,
    RotatedAnimation, RotatedRenderOpts, Sprite, SpriteSheet, TintableRenderOpts, merge_layers,
};
use crate::FactorioArray;

#[derive(Debug, Clone, Copy)]
pub struct VariationRenderOpts<M = TintableRenderOpts> {
    pub variation: NonZeroU32,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for VariationRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

impl<M> std::ops::DerefMut for VariationRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M: Default> Default for VariationRenderOpts<M> {
    #[expect(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            variation: NonZeroU32::new(1).unwrap(),
            more: M::default(),
        }
    }
}

impl<M> VariationRenderOpts<M> {
    pub const fn new(variation: NonZeroU32, more: M) -> Self {
        Self { variation, more }
    }
}

/// [`Types/SpriteVariations`](https://lua-api.factorio.com/latest/types/SpriteVariations.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpriteVariations {
    Struct { sheet: SpriteSheet },
    Sheet(SpriteSheet),
    Array(FactorioArray<Sprite>),
}

impl RenderableGraphics for SpriteVariations {
    type RenderOpts = VariationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::Struct { sheet } | Self::Sheet(sheet) => {
                sheet.render(scale, used_mods, image_cache, opts)
            }
            Self::Array(sprites) => sprites.get(opts.variation.get() as usize - 1)?.render(
                scale,
                used_mods,
                image_cache,
                opts,
            ),
        }
    }
}

/// [`Types/LayeredSpriteVariations`](https://lua-api.factorio.com/latest/types/LayeredSpriteVariations.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredSpriteVariations(FactorioArray<LayeredSprite>);

impl std::ops::Deref for LayeredSpriteVariations {
    type Target = FactorioArray<LayeredSprite>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderableGraphics for LayeredSpriteVariations {
    type RenderOpts = VariationRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        self.get(opts.variation.get() as usize - 1)?
            .render(scale, used_mods, image_cache, opts)
    }
}

/// [`Types/AnimationVariations`](https://lua-api.factorio.com/latest/types/AnimationVariations.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnimationVariations {
    SingleSheet {
        sheet: AnimationSheet,
    },
    MultiSheets {
        sheets: FactorioArray<AnimationSheet>,
    },
    Animation(Animation),
    Array(FactorioArray<Animation>),
}

impl RenderableGraphics for AnimationVariations {
    type RenderOpts = VariationRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::SingleSheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::MultiSheets { sheets } => {
                merge_layers(sheets, scale, used_mods, image_cache, opts)
            }
            Self::Animation(anim) => anim.render(scale, used_mods, image_cache, opts),
            Self::Array(arr) => arr.get(opts.variation.get() as usize - 1)?.render(
                scale,
                used_mods,
                image_cache,
                opts,
            ),
        }
    }
}

/// [`Types/RotatedAnimationVariations`](https://lua-api.factorio.com/latest/types/RotatedAnimationVariations.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotatedAnimationVariations {
    Single(RotatedAnimation),
    Array(FactorioArray<RotatedAnimation>),
}

impl RenderableGraphics for RotatedAnimationVariations {
    type RenderOpts = VariationRenderOpts<RotatedRenderOpts<AnimationRenderOpts>>;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::Single(rot_anim) => rot_anim.render(scale, used_mods, image_cache, opts),
            Self::Array(arr) => arr.get(opts.variation.get() as usize - 1)?.render(
                scale,
                used_mods,
                image_cache,
                opts,
            ),
        }
    }
}

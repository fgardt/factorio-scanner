use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    AnimationParameters, AnimationRenderOpts, LayeredGraphic, MultiSingleSource,
    RenderableGraphics, Sprite, SpriteParameters, TintableRenderOpts, VariationRenderOpts,
};
use crate::{Direction, FactorioArray};

#[derive(Debug, Clone, Copy, Default)]
pub struct DirectionalRenderOpts<M = TintableRenderOpts> {
    pub direction: Direction,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for DirectionalRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

/// [`Types/SpriteSheet`](https://lua-api.factorio.com/latest/types/SpriteSheet.html)
pub type SpriteSheet = LayeredGraphic<SpriteSheetData>;

/// [`Types/SpriteSheet`](https://lua-api.factorio.com/latest/types/SpriteSheet.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct SpriteSheetData {
    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub variation_count: u32,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub repeat_count: u32,

    // defaults to variation_count
    pub line_length: Option<u32>,

    // pub dice: Option<SpriteSizeType>,
    // pub dice_x: Option<SpriteSizeType>,
    // pub dice_y: Option<SpriteSizeType>,
    #[serde(flatten)]
    parent: Box<SpriteParameters<MultiSingleSource>>,
}

impl std::ops::Deref for SpriteSheetData {
    type Target = SpriteParameters<MultiSingleSource>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for SpriteSheetData {
    type RenderOpts = VariationRenderOpts;

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

/// [`Types/SpriteNWaySheet`](https://lua-api.factorio.com/latest/types/SpriteNWaySheet.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteNWaySheet<const N: u32> {
    #[serde(
        default = "SpriteNWaySheet::<N>::f_count",
        skip_serializing_if = "SpriteNWaySheet::<N>::is_f_count"
    )]
    pub frames: u32,

    #[serde(flatten)]
    parent: Box<SpriteParameters>,
}

impl<const N: u32> std::ops::Deref for SpriteNWaySheet<N> {
    type Target = SpriteParameters;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl<const N: u32> SpriteNWaySheet<N> {
    const fn f_count() -> u32 {
        N
    }

    #[expect(clippy::trivially_copy_pass_by_ref)]
    const fn is_f_count(val: &u32) -> bool {
        *val == N
    }
}

impl<const N: u32> RenderableGraphics for SpriteNWaySheet<N> {
    type RenderOpts = DirectionalRenderOpts;

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

/// [`Types/Sprite4Way`](https://lua-api.factorio.com/latest/types/Sprite4Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite4Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<4>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<4>,
    },
    Struct {
        north: Box<Sprite>,
        east: Box<Sprite>,
        south: Box<Sprite>,
        west: Box<Sprite>,
    },
    Single(Sprite),
}

impl RenderableGraphics for Sprite4Way {
    type RenderOpts = DirectionalRenderOpts;

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

/// [`Types/Sprite16Way`](https://lua-api.factorio.com/latest/types/Sprite16Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum Sprite16Way {
    Sheets {
        sheets: FactorioArray<SpriteNWaySheet<16>>,
    },
    Sheet {
        sheet: SpriteNWaySheet<16>,
    },
    Struct {
        north: Box<Sprite>,
        north_north_east: Box<Sprite>,
        north_east: Box<Sprite>,
        east_north_east: Box<Sprite>,
        east: Box<Sprite>,
        east_south_east: Box<Sprite>,
        south_east: Box<Sprite>,
        south_south_east: Box<Sprite>,
        south: Box<Sprite>,
        south_south_west: Box<Sprite>,
        south_west: Box<Sprite>,
        west_south_west: Box<Sprite>,
        west: Box<Sprite>,
        west_north_west: Box<Sprite>,
        north_west: Box<Sprite>,
        north_north_west: Box<Sprite>,
    },
}

impl RenderableGraphics for Sprite16Way {
    type RenderOpts = DirectionalRenderOpts;

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

/// [`Types/AnimationSheet`](https://lua-api.factorio.com/latest/types/AnimationSheet.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSheet {
    pub variation_count: u32,

    #[serde(flatten)]
    parent: AnimationParameters<MultiSingleSource>,
}

impl std::ops::Deref for AnimationSheet {
    type Target = AnimationParameters<MultiSingleSource>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for AnimationSheet {
    type RenderOpts = VariationRenderOpts<AnimationRenderOpts>;

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

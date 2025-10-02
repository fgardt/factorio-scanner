use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;
use tracing::warn;

use super::{
    AnimationParameters, AnimationRenderOpts, LayeredGraphic, MultiSingleSource,
    MultiSingleSourceFetchArgs, RenderableGraphics, Sprite, SpriteParameters, TintableRenderOpts,
    VariationRenderOpts, merge_layers,
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

impl<M> std::ops::DerefMut for DirectionalRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M> DirectionalRenderOpts<M> {
    pub const fn new(direction: Direction, more: M) -> Self {
        Self { direction, more }
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
        self.fetch_scale_tint(
            scale,
            used_mods,
            image_cache,
            MultiSingleSourceFetchArgs {
                index: u32::from(opts.variation) % self.variation_count,
                line_length: self.line_length.unwrap_or(self.variation_count),
            },
            opts.runtime_tint,
        )
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
        let idx = match N {
            4 => match opts.direction {
                Direction::North => 0,
                Direction::East => 1,
                Direction::South => 2,
                Direction::West => 3,
                _ => {
                    warn!(
                        "Sprite4WaySheet render called with invalid direction {:?}",
                        opts.direction
                    );
                    return None;
                }
            },
            16 => opts.direction as u32,
            _ => {
                warn!("SpriteNWaySheet render called with invalid N: {N}");
                return None;
            }
        } % self.frames;

        self.fetch_offset_scale_tint(
            scale,
            used_mods,
            image_cache,
            (),
            opts.runtime_tint,
            (idx as i16, 0),
        )
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
        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Struct {
                north,
                east,
                south,
                west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::East => east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::West => west.as_ref(),
                _ => {
                    warn!(
                        "Sprite4Way render called with invalid direction {:?}",
                        opts.direction
                    );
                    return None;
                }
            }
            .render(scale, used_mods, image_cache, opts),
            Self::Single(sprite) => sprite.render(scale, used_mods, image_cache, opts),
        }
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
        match self {
            Self::Sheets { sheets } => merge_layers(sheets, scale, used_mods, image_cache, opts),
            Self::Sheet { sheet } => sheet.render(scale, used_mods, image_cache, opts),
            Self::Struct {
                north,
                north_north_east,
                north_east,
                east_north_east,
                east,
                east_south_east,
                south_east,
                south_south_east,
                south,
                south_south_west,
                south_west,
                west_south_west,
                west,
                west_north_west,
                north_west,
                north_north_west,
            } => match opts.direction {
                Direction::North => north.as_ref(),
                Direction::NorthNorthEast => north_north_east.as_ref(),
                Direction::NorthEast => north_east.as_ref(),
                Direction::EastNorthEast => east_north_east.as_ref(),
                Direction::East => east.as_ref(),
                Direction::EastSouthEast => east_south_east.as_ref(),
                Direction::SouthEast => south_east.as_ref(),
                Direction::SouthSouthEast => south_south_east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::SouthSouthWest => south_south_west.as_ref(),
                Direction::SouthWest => south_west.as_ref(),
                Direction::WestSouthWest => west_south_west.as_ref(),
                Direction::West => west.as_ref(),
                Direction::WestNorthWest => west_north_west.as_ref(),
                Direction::NorthWest => north_west.as_ref(),
                Direction::NorthNorthWest => north_north_west.as_ref(),
            }
            .render(scale, used_mods, image_cache, opts),
        }
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
        self.fetch_scale_tint(
            scale,
            used_mods,
            image_cache,
            MultiSingleSourceFetchArgs {
                index: self.frame_count * opts.variation.get() + self.anim_idx(opts.progress),
                line_length: self.line_length.unwrap_or(self.variation_count),
            },
            opts.runtime_tint,
        )
    }
}

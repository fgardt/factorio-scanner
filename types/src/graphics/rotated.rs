use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;
use tracing::warn;

use super::{
    AnimationParameters, AnimationRenderOpts, DirectionalRenderOpts, LayeredGraphic,
    MultiSingleSource, MultiSingleSourceFetchArgs, RenderableGraphics, SpriteParameters,
    SpriteSizeType, TintableRenderOpts,
};
use crate::{Direction, FactorioArray, RealOrientation, StripeMultiSingleSourceFetchArgs, Vector};

/// [`Types/RotatedAnimation`](https://lua-api.factorio.com/latest/types/RotatedAnimation.html)
pub type RotatedAnimation = LayeredGraphic<RotatedAnimationData>;

/// [`Types/RotatedAnimation`](https://lua-api.factorio.com/latest/types/RotatedAnimation.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatedAnimationData {
    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub direction_count: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub still_frame: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub counterclockwise: bool,

    #[serde(default = "ro_05", skip_serializing_if = "is_ro_05")]
    pub middle_orientation: RealOrientation,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub orientation_range: f32,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(flatten)]
    parent: AnimationParameters,
}

const fn ro_05() -> RealOrientation {
    RealOrientation::new(0.5)
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_ro_05(ro: &RealOrientation) -> bool {
    *ro == ro_05()
}

impl std::ops::Deref for RotatedAnimationData {
    type Target = AnimationParameters;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for RotatedAnimationData {
    type RenderOpts = RotatedRenderOpts<AnimationRenderOpts>;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        let orientation = if self.apply_projection {
            opts.orientation.projected_orientation()
        } else {
            opts.orientation
        };

        let rot_idx = opts.override_rot_index.unwrap_or_else(|| {
            orientation_to_index(orientation, self.direction_count as u16, false)
        });

        let anim_idx = opts
            .override_anim_index
            .unwrap_or_else(|| self.anim_idx(opts.progress));

        self.fetch_scale_tint(
            scale,
            used_mods,
            image_cache,
            StripeMultiSingleSourceFetchArgs {
                index: self.frame_count * u32::from(rot_idx) + anim_idx,
                line_length: self.line_length.unwrap_or(0),
                direction_count: Some(self.direction_count),
            },
            opts.runtime_tint,
        )
    }
}

/// [`Types/RotatedAnimation8Way`](https://lua-api.factorio.com/latest/types/RotatedAnimation8Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[skip_serializing_none]
pub enum RotatedAnimation8Way {
    Struct {
        north: Box<RotatedAnimation>,
        north_east: Option<Box<RotatedAnimation>>,
        east: Option<Box<RotatedAnimation>>,
        south_east: Option<Box<RotatedAnimation>>,
        south: Option<Box<RotatedAnimation>>,
        south_west: Option<Box<RotatedAnimation>>,
        west: Option<Box<RotatedAnimation>>,
        north_west: Option<Box<RotatedAnimation>>,
    },
    Single(RotatedAnimation),
}

impl RenderableGraphics for RotatedAnimation8Way {
    type RenderOpts = DirectionalRenderOpts<RotatedRenderOpts<AnimationRenderOpts>>;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        match self {
            Self::Struct {
                north,
                north_east,
                east,
                south_east,
                south,
                south_west,
                west,
                north_west,
            } => match opts.direction {
                Direction::North => Some(north),
                Direction::NorthEast => north_east.as_ref(),
                Direction::East => east.as_ref(),
                Direction::SouthEast => south_east.as_ref(),
                Direction::South => south.as_ref(),
                Direction::SouthWest => south_west.as_ref(),
                Direction::West => west.as_ref(),
                Direction::NorthWest => north_west.as_ref(),
                _ => {
                    warn!(
                        "RotatedAnimation8Way render called with invalid direction: {:?}",
                        opts.direction
                    );
                    return None;
                }
            }
            .unwrap_or(north)
            .render(scale, used_mods, image_cache, opts),
            Self::Single(g) => g.render(scale, used_mods, image_cache, opts),
        }
    }
}

/// [`Types/RotatedSpriteFrame`](https://lua-api.factorio.com/latest/types/RotatedSpriteFrame.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct RotatedSpriteFrame {
    pub width: Option<SpriteSizeType>,
    pub height: Option<SpriteSizeType>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub x: SpriteSizeType,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub y: SpriteSizeType,
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub shift: Vector,
}

/// [`Types/RotatedSprite`](https://lua-api.factorio.com/latest/types/RotatedSprite.html)
pub type RotatedSprite = LayeredGraphic<RotatedSpriteData>;

/// [`Types/RotatedSprite`](https://lua-api.factorio.com/latest/types/RotatedSprite.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatedSpriteData {
    pub direction_count: u16,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub back_equals_front: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub apply_projection: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub counterclockwise: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub line_length: u32,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_low_quality_rotation: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: FactorioArray<RotatedSpriteFrame>,

    #[serde(flatten)]
    parent: Box<SpriteParameters<MultiSingleSource>>,
}

impl std::ops::Deref for RotatedSpriteData {
    type Target = SpriteParameters<MultiSingleSource>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl RenderableGraphics for RotatedSpriteData {
    type RenderOpts = RotatedRenderOpts;

    fn render(
        &self,
        scale: f64,
        used_mods: &mod_util::UsedMods,
        image_cache: &mut crate::ImageCache,
        opts: &Self::RenderOpts,
    ) -> Option<super::GraphicsOutput> {
        let orientation = if self.apply_projection {
            opts.orientation.projected_orientation()
        } else {
            opts.orientation
        };

        let idx = opts.override_rot_index.unwrap_or_else(|| {
            orientation_to_index(orientation, self.direction_count, self.back_equals_front)
        });

        // TODO: implement `frames` logic, have not seen it used in base / DLC

        let (img, shift) = self.fetch_scale_tint(
            scale,
            used_mods,
            image_cache,
            MultiSingleSourceFetchArgs {
                index: u32::from(idx),
                line_length: self.line_length,
            },
            opts.runtime_tint,
        )?;

        let shift = if self.rotate_shift {
            shift.rotate(orientation)
        } else {
            shift
        };

        Some((img, shift))
    }
}

fn orientation_to_index(
    orientation: RealOrientation,
    direction_count: u16,
    back_equals_front: bool,
) -> u16 {
    let orientation = if back_equals_front {
        orientation * 2.0 % 1.0
    } else {
        orientation
    };

    (f64::from(direction_count) * orientation).round() as u16 % direction_count
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RotatedRenderOpts<M = TintableRenderOpts> {
    pub orientation: RealOrientation,
    pub override_rot_index: Option<u16>,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for RotatedRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

impl<M> std::ops::DerefMut for RotatedRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M> RotatedRenderOpts<M> {
    pub const fn new(orientation: RealOrientation, more: M) -> Self {
        Self {
            orientation,
            override_rot_index: None,
            more,
        }
    }

    pub const fn new_override(override_index: u16, more: M) -> Self {
        Self {
            orientation: RealOrientation::new(0.0),
            override_rot_index: Some(override_index),
            more,
        }
    }
}

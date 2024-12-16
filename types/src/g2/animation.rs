use serde::{Deserialize, Serialize};
use serde_helper as helper;
use serde_with::skip_serializing_none;

use super::{
    DirectionalRenderOpts, LayeredGraphic, RenderLayer, RenderableGraphics, SourceProvider,
    SpriteParameters, StripeMultiSingleSource, TintableRenderOpts,
};
use crate::FactorioArray;

#[derive(Debug, Clone, Copy, Default)]
pub struct AnimationRenderOpts<M = TintableRenderOpts> {
    pub progress: f64,

    pub(crate) more: M,
}

impl<M> std::ops::Deref for AnimationRenderOpts<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.more
    }
}

impl<M> std::ops::DerefMut for AnimationRenderOpts<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.more
    }
}

impl<M> AnimationRenderOpts<M> {
    pub const fn new(progress: f64, more: M) -> Self {
        Self { progress, more }
    }
}

// TODO: truncating deserializer for arrays....
/// [`Types/AnimationFrameSequence`](https://lua-api.factorio.com/latest/types/AnimationFrameSequence.html)
pub type AnimationFrameSequence = FactorioArray<u16>;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationRunMode {
    #[default]
    Forward,
    Backward,
    ForwardThenBackward,
}

/// [`Types/AnimationParameters`](https://lua-api.factorio.com/latest/types/AnimationParameters.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct AnimationParameters<S: SourceProvider = StripeMultiSingleSource> {
    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub run_mode: AnimationRunMode,

    #[serde(default = "helper::u32_1", skip_serializing_if = "helper::is_1_u32")]
    pub frame_count: u32,

    // default of 0 gets overridden in AnimationSheet to variation_count
    pub line_length: Option<u32>,

    #[serde(default = "helper::f32_1", skip_serializing_if = "helper::is_1_f32")]
    pub animation_speed: f32,

    #[serde(
        default = "helper::f32_max",
        skip_serializing_if = "helper::is_max_f32"
    )]
    pub max_advance: f32,

    #[serde(default = "helper::u8_1", skip_serializing_if = "helper::is_1_u8")]
    pub repeat_count: u8,
    // pub dice: Option<u8>,
    // pub dice_x: Option<u8>,
    // pub dice_y: Option<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frame_sequence: AnimationFrameSequence,

    #[serde(flatten)]
    parent: Box<SpriteParameters<S>>,
}

impl<S: SourceProvider> std::ops::Deref for AnimationParameters<S> {
    type Target = SpriteParameters<S>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

/// [`Types/Animation`](https://lua-api.factorio.com/latest/types/Animation.html)
pub type Animation = LayeredGraphic<AnimationData>;

/// [`Types/Animation`](https://lua-api.factorio.com/latest/types/Animation.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationData(AnimationParameters);

impl RenderableGraphics for AnimationData {
    type RenderOpts = AnimationRenderOpts;

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

impl std::ops::Deref for AnimationData {
    type Target = AnimationParameters;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LayeredGraphic<AnimationData> {
    #[must_use]
    pub fn frame_count(&self) -> u32 {
        match self {
            Self::Layered { layers } => layers.first().map_or(1, Self::frame_count),
            Self::Data(d) => d.frame_count,
        }
    }
}

/// [`Types/AnimationElement`](https://lua-api.factorio.com/latest/types/AnimationElement.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct AnimationElement {
    #[serde(default = "rl_obj", skip_serializing_if = "is_rl_obj")]
    pub render_layer: RenderLayer,

    pub secondary_draw_order: Option<i8>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub apply_tint: bool,
    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub always_draw: bool,

    pub animation: Option<Animation>,
}

const fn rl_obj() -> RenderLayer {
    RenderLayer::Object
}

#[expect(clippy::trivially_copy_pass_by_ref)]
fn is_rl_obj(rl: &RenderLayer) -> bool {
    *rl == RenderLayer::Object
}

impl RenderableGraphics for AnimationElement {
    type RenderOpts = AnimationRenderOpts;

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

/// [`Types/Animation4Way`](https://lua-api.factorio.com/latest/types/Animation4Way.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Animation4Way {
    Struct {
        north: Box<Animation>,
        north_east: Option<Box<Animation>>,
        east: Option<Box<Animation>>,
        south_east: Option<Box<Animation>>,
        south: Option<Box<Animation>>,
        south_west: Option<Box<Animation>>,
        west: Option<Box<Animation>>,
        north_west: Option<Box<Animation>>,
    },
    Single(Animation),
}

impl RenderableGraphics for Animation4Way {
    type RenderOpts = DirectionalRenderOpts<AnimationRenderOpts>;

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

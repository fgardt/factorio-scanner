use std::num::NonZeroU32;

use mod_util::UsedMods;
use types::*;

#[derive(Debug, Clone, Default)]
pub struct RenderOpts {
    pub position: MapPosition,

    pub direction: Direction,
    pub orientation: Option<RealOrientation>,
    pub mirrored: bool,
    pub elevated: bool,

    pub variation: Option<NonZeroU32>,

    pub pickup_position: Option<Vector>,

    pub connections: Option<ConnectedDirections>,

    pub underground_in: Option<bool>,

    pub connected_gates: Vec<Direction>,
    pub draw_gate_patch: bool,

    pub arithmetic_operation: Option<ArithmeticOperation>,
    pub decider_operation: Option<Comparator>,
    pub selector_operation: Option<SelectorOperation>,

    pub runtime_tint: Option<Color>,

    pub entity_id: u64,
    pub circuit_connected: bool,
    pub logistic_connected: bool,

    pub fluid_recipe: (bool, bool),
}

// From impls for RenderOpts variants from types
impl From<&RenderOpts> for TintableRenderOpts {
    fn from(opts: &RenderOpts) -> Self {
        Self {
            runtime_tint: opts.runtime_tint,
        }
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for RotatedRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.orientation.unwrap_or_default(), opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for DirectionalRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.direction, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for VariationRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        #[expect(unsafe_code)]
        Self::new(
            opts.variation
                .unwrap_or(unsafe { NonZeroU32::new_unchecked(1) }),
            opts.into(),
        )
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for AnimationRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(0.0, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for LocationalRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.position, opts.into())
    }
}

impl<'a, M: From<&'a RenderOpts>> From<&'a RenderOpts> for ConnectedRenderOpts<M> {
    fn from(opts: &'a RenderOpts) -> Self {
        Self::new(opts.connections, opts.into())
    }
}

pub type RenderOutput = Option<()>;

pub trait Renderable {
    fn render(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> RenderOutput;

    fn fluid_box_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &RenderOpts) -> Vec<(MapPosition, Direction)> {
        Vec::with_capacity(0)
    }

    fn recipe_visible(&self) -> bool {
        false
    }

    fn render_debug(
        &self,
        options: &RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
    ) {
        // empty default impl
    }
}

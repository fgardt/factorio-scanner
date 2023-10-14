use serde::{Deserialize, Serialize};

use super::EntityWithOwnerPrototype;
use crate::data_raw::types::*;

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipePrototype(EntityWithOwnerPrototype<PipeData>);

impl super::Renderable for PipePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeData {
    pub fluid_box: FluidBox,
    pub horizontal_window_bounding_box: BoundingBox,
    pub vertical_window_bounding_box: BoundingBox,
    pub pictures: PipePictures,
}

impl super::Renderable for PipeData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PipePictures {
    pub straight_vertical_single: Sprite,
    pub straight_vertical: Sprite,
    pub straight_vertical_window: Sprite,
    pub straight_horizontal: Sprite,
    pub straight_horizontal_window: Sprite,
    pub corner_up_right: Sprite,
    pub corner_up_left: Sprite,
    pub corner_down_right: Sprite,
    pub corner_down_left: Sprite,
    pub t_up: Sprite,
    pub t_down: Sprite,
    pub t_right: Sprite,
    pub t_left: Sprite,
    pub cross: Sprite,
    pub ending_up: Sprite,
    pub ending_down: Sprite,
    pub ending_right: Sprite,
    pub ending_left: Sprite,
    pub horizontal_window_background: Sprite,
    pub vertical_window_background: Sprite,
    pub fluid_background: Sprite,
    pub low_temperature_flow: Sprite,
    pub middle_temperature_flow: Sprite,
    pub high_temperature_flow: Sprite,
    pub gas_flow: Animation,
}

/// [`Prototypes/InfinityPipePrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityPipePrototype(EntityWithOwnerPrototype<InfinityPipeData>);

impl super::Renderable for InfinityPipePrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/InfinityPipePrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityPipeData {
    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(flatten)]
    pub parent: PipeData,
}

impl super::Renderable for InfinityPipeData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundPrototype(EntityWithOwnerPrototype<PipeToGroundData>);

impl super::Renderable for PipeToGroundPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundData {
    pub fluid_box: FluidBox,
    pub pictures: PipeToGroundPictures,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_fluid_icon_override: bool,
}

impl super::Renderable for PipeToGroundData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundPictures {
    pub down: Sprite,
    pub up: Sprite,
    pub left: Sprite,
    pub right: Sprite,
}

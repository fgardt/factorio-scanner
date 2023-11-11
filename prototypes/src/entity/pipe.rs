use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
pub type PipePrototype = EntityWithOwnerPrototype<PipeData>;

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeData {
    pub fluid_box: FluidBox,
    pub horizontal_window_bounding_box: BoundingBox,
    pub vertical_window_bounding_box: BoundingBox,
    pub pictures: PipePictures,
}

impl super::Renderable for PipeData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let res = match options.connections.unwrap_or_default() {
            super::ConnectedDirections::None => &self.pictures.straight_vertical_single,
            super::ConnectedDirections::Up => &self.pictures.ending_up,
            super::ConnectedDirections::Down => &self.pictures.ending_down,
            super::ConnectedDirections::Left => &self.pictures.ending_left,
            super::ConnectedDirections::Right => &self.pictures.ending_right,
            super::ConnectedDirections::UpDown => &self.pictures.straight_vertical,
            super::ConnectedDirections::UpLeft => &self.pictures.corner_up_left,
            super::ConnectedDirections::UpRight => &self.pictures.corner_up_right,
            super::ConnectedDirections::DownLeft => &self.pictures.corner_down_left,
            super::ConnectedDirections::DownRight => &self.pictures.corner_down_right,
            super::ConnectedDirections::LeftRight => &self.pictures.straight_horizontal,
            super::ConnectedDirections::UpDownLeft => &self.pictures.t_left,
            super::ConnectedDirections::UpDownRight => &self.pictures.t_right,
            super::ConnectedDirections::UpLeftRight => &self.pictures.t_up,
            super::ConnectedDirections::DownLeftRight => &self.pictures.t_down,
            super::ConnectedDirections::All => &self.pictures.cross,
        }
        .render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
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
pub type InfinityPipePrototype = EntityWithOwnerPrototype<InfinityPipeData>;

/// [`Prototypes/InfinityPipePrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityPipeData {
    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(flatten)]
    pub parent: PipeData,
}

impl Deref for InfinityPipeData {
    type Target = PipeData;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl super::Renderable for InfinityPipeData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }
}

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
pub type PipeToGroundPrototype = EntityWithOwnerPrototype<PipeToGroundData>;

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundData {
    pub fluid_box: FluidBox,
    pub pictures: PipeToGroundPictures,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub draw_fluid_icon_override: bool,
}

impl super::Renderable for PipeToGroundData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        let res = match options.direction {
            Direction::North => &self.pictures.up,
            Direction::East => &self.pictures.right,
            Direction::South => &self.pictures.down,
            Direction::West => &self.pictures.left,
            _ => unimplemented!("PipeToGround only supports cardinal directions"),
        }
        .render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundPictures {
    pub down: Sprite,
    pub up: Sprite,
    pub left: Sprite,
    pub right: Sprite,
}

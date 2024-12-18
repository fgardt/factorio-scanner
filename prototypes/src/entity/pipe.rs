use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, with_suffix};

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, FluidBoxEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
pub type PipePrototype = EntityWithOwnerPrototype<FluidBoxEntityData<PipeData>>;

/// [`Prototypes/PipePrototype`](https://lua-api.factorio.com/latest/prototypes/PipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeData {
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
    ) -> super::RenderOutput {
        let pictures = self.pictures.normal.as_ref();
        let res = match options.connections.unwrap_or_default() {
            super::ConnectedDirections::None => pictures.straight_vertical_single.as_ref(),
            super::ConnectedDirections::Up => pictures.ending_up.as_ref(),
            super::ConnectedDirections::Down => pictures.ending_down.as_ref(),
            super::ConnectedDirections::Left => pictures.ending_left.as_ref(),
            super::ConnectedDirections::Right => pictures.ending_right.as_ref(),
            super::ConnectedDirections::UpDown => pictures.straight_vertical.as_ref(),
            super::ConnectedDirections::UpLeft => pictures.corner_up_left.as_ref(),
            super::ConnectedDirections::UpRight => pictures.corner_up_right.as_ref(),
            super::ConnectedDirections::DownLeft => pictures.corner_down_left.as_ref(),
            super::ConnectedDirections::DownRight => pictures.corner_down_right.as_ref(),
            super::ConnectedDirections::LeftRight => pictures.straight_horizontal.as_ref(),
            super::ConnectedDirections::UpDownLeft => pictures.t_left.as_ref(),
            super::ConnectedDirections::UpDownRight => pictures.t_right.as_ref(),
            super::ConnectedDirections::UpLeftRight => pictures.t_up.as_ref(),
            super::ConnectedDirections::DownLeftRight => pictures.t_down.as_ref(),
            super::ConnectedDirections::All => pictures.cross.as_ref(),
        }
        .and_then(|s| {
            s.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

/// [`Types/PipePictures`](https://lua-api.factorio.com/latest/types/PipePictures.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PipePictures {
    #[serde(flatten)]
    pub normal: Box<PipePicturesGroup>,
    #[serde(flatten, with = "suffix_frozen")]
    pub frozen: Box<PipePicturesGroup>,
    #[serde(flatten, with = "suffix_visualization")]
    pub visualization: Box<PipePicturesGroup>,
    #[serde(flatten, with = "suffix_disabled_visualization")]
    pub disabled_visualization: Box<PipePicturesGroup>,

    pub horizontal_window_background: Option<Sprite>,
    pub vertical_window_background: Option<Sprite>,
    pub fluid_background: Option<Sprite>,
    pub low_temperature_flow: Option<Sprite>,
    pub middle_temperature_flow: Option<Sprite>,
    pub high_temperature_flow: Option<Sprite>,
    pub gas_flow: Option<Animation>,
}

with_suffix!(suffix_frozen "_frozen");
with_suffix!(suffix_visualization "_visualization");
with_suffix!(suffix_disabled_visualization "_disabled_visualization");

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PipePicturesGroup {
    pub straight_vertical_single: Option<Sprite>,
    pub straight_vertical: Option<Sprite>,
    pub straight_vertical_window: Option<Sprite>,
    pub straight_horizontal: Option<Sprite>,
    pub straight_horizontal_window: Option<Sprite>,
    pub corner_up_right: Option<Sprite>,
    pub corner_up_left: Option<Sprite>,
    pub corner_down_right: Option<Sprite>,
    pub corner_down_left: Option<Sprite>,
    pub t_up: Option<Sprite>,
    pub t_down: Option<Sprite>,
    pub t_right: Option<Sprite>,
    pub t_left: Option<Sprite>,
    pub cross: Option<Sprite>,
    pub ending_up: Option<Sprite>,
    pub ending_down: Option<Sprite>,
    pub ending_right: Option<Sprite>,
    pub ending_left: Option<Sprite>,
}

/// [`Prototypes/InfinityPipePrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityPipePrototype.html)
pub type InfinityPipePrototype = EntityWithOwnerPrototype<FluidBoxEntityData<InfinityPipeData>>;

/// [`Prototypes/InfinityPipePrototype`](https://lua-api.factorio.com/latest/prototypes/InfinityPipePrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct InfinityPipeData {
    #[serde(default = "GuiMode::all", skip_serializing_if = "GuiMode::is_all")]
    pub gui_mode: GuiMode,

    #[serde(flatten)]
    parent: PipeData,
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
    ) -> super::RenderOutput {
        self.parent
            .render(options, used_mods, render_layers, image_cache)
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        self.parent.fluid_box_connections(options)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<MapPosition> {
        self.parent.heat_buffer_connections(options)
    }
}

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
pub type PipeToGroundPrototype = EntityWithOwnerPrototype<FluidBoxEntityData<PipeToGroundData>>;

/// [`Prototypes/PipeToGroundPrototype`](https://lua-api.factorio.com/latest/prototypes/PipeToGroundPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct PipeToGroundData {
    pub pictures: Option<Sprite4Way>,
    pub frozen_patch: Option<Sprite4Way>,
    pub visualization: Option<Sprite4Way>,
    pub disabled_visualization: Option<Sprite4Way>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub draw_fluid_icon_override: bool,
}

impl super::Renderable for PipeToGroundData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.pictures.as_ref().and_then(|p| {
            p.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        })?;

        render_layers.add_entity(res, &options.position);

        Some(())
    }
}

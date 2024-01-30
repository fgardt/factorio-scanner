use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/TrainStopPrototype`](https://lua-api.factorio.com/latest/prototypes/TrainStopPrototype.html)
pub type TrainStopPrototype = EntityWithOwnerPrototype<WireEntityData<TrainStopData>>;

/// [`Prototypes/TrainStopPrototype`](https://lua-api.factorio.com/latest/prototypes/TrainStopPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStopData {
    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub animation_ticks_per_frame: u32,

    pub rail_overlay_animations: Option<Animation4Way>,
    pub animations: Option<Animation4Way>,
    pub top_animations: Option<Animation4Way>,

    pub default_train_stopped_signal: Option<SignalIDConnector>,
    pub default_trains_count_signal: Option<SignalIDConnector>,
    pub default_trains_limit_signal: Option<SignalIDConnector>,

    pub color: Option<Color>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub chart_name: bool,

    pub light1: Option<TrainStopLight>,
    pub light2: Option<TrainStopLight>,

    pub drawing_boxes: Option<TrainStopDrawingBoxes>,
    // TODO: overrides build_grid_size to 2
}

impl super::Renderable for TrainStopData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let mut empty = true;

        if let Some(rail) = self.rail_overlay_animations.as_ref().and_then(|r| {
            r.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(
                rail,
                &options.position,
                crate::InternalRenderLayer::RailBackplate,
            );
        }

        if let Some(anim) = self.animations.as_ref().and_then(|a| {
            a.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add_entity(anim, &options.position);
        }

        if let Some(top_anim) = self.top_animations.as_ref().and_then(|t| {
            t.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
        }) {
            empty = false;

            render_layers.add(
                top_anim,
                &options.position,
                crate::InternalRenderLayer::AboveEntity,
            );
        }

        let l1 = self
            .light1
            .as_ref()
            .and_then(|l| l.render(options, used_mods, render_layers, image_cache));

        let l2 = self
            .light2
            .as_ref()
            .and_then(|l| l.render(options, used_mods, render_layers, image_cache));

        if empty && l1.is_none() && l2.is_none() {
            None
        } else {
            Some(())
        }
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStopDrawingBoxes {
    pub north: BoundingBox,
    pub east: BoundingBox,
    pub south: BoundingBox,
    pub west: BoundingBox,
}

/// [`Types/TrainStopLight`](https://lua-api.factorio.com/latest/types/TrainStopLight.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainStopLight {
    pub picture: Sprite4Way,
    pub red_picture: Sprite4Way,
    pub light: LightDefinition,
}

impl super::Renderable for TrainStopLight {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let res = self.picture.render(
            render_layers.scale(),
            used_mods,
            image_cache,
            &options.into(),
        )?;

        render_layers.add(
            res,
            &options.position,
            crate::InternalRenderLayer::AboveEntity,
        );

        Some(())
    }

    fn fluid_box_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }

    fn heat_buffer_connections(&self, options: &super::RenderOpts) -> Vec<types::MapPosition> {
        Vec::with_capacity(0)
    }
}

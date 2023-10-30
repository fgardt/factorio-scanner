use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/StorageTankPrototype`](https://lua-api.factorio.com/latest/prototypes/StorageTankPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageTankPrototype(EntityWithOwnerPrototype<StorageTankData>);

impl super::Renderable for StorageTankPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/StorageTankPrototype`](https://lua-api.factorio.com/latest/prototypes/StorageTankPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTankData {
    pub fluid_box: FluidBox,
    pub window_bounding_box: BoundingBox,
    pub pictures: StorageTankPictures,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub flow_length_in_ticks: u32,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub two_direction_only: bool,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub circuit_wire_connection_points: Option<(
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
        WireConnectionPoint,
    )>,
    pub circuit_connector_sprites: Option<(
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
        CircuitConnectorSprites,
    )>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub scale_info_icons: bool,
}

impl super::Renderable for StorageTankData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        let background = self
            .pictures
            .window_background
            .render(options.factorio_dir, &options.used_mods, &options.into())
            .map(|(img, scale, (shift_x, shift_y))| {
                let (tl_x, tl_y) = self.window_bounding_box.0.as_tuple();
                let (br_x, br_y) = self.window_bounding_box.1.as_tuple();

                (
                    img,
                    scale,
                    (
                        shift_x, // + f64::from(br_x - tl_x) / 2.0, // TODO: figure out how to calculate this position
                        shift_y + tl_y + (br_y - tl_y) / 2.0,
                    ),
                )
            });

        merge_renders(&[
            background,
            self.pictures
                .picture
                .render(options.factorio_dir, &options.used_mods, &options.into()),
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageTankPictures {
    pub picture: Sprite4Way,
    pub window_background: Sprite,
    pub fluid_background: Sprite,
    pub flow_sprite: Sprite,
    pub gas_flow: Animation,
}

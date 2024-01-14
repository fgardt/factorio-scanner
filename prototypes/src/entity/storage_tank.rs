use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/StorageTankPrototype`](https://lua-api.factorio.com/latest/prototypes/StorageTankPrototype.html)
pub type StorageTankPrototype = EntityWithOwnerPrototype<StorageTankData>;

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
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        let background = self
            .pictures
            .window_background
            .render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &options.into(),
            )
            .map(|(img, shift)| {
                let (shift_x, shift_y) = shift.as_tuple();
                let (tl_x, tl_y) = self.window_bounding_box.0.as_tuple();
                let (br_x, br_y) = self.window_bounding_box.1.as_tuple();

                (
                    img,
                    (
                        shift_x, // + f64::from(br_x - tl_x) / 2.0, // TODO: figure out how to calculate this position
                        shift_y + tl_y + (br_y - tl_y) / 2.0,
                    )
                        .into(),
                )
            });

        let res = merge_renders(
            &[
                background,
                self.pictures.picture.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                ),
            ],
            render_layers.scale(),
        )?;

        render_layers.add_entity(res, &options.position);

        Some(())
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

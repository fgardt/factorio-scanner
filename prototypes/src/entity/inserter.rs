use image::{DynamicImage, GenericImageView};
use imageproc::geometric_transformations::{self, rotate_about_center};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
pub type InserterPrototype = EntityWithOwnerPrototype<InserterData>;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct InserterData {
    pub extension_speed: f64,
    pub rotation_speed: f64,
    pub insert_position: Vector,
    pub pickup_position: Vector,

    pub platform_picture: Sprite4Way,
    pub hand_base_picture: Sprite,
    pub hand_open_picture: Sprite,
    pub hand_closed_picture: Sprite,
    pub hand_base_shadow: Sprite,
    pub hand_open_shadow: Sprite,
    pub hand_closed_shadow: Sprite,

    pub energy_source: AnyEnergySource,
    pub energy_per_movement: Option<Energy>,
    pub energy_per_rotation: Option<Energy>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub stack: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_custom_vectors: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_burner_leech: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_held_item: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub use_easter_egg: bool, // can the inserter fish or not?

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub filter_count: u8,

    #[serde(
        default = "helper::f64_075",
        skip_serializing_if = "helper::is_075_f64"
    )]
    pub hand_size: f64,

    #[serde(default, skip_serializing_if = "helper::is_0_f64")]
    pub circuit_wire_max_distance: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_copper_wires: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_circuit_wires: bool,

    pub default_stack_control_input_signal: Option<SignalIDConnector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_inserter_arrow: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub chase_belt_items: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub stack_size_bonus: u8,

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
}

impl super::Renderable for InserterData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> crate::RenderOutput {
        const TILE_RES: f64 = 32.0;

        let direction = options.direction;

        let hand = self
            .hand_open_picture
            .render(used_mods, image_cache, &options.into())
            .map(|(img, scale, shift)| {
                let raw_pickup_pos = options.pickup_position.unwrap_or(self.pickup_position);
                let pickup_pos = direction.rotate_vector(raw_pickup_pos);

                let length = pickup_pos.x().hypot(pickup_pos.y());
                let angle = pickup_pos.y().atan2(pickup_pos.x()) + (std::f64::consts::PI / 2.0);

                let (width, height) = img.dimensions();
                let diagonal = f64::from(width).hypot(f64::from(height));

                let size = diagonal * length;
                let stretch_lentgh = f64::from(height) * length;
                let mut hand = DynamicImage::new_rgba8(size.round() as u32, size.round() as u32);
                image::imageops::overlay(
                    &mut hand,
                    &img.resize_exact(
                        width,
                        stretch_lentgh.round() as u32,
                        image::imageops::FilterType::Nearest,
                    ),
                    (size / 2.0 - f64::from(width) / 2.0).round() as i64,
                    (size / 2.0 - stretch_lentgh / 2.0).round() as i64,
                );

                let rotated_hand = rotate_about_center(
                    &hand.to_rgba8(),
                    angle as f32,
                    geometric_transformations::Interpolation::Nearest,
                    image::Rgba([0, 0, 0, 0]),
                );

                let shift_amount = stretch_lentgh / 2.0 / (TILE_RES / scale);

                (
                    rotated_hand.into(),
                    scale,
                    (shift_amount * angle.sin(), shift_amount * -angle.cos()).into(),
                )
            });

        let platform_options = &super::RenderOpts {
            direction: direction.flip(),
            ..options.clone()
        };

        merge_renders(&[
            self.platform_picture
                .render(used_mods, image_cache, &platform_options.into()),
            hand,
        ])
    }
}

use image::{DynamicImage, GenericImageView};
use imageproc::geometric_transformations::{self, rotate_about_center};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::{EnergyEntityData, EntityWithOwnerPrototype, WireEntityData};
use mod_util::UsedMods;
use types::*;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
pub type InserterPrototype =
    EntityWithOwnerPrototype<WireEntityData<EnergyEntityData<InserterData>>>;

/// [`Prototypes/InserterPrototype`](https://lua-api.factorio.com/latest/prototypes/InserterPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct InserterData {
    pub extension_speed: f64,
    pub rotation_speed: f64,

    #[serde(default = "helper::f64_07", skip_serializing_if = "helper::is_07_f64")]
    pub starting_distance: f64,

    pub insert_position: Vector,
    pub pickup_position: Vector,

    pub platform_picture: Option<Sprite4Way>,
    pub platform_frozen: Option<Sprite4Way>,
    pub hand_base_picture: Option<Sprite>,
    pub hand_open_picture: Option<Sprite>,
    pub hand_closed_picture: Option<Sprite>,
    pub hand_base_frozen: Option<Sprite>,
    pub hand_open_frozen: Option<Sprite>,
    pub hand_closed_frozen: Option<Sprite>,
    pub hand_base_shadow: Option<Sprite>,
    pub hand_open_shadow: Option<Sprite>,
    pub hand_closed_shadow: Option<Sprite>,

    pub energy_per_movement: Option<Energy>,
    pub energy_per_rotation: Option<Energy>,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub bulk: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_custom_vectors: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub allow_burner_leech: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_held_item: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub use_easter_egg: bool, // can the inserter fish or not?

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub grab_less_to_match_belt_stack: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub wait_for_full_hand: bool,

    #[serde(default, skip_serializing_if = "helper::is_default")]
    pub enter_drop_mode_if_held_stack_spoiled: bool,

    #[serde(
        default = "helper::u8_1",
        skip_serializing_if = "helper::is_1_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub max_belt_stack_size: u8,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub filter_count: u8,

    #[serde(
        default = "helper::f64_075",
        skip_serializing_if = "helper::is_075_f64"
    )]
    pub hand_size: f64,

    pub default_stack_control_input_signal: Option<SignalIDConnector>,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub draw_inserter_arrow: bool,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub chases_belt_items: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_default",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub stack_size_bonus: u8,
}

impl super::Renderable for InserterData {
    fn render(
        &self,
        options: &super::RenderOpts,
        used_mods: &UsedMods,
        render_layers: &mut crate::RenderLayerBuffer,
        image_cache: &mut ImageCache,
    ) -> super::RenderOutput {
        const TILE_RES: f64 = 32.0;

        let direction = options.direction;

        let hand = self
            .hand_open_picture
            .as_ref()
            .and_then(|hop| {
                hop.render(
                    render_layers.scale(),
                    used_mods,
                    image_cache,
                    &options.into(),
                )
            })
            .and_then(|(img, shift)| {
                let pickup_pos = self.get_pickup_position(direction, options.pickup_position);

                let length = pickup_pos.x().hypot(pickup_pos.y()) - 0.25;
                let angle = pickup_pos.y().atan2(pickup_pos.x()) + std::f64::consts::FRAC_PI_2;

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

                let (w, h) = hand.dimensions();
                if w == 0 || h == 0 {
                    return None;
                }

                let rotated_hand = rotate_about_center(
                    &hand.to_rgba8(),
                    angle as f32,
                    geometric_transformations::Interpolation::Nearest,
                    image::Rgba([0, 0, 0, 0]),
                );

                let shift_amount = stretch_lentgh / 2.0 / (TILE_RES / render_layers.scale());

                Some((
                    rotated_hand.into(),
                    (shift_amount * angle.sin(), shift_amount * -angle.cos()).into(),
                ))
            });

        let mut empty = true;
        if let Some(hand) = hand {
            empty = false;
            render_layers.add(
                hand,
                &options.position,
                RenderLayer::HigherObjectUnder, // TODO: is this sensible?
            );
        }

        let platform_options = &super::RenderOpts {
            direction: direction.flip(),
            ..options.clone()
        };

        let platform = self.platform_picture.as_ref().and_then(|pp| {
            pp.render(
                render_layers.scale(),
                used_mods,
                image_cache,
                &platform_options.into(),
            )
        });

        if let Some(platform) = platform {
            empty = false;
            render_layers.add_entity(platform, &options.position);
        }

        if empty {
            None
        } else {
            Some(())
        }
    }
}

impl InserterData {
    #[must_use]
    pub fn get_pickup_position(&self, direction: Direction, custom: Option<Vector>) -> Vector {
        custom.unwrap_or_else(|| direction.rotate_vector(self.pickup_position))
    }

    #[must_use]
    pub fn get_insert_position(&self, direction: Direction, custom: Option<Vector>) -> Vector {
        custom.unwrap_or_else(|| direction.rotate_vector(self.insert_position))
    }
}

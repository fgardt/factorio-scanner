use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use serde_helper as helper;

use super::EntityWithOwnerPrototype;
use types::*;

/// [`Prototypes/ArtilleryTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryTurretPrototype.html)
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtilleryTurretPrototype(EntityWithOwnerPrototype<ArtilleryTurretData>);

impl super::Renderable for ArtilleryTurretPrototype {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        self.0.render(options)
    }
}

/// [`Prototypes/ArtilleryTurretPrototype`](https://lua-api.factorio.com/latest/prototypes/ArtilleryTurretPrototype.html)
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtilleryTurretData {
    pub gun: ItemID,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub inventory_size: ItemStackIndex,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub ammo_stack_limit: ItemCountType,

    #[serde(deserialize_with = "helper::truncating_deserializer")]
    pub automated_ammo_count: ItemCountType,
    pub turret_rotation_speed: f64,
    pub manual_range_modifier: f64,

    #[serde(default = "helper::bool_true", skip_serializing_if = "Clone::clone")]
    pub alert_when_attacking: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub disable_automatic_firing: bool,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u8",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub base_picture_secondary_draw_order: u8,

    pub base_picture_render_layer: Option<RenderLayer>,
    pub base_shift: Option<Vector>,
    pub base_picture: Option<Animation4Way>,
    pub cannon_base_pictures: Option<RotatedSprite>,
    pub cannon_barrel_pictures: Option<RotatedSprite>,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub turn_after_shooting_cooldown: u16,

    #[serde(
        default,
        skip_serializing_if = "helper::is_0_u16",
        deserialize_with = "helper::truncating_deserializer"
    )]
    pub cannon_parking_frame_count: u16,

    #[serde(default = "helper::f64_1", skip_serializing_if = "helper::is_1_f64")]
    pub cannon_parking_speed: f64,

    #[serde(flatten)]
    pub cannon_barrel_recoil_shiftings: Option<ArtilleryTurretCannonBarrelShiftings>,
    // not implemented
    // pub rotating_sound: Option<InterruptibleSound>,
    // pub rotating_stopped_sound: Option<Sound>,
}

impl super::Renderable for ArtilleryTurretData {
    fn render(&self, options: &super::RenderOpts) -> Option<GraphicsOutput> {
        merge_renders(&[
            self.base_picture
                .as_ref()
                .and_then(|a| a.render(options.factorio_dir, &options.used_mods, &options.into())),
            self.cannon_barrel_pictures
                .as_ref()
                .and_then(|s| s.render(options.factorio_dir, &options.used_mods, &options.into())),
            self.cannon_base_pictures
                .as_ref()
                .and_then(|s| s.render(options.factorio_dir, &options.used_mods, &options.into())),
        ])
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ArtilleryTurretCannonBarrelShiftings {
    #[serde(
        default,
        rename = "cannon_barrel_recoil_shiftings",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shiftings: Vec<Vector3D>,

    #[serde(
        default,
        rename = "cannon_barrel_recoil_shiftings_load_correction_matrix",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub load_correction_matrix: Vec<Vector3D>,

    #[serde(rename = "cannon_barrel_light_direction")]
    pub light_direction: Option<Vector3D>,
}
